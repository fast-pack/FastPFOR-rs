// The crate keeps using its own names deprecated in 0.9.2 internally; 0.10 removes them.
#![allow(deprecated, reason = "migrated in 0.10")]

mod composite;
mod cursor;
mod fastpfor_codec;
mod integer_compression;
mod kernels;

pub use composite::CompositeCodec;
/// Any-length `FastPFOR` codecs: `FastPForSequential32x*` for `u32`, `FastPForSequential64x*` for `u64`.
/// The other names are deprecated.
pub use fastpfor_codec::{
    FastPFor128, FastPFor256, FastPForSequential32x128, FastPForSequential32x256,
    FastPForSequential64x128, FastPForSequential64x256, FastPForSequentialBlock32x128,
    FastPForSequentialBlock32x256, FastPForSequentialBlock64x128, FastPForSequentialBlock64x256,
    FastPForSimd128, FastPForSimd256, FastPForSimdWide128, FastPForSimdWide256, FastPForWide128,
    FastPForWide256,
};
/// Type-safe block codec with block size encoded in the type.
pub use integer_compression::fastpfor::{FastPFor, FastPForBlock};
pub use integer_compression::fastpfor32::{
    FastPForBlock128, FastPForBlock256, FastPForSimdBlock128, FastPForSimdBlock256,
};
pub use integer_compression::fastpfor64::{
    FastPForBlockWide128, FastPForBlockWide256, FastPForSimdBlockWide128, FastPForSimdBlockWide256,
};
/// Pass-through codec — implements [`AnyLenCodec`](crate::codec::AnyLenCodec).
pub use integer_compression::just_copy::JustCopy;
/// Variable-byte codec — implements [`AnyLenCodec`](crate::codec::AnyLenCodec).
pub use integer_compression::variable_byte::VariableByte;
#[cfg(feature = "__testing")]
pub use kernels::with_simd_fallback;
pub use kernels::{Auto, Kernels, Layout, Portable, Scalar, Sequential, Simd};
