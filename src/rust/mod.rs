mod composite;
mod cursor;
mod fastpfor_codec;
mod integer_compression;

pub use composite::CompositeCodec;
/// Any-length `FastPFOR` codecs supporting both `u32` and `u64`.
pub use fastpfor_codec::{FastPFor128, FastPFor256};
/// Type-safe block codec with block size encoded in the type.
pub use integer_compression::fastpfor::{FastPFor, FastPForBlock128, FastPForBlock256};
/// Pass-through codec — implements [`AnyLenCodec`](crate::codec::AnyLenCodec).
pub use integer_compression::just_copy::JustCopy;
/// Variable-byte codec — implements [`AnyLenCodec`](crate::codec::AnyLenCodec).
pub use integer_compression::variable_byte::VariableByte;
