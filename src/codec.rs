use bytemuck::{Pod, cast_slice};

use crate::FastPForError;

/// Internal default for max decompressed length. Used by trait defaults and C++ FFI.
#[inline]
pub(crate) fn default_max_decoded_len(compressed_words: usize) -> usize {
    compressed_words.saturating_mul(1024)
}

/// Compresses and decompresses fixed-size blocks of `u32` values.
///
/// The associated type [`Block`](BlockCodec::Block) is the concrete fixed-size
/// array, e.g. `[u32; 256]`. Using an associated *type* (not an associated
/// constant) lets `CompositeCodec<Blocks, Tail>` be a clean two-parameter
/// struct on stable Rust, with no extra `const N` leaking into user-facing
/// signatures.
///
/// # Compile-time safety
///
/// Passing a `&[[u32; 128]]` slice to a codec whose `Block = [u32; 256]`
/// is a **compile error** — the slice element types simply don't match.
///
/// # Implementing this trait
///
/// ```rust,ignore
/// impl BlockCodec for MyCodec {
///     type Block = [u32; 256];
///     fn encode_blocks(&self, blocks: &[[u32; 256]], out: &mut Vec<u32>)
///         -> Result<(), FastPForError> { ... }
///     fn decode_blocks(&self, input: &[u32], expected_len: Option<u32>,
///         out: &mut Vec<u32>) -> Result<usize, FastPForError> { ... }
/// }
/// ```
pub trait BlockCodec {
    /// The fixed-size block type.  Must be plain-old-data (`Pod`).
    /// In practice this will be `[u32; 128]` or `[u32; 256]`.
    type Block: Pod;

    /// Number of `u32` elements in one block.
    ///
    /// Equal to `size_of::<Self::Block>() / 4`. Use this when computing
    /// element counts from block counts, e.g. `n_blocks * codec.elements_per_block()`.
    #[inline]
    #[must_use]
    fn size() -> usize
    where
        Self: Sized,
    {
        size_of::<Self::Block>() / size_of::<u32>()
    }

    /// Compress a slice of complete, fixed-size blocks.
    ///
    /// No remainder is possible — the caller must split the input first using
    /// [`slice_to_blocks`] and handle any remainder separately.
    fn encode_blocks(
        &mut self,
        blocks: &[Self::Block],
        out: &mut Vec<u32>,
    ) -> Result<(), FastPForError>;

    /// Decompress blocks from `input`, using the length stored in the header.
    ///
    /// When `expected_len` is `Some(n)`:
    /// - Validates that the header value equals `n` (must be a multiple of
    ///   [`size`](BlockCodec::size)).
    ///
    /// When `expected_len` is `None`:
    /// - Validates the header value against
    ///   [`max_decompressed_len`](BlockCodec::max_decompressed_len)(`input.len()`)
    ///   to avoid allocation from malicious or corrupted data.
    fn decode_blocks(
        &mut self,
        input: &[u32],
        expected_len: Option<u32>,
        out: &mut Vec<u32>,
    ) -> Result<usize, FastPForError>;

    /// Maximum decompressed element count for a given compressed input length.
    /// Reject `expected_len` values exceeding this to avoid allocation from bad data.
    #[inline]
    #[must_use]
    fn max_decompressed_len(compressed_words: usize) -> usize
    where
        Self: Sized,
    {
        default_max_decoded_len(compressed_words)
    }
}

/// Codec that supports compressing 64-bit integers into a 32-bit word stream.
///
/// Only three C++ codecs implement this trait: `CppFastPFor128`,
/// `CppFastPFor256`, and `CppVarInt`. For simple use, call
/// `encode64` / `decode64` directly on the struct — no trait import required.
///
/// Import `BlockCodec64` only when writing generic code over multiple codecs
/// that support 64-bit compression.
#[cfg(feature = "cpp")]
pub trait BlockCodec64 {
    /// Compress 64-bit integers into a 32-bit word stream.
    fn encode64(&mut self, input: &[u64], out: &mut Vec<u32>) -> Result<(), FastPForError>;
    /// Decompress 64-bit integers from a 32-bit word stream.
    fn decode64(&mut self, input: &[u32], out: &mut Vec<u64>) -> Result<(), FastPForError>;
}

/// Compresses and decompresses an arbitrary-length `&[u32]` slice.
///
/// Handles any input length including sub-block remainders.  All pure
/// variable-length codecs (e.g. `VariableByte`, `JustCopy`) implement this
/// trait directly.  Block-oriented codecs are wrapped in `CompositeCodec`
/// to produce an `AnyLenCodec`.
pub trait AnyLenCodec {
    /// Compress an arbitrary-length slice of `u32` values.
    fn encode(&mut self, input: &[u32], out: &mut Vec<u32>) -> Result<(), FastPForError>;

    /// Maximum decompressed element count for a given compressed input length.
    /// Reject `expected_len` values exceeding this to avoid allocation from bad data.
    #[inline]
    #[must_use]
    fn max_decompressed_len(compressed_words: usize) -> usize
    where
        Self: Sized,
    {
        default_max_decoded_len(compressed_words)
    }

    /// Decompress a previously compressed slice of `u32` values.
    ///
    /// When `expected_len` is `Some(n)`:
    /// - Rejects if `n` exceeds [`max_decompressed_len`](AnyLenCodec::max_decompressed_len)(`input.len()`)
    /// - Pre-allocates output capacity for `n` elements
    /// - Returns [`DecodedCountMismatch`](crate::FastPForError::DecodedCountMismatch) if the
    ///   actual decoded count differs from `n`
    ///
    /// The hint is not trusted: values from untrusted metadata are capped before use.
    fn decode(
        &mut self,
        input: &[u32],
        out: &mut Vec<u32>,
        expected_len: Option<u32>,
    ) -> Result<(), FastPForError>;
}

/// Split a flat `&[u32]` into `(&[Blocks::Block], &[u32])` without copying.
///
/// Uses `size_of::<Blocks::Block>() / 4` to determine the block
/// size, then [`cast_slice`] for the zero-copy reinterpretation.
///
/// The first return value is the largest aligned prefix; the second is the
/// sub-block remainder (`0..block_size - 1` values) that the caller must
/// handle separately (e.g. with a `VariableByte` tail).
///
/// # Example
///
/// ```rust,ignore
/// let data: Vec<u32> = (0..600).collect(); // 2 × 256 + 88 remainder
/// let (blocks, remainder) = slice_to_blocks::<FastPForBlock256>(&data);
/// assert_eq!(blocks.len(), 2);    // 2 blocks of [u32; 256]
/// assert_eq!(remainder.len(), 88);
/// ```
#[must_use]
pub fn slice_to_blocks<Blocks: BlockCodec + Sized>(input: &[u32]) -> (&[Blocks::Block], &[u32]) {
    let block_u32s = Blocks::size();
    let aligned_down = (input.len() / block_u32s) * block_u32s;
    let (aligned, remainder) = input.split_at(aligned_down);
    let blocks: &[Blocks::Block] = cast_slice(aligned); // must not panic
    (blocks, remainder)
}
