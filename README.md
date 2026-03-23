# `FastPFor` for Rust

[![GitHub repo](https://img.shields.io/badge/github-fast--pack/FastPFOR--rs-8da0cb?logo=github)](https://github.com/fast-pack/FastPFOR-rs)
[![crates.io version](https://img.shields.io/crates/v/fastpfor)](https://crates.io/crates/fastpfor)
[![crate usage](https://img.shields.io/crates/d/fastpfor)](https://crates.io/crates/fastpfor)
[![docs.rs status](https://img.shields.io/docsrs/fastpfor)](https://docs.rs/fastpfor)
[![crates.io license](https://img.shields.io/crates/l/fastpfor)](https://github.com/fast-pack/FastPFOR-rs/blob/main/LICENSE-APACHE)
[![CI build status](https://github.com/fast-pack/FastPFOR-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/fast-pack/FastPFOR-rs/actions)
[![Codecov](https://img.shields.io/codecov/c/github/fast-pack/FastPFOR-rs)](https://app.codecov.io/gh/fast-pack/FastPFOR-rs)

Fast integer compression for Rust — both a pure-Rust implementation and a wrapper around the [C++ FastPFor library](https://github.com/fast-pack/FastPFor).
Supports 32-bit (and for some codecs 64-bit) integers.
Based on the [Decoding billions of integers per second through vectorization, 2012](https://arxiv.org/abs/1209.2137) paper.

The Rust **decoder** is about 29% faster than the C++ version. The Rust implementation contains no `unsafe` code, and when built without the `cpp` feature this crate has `#![forbid(unsafe_code)]`.

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

### C++ Wrapper (`cpp` feature)

Enable the `cpp` feature in `Cargo.toml`:

```toml
fastpfor = { version = "0.1", features = ["cpp"] }
```

All C++ codecs implement the same `AnyLenCodec` trait (`encode` / `decode`), so
the usage pattern is identical to the Rust examples above — just swap the codec type,
e.g. `cpp::CppFastPFor128::new()`.

**Thread safety:** C++ codec instances have internal state and are **not thread-safe**.
Create one instance per thread or synchronize access externally.

## Crate Features

| Feature        | Default | Description                                                                                  |
|----------------|---------|----------------------------------------------------------------------------------------------|
| `rust`         | **yes** | Pure-Rust implementation — no `unsafe`, no build dependencies                                |
| `cpp`          | no      | C++ wrapper via CXX — requires a C++14 compiler with SIMD support                            |
| `cpp_portable` | no      | Enables `cpp`, compiles C++ with SSE4.2 baseline (runs on any x86-64 from ~2008+)            |
| `cpp_native`   | no      | Enables `cpp`, compiles C++ with `-march=native` for maximum throughput on the build machine |

The `FASTPFOR_SIMD_MODE` environment variable (`portable` or `native`) can override the SIMD mode at build time.

**Recommendation:** Use `cpp_portable` (not `cpp_native`) for distributable binaries.

## Supported Algorithms

### Rust (`rust` feature)

Rust block codecs require block-aligned input. `CompositeCodec` chains a block codec with a tail codec (e.g. `VariableByte`) to handle arbitrary-length input. `FastPFor256` and `FastPFor128` are type aliases for such composites.

| Codec              | Description                                                  |
|--------------------|--------------------------------------------------------------|
| `FastPFor256`      | `CompositeCodec` of `FastPForBlock256` + `VariableByte`      |
| `FastPFor128`      | `CompositeCodec` of `FastPForBlock128` + `VariableByte`      |
| `VariableByte`     | Variable-byte encoding, MSB is opposite to protobuf's varint |
| `JustCopy`         | No compression; useful as a baseline                         |
| `FastPForBlock256` | `FastPFor` with 256-element blocks; block-aligned input only |
| `FastPForBlock128` | `FastPFor` with 128-element blocks; block-aligned input only |

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
