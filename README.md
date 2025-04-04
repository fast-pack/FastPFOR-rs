# FastPFor for Rust

[![GitHub](https://img.shields.io/badge/github-fastpfor-8da0cb?logo=github)](https://github.com/jjcfrancisco/fastpfor)
[![crates.io version](https://img.shields.io/crates/v/fastpfor.svg)](https://crates.io/crates/fastpfor)
[![docs.rs docs](https://docs.rs/fastpfor/badge.svg)](https://docs.rs/fastpfor)
[![license](https://img.shields.io/crates/l/fastpfor.svg)](https://github.com/jjcfrancisco/fastpfor/blob/main/LICENSE-APACHE)
[![CI build](https://github.com/jjcfrancisco/fastpfor/actions/workflows/ci.yml/badge.svg)](https://github.com/jjcfrancisco/fastpfor/actions)

This is a Rust wrapper for the [C++ FastPFor library](https://github.com/fast-pack/FastPFor), as well as a pure Rust re-implementation (work in progress).  Supports 32-bit and 64-bit integers, and SIMD-optimized codecs for 128-bit and 256-bit vectors. Based on the [Decoding billions of integers per second through vectorization, 2012](https://arxiv.org/abs/1209.2137) paper.

### Supported algorithms
Unless otherwise specified, all codecs support `&[u32]` only.
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

## Usage

### Crate Features
* `cpp` - C++ implementation (default)
* `rust` - Rust implementation (work in progress, opt-in)

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

When using the Rust implementation, no additional dependencies are required.
When using the C++ implementation, you need to have a C++ compiler that supports C++14 and SIMD intrinsics. See [FastPFor C++ requirements](https://github.com/fast-pack/FastPFor?tab=readme-ov-file#software-requirements).

### Linux
The default GitHub action runner for Linux has all the needed dependencies. For local development, you may need to install the following packages.

```bash
# This list may be incomplete
sudo apt-get install build-essential libsimde-dev
```

### macOS
To build FastPFor on macOS, you'll need to install SIMDe. Since Homebrew installs packages in `/opt/homebrew` (for Apple Silicon), you'll also need to explicitly set the include paths.

```bash
# install SIMDe via Homebrew
brew install simde
```

```bash
# Ensure the compiler can find the required headers before building
export CXXFLAGS="-I/opt/homebrew/include"
export CFLAGS="-I/opt/homebrew/include"
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
