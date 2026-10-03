mod composite;
mod cursor;
mod fastpfor_codec;
mod integer_compression;
mod kernels;

pub use composite::CompositeCodec;
/// Any-length `FastPFOR` codecs: `FastPFor*` for `u32`, `FastPForWide*` for `u64`.
pub use fastpfor_codec::{
    FastPFor128, FastPFor256, FastPForInterleaved128, FastPForInterleaved256,
    FastPForInterleavedWide128, FastPForInterleavedWide256, FastPForSimd128, FastPForSimd256,
    FastPForSimdWide128, FastPForSimdWide256, FastPForWide128, FastPForWide256,
};
/// Type-safe block codec with block size encoded in the type.
pub use integer_compression::fastpfor::FastPFor;
pub use integer_compression::fastpfor32::{
    FastPForBlock128, FastPForBlock256, FastPForInterleavedBlock128, FastPForInterleavedBlock256,
    FastPForSimdBlock128, FastPForSimdBlock256,
};
pub use integer_compression::fastpfor64::{
    FastPForBlockWide128, FastPForBlockWide256, FastPForInterleavedBlockWide128,
    FastPForInterleavedBlockWide256, FastPForSimdBlockWide128, FastPForSimdBlockWide256,
};
/// Pass-through codec — implements [`AnyLenCodec`](crate::codec::AnyLenCodec).
pub use integer_compression::just_copy::JustCopy;
/// Variable-byte codec — implements [`AnyLenCodec`](crate::codec::AnyLenCodec).
pub use integer_compression::variable_byte::VariableByte;
#[cfg(feature = "__testing")]
pub use kernels::with_simd_fallback;
pub use kernels::{Interleaved, InterleavedPortable, Kernels, Scalar, Simd};
