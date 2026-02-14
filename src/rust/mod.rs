mod cursor;
mod error;
mod integer_compression;

pub use cursor::IncrementCursor;
pub use error::{FastPForError, FastPForResult};
pub use integer_compression::bitpacking::{fast_pack, fast_unpack};
pub use integer_compression::codec::Codec;
pub use integer_compression::composition::Composition;
pub use integer_compression::differential::Delta;
pub use integer_compression::fastpfor::{
    FastPFOR, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE,
};
pub use integer_compression::integer_codec::Integer;
pub use integer_compression::just_copy::JustCopy;
pub use integer_compression::skippable_codec::Skippable;
pub use integer_compression::variable_byte::VariableByte;
