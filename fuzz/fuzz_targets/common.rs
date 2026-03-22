// The fuzz crate always enables both "rust" and "cpp" features of fastpfor.
// Items here may only be used by some binaries; suppress dead_code lint.
#![allow(dead_code)]

use fastpfor::cpp::*;
use fastpfor::{AnyLenCodec, FastPFor128, FastPFor256, JustCopy, VariableByte};
// ── Debug helper ─────────────────────────────────────────────────────────────

pub struct HexSlice<'a>(pub &'a [u32]);

impl std::fmt::Debug for HexSlice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MAX: usize = 20;
        let total = self.0.len();
        let shown = total.min(MAX);
        let mut list = f.debug_list();
        for v in &self.0[..shown] {
            list.entry(&format_args!("{v:#010x}"));
        }
        if total > MAX {
            list.entry(&format_args!(".. out of {total} total"));
        }
        list.finish()
    }
}

/// A fuzz input pairing arbitrary data with a codec selector.
#[derive(arbitrary::Arbitrary, Debug)]
pub struct FuzzInput<C> {
    pub data: Vec<u32>,
    pub codec: C,
}

pub type AnyLen = Box<dyn AnyLenCodec>;

// ── List entry type ───────────────────────────────────────────────────────────

pub type CodecEntry = (&'static str, fn() -> AnyLen);

// ── Two codec lists ──────────────────────────────────────────────────────────

/// Generates `(name, || Box::new(T::default()))` entries from a list of types.
macro_rules! codec_list {
    ($($t:ty),* $(,)?) => {
        &[
            $( (stringify!($t), || Box::new(<$t>::default())) ),*
        ]
    };
}

/// Rust codecs. Block codecs are wrapped in `CompositeCodec<_, VariableByte>`.
pub static RUST: &[CodecEntry] = codec_list!(FastPFor256, FastPFor128, VariableByte, JustCopy,);

/// C++ codecs (any-length; block codecs are already composites in the C++ library).
pub static CPP: &[CodecEntry] = codec_list!(
    CppBP32,
    CppCopy,
    CppFastBinaryPacking8,
    CppFastPFor128,
    CppFastPFor256,
    CppFastBinaryPacking16,
    CppFastBinaryPacking32,
    CppMaskedVByte,
    CppNewPFor,
    CppOptPFor,
    CppPFor2008,
    CppPFor,
    CppSimdBinaryPacking,
    CppSimdFastPFor128,
    CppSimdFastPFor256,
    CppSimdGroupSimple,
    CppSimdGroupSimpleRingBuf,
    CppSimdNewPFor,
    CppSimdOptPFor,
    CppSimdPFor,
    CppSimdSimplePFor,
    // Simple16 / Simple8b / Simple8bRle / Simple9 / Simple9Rle / SimplePFor:
    //   cannot encode arbitrary u32 values.
    // Snappy / VarIntG8iu: conditional #ifdef in C++.
    // VsEncoding: leaks memory.
    CppStreamVByte,
    CppVByte,
    CppVarInt,
    CppVarIntGb,
);

// ── Codec selector (Arbitrary) ─────────────────────────────────────────────────

/// Selects a codec by index. `idx` is wrapped modulo the list length.
/// `use_cpp` switches between [`RUST`] and [`CPP`].
#[derive(arbitrary::Arbitrary, Clone, Copy, Debug)]
pub struct AnyLenSelector {
    pub idx: u8,
    pub use_cpp: bool,
}

/// Instantiate a codec, returning `(name, codec)`.
pub fn instantiate_anylen_codec(sel: AnyLenSelector) -> (&'static str, AnyLen) {
    let list = if sel.use_cpp { CPP } else { RUST };
    let (name, make) = list[sel.idx as usize % list.len()];
    (name, make())
}

// ── Encode compare: Rust vs C++ bit-identical pairs ────────────────────────────

/// A pair of codecs that should produce bit-identical compressed output.
/// `rust` = Rust implementation, `cpp` = C++ implementation
#[derive(Clone, Copy)]
pub struct CodecPair {
    pub name: &'static str,
    pub make_rust: fn() -> AnyLen,
    pub make_cpp: fn() -> AnyLen,
}

macro_rules! codec_pair {
    ($name:expr, $rust:ty, $cpp:ty) => {
        CodecPair {
            name: $name,
            make_rust: || Box::new(<$rust>::default()),
            make_cpp: || Box::new(<$cpp>::default()),
        }
    };
    ($name:expr, $rust:ty, $cpp:ty, $cpp_alt:ty) => {
        CodecPair {
            name: $name,
            make_rust: || Box::new(<$rust>::default()),
            make_cpp: || Box::new(<$cpp>::default()),
        }
    };
}

/// Pairs of Rust and C++ codecs expected to produce bit-identical output.
pub static ENCODE_COMPARE_PAIRS: &[CodecPair] = &[
    codec_pair!("FastPFor128", FastPFor128, CppFastPFor128),
    codec_pair!("FastPFor256", FastPFor256, CppFastPFor256),
    codec_pair!("VariableByte", VariableByte, CppVarInt),
    codec_pair!("JustCopy", JustCopy, CppCopy),
];

/// Optional pair filter: if set, only the named pair is tested.
/// Set `FUZZ_PAIR=FastPFor128` or `FUZZ_ENCODE_COMPARE_PAIR=FastPFor128` to restrict.
pub fn encode_compare_pair_filter() -> Option<String> {
    std::env::var("FUZZ_PAIR")
        .ok()
        .or_else(|| std::env::var("FUZZ_ENCODE_COMPARE_PAIR").ok())
}

/// Resolve a pair index to a `CodecPair`, applying filter and alternative substitution.
pub fn resolve_encode_compare_pair(idx: u8) -> Option<CodecPair> {
    let filter = encode_compare_pair_filter();
    let pairs = ENCODE_COMPARE_PAIRS;
    let i = idx as usize % pairs.len();
    let pair = pairs[i];
    if let Some(ref f) = filter {
        if !f.eq_ignore_ascii_case(pair.name) {
            return None;
        }
    }
    Some(pair)
}

/// Instantiate both codecs for a pair, using the alternative C++ when requested.
pub fn instantiate_pair(pair: CodecPair) -> (AnyLen, AnyLen) {
    let rust_codec = (pair.make_rust)();
    let cpp_codec = (pair.make_cpp)();
    (rust_codec, cpp_codec)
}
