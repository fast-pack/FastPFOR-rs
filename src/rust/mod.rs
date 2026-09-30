mod composite;
mod cursor;
mod fastpfor_codec;
mod integer_compression;
mod kernels;

pub use composite::CompositeCodec;
/// Any-length `FastPFOR` codecs: `FastPFor*` for `u32`, `FastPForWide*` for `u64`.
pub use fastpfor_codec::{
    FastPFor128, FastPFor256, FastPForSimd128, FastPForSimd256, FastPForSimdWide128,
    FastPForSimdWide256, FastPForWide128, FastPForWide256,
};
/// Type-safe block codec with block size encoded in the type.
pub use integer_compression::fastpfor::FastPFor;
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
pub use kernels::{Kernels, Scalar, Simd};
