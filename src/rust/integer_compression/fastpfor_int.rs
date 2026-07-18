//! Element-width abstraction shared by the 32- and 64-bit `FastPFOR` codecs.
//!
//! The block-splitting, best-bit search, exception handling, and metadata layout are
//! identical for `u32` and `u64`; only the element width differs.
//! [`FastPForInt`] abstracts the width-specific pieces so a single [`FastPFor`](super::fastpfor::FastPFor)
//! implements the algorithm once. Ordinary arithmetic uses the standard operator traits
//! (`>>`, `<<`, `&`, `|=`) that the trait requires as bounds; only the genuinely
//! width-specific pieces (bit-packing kernels and the exception bitmap layout) are methods.
//! `u32` keeps its hand-unrolled bit-packing kernels; `u64` uses the generic wide packer.
//!
//! The trait is **sealed**: only [`u32`] and [`u64`] implement it, so callers cannot plug in
//! an unsupported element type. Each implementor also fixes the exact size of the per-block
//! scratch buffers (`WIDTH + 1` buckets) as associated types, so no space is wasted and the
//! bucket count never leaks into the public [`FastPFor`](super::fastpfor::FastPFor) signature.

use std::array;
use std::fmt::Debug;
use std::ops::{BitAnd, BitOrAssign, Index, IndexMut, Shl, Shr};

use crate::helpers::GetWithErr;
use crate::rust::integer_compression::{bit_pack32, bit_pack64, bit_unpack32};
use crate::{FastPForError, FastPForResult};

mod sealed {
    pub trait Sealed {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

/// Element type of a `FastPFOR` stream: [`u32`] or [`u64`].
///
/// The operator bounds (`Shr`/`Shl` by `u8`, `BitAnd`, `BitOrAssign`) let the codec use plain
/// `>>`, `<<`, `&`, and `|=` on values; only the width-specific pieces are methods. Implementors
/// also supply the concrete scratch-buffer array types (one bucket per possible bit width,
/// i.e. `WIDTH + 1`). The exception bitmap spans [`BITMAP_WORDS`](Self::BITMAP_WORDS) output words.
///
/// This trait is sealed and cannot be implemented outside this crate.
pub trait FastPForInt:
    Copy
    + 'static
    + Eq
    + sealed::Sealed
    + Shr<u8, Output = Self>
    + Shl<u8, Output = Self>
    + BitAnd<Output = Self>
    + BitOrAssign
{
    /// Bit width of the element: 32 or 64.
    const WIDTH: u8 = (size_of::<Self>() * 8) as u8;
    /// Output words occupied by the exception bitmap: 1 for `u32`, 2 for `u64`.
    const BITMAP_WORDS: u32 = Self::WIDTH as u32 / u32::BITS;
    /// The zero value.
    const ZERO: Self;
    /// The one value.
    const ONE: Self;

    /// Exception values grouped by bit-width bucket: `[Vec<Self>; WIDTH + 1]`.
    type ExceptionBuffers: Index<usize, Output = Vec<Self>> + IndexMut<usize> + Debug;
    /// Per-bit-width frequency counts: `[u32; WIDTH + 1]`.
    type Freqs: Index<usize, Output = u32> + IndexMut<usize> + AsMut<[u32]> + Debug;
    /// Write positions into `ExceptionBuffers`: `[usize; WIDTH + 1]`.
    type DataPointers: Index<usize, Output = usize> + IndexMut<usize> + AsMut<[usize]> + Debug;

    /// Fresh, empty exception buffers.
    fn new_exception_buffers() -> Self::ExceptionBuffers;
    /// Fresh, zeroed frequency counts.
    fn new_freqs() -> Self::Freqs;
    /// Fresh, zeroed data pointers.
    fn new_data_pointers() -> Self::DataPointers;

    /// Number of significant bits, i.e. `WIDTH - leading_zeros` (0 for a zero value).
    fn significant_bits(self) -> u8;

    /// Pack 32 values at `bit` bits each into `out`.
    fn fast_pack(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8);
    /// Unpack 32 values at `bit` bits each from `src`.
    fn fast_unpack(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8);

    /// Write the exception bitmap as [`BITMAP_WORDS`](Self::BITMAP_WORDS) words at `out`.
    fn write_bitmap(bitmap: Self, out: &mut [u32]);
    /// Read the exception bitmap from [`BITMAP_WORDS`](Self::BITMAP_WORDS) words at `pos`.
    fn read_bitmap(input: &[u32], pos: u32) -> FastPForResult<Self>;
}

#[allow(
    clippy::use_self,
    reason = "u32 here is the stream word type, not the Self element type"
)]
impl FastPForInt for u32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    type ExceptionBuffers = [Vec<Self>; u32::BITS as usize + 1];
    type Freqs = [u32; u32::BITS as usize + 1];
    type DataPointers = [usize; u32::BITS as usize + 1];

    fn new_exception_buffers() -> Self::ExceptionBuffers {
        array::from_fn(|_| Vec::new())
    }
    fn new_freqs() -> Self::Freqs {
        [0; u32::BITS as usize + 1]
    }
    fn new_data_pointers() -> Self::DataPointers {
        [0; u32::BITS as usize + 1]
    }

    #[inline]
    fn significant_bits(self) -> u8 {
        Self::WIDTH - self.leading_zeros() as u8
    }
    // `inline(always)`: this is a thin forwarder; without it the wrapper accumulates the whole
    // inlined kernel and then exceeds the inline threshold, so `decode_page`/`encode_page` would
    // emit a real call per 32-value group instead of inlining the kernel (as the concrete `u32`
    // code on `main` does). See the packing-kernel benchmarks.
    #[inline(always)]
    #[allow(clippy::inline_always, reason = "thin forwarder; see comment above")]
    fn fast_pack(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
        bit_pack32::fast_pack(src, inpos, out, outpos, bit);
    }
    #[inline(always)]
    #[allow(clippy::inline_always, reason = "thin forwarder; see comment above")]
    fn fast_unpack(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8) {
        bit_unpack32::fast_unpack(src, inpos, out, outpos, bit);
    }
    #[inline]
    fn write_bitmap(bitmap: Self, out: &mut [u32]) {
        out[0] = bitmap;
    }
    #[inline]
    fn read_bitmap(input: &[u32], pos: u32) -> FastPForResult<Self> {
        input.get_val(pos)
    }
}

#[allow(
    clippy::use_self,
    reason = "u32 here is the stream word type, not the Self element type"
)]
impl FastPForInt for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    type ExceptionBuffers = [Vec<Self>; u64::BITS as usize + 1];
    type Freqs = [u32; u64::BITS as usize + 1];
    type DataPointers = [usize; u64::BITS as usize + 1];

    fn new_exception_buffers() -> Self::ExceptionBuffers {
        array::from_fn(|_| Vec::new())
    }
    fn new_freqs() -> Self::Freqs {
        [0; u64::BITS as usize + 1]
    }
    fn new_data_pointers() -> Self::DataPointers {
        [0; u64::BITS as usize + 1]
    }

    #[inline]
    fn significant_bits(self) -> u8 {
        Self::WIDTH - self.leading_zeros() as u8
    }
    // `inline(always)`: forward directly to the wide kernel (see the `u32` impl for rationale).
    #[inline(always)]
    #[allow(clippy::inline_always, reason = "thin forwarder; see comment above")]
    fn fast_pack(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
        bit_pack64::pack_wide(src, inpos, out, outpos, bit);
    }
    #[inline(always)]
    #[allow(clippy::inline_always, reason = "thin forwarder; see comment above")]
    fn fast_unpack(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8) {
        bit_pack64::unpack_wide(src, inpos, out, outpos, bit);
    }
    #[inline]
    fn write_bitmap(bitmap: Self, out: &mut [u32]) {
        out[0] = bitmap as u32;
        out[1] = (bitmap >> 32) as u32;
    }
    #[inline]
    fn read_bitmap(input: &[u32], pos: u32) -> FastPForResult<Self> {
        let lo: u32 = input.get_val(pos)?;
        let hi_pos = pos.checked_add(1).ok_or(FastPForError::NotEnoughData)?;
        let hi: u32 = input.get_val(hi_pos)?;
        Ok(u64::from(lo) | (u64::from(hi) << 32))
    }
}
