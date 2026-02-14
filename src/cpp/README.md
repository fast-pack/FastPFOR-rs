# `FastPFOR` Rust Wrapper

Rust wrapper for the [FastPFOR C++ library](https://github.com/fast-pack/FastPFor), providing fast integer compression codecs optimized for sorted sequences.

## Quick Start

```rust
use fastpfor::cpp::{FastPFor128Codec, Codec32};

let codec = FastPFor128Codec::new();
let input = vec![1, 2, 3, 4, 5, 100, 200, 300];
let mut compressed = vec![0u32; input.len() + 1024];

let encoded = codec.encode32(&input, &mut compressed).unwrap();
let mut decompressed = vec![0u32; input.len()];
let decoded = codec.decode32(encoded, &mut decompressed).unwrap();

assert_eq!(decoded, input.as_slice());
```

### 64-bit Integers

Some codecs support 64-bit integers via the [`Codec64`] trait:

```rust
use fastpfor::cpp::{FastPFor128Codec, Codec64};

let codec = FastPFor128Codec::new();
let input = vec![100u64, 200, 300, 400];
let mut compressed = vec![0u32; input.len() * 2 + 1024];
let encoded = codec.encode64(&input, &mut compressed).unwrap();

let mut decompressed = vec![0u64; input.len()];
let decoded = codec.decode64(encoded, &mut decompressed).unwrap();
assert_eq!(decoded, input.as_slice());
```

See [`Codec32`] and [`Codec64`] trait documentation for buffer sizing guidelines.

## Codec Selection

> **Note:** See individual codec documentation below for detailed descriptions and use-case recommendations.

### General Purpose (Recommended)

- [`FastPFor128Codec`], [`FastPFor256Codec`] - Best all-around choice. Fast decode, good compression for sorted/clustered data. Support 64-bit.
- [`SimdFastPFor128Codec`], [`SimdFastPFor256Codec`] - SIMD-optimized variants for maximum throughput

### Patched Frame-of-Reference Variants

Frame-of-reference encoding with exception handling. Excellent for monotonic sequences (timestamps, IDs).

- [`PForCodec`] - Standard implementation
- [`SimplePForCodec`] - Simplified variant with lower complexity
- [`NewPForCodec`] - Enhanced exception handling
- [`OptPForCodec`] - Optimized for common patterns
- [`PFor2008Codec`] - Reference implementation from research paper
- **SIMD variants:** [`SimdPForCodec`], [`SimdNewPForCodec`], [`SimdOptPForCodec`], [`SimdSimplePForCodec`]

### Binary Packing

Bit-packing based on maximum bit width. Good for uniform data distributions.

- [`BP32Codec`] - Standard 32-bit block binary packing
- [`FastBinaryPacking8Codec`], [`FastBinaryPacking16Codec`], [`FastBinaryPacking32Codec`] - Different block sizes
- [`SimdBinaryPackingCodec`] - SIMD-optimized variant

### Variable Byte Encoding

Best for unsorted data and small integers. Simple and widely compatible.

- [`VByteCodec`] - Standard variable byte encoding (1-5 bytes per integer)
- [`VarIntCodec`] - Standard varint format. Supports 64-bit.
- [`VarIntGbCodec`] - Group varint with shared control information
- **SIMD variants:** [`MaskedVByteCodec`], [`StreamVByteCodec`] - Excellent decode speed on modern CPUs

### Simple Encodings

Efficient for small positive integers (typically < 2^16).
**Does not support arbitrary u32 inputs.**.

- [`Simple16Codec`] - 16 packing modes in 32-bit words
- [`Simple9Codec`] - 9 packing modes for flexibility
- [`Simple8bCodec`] - 8 packing modes in 64-bit words
- [`Simple9RleCodec`], [`Simple8bRleCodec`] - With run-length encoding for repeated values
- [`SimdGroupSimpleCodec`], [`SimdGroupSimpleRingBufCodec`] - SIMD-optimized

### Utility

- [`CopyCodec`] - No compression (baseline for benchmarking)

## Thread Safety

Codec instances **have internal state** that is **cleared after each operation**.
They are **not thread-safe** during concurrent encode/decode operations.

Use one of these strategies:

- Create separate codec instances per thread
- Synchronize access with mutexes
- Use thread-local storage for codec instances

## Architecture

This module uses [CXX](https://cxx.rs/) to safely bridge Rust and C++:

- Each codec wraps a C++ `IntegerCODEC` instance via [`UniquePtr`]
- The [`Codec32`] and [`Codec64`] traits provide the Rust API
- Memory is automatically managed by CXX and Rust's ownership system

See the [FastPFOR C++ library documentation](https://github.com/fast-pack/FastPFor) for underlying implementation details.

[`Codec32`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/trait.Codec32.html
[`Codec64`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/trait.Codec64.html
[`Exception`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/type.Exception.html
[`UniquePtr`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/https://docs.rs/cxx/latest/cxx/struct.UniquePtr.html
[`BP32Codec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.BP32Codec.html
[`CopyCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.CopyCodec.html
[`FastBinaryPacking8Codec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.FastBinaryPacking8Codec.html
[`FastBinaryPacking16Codec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.FastBinaryPacking16Codec.html
[`FastBinaryPacking32Codec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.FastBinaryPacking32Codec.html
[`FastPFor128Codec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.FastPFor128Codec.html
[`FastPFor256Codec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.FastPFor256Codec.html
[`MaskedVByteCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.MaskedVByteCodec.html
[`NewPForCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.NewPForCodec.html
[`OptPForCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.OptPForCodec.html
[`PFor2008Codec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.PFor2008Codec.html
[`PForCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.PForCodec.html
[`SimdBinaryPackingCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.SimdBinaryPackingCodec.html
[`SimdFastPFor128Codec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.SimdFastPFor128Codec.html
[`SimdFastPFor256Codec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.SimdFastPFor256Codec.html
[`SimdGroupSimpleCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.SimdGroupSimpleCodec.html
[`SimdGroupSimpleRingBufCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.SimdGroupSimpleRingBufCodec.html
[`SimdNewPForCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.SimdNewPForCodec.html
[`SimdOptPForCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.SimdOptPForCodec.html
[`SimdPForCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.SimdPForCodec.html
[`SimdSimplePForCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.SimdSimplePForCodec.html
[`Simple16Codec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.Simple16Codec.html
[`Simple8bCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.Simple8bCodec.html
[`Simple8bRleCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.Simple8bRleCodec.html
[`Simple9Codec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.Simple9Codec.html
[`Simple9RleCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.Simple9RleCodec.html
[`SimplePForCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.SimplePForCodec.html
[`StreamVByteCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.StreamVByteCodec.html
[`VByteCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.VByteCodec.html
[`VarIntCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.VarIntCodec.html
[`VarIntGbCodec`]: https://docs.rs/fastpfor/latest/fastpfor/cpp/struct.VarIntGbCodec.html
