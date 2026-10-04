//! Counts the CPU instructions (and, with `perf`, cycles) each `FastPFor` implementation
//! executes on the same workload, and prints summary tables.
//!
//! Compares C++ and Rust, the `Portable` and `Auto` (SIMD) Rust kernels, and the sequential
//! (C++ `FastPFor`) and interleaved (C++ `SIMDFastPFor`) wire formats, for `u32` and `u64`
//! values with 128- and 256-value blocks. C++ has no interleaved `u64` codec.
//!
//! The binary re-runs itself once per codec and operation, and only the code inside `measured`
//! is counted:
//! - `just bench instructions`: under valgrind's callgrind. Exact and repeatable, works anywhere
//!   valgrind does, but slow and instructions only.
//! - `just bench perf`: under `perf stat`, with the CPU's hardware counters. Full speed, and also
//!   counts cycles, but needs `kernel.perf_event_paranoid <= 2`.
//!
//! `--save <file>` writes the raw counts, and `--baseline <file>` adds tables comparing against
//! counts saved earlier, e.g. from the base commit of a pull request.

use std::collections::HashMap;
use std::hint::black_box;
use std::io::{Read as _, Write as _};
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{env, fmt, fs, thread};

#[cfg(feature = "cpp")]
use fastpfor::BlockCodec64;
#[cfg(feature = "cpp")]
use fastpfor::cpp::{CppFastPFor128, CppFastPFor256, CppSimdFastPFor128, CppSimdFastPFor256};
use fastpfor::{AnyLenCodec, Auto, FastPForCodec, Interleaved, Portable, Sequential};
use rand::rngs::StdRng;
use rand::{RngExt as _, SeedableRng as _};

/// Values per workload: two full pages of 65536 plus a tail that is not a whole block.
const N_VALUES: usize = 2 * 65536 + 100;

/// Measured calls per run with `perf`, to average out cycle noise. Callgrind runs one.
const PERF_REPEATS: usize = 20;

/// Environment variables through which the `perf` driver passes its control FIFOs.
const PERF_CTL_ENV: &str = "FASTPFOR_PERF_CTL";
const PERF_ACK_ENV: &str = "FASTPFOR_PERF_ACK";

/// Optional CPU to pin `perf` runs to, for steadier cycle counts.
const CPU_ENV: &str = "FASTPFOR_BENCH_CPU";

/// A codec of either value width, as seen by the workload.
trait Codec<T>: Default {
    fn encode(&mut self, input: &[T], out: &mut Vec<u32>);
    fn decode(&mut self, input: &[u32], out: &mut Vec<T>, len: usize);
}

/// Adapts an [`AnyLenCodec`]: every Rust codec, and the `u32` C++ codecs.
#[derive(Default)]
struct AnyLen<C>(C);

impl<C: AnyLenCodec> Codec<C::Elem> for AnyLen<C> {
    fn encode(&mut self, input: &[C::Elem], out: &mut Vec<u32>) {
        self.0.encode(input, out).expect("encode failed");
    }
    fn decode(&mut self, input: &[u32], out: &mut Vec<C::Elem>, len: usize) {
        let len = u32::try_from(len).expect("length fits in u32");
        self.0.decode(input, out, Some(len)).expect("decode failed");
    }
}

/// Adapts a C++ codec's 64-bit path.
#[cfg(feature = "cpp")]
#[derive(Default)]
struct Wide<C>(C);

#[cfg(feature = "cpp")]
impl<C: BlockCodec64 + Default> Codec<u64> for Wide<C> {
    fn encode(&mut self, input: &[u64], out: &mut Vec<u32>) {
        self.0.encode64(input, out).expect("encode failed");
    }
    fn decode(&mut self, input: &[u32], out: &mut Vec<u64>, _len: usize) {
        self.0.decode64(input, out).expect("decode failed");
    }
}

/// The values of one workload: a mix of the patterns the criterion benchmarks use.
trait Workload: Copy + PartialEq + fmt::Debug + Default {
    fn workload() -> Vec<Self>;
}

impl Workload for u32 {
    fn workload() -> Vec<Self> {
        let mut rng = StdRng::seed_from_u64(456);
        let part = N_VALUES / 5;
        let mut base = 0;
        let mut data: Vec<Self> = Vec::with_capacity(N_VALUES);
        // Small uniform values.
        data.extend((0..part).map(|_| rng.random_range(0..1000)));
        // Clustered values.
        data.extend((0..part).map(|_| {
            if rng.random_bool(0.1) {
                base = rng.random_range(0..1000);
            }
            base + rng.random_range(0..10)
        }));
        // Ascending values.
        data.extend((0..part as Self).map(|i| i + 1_000_000));
        // Mostly zeros with rare large values, i.e. exceptions.
        data.extend((0..part).map(|_| {
            if rng.random_bool(0.9) {
                0
            } else {
                rng.random::<Self>()
            }
        }));
        // Large uniform values.
        data.extend((data.len()..N_VALUES).map(|_| rng.random::<Self>()));
        data
    }
}

impl Workload for u64 {
    fn workload() -> Vec<Self> {
        let mut rng = StdRng::seed_from_u64(456);
        let part = N_VALUES / 5;
        let mut data: Vec<Self> = u32::workload()[..3 * part]
            .iter()
            .map(|&v| Self::from(v))
            .collect();
        // Mostly small values with rare 64-bit values, i.e. wide exceptions.
        data.extend((0..part).map(|_| {
            if rng.random_bool(0.9) {
                rng.random_range(0..1000)
            } else {
                rng.random::<Self>()
            }
        }));
        // Values wider than 32 bits.
        data.extend((data.len()..N_VALUES).map(|_| rng.random_range(0..1 << 44)));
        data
    }
}

/// The only code that is counted, including everything it calls: callgrind toggles collection
/// on entering and leaving it (`--toggle-collect`), and under `perf` it enables the counters.
#[inline(never)]
fn measured(f: &mut dyn FnMut(), repeats: usize, perf: Option<&PerfControl>) {
    if let Some(perf) = perf {
        perf.send("enable");
    }
    for _ in 0..repeats {
        f();
    }
    if let Some(perf) = perf {
        perf.send("disable");
    }
}

/// The FIFOs of `perf stat --control`, with which a run turns the counters on and off.
struct PerfControl {
    ctl: fs::File,
    ack: fs::File,
}

impl PerfControl {
    fn from_env() -> Option<Self> {
        let ctl = env::var_os(PERF_CTL_ENV)?;
        let ack = env::var_os(PERF_ACK_ENV)?;
        Some(Self {
            ctl: fs::OpenOptions::new()
                .write(true)
                .open(ctl)
                .expect("open perf ctl fifo"),
            ack: fs::File::open(ack).expect("open perf ack fifo"),
        })
    }

    /// Sends a command and waits until `perf` has applied it.
    fn send(&self, command: &str) {
        (&self.ctl)
            .write_all(format!("{command}\n").as_bytes())
            .expect("write perf ctl fifo");
        let mut reply = Vec::new();
        let mut buf = [0u8; 16];
        while !reply.windows(4).any(|w| w == b"ack\n") {
            let n = (&self.ack).read(&mut buf).expect("read perf ack fifo");
            assert!(n > 0, "perf closed the ack fifo");
            reply.extend_from_slice(&buf[..n]);
        }
    }
}

/// Runs one operation of codec `C` with a warm-up call first, so that only steady-state work
/// (no first-call allocation growth or CPU feature detection) is counted.
fn run<T: Workload, C: Codec<T>>(op: Op, repeats: usize) {
    let data = T::workload();
    let perf = PerfControl::from_env();
    let perf = perf.as_ref();
    let mut codec = C::default();
    let mut encoded = Vec::new();
    codec.encode(&data, &mut encoded);
    let mut decoded = Vec::new();
    match op {
        Op::Encode => {
            measured(
                &mut || {
                    encoded.clear();
                    codec.encode(black_box(&data), &mut encoded);
                },
                repeats,
                perf,
            );
        }
        Op::Decode => {
            codec.decode(&encoded, &mut decoded, data.len());
            measured(
                &mut || {
                    decoded.clear();
                    codec.decode(black_box(&encoded), &mut decoded, data.len());
                },
                repeats,
                perf,
            );
            assert_eq!(decoded, data, "roundtrip mismatch");
        }
    }
    black_box((encoded, decoded));
}

#[derive(Clone, Copy, PartialEq)]
enum Op {
    Encode,
    Decode,
}

impl Op {
    fn name(self) -> &'static str {
        match self {
            Self::Encode => "encode",
            Self::Decode => "decode",
        }
    }
}

/// Implementation columns of the summary table.
const IMPLS: [&str; 3] = ["Portable", "Auto", "C++"];

struct Case {
    layout: &'static str,
    width: u32,
    block: u32,
    implementation: &'static str,
    run: fn(Op, usize),
}

impl Case {
    fn id(&self) -> String {
        let implementation = self.implementation.replace([' ', '+'], "");
        format!(
            "{}-u{}-{}-{implementation}",
            self.layout, self.width, self.block
        )
    }
}

macro_rules! rust_cases {
    ($($layout:ident $t:ident $n:literal),* $(,)?) => {
        vec![$(
            Case {
                layout: stringify!($layout),
                width: $t::BITS,
                block: $n,
                implementation: IMPLS[0],
                run: run::<$t, AnyLen<FastPForCodec<$layout, $t, $n, Portable>>>,
            },
            Case {
                layout: stringify!($layout),
                width: $t::BITS,
                block: $n,
                implementation: IMPLS[1],
                run: run::<$t, AnyLen<FastPForCodec<$layout, $t, $n, Auto>>>,
            },
        )*]
    };
}

fn cases() -> Vec<Case> {
    #[cfg_attr(
        not(feature = "cpp"),
        expect(unused_mut, reason = "C++ cases need `cpp`")
    )]
    let mut cases = rust_cases![
        Sequential u32 128,
        Sequential u32 256,
        Sequential u64 128,
        Sequential u64 256,
        Interleaved u32 128,
        Interleaved u32 256,
        Interleaved u64 128,
        Interleaved u64 256,
    ];
    #[cfg(feature = "cpp")]
    {
        let cpp = |layout, width, block, run| Case {
            layout,
            width,
            block,
            implementation: IMPLS[2],
            run,
        };
        cases.extend([
            cpp("Sequential", 32, 128, run::<u32, AnyLen<CppFastPFor128>>),
            cpp("Sequential", 32, 256, run::<u32, AnyLen<CppFastPFor256>>),
            cpp("Sequential", 64, 128, run::<u64, Wide<CppFastPFor128>>),
            cpp("Sequential", 64, 256, run::<u64, Wide<CppFastPFor256>>),
            cpp(
                "Interleaved",
                32,
                128,
                run::<u32, AnyLen<CppSimdFastPFor128>>,
            ),
            cpp(
                "Interleaved",
                32,
                256,
                run::<u32, AnyLen<CppSimdFastPFor256>>,
            ),
        ]);
    }
    cases
}

/// glibc tunables that keep `memset`/`memcpy` off `rep stosb`/`rep movsb` (x86 only, ignored elsewhere).
const NO_REP_STRINGS: &str = "glibc.cpu.x86_rep_stosb_threshold=0x7fffffffffffffff:\
                              glibc.cpu.x86_rep_movsb_threshold=0x7fffffffffffffff";

/// How runs are counted.
#[derive(Clone, Copy, PartialEq)]
enum Tool {
    Callgrind,
    Perf,
}

impl Tool {
    /// The metrics each run counts. The first, instructions, is the only one compared with a
    /// baseline: cycle counts of two runs differ by up to ~15% with no code change at all.
    fn metrics(self) -> &'static [&'static str] {
        match self {
            Self::Callgrind => &["CPU instructions"],
            Self::Perf => &["CPU instructions", "CPU cycles"],
        }
    }
}

/// Runs one case under callgrind and returns the instructions executed inside `measured`.
///
/// For C++ codecs only the C++ library call (`CompositeCodec::encodeArray`/`decodeArray`) is
/// counted: the Rust FFI wrappers around it zero-fill oversized output buffers, which would
/// otherwise dominate the count.
fn callgrind(exe: &Path, dir: &Path, case: &Case, op: Op) -> Vec<Option<u64>> {
    let out_file = dir.join(format!("{}-{}.callgrind", case.id(), op.name()));
    let status = Command::new("valgrind")
        // Callgrind counts every byte of a `rep stosb`/`rep movsb` as one instruction, so make
        // glibc's large `memset`/`memcpy` use its vector loops instead of these string instructions.
        .env("GLIBC_TUNABLES", NO_REP_STRINGS)
        .args(["--tool=callgrind", "--quiet", "--toggle-collect=*measured*"])
        .arg(format!("--callgrind-out-file={}", out_file.display()))
        .arg(exe)
        .args(["--run", &case.id(), op.name(), "1"])
        .status()
        .expect("failed to start valgrind; is it installed?");
    assert!(
        status.success(),
        "{} {} failed under valgrind",
        case.id(),
        op.name()
    );
    let annotated = Command::new("callgrind_annotate")
        .args(["--inclusive=yes", "--threshold=100"])
        .arg(&out_file)
        .output()
        .expect("failed to start callgrind_annotate");
    assert!(annotated.status.success(), "callgrind_annotate failed");
    let function = if case.implementation == IMPLS[2] {
        "FastPForLib::CompositeCodec<"
    } else {
        "instructions::measured"
    };
    let instructions = String::from_utf8_lossy(&annotated.stdout)
        .lines()
        .filter(|line| line.contains(function) && !line.contains("::~"))
        .filter_map(|line| {
            line.split_whitespace()
                .next()?
                .replace(',', "")
                .parse()
                .ok()
        })
        .max()
        .unwrap_or_else(|| panic!("no `{function}` cost in {}", out_file.display()));
    vec![Some(instructions)]
}

/// Runs one case under `perf stat` and returns the user-space instructions and cycles counted
/// while `measured` had the counters enabled, for [`PERF_REPEATS`] calls.
///
/// C++ codecs are counted with the Rust FFI wrapper, which zero-fills its output buffers.
fn perf(exe: &Path, dir: &Path, case: &Case, op: Op) -> Vec<Option<u64>> {
    let name = format!("{}-{}", case.id(), op.name());
    let (ctl, ack, out_file) = (
        dir.join(format!("{name}.ctl")),
        dir.join(format!("{name}.ack")),
        dir.join(format!("{name}.perf.csv")),
    );
    for fifo in [&ctl, &ack] {
        let _ = fs::remove_file(fifo);
        let made = Command::new("mkfifo")
            .arg(fifo)
            .status()
            .expect("run mkfifo");
        assert!(made.success(), "mkfifo {} failed", fifo.display());
    }
    let mut command = match env::var(CPU_ENV) {
        Ok(cpu) if !cpu.is_empty() => {
            let mut taskset = Command::new("taskset");
            taskset.args(["-c", &cpu, "perf"]);
            taskset
        }
        _ => Command::new("perf"),
    };
    let output = command
        .args(["stat", "-x,", "-e", "instructions:u,cycles:u", "--delay=-1"])
        .arg(format!(
            "--control=fifo:{},{}",
            ctl.display(),
            ack.display()
        ))
        .arg("-o")
        .arg(&out_file)
        .arg("--")
        .arg(exe)
        .args(["--run", &case.id(), op.name(), &PERF_REPEATS.to_string()])
        .env(PERF_CTL_ENV, &ctl)
        .env(PERF_ACK_ENV, &ack)
        .output()
        .expect("failed to start perf; is it installed?");
    assert!(
        output.status.success(),
        "{name} failed under perf:\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let csv = fs::read_to_string(&out_file).expect("perf wrote no output");
    // Hybrid CPUs report each event once per core type (`cpu_core/...`, `cpu_atom/...`): sum them.
    let total = |event: &str| {
        csv.lines()
            .filter(|line| !line.starts_with('#'))
            .map(|line| line.split(',').collect::<Vec<_>>())
            .filter(|fields| fields.get(2).is_some_and(|f| f.contains(event)))
            .filter_map(|fields| fields[0].trim().parse::<u64>().ok())
            .reduce(|a, b| a + b)
    };
    let per_call = |n: u64| n / PERF_REPEATS as u64;
    vec![
        total("instructions").map(per_call),
        total("cycles").map(per_call),
    ]
}

/// A table row: wire format, value width and block size.
type Row = (&'static str, u32, u32);

/// Raw counts of all runs: `(case id, operation) -> one count per metric`.
type Results = HashMap<(String, &'static str), Vec<Option<u64>>>;

fn measure(tool: Tool) -> Results {
    let exe = env::current_exe().expect("current exe");
    let dir = exe.with_extension(match tool {
        Tool::Callgrind => "callgrind",
        Tool::Perf => "perf",
    });
    fs::create_dir_all(&dir).expect("create output dir");

    let cases = cases();
    let jobs: Vec<(&Case, Op)> = cases
        .iter()
        .flat_map(|c| [(c, Op::Encode), (c, Op::Decode)])
        .collect();
    let results = Mutex::new(Results::new());
    let next = AtomicUsize::new(0);
    // Hardware counters are counted one run at a time, so that runs do not slow each other down.
    let threads = match tool {
        Tool::Callgrind => thread::available_parallelism().map_or(4, usize::from),
        Tool::Perf => 1,
    };
    eprintln!(
        "Counting {} runs of {N_VALUES} values ({threads} at a time)...",
        jobs.len()
    );
    thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                while let Some(&(case, op)) = jobs.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let counts = match tool {
                        Tool::Callgrind => callgrind(&exe, &dir, case, op),
                        Tool::Perf => perf(&exe, &dir, case, op),
                    };
                    let mut results = results.lock().expect("results lock");
                    results.insert((case.id(), op.name()), counts);
                }
            });
        }
    });
    eprintln!("Raw output: {}", dir.display());
    results.into_inner().expect("results lock")
}

/// Writes results as `case<TAB>operation<TAB>metric<TAB>count` lines.
fn save(results: &Results, path: &str) {
    let mut lines: Vec<String> = results
        .iter()
        .flat_map(|((id, op), counts)| {
            counts
                .iter()
                .enumerate()
                .filter_map(move |(metric, count)| {
                    Some(format!("{id}\t{op}\t{metric}\t{}", (*count)?))
                })
        })
        .collect();
    lines.sort();
    fs::write(path, lines.join("\n") + "\n").expect("write results");
}

/// Reads results written by [`save`].
fn load(path: &str) -> Results {
    let mut results = Results::new();
    for line in fs::read_to_string(path).expect("read baseline").lines() {
        let [id, op, metric, count] = line.split('\t').collect::<Vec<_>>()[..] else {
            panic!("bad baseline line: {line}");
        };
        let op = if op == Op::Encode.name() {
            Op::Encode.name()
        } else {
            Op::Decode.name()
        };
        let metric: usize = metric.parse().expect("metric index");
        let counts = results.entry((id.to_string(), op)).or_default();
        if counts.len() <= metric {
            counts.resize(metric + 1, None);
        }
        counts[metric] = Some(count.parse().expect("count"));
    }
    results
}

/// Value of the option `name` among the command-line arguments.
fn option<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    let at = args.iter().position(|a| a == name)?;
    Some(
        args.get(at + 1)
            .unwrap_or_else(|| panic!("{name} needs a value")),
    )
}

fn summarize(tool: Tool, args: &[String]) {
    let results = measure(tool);
    if let Some(path) = option(args, "--save") {
        save(&results, path);
    }
    let baseline = option(args, "--baseline").map(load);

    let cases = cases();
    let rows: Vec<Row> = ["Sequential", "Interleaved"]
        .into_iter()
        .flat_map(|layout| {
            [32, 64]
                .into_iter()
                .flat_map(move |width| [128, 256].map(|block| (layout, width, block)))
        })
        .collect();
    let id = |(layout, width, block): Row, implementation: &str| {
        cases
            .iter()
            .find(|c| {
                (c.layout, c.width, c.block, c.implementation)
                    == (layout, width, block, implementation)
            })
            .map(Case::id)
    };
    let count = |results: &Results, row: Row, implementation: &str, op: Op, metric: usize| {
        let counts = results.get(&(id(row, implementation)?, op.name()))?;
        *counts.get(metric)?
    };
    #[expect(clippy::cast_precision_loss, reason = "counts are far below 2^52")]
    let per_value = |count: u64| count as f64 / N_VALUES as f64;

    for (metric, title) in tool.metrics().iter().enumerate() {
        let best = |op: Op| {
            rows.iter()
                .flat_map(|&row| IMPLS.map(|i| count(&results, row, i, op, metric)))
                .flatten()
                .min()
        };
        let best = [best(Op::Encode), best(Op::Decode)];
        print_table(
            &format!("{title} per value ({N_VALUES} values; lower is better)"),
            &rows,
            |row, implementation, op| {
                let Some(value) = count(&results, row, implementation, op, metric) else {
                    return "n/a".to_string();
                };
                let best = best[usize::from(op == Op::Decode)].expect("a best value");
                let above = (per_value(value) / per_value(best) - 1.0) * 100.0;
                let above = if value == best {
                    "best".to_string()
                } else if above < 10.0 {
                    format!("+{above:.1}%")
                } else {
                    format!("+{above:.0}%")
                };
                format!("{:.2} ({above})", per_value(value))
            },
        );
        if let Some(baseline) = baseline.as_ref().filter(|_| metric == 0) {
            print_table(
                &format!("{title}: change from the base commit"),
                &rows,
                |row, implementation, op| {
                    let head = count(&results, row, implementation, op, metric);
                    let base = count(baseline, row, implementation, op, metric);
                    match (head, base) {
                        (Some(head), Some(base)) if head == base => "=".to_string(),
                        (Some(head), Some(base)) => {
                            let change = (per_value(head) / per_value(base) - 1.0) * 100.0;
                            if change.abs() < 0.05 {
                                "~0%".to_string()
                            } else {
                                format!("{change:+.1}%")
                            }
                        }
                        (Some(_), None) => "new".to_string(),
                        (None, Some(_)) => "removed".to_string(),
                        (None, None) => "n/a".to_string(),
                    }
                },
            );
        }
    }
    print_notes(tool, baseline.is_some());
}

/// Explains the tables.
fn print_notes(tool: Tool, has_baseline: bool) {
    println!(
        "Percentages in parentheses: how much more than the lowest value in the table, \
         compared separately for encoding and decoding."
    );
    if has_baseline {
        println!(
            "Change from the base commit: `=` same count, `~0%` within 0.05% (e.g. a different memory layout \
             of the base build steers glibc's memcpy and memset slightly differently)."
        );
    }
    println!(
        "n/a: not implemented (C++ has no interleaved u64 codec), not counted, or built without `cpp`."
    );
    match tool {
        Tool::Callgrind => println!(
            "Rust: the whole `AnyLenCodec` call. C++: the C++ library call only, without the Rust FFI wrapper."
        ),
        Tool::Perf => println!(
            "User-space hardware counts, averaged over {PERF_REPEATS} calls. \
             Rust: the whole `AnyLenCodec` call. C++: the call including the Rust FFI wrapper."
        ),
    }
    println!(
        "Sequential = C++ `FastPFor` wire format, interleaved = C++ `SIMDFastPFor` wire format."
    );
}

/// Prints a Markdown table with one row per `(layout, width, block)` and one column per
/// operation and implementation, right-aligned.
fn print_table(title: &str, rows: &[Row], cell: impl Fn(Row, &str, Op) -> String) {
    let columns: Vec<(Op, &str)> = [Op::Encode, Op::Decode]
        .into_iter()
        .flat_map(|op| IMPLS.map(|i| (op, i)))
        .collect();
    let mut table: Vec<Vec<String>> = vec![
        columns
            .iter()
            .map(|(op, i)| format!("{} {i}", op.name()))
            .collect(),
    ];
    table.extend(
        rows.iter()
            .map(|&row| columns.iter().map(|&(op, i)| cell(row, i, op)).collect()),
    );
    let widths: Vec<usize> = (0..columns.len())
        .map(|c| table.iter().map(|r| r[c].len()).max().unwrap_or(0))
        .collect();
    let line = |r: &[String]| {
        r.iter()
            .zip(&widths)
            .map(|(cell, &w)| format!("{cell:>w$}"))
            .collect::<Vec<_>>()
            .join(" | ")
    };
    println!();
    println!("{title}");
    println!();
    println!(
        "| {:<11} | Type | Block | {} |",
        "Wire format",
        line(&table[0])
    );
    let rule: Vec<String> = widths.iter().map(|&w| "-".repeat(w + 1)).collect();
    println!("|-------------|------|-------|{}:|", rule.join(":|"));
    for (&(layout, width, block), cells) in rows.iter().zip(&table[1..]) {
        println!(
            "| {layout:<11} | u{width}  | {block:>5} | {} |",
            line(cells)
        );
    }
    println!();
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.iter().position(|a| a == "--run") {
        Some(at) => {
            let (id, op, repeats) = (&args[at + 1], &args[at + 2], &args[at + 3]);
            let op = if op == "encode" {
                Op::Encode
            } else {
                Op::Decode
            };
            let case = cases()
                .into_iter()
                .find(|c| c.id() == *id)
                .expect("unknown case");
            (case.run)(op, repeats.parse().expect("repeat count"));
        }
        // `cargo test --all-targets` also runs this binary, without `--bench`: just check that every
        // case round-trips, without valgrind or perf.
        None if !args.iter().any(|a| a == "--bench") => {
            for case in cases() {
                (case.run)(Op::Decode, 1);
            }
            eprintln!("All cases round-trip; run with `cargo bench` to count instructions.");
        }
        None if args.iter().any(|a| a == "--perf") => summarize(Tool::Perf, &args),
        None => summarize(Tool::Callgrind, &args),
    }
}
