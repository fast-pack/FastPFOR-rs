# `FastPFor` for Rust

[![GitHub](https://img.shields.io/badge/github-fastpfor-8da0cb?logo=github)](https://github.com/jjcfrancisco/fastpfor)
[![crates.io version](https://img.shields.io/crates/v/fastpfor.svg)](https://crates.io/crates/fastpfor)
[![docs.rs docs](https://docs.rs/fastpfor/badge.svg)](https://docs.rs/fastpfor)
[![license](https://img.shields.io/crates/l/fastpfor.svg)](https://github.com/jjcfrancisco/fastpfor/blob/main/LICENSE-APACHE)
[![CI build](https://github.com/jjcfrancisco/fastpfor/actions/workflows/ci.yml/badge.svg)](https://github.com/jjcfrancisco/fastpfor/actions)

This is a Rust wrapper for the [C++ FastPFor library](https://github.com/fast-pack/FastPFor), as well as a pure Rust re-implementation.  Supports 32-bit and 64-bit integers, and SIMD-optimized codecs for 128-bit and 256-bit vectors. Based on the [Decoding billions of integers per second through vectorization, 2012](https://arxiv.org/abs/1209.2137) paper.

The Rust **decoder** is about 29% faster than the C++ version. Rust code prohibits `unsafe` keyword.

### Supported algorithms
Unless otherwise specified, all codecs support `&[u32]` only.

```text
* BP32
* Copy
* FastBinaryPacking8
* FastPFor128 (both `&[u32]` and `&[u64]`)
* FastPFor256 (both `&[u32]` and `&[u64]`)
* FastBinaryPacking16
* FastBinaryPacking32
* MaskedVByte
* NewPFor
* OptPFor
* PFor2008
* PFor
* SimdBinaryPacking
* SimdFastPFor128
* SimdFastPFor256
* SimdGroupSimple
* SimdGroupSimpleRingBuf
* SimdNewPFor
* SimdOptPFor
* SimdPFor
* SimdSimplePFor
* Simple16
* Simple8b
* Simple8bRle
* Simple9
* Simple9Rle
* SimplePFor
* StreamVByte
* VByte
* VarInt (both `&[u32]` and `&[u64]`)
* VarIntGb
```

## Benchmarks
### Decoding speed (lower is better)

Using Linux x86-64 running `just bench::cpp-vs-rust-decode native`

| name                                  | cpp    | rust   | % faster |
|---------------------------------------|--------|--------|----------|
| clustered/1024                        | 643.24 | 392.93 | 38.91%   |
| clustered/4096                        | 1986   | 1414.8 | 28.76%   |
| sequential/1024                       | 653.69 | 396.02 | 39.42%   |
| sequential/4096                       | 2106   | 1476.2 | 29.91%   |
| sparse/1024                           | 428.8  | 352.38 | 17.82%   |
| sparse/4096                           | 1114   | 1179.5 | -5.88%   |
| uniform_large_value_distribution/1024 | 286.74 | 153.06 | 46.62%   |
| uniform_large_value_distribution/4096 | 748.19 | 558.05 | 25.41%   |
| uniform_small_value_distribution/1024 | 606.4  | 405.44 | 33.14%   |
| uniform_small_value_distribution/4096 | 2017.3 | 1403.7 | 30.42%   |

Rust Encoding has not yet been either optimized or even fully verified.

## Usage

### Crate Features
* `cpp` - C++ implementation (default, uses portable SIMD mode)
* `rust` - Rust implementation (work in progress, opt-in)

#### SIMD Mode Configuration

The C++ backend can be compiled with different SIMD instruction sets. Control this by enabling one of these features:
| Mode | Description |
|------|-------------|
| `cpp_portable` | **Default.** Uses SSE4.2 baseline only. Binaries run on any x86-64 CPU from ~2008+. Best for distributable libraries. |
| `cpp_native` | Uses `-march=native` to enable all SIMD instructions supported by the build machine (AVX, AVX2, etc.). Maximum performance but may crash on CPUs lacking those instructions. |

Feature selection can be overridden with the `FASTPFOR_SIMD_MODE` environment variable set to "portable" or "native".

**Recommendation:** Use `portable` (default) for libraries and distributed binaries. Use `native` only when building for a specific machine where you need maximum performance.

### Using C++ Wrapper

```rust
use fastpfor::cpp::{Codec32 as _, SimdFastPFor128Codec};

fn main() {
  let mut codec = SimdFastPFor128Codec::new();

  // Encode
  let mut input = vec![1, 2, 3, 4, 5];
  let mut output = vec![0; 10];  // must be large enough
  let enc_slice = codec.encode32(&input, &mut output).unwrap();

  // Decode
  let mut decoded = vec![0; 10]; // must be large enough
  let dec_slice = codec.decode32(&enc_slice, &mut decoded).unwrap();

  assert_eq!(input, dec_slice);
}
```

## Build Requirements

- When using the **Rust implementation**:
  no additional dependencies are required.
- When using the **C++ implementation**:
  you need to have a C++ compiler that supports C++14 and SIMD intrinsics.
  See [FastPFor C++ requirements](https://github.com/fast-pack/FastPFor?tab=readme-ov-file#software-requirements).

### Linux

The default GitHub action runner for Linux has all the needed dependencies.

For local development, you may need to install the following packages:

```bash
# This list may be incomplete
sudo apt-get install build-essential
```

`libsimde-dev` is optional. On ARM/aarch64, the C++ build fetches `SIMDe` via `CMake`,
and the Rust CXX bridge now reuses that fetched include path automatically.
Install `libsimde-dev` only if you prefer a system package fallback.

### macOS
On Apple Silicon, manual `SIMDe` installation is usually not required.
The C++ build fetches `SIMDe` via `CMake`, and the Rust CXX bridge reuses that path.

If you prefer a system package fallback, install `SIMDe` with Homebrew and set include flags.

```bash
# optional: install SIMDe via Homebrew
brew install simde

# optional fallback: ensure the compiler can find Homebrew headers
export CXXFLAGS="-I/opt/homebrew/include"
export CFLAGS="-I/opt/homebrew/include"
```

## Development

* This project is easier to develop with [just](https://github.com/casey/just#readme), a modern alternative to `make`.
  Install it with `cargo install just`.
* To get a list of available commands, run `just`.
* To run tests, use `just test`.

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
