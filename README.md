# `FastPFor` for Rust

[![GitHub repo](https://img.shields.io/badge/github-fast--pack/FastPFOR--rs-8da0cb?logo=github)](https://github.com/fast-pack/FastPFOR-rs)
[![crates.io version](https://img.shields.io/crates/v/fastpfor)](https://crates.io/crates/fastpfor)
[![crate usage](https://img.shields.io/crates/d/fastpfor)](https://crates.io/crates/fastpfor)
[![docs.rs status](https://img.shields.io/docsrs/fastpfor)](https://docs.rs/fastpfor)
[![crates.io license](https://img.shields.io/crates/l/fastpfor)](https://github.com/fast-pack/FastPFOR-rs/blob/main/LICENSE-APACHE)
[![CI build status](https://github.com/fast-pack/FastPFOR-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/fast-pack/FastPFOR-rs/actions)
[![Codecov](https://img.shields.io/codecov/c/github/fast-pack/FastPFOR-rs)](https://app.codecov.io/gh/fast-pack/FastPFOR-rs)

A Rust implementation of [FastPFOR](https://github.com/fast-pack/FastPFor) integer compression
([Decoding billions of integers per second through vectorization, 2012](https://arxiv.org/abs/1209.2137)).

* **Pure Rust, `u32` and `u64`:** `FastPFor` codecs with 128- or 256-value blocks for both integer widths,
  in the two wire formats of the C++ library: the standard one (`FastPFor*`) and the interleaved one of its
  `SIMDFastPFor` (`FastPForInterleaved*`). Each has portable kernels and vector kernels: AVX2 (selected at runtime)
  or SSE2 on `x86_64`, NEON on `aarch64`. The Rust **decoder** is about 29% faster than the C++ version.
  The Rust code is safe except for one `unsafe` call into the AVX2 kernels, made after runtime CPU feature detection;
  the crate has `#![deny(unsafe_code)]`, with the generated C++ FFI bridge as the only other exemption.
* **Optional C++ wrappers:** the `cpp` feature wraps the original [C++ library](https://github.com/fast-pack/FastPFor),
  including its other codecs.

## Wire format

There are two formats. They are **not interchangeable**, and decoding one as the other is not detected:
the output is silently wrong. Pick one per data set.

| Format | Rust codecs | Compatible with C++ | Kernels |
|---|---|---|---|
| **Standard** | `FastPFor128`, `FastPFor256`, `FastPForWide*` (`u64`), `FastPForBlock*`, and the `FastPForSimd*` variants | `CppFastPFor128` / `CppFastPFor256`, byte-identical, `u32` and `u64` | scalar; `Simd`: AVX2 (runtime-detected) or NEON |
| **Interleaved** | `FastPForInterleaved128`, `FastPForInterleaved256`, `FastPForInterleavedWide*` (`u64`), `FastPForInterleavedBlock*` | `CppSimdFastPFor128` / `CppSimdFastPFor256`, byte-identical, `u32` only | portable; `Interleaved`: SSE2 on `x86_64` or NEON on little-endian `aarch64`, always available there; portable elsewhere |

The kernels of one format all write the same bytes: scalar and `Simd` for the standard format, portable and `Interleaved`
for the interleaved one, so encoders and decoders can be mixed freely within a format.
Tests and fuzzing check the Rust codecs byte-for-byte against the C++ library (the real `FastPFor` and `SIMDFastPFor`),
and the vector kernels against the portable ones, on `x86_64` and `aarch64`.

**How the interleaved format differs.** It is the layout of C++ `SIMDFastPFor`:

* Blocks are packed 128 values at a time into four interleaved lanes: value `i` goes to lane `i % 4`, and each lane is
  bit-packed on its own. The bulk of each exception array uses the same layout, with the remainder packed as in the
  standard format. The standard format packs consecutive values into one continuous bitstream, 32 at a time.
* The encoder's bit-width choice differs slightly (the standard one discounts exceptions that are one bit wider),
  so sizes can differ by a word or two. Neither format pads each block.
* C++ has no 64-bit `SIMDFastPFor`. The `u64` interleaved format is this crate's extension of the same layout to two
  64-bit lanes per 128-bit vector, so for `u64` there is nothing in C++ to interoperate with.

**Which to use.** Use the standard format to stay compatible with existing streams and with the C++ `FastPFor`.
The interleaved format needs only baseline vector instructions, so it needs no runtime detection, and in local
benchmarks its `u32` decoder was 10-25% faster than the AVX2 kernels of the standard format for narrow widths.

## Usage

### Rust Implementation (default)

The simplest way is `FastPFor256` — a composite codec that handles any input
length by compressing aligned 256-element blocks with `FastPForBlock256` and encoding any
leftover values with `VariableByte`.

```rust
use fastpfor::{AnyLenCodec, FastPFor256};

let mut codec = FastPFor256::default();
let input: Vec<u32> = (0..1000).collect();

let mut encoded = Vec::new();
codec.encode(&input, &mut encoded).unwrap();

let mut decoded = Vec::new();
codec.decode(&encoded, &mut decoded, None).unwrap();

assert_eq!(decoded, input);
```

For block-aligned inputs you can use the lower-level `BlockCodec` API:

```rust
use fastpfor::{BlockCodec, FastPForBlock256, slice_to_blocks};

let mut codec = FastPForBlock256::default();
let input: Vec<u32> = (0..512).collect();   // exactly 2 blocks of 256

let (blocks, remainder) = slice_to_blocks::<FastPForBlock256>(&input);
assert_eq!(blocks.len(), 2);
assert!(remainder.is_empty());

let mut encoded = Vec::new();
codec.encode_blocks(blocks, &mut encoded).unwrap();

let mut decoded = Vec::new();
codec.decode_blocks(&encoded, Some(u32::try_from(blocks.len() * 256).expect("block count fits in u32")), &mut decoded).unwrap();

assert_eq!(decoded, input);
```

### 64-bit integers (`u64`)

The `FastPForWide128` / `FastPForWide256` codecs compress `u64` values.
They implement `AnyLenCodec` (with `Elem = u64`) for native use, and `BlockCodec64`
(`encode64` / `decode64`) for comparison against the C++ codecs.
The wire format is byte-compatible with the C++ `CppFastPFor128` / `CppFastPFor256` 64-bit paths.

```rust
use fastpfor::{AnyLenCodec, FastPForWide256};

let mut codec = FastPForWide256::default();
let input: Vec<u64> = (0..600).map(|i| i * 1_000_000_000).collect();

let mut encoded = Vec::new();
codec.encode(&input, &mut encoded).unwrap();

let mut decoded = Vec::new();
codec.decode(&encoded, &mut decoded, None).unwrap();

assert_eq!(decoded, input);
```

### SIMD kernels

The `FastPForSimd*` codecs (`FastPForSimd128`, `FastPForSimd256`, `FastPForSimdWide128`, `FastPForSimdWide256`,
and the matching `FastPForSimdBlock*` block codecs) are drop-in replacements for the codecs above.
They produce **byte-identical** output and decode each other's streams, so encoders and decoders can be mixed freely.

- `x86_64`: AVX2 kernels, selected at runtime; CPUs without AVX2 use the scalar kernels.
- `aarch64`: NEON kernels. `u64` values wider than 32 bits use the scalar kernels.
- Other targets: the scalar kernels.

```rust
use fastpfor::{AnyLenCodec, FastPFor256, FastPForSimd256};

let input: Vec<u32> = (0..1000).collect();

let mut encoded = Vec::new();
FastPForSimd256::default().encode(&input, &mut encoded).unwrap();

let mut scalar_encoded = Vec::new();
FastPFor256::default().encode(&input, &mut scalar_encoded).unwrap();
assert_eq!(encoded, scalar_encoded);

let mut decoded = Vec::new();
FastPFor256::default().decode(&encoded, &mut decoded, None).unwrap();
assert_eq!(decoded, input);
```

The `FastPForSimd*` codecs use the standard format. The C++ `CppSimdFastPFor*` codecs use the interleaved one:
use `FastPForInterleaved*` for those, see [Wire format](#wire-format).

### Interleaved format (C++ `SIMDFastPFor` compatible)

`FastPForInterleaved128` / `FastPForInterleaved256` (`u32`) and `FastPForInterleavedWide128` / `FastPForInterleavedWide256`
(`u64`) are used like the other codecs, but write the interleaved format, see [Wire format](#wire-format).
Their `u32` output is byte-identical to the C++ `CppSimdFastPFor128` / `CppSimdFastPFor256`.

```rust
use fastpfor::{AnyLenCodec, FastPFor256, FastPForInterleaved256};

let input: Vec<u32> = (0..1000).collect();

let mut encoded = Vec::new();
FastPForInterleaved256::default().encode(&input, &mut encoded).unwrap();

let mut decoded = Vec::new();
FastPForInterleaved256::default().decode(&encoded, &mut decoded, None).unwrap();
assert_eq!(decoded, input);

// A different format: the standard codec writes other bytes for the same input.
let mut standard = Vec::new();
FastPFor256::default().encode(&input, &mut standard).unwrap();
assert_ne!(encoded, standard);
```

### C++ Wrapper (`cpp` feature)

Enable the `cpp` feature in `Cargo.toml`:

```toml
fastpfor = { version = "0.9", features = ["cpp"] }
```

All C++ codecs implement the same `AnyLenCodec` trait (`encode` / `decode`), so
the usage pattern is identical to the Rust examples above — just swap the codec type,
e.g. `cpp::CppFastPFor128::new()`.

**Thread safety:** C++ codec instances have internal state and are **not thread-safe**.
Create one instance per thread or synchronize access externally.

## Crate Features

| Feature        | Default | Description                                                                                  |
|----------------|---------|----------------------------------------------------------------------------------------------|
| `rust`         | **yes** | Pure-Rust implementation — safe code, no build dependencies                                  |
| `cpp`          | no      | C++ wrapper via CXX — requires a C++14 compiler with SIMD support                            |
| `cpp_portable` | no      | Enables `cpp`, compiles C++ with SSE4.2 baseline (runs on any x86-64 from ~2008+)            |
| `cpp_native`   | no      | Enables `cpp`, compiles C++ with `-march=native` for maximum throughput on the build machine |

The `FASTPFOR_SIMD_MODE` environment variable (`portable` or `native`) can override the SIMD mode at build time.

**Recommendation:** Use `cpp_portable` (not `cpp_native`) for distributable binaries.

## Supported Algorithms

### Rust (`rust` feature)

Rust block codecs require block-aligned input. `CompositeCodec` chains a block codec with a tail codec (e.g. `VariableByte`) to handle arbitrary-length input. `FastPFor256`/`FastPFor128` (for `u32`) and `FastPForWide256`/`FastPForWide128` (for `u64`) are type aliases for such composites.

| Codec              | Description                                                     |
|--------------------|-----------------------------------------------------------------|
| `FastPFor256`      | `CompositeCodec` of `FastPForBlock256` + `VariableByte` (`u32`)  |
| `FastPFor128`      | `CompositeCodec` of `FastPForBlock128` + `VariableByte` (`u32`)  |
| `FastPForWide256`  | `CompositeCodec` of `FastPForBlockWide256` + `VariableByte` (`u64`) |
| `FastPForWide128`  | `CompositeCodec` of `FastPForBlockWide128` + `VariableByte` (`u64`) |
| `VariableByte`     | Variable-byte encoding, MSB is opposite to protobuf's varint    |
| `JustCopy`         | No compression; useful as a baseline                            |
| `FastPForBlock256` | `FastPFor` with 256-element `u32` blocks; block-aligned input only |
| `FastPForBlock128` | `FastPFor` with 128-element `u32` blocks; block-aligned input only |
| `FastPForSimd*`    | Same as the codec without `Simd`, using SIMD kernels; byte-identical output |
| `FastPForInterleaved*` | Interleaved format of C++ `SIMDFastPFor` (`u32`, byte-identical) and its `u64` extension; **not** compatible with the codecs above |

### C++ (`cpp` feature)

All C++ codecs are composite (any-length) and implement `AnyLenCodec` only.
`u64`-capable codecs (`CppFastPFor128`, `CppFastPFor256`, `CppVarInt`) also implement `BlockCodec64` with `encode64` / `decode64`.

| Codec                       | Notes                                                                  |
|-----------------------------|------------------------------------------------------------------------|
| `CppFastPFor128`            | `FastPFor + VByte` composite, 128-element blocks. Also supports `u64`. |
| `CppFastPFor256`            | `FastPFor + VByte` composite, 256-element blocks. Also supports `u64`. |
| `CppSimdFastPFor128`        | SIMD-optimized 128-element variant                                     |
| `CppSimdFastPFor256`        | SIMD-optimized 256-element variant                                     |
| `CppBP32`                   | Binary packing, 32-bit blocks                                          |
| `CppFastBinaryPacking8`     | Binary packing, 8-bit groups                                           |
| `CppFastBinaryPacking16`    | Binary packing, 16-bit groups                                          |
| `CppFastBinaryPacking32`    | Binary packing, 32-bit groups                                          |
| `CppSimdBinaryPacking`      | SIMD-optimized binary packing                                          |
| `CppPFor`                   | Patched frame-of-reference                                             |
| `CppSimplePFor`             | Simplified `PFor` variant                                              |
| `CppNewPFor`                | `PFor` with improved exception handling                                |
| `CppOptPFor`                | Optimized `PFor`                                                       |
| `CppPFor2008`               | Reference implementation from original paper                           |
| `CppSimdPFor`               | SIMD `PFor`                                                            |
| `CppSimdSimplePFor`         | SIMD `SimplePFor`                                                      |
| `CppSimdNewPFor`            | SIMD `NewPFor`                                                         |
| `CppSimdOptPFor`            | SIMD `OptPFor`                                                         |
| `CppSimple16`               | 16 packing modes in 32-bit words                                       |
| `CppSimple9`                | 9 packing modes                                                        |
| `CppSimple9Rle`             | Simple9 with run-length encoding                                       |
| `CppSimple8b`               | 8 packing modes in 64-bit words                                        |
| `CppSimple8bRle`            | Simple8b with run-length encoding                                      |
| `CppSimdGroupSimple`        | SIMD group-simple encoding                                             |
| `CppSimdGroupSimpleRingBuf` | SIMD group-simple with ring buffer                                     |
| `CppVByte`                  | Standard variable-byte encoding                                        |
| `CppMaskedVByte`            | SIMD masked variable-byte                                              |
| `CppStreamVByte`            | SIMD stream variable-byte                                              |
| `CppVarInt`                 | Standard varint. Also supports `u64`.                                  |
| `CppVarIntGb`               | Group varint                                                           |
| `CppCopy`                   | No compression (baseline)                                              |

## Benchmarks

### Decoding

Using Linux x86-64 running `just bench::cpp-vs-rust-decode native`. The values below are time measurements; smaller values indicate faster decoding.

| name                                    | cpp (ns) | rust (ns) | % faster |
|-----------------------------------------|----------|-----------|----------|
| `clustered/1024`                        | 643.24   | 392.93    | 38.91%   |
| `clustered/4096`                        | 1986     | 1414.8    | 28.76%   |
| `sequential/1024`                       | 653.69   | 396.02    | 39.42%   |
| `sequential/4096`                       | 2106     | 1476.2    | 29.91%   |
| `sparse/1024`                           | 428.8    | 352.38    | 17.82%   |
| `sparse/4096`                           | 1114     | 1179.5    | -5.88%   |
| `uniform_large_value_distribution/1024` | 286.74   | 153.06    | 46.62%   |
| `uniform_large_value_distribution/4096` | 748.19   | 558.05    | 25.41%   |
| `uniform_small_value_distribution/1024` | 606.4    | 405.44    | 33.14%   |
| `uniform_small_value_distribution/4096` | 2017.3   | 1403.7    | 30.42%   |

Rust encoding has not yet been fully optimized or verified.

## Build Requirements

- **Rust feature** (`rust`, the default): no additional dependencies.
- **C++ feature** (`cpp`): requires a C++14-capable compiler with SIMD intrinsics.
  See [FastPFor C++ requirements](https://github.com/fast-pack/FastPFor?tab=readme-ov-file#software-requirements).

### Linux

The default GitHub Actions runner has all needed dependencies.

For local development:

```bash
# This list may be incomplete
sudo apt-get install build-essential
```

`libsimde-dev` is optional. On ARM/aarch64, the C++ build fetches `SIMDe` via `CMake`
and the CXX bridge reuses that include path automatically.

### macOS

On Apple Silicon, `SIMDe` installation is usually not required — the C++ build fetches it via `CMake`.

If you prefer a Homebrew fallback:

```bash
brew install simde
export CXXFLAGS="-I/opt/homebrew/include"
export CFLAGS="-I/opt/homebrew/include"
```

## Development

This project uses [just](https://github.com/casey/just#readme) as a task runner:

```bash
cargo install just   # install once
just                 # list available commands
just test            # run all tests
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual-licensed as above, without any
additional terms or conditions.
