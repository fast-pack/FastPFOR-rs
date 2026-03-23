mod composite;
mod cursor;
mod integer_compression;

pub use composite::CompositeCodec;
/// Type-safe block codec with block size encoded in the type.
pub use integer_compression::fastpfor::{FastPFor, FastPForBlock128, FastPForBlock256};
/// Pass-through codec — implements [`AnyLenCodec`](crate::codec::AnyLenCodec).
pub use integer_compression::just_copy::JustCopy;
/// Variable-byte codec — implements [`AnyLenCodec`](crate::codec::AnyLenCodec).
pub use integer_compression::variable_byte::VariableByte;

/// `FastPForBlock256` blocks + `VariableByte` remainder — the most common composite.
pub type FastPFor256 = CompositeCodec<FastPForBlock256, VariableByte>;

/// `FastPForBlock128` blocks + `VariableByte` remainder.
pub type FastPFor128 = CompositeCodec<FastPForBlock128, VariableByte>;
