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

* **Pure Rust, `u32` and `u64`:** `FastPFor` codecs with 128- or 256-value blocks for both integer widths.
  Each has portable kernels (`Portable`) and SIMD kernels (`Auto`: AVX2 on `x86_64`, selected at runtime; NEON on
  `aarch64`; and falls back to the portable kernel everywhere else). The Rust **decoder** is about 29% faster than the C++ version.
  The Rust code is safe except for one `unsafe` call into the AVX2 kernels, made after runtime CPU feature detection;
  the crate has `#![deny(unsafe_code)]`, with the generated C++ FFI bridge as the only other exemption.
* **Optional C++ wrappers:** the `cpp` feature wraps the original [C++ library](https://github.com/fast-pack/FastPFor),
  including its other codecs.

## Wire format

The Rust `FastPFor` codecs, `Portable` **and** `Auto`, write byte-identical streams, and those streams are identical to
the **non-SIMD** C++ `FastPFor` codec (`CppFastPFor128` / `CppFastPFor256`) for both `u32` and `u64`.
`Auto` is a faster implementation of the same format, so encoders and decoders can be mixed freely.
Tests and fuzzing check the portable codecs byte-for-byte against the C++ library, and `Auto` against `Portable`,
on `x86_64` and `aarch64`. Version 0.10 calls this format *sequential*, hence the `FastPForSequential*` names.

The C++ **`SIMDFastPFor`** codec (`CppSimdFastPFor128` / `CppSimdFastPFor256`) uses a *different* format, and the Rust
codecs do not support it:

* It packs values in an interleaved 4-lane layout over 128-value groups, which the exception arrays use as well,
  instead of consecutive values; its bit-width choice also differs slightly.
  It is not a padded variant of the standard format: its streams are about the same size.
* It exists for `u32` only.
* The two formats are not interchangeable. Decoding one as the other is not detected: the output is silently wrong.
  Use the `cpp` wrappers if you need to read or write that format.

## Usage

### Rust Implementation (default)

The simplest way is `FastPForSequential32x256`: `u32` values in 256-value blocks. It handles any input length by
compressing whole blocks with `FastPForBlock` and the remaining values with `VariableByte`.

```rust
use fastpfor::{AnyLenCodec, FastPForSequential32x256};

let mut codec = FastPForSequential32x256::default();
let input: Vec<u32> = (0..1000).collect();

let mut encoded = Vec::new();
codec.encode(&input, &mut encoded).unwrap();

let mut decoded = Vec::new();
codec.decode(&encoded, &mut decoded, None).unwrap();

assert_eq!(decoded, input);
```

For block-aligned inputs you can use the lower-level `BlockCodec` API:

```rust
use fastpfor::{BlockCodec, FastPForSequentialBlock32x256, slice_to_blocks};

type Codec = FastPForSequentialBlock32x256;

let mut codec = Codec::default();
let input: Vec<u32> = (0..512).collect();   // exactly 2 blocks of 256

let (blocks, remainder) = slice_to_blocks::<Codec>(&input);
assert_eq!(blocks.len(), 2);
assert!(remainder.is_empty());

let mut encoded = Vec::new();
codec.encode_blocks(blocks, &mut encoded).unwrap();

let mut decoded = Vec::new();
codec.decode_blocks(&encoded, Some(u32::try_from(blocks.len() * 256).expect("block count fits in u32")), &mut decoded).unwrap();

assert_eq!(decoded, input);
```

### 64-bit integers (`u64`)

The `FastPForSequential64x128` / `FastPForSequential64x256` codecs compress `u64` values.
They implement `AnyLenCodec` (with `Elem = u64`) for native use, and `BlockCodec64`
(`encode64` / `decode64`) for comparison against the C++ codecs.
The wire format is byte-compatible with the C++ `CppFastPFor128` / `CppFastPFor256` 64-bit paths.

```rust
use fastpfor::{AnyLenCodec, FastPForSequential64x256};

let mut codec = FastPForSequential64x256::default();
let input: Vec<u64> = (0..600).map(|i| i * 1_000_000_000).collect();

let mut encoded = Vec::new();
codec.encode(&input, &mut encoded).unwrap();

let mut decoded = Vec::new();
codec.decode(&encoded, &mut decoded, None).unwrap();

assert_eq!(decoded, input);
```

### SIMD kernels

The `FastPForSequential*` codecs and `FastPForBlock` use the `Auto` kernels by default, the fastest available:

- `x86_64`: AVX2 kernels, selected at runtime; CPUs without AVX2 use the portable kernels.
- `aarch64`: NEON kernels. `u64` values wider than 32 bits use the portable kernels.
- Other targets: the portable kernels.

All kernels produce **byte-identical** output and decode each other's streams. To always run the portable kernels,
pass `Portable` as the last type parameter of `FastPForBlock`:

```rust
use fastpfor::{
    AnyLenCodec, CompositeCodec, FastPForBlock, FastPForSequential32x256, Portable, Sequential,
    VariableByte,
};

type PortableCodec = CompositeCodec<FastPForBlock<Sequential, u32, 256, Portable>, VariableByte>;

let input: Vec<u32> = (0..1000).collect();

let mut encoded = Vec::new();
FastPForSequential32x256::default().encode(&input, &mut encoded).unwrap();

let mut portable_encoded = Vec::new();
PortableCodec::default().encode(&input, &mut portable_encoded).unwrap();
assert_eq!(encoded, portable_encoded);

let mut decoded = Vec::new();
PortableCodec::default().decode(&encoded, &mut decoded, None).unwrap();
assert_eq!(decoded, input);
```

Note that the C++ `CppSimdFastPFor*` codecs use a different, interleaved bit layout and are **not**
compatible with either the Rust codecs or the C++ `CppFastPFor*` codecs.

### Names deprecated in 0.9.2

Version 0.10 names each codec by its wire format, value width and block size, and adds a second format
(C++ `SIMDFastPFor`, called *interleaved*). Version 0.9.2 adds the new names for the existing codecs and
deprecates the old ones, which 0.10 removes:

| Deprecated                                              | Use instead                                    |
|---------------------------------------------------------|------------------------------------------------|
| `FastPFor128`, `FastPForSimd128`                        | `FastPForSequential32x128`                     |
| `FastPFor256`, `FastPForSimd256`                        | `FastPForSequential32x256`                     |
| `FastPForWide128`, `FastPForSimdWide128`                | `FastPForSequential64x128`                     |
| `FastPForWide256`, `FastPForSimdWide256`                | `FastPForSequential64x256`                     |
| `FastPForBlock128`, `FastPForSimdBlock128`             | `FastPForSequentialBlock32x128`                |
| `FastPForBlock256`, `FastPForSimdBlock256`             | `FastPForSequentialBlock32x256`                |
| `FastPForBlockWide128`, `FastPForSimdBlockWide128`     | `FastPForSequentialBlock64x128`                |
| `FastPForBlockWide256`, `FastPForSimdBlockWide256`     | `FastPForSequentialBlock64x256`                |
| `FastPFor<N, T, K>`                                     | `FastPForBlock<Sequential, T, N, K>`           |
| `Scalar` / `Simd` kernels                               | `Portable` / `Auto`                            |

The new names use the `Auto` (SIMD) kernels by default where the old ones used `Scalar`. The output is the same.

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

Rust block codecs require block-aligned input. `CompositeCodec` chains a block codec with a tail codec (e.g. `VariableByte`) to handle arbitrary-length input. The `FastPForSequential32x*` and `FastPForSequential64x*` codecs are type aliases for such composites.

| Codec                      | Description                                                                 |
|----------------------------|-----------------------------------------------------------------------------|
| `FastPForSequential32x256` | `CompositeCodec` of `FastPForBlock<Sequential, u32, 256>` + `VariableByte`  |
| `FastPForSequential32x128` | `CompositeCodec` of `FastPForBlock<Sequential, u32, 128>` + `VariableByte`  |
| `FastPForSequential64x256` | `CompositeCodec` of `FastPForBlock<Sequential, u64, 256>` + `VariableByte`  |
| `FastPForSequential64x128` | `CompositeCodec` of `FastPForBlock<Sequential, u64, 128>` + `VariableByte`  |
| `VariableByte`             | Variable-byte encoding, MSB is opposite to protobuf's varint                |
| `JustCopy`                 | No compression; useful as a baseline                                        |
| `FastPForSequentialBlock*` | The block codec of each, for block-aligned input only                       |
| `FastPForBlock`            | The same, generically: `FastPForBlock<Sequential, T, N, Kernels>`           |

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
