use bytemuck::{Pod, cast_slice};

use crate::FastPForResult;

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
/// ```
/// # use fastpfor::{BlockCodec, FastPForResult};
/// #[derive(Default)]
/// struct MyCodec;
/// impl BlockCodec for MyCodec {
///     type Elem = u32;
///     type Block = [u32; 256];
///     fn encode_blocks(&mut self, blocks: &[[u32; 256]], out: &mut Vec<u32>)
///         -> FastPForResult<()> { todo!() }
///     fn decode_blocks(&mut self, input: &[u32], expected_len: Option<u32>,
///         out: &mut Vec<u32>) -> FastPForResult<usize> { todo!() }
/// }
/// ```
pub trait BlockCodec: Default {
    /// The unpacked element type: [`u32`] or [`u64`].
    /// The compressed stream is always `Vec<u32>`; only decoded values use this width.
    type Elem: Pod;

    /// The fixed-size block type, which must be plain-old-data (`Pod`).
    /// In practice `[u32; 128]`, `[u32; 256]`, `[u64; 128]`, or `[u64; 256]`.
    type Block: Pod;

    /// Number of [`Elem`](BlockCodec::Elem) values in one block.
    ///
    /// Equal to `size_of::<Self::Block>() / size_of::<Self::Elem>()`.
    /// Use this when computing element counts from block counts.
    #[inline]
    #[must_use]
    fn size() -> usize
    where
        Self: Sized,
    {
        size_of::<Self::Block>() / size_of::<Self::Elem>()
    }

    /// Compress a slice of complete, fixed-size blocks.
    ///
    /// No remainder is possible — the caller must split the input first using
    /// [`slice_to_blocks`] and handle any remainder separately.
    fn encode_blocks(&mut self, blocks: &[Self::Block], out: &mut Vec<u32>) -> FastPForResult<()>;

    /// Decompress blocks from `input`, using the length stored in the header.
    ///
    /// Returns the number of input `u32` words consumed, so the caller (e.g.
    /// [`CompositeCodec`](crate::CompositeCodec)) can locate the tail without parsing the block format.
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
        out: &mut Vec<Self::Elem>,
    ) -> FastPForResult<usize>;

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
/// Implemented by the pure-Rust [`FastPForWide128`](crate::FastPForWide128) and
/// [`FastPForWide256`](crate::FastPForWide256) codecs.
/// With the `cpp` feature, `CppFastPFor128`, `CppFastPFor256`, and `CppVarInt` also implement it.
///
/// This is a shared interface for cross-codec comparison; for native Rust use,
/// [`FastPForWide128`](crate::FastPForWide128) also implements [`AnyLenCodec`] with `Elem = u64`.
/// Import `BlockCodec64` only when writing generic code over several 64-bit codecs.
pub trait BlockCodec64 {
    /// Compress 64-bit integers into a 32-bit word stream.
    fn encode64(&mut self, input: &[u64], out: &mut Vec<u32>) -> FastPForResult<()>;
    /// Decompress 64-bit integers from a 32-bit word stream.
    fn decode64(&mut self, input: &[u32], out: &mut Vec<u64>) -> FastPForResult<()>;
}

/// Compresses and decompresses an arbitrary-length `&[u32]` slice.
///
/// Handles any input length including sub-block remainders.  All pure
/// variable-length codecs (e.g. `VariableByte`, `JustCopy`) implement this
/// trait directly.  Block-oriented codecs are wrapped in `CompositeCodec`
/// to produce an `AnyLenCodec`.
pub trait AnyLenCodec: Default {
    /// The unpacked element type: [`u32`] or [`u64`].
    /// The compressed stream is always `Vec<u32>`; only decoded values use this width.
    type Elem;

    /// Compress an arbitrary-length slice of [`Elem`](AnyLenCodec::Elem) values.
    fn encode(&mut self, input: &[Self::Elem], out: &mut Vec<u32>) -> FastPForResult<()>;

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
        out: &mut Vec<Self::Elem>,
        expected_len: Option<u32>,
    ) -> FastPForResult<()>;
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
/// ```
/// # use fastpfor::{slice_to_blocks, FastPForBlock256};
/// let data: Vec<u32> = (0..600).collect(); // 2 × 256 + 88 remainder
/// let (blocks, remainder) = slice_to_blocks::<FastPForBlock256>(&data);
/// assert_eq!(blocks.len(), 2);    // 2 blocks of [u32; 256]
/// assert_eq!(remainder.len(), 88);
/// ```
#[must_use]
pub fn slice_to_blocks<Blocks: BlockCodec + Sized>(
    input: &[Blocks::Elem],
) -> (&[Blocks::Block], &[Blocks::Elem]) {
    let block_elems = Blocks::size();
    let aligned_down = (input.len() / block_elems) * block_elems;
    let (aligned, remainder) = input.split_at(aligned_down);
    let blocks: &[Blocks::Block] = cast_slice(aligned); // must not panic
    (blocks, remainder)
}
