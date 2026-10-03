mod composite;
mod cursor;
mod fastpfor_codec;
mod integer_compression;
mod kernels;

pub use composite::CompositeCodec;
/// Any-length `FastPFOR` codecs, named `FastPFor<Layout><value bits>x<block size>`.
pub use fastpfor_codec::{
    FastPForCodec, FastPForInterleaved32x128, FastPForInterleaved32x256, FastPForInterleaved64x128,
    FastPForInterleaved64x256, FastPForInterleavedBlock32x128, FastPForInterleavedBlock32x256,
    FastPForInterleavedBlock64x128, FastPForInterleavedBlock64x256, FastPForSequential32x128,
    FastPForSequential32x256, FastPForSequential64x128, FastPForSequential64x256,
    FastPForSequentialBlock32x128, FastPForSequentialBlock32x256, FastPForSequentialBlock64x128,
    FastPForSequentialBlock64x256,
};
/// Block-only `FastPFOR` codec, with the block size in the type.
pub use integer_compression::fastpfor::FastPForBlock;
/// Pass-through codec — implements [`AnyLenCodec`](crate::codec::AnyLenCodec).
pub use integer_compression::just_copy::JustCopy;
/// Variable-byte codec — implements [`AnyLenCodec`](crate::codec::AnyLenCodec).
pub use integer_compression::variable_byte::VariableByte;
pub use kernels::{Auto, Interleaved, Kernels, Layout, Portable, Sequential};
#[cfg(feature = "__testing")]
pub use kernels::{Implementation, implementation, with_simd_fallback};
