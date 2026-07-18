//! Element-width abstraction shared by the 32- and 64-bit `FastPFOR` codecs.
//!
//! The block-splitting, best-bit search, exception handling, and metadata layout are
//! identical for `u32` and `u64`; only the element width differs.
//! [`FastPForInt`] abstracts the width-specific pieces so a single [`FastPFor`](super::fastpfor::FastPFor)
//! implements the algorithm once.
//! `u32` keeps its hand-unrolled bit-packing kernels; `u64` uses the generic wide packer.
//!
//! The trait is **sealed**: only [`u32`] and [`u64`] implement it, so callers cannot plug in
//! an unsupported element type. Each implementor also fixes the exact size of the per-block
//! scratch buffers (`WIDTH + 1` buckets) as associated types, so no space is wasted and the
//! bucket count never leaks into the public [`FastPFor`](super::fastpfor::FastPFor) signature.

use std::array;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use crate::helpers::GetWithErr;
use crate::rust::integer_compression::{bitpacking, bitpacking_wide, bitunpacking};
use crate::{FastPForError, FastPForResult};

mod sealed {
    pub trait Sealed {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

/// Element type of a `FastPFOR` stream: [`u32`] or [`u64`].
///
/// Implementors supply the width-specific operations the engine needs, plus the concrete
/// scratch-buffer array types (one bucket per possible bit width, i.e. `WIDTH + 1`).
/// The exception bitmap spans [`BITMAP_WORDS`](Self::BITMAP_WORDS) output words.
///
/// This trait is sealed and cannot be implemented outside this crate.
pub trait FastPForInt: Copy + 'static + sealed::Sealed {
    /// Bit width of the element: 32 or 64.
    const WIDTH: u8;
    /// Output words occupied by the exception bitmap: 1 for `u32`, 2 for `u64`.
    const BITMAP_WORDS: u32;
    /// The zero value.
    const ZERO: Self;

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
    /// Logical right shift by `n`, where `n < WIDTH`.
    fn shr(self, n: u8) -> Self;
    /// Whether the value is zero.
    fn is_zero(self) -> bool;
    /// `*dst |= val << shift`, where `shift < WIDTH`.
    fn or_shl_assign(dst: &mut Self, val: Self, shift: u8);
    /// `1 << shift`, where `shift < WIDTH`.
    fn one_shl(shift: u8) -> Self;

    /// Pack 32 values at `bit` bits each into `out`.
    fn fast_pack(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8);
    /// Unpack 32 values at `bit` bits each from `src`.
    fn fast_unpack(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8);

    /// Write the exception bitmap as [`BITMAP_WORDS`](Self::BITMAP_WORDS) words at `out`.
    fn write_bitmap(bitmap: u64, out: &mut [u32]);
    /// Read the exception bitmap from [`BITMAP_WORDS`](Self::BITMAP_WORDS) words at `pos`.
    fn read_bitmap(input: &[u32], pos: u32) -> FastPForResult<u64>;
}

#[allow(
    clippy::use_self,
    reason = "u32 literals here are stream words, not the Self element type"
)]
impl FastPForInt for u32 {
    const WIDTH: u8 = 32;
    const BITMAP_WORDS: u32 = 1;
    const ZERO: Self = 0;

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
        (32 - self.leading_zeros()) as u8
    }
    #[inline]
    fn shr(self, n: u8) -> Self {
        self >> n
    }
    #[inline]
    fn is_zero(self) -> bool {
        self == 0
    }
    #[inline]
    fn or_shl_assign(dst: &mut Self, val: Self, shift: u8) {
        *dst |= val << shift;
    }
    #[inline]
    fn one_shl(shift: u8) -> Self {
        1 << shift
    }
    #[inline]
    fn fast_pack(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
        bitpacking::fast_pack(src, inpos, out, outpos, bit);
    }
    #[inline]
    fn fast_unpack(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8) {
        bitunpacking::fast_unpack(src, inpos, out, outpos, bit);
    }
    #[inline]
    fn write_bitmap(bitmap: u64, out: &mut [u32]) {
        out[0] = bitmap as u32;
    }
    #[inline]
    fn read_bitmap(input: &[u32], pos: u32) -> FastPForResult<u64> {
        let word: u32 = input.get_val(pos)?;
        Ok(u64::from(word))
    }
}

#[allow(
    clippy::use_self,
    reason = "u32 literals here are stream words, not the Self element type"
)]
impl FastPForInt for u64 {
    const WIDTH: u8 = 64;
    const BITMAP_WORDS: u32 = 2;
    const ZERO: Self = 0;

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
        (64 - self.leading_zeros()) as u8
    }
    #[inline]
    fn shr(self, n: u8) -> Self {
        self >> n
    }
    #[inline]
    fn is_zero(self) -> bool {
        self == 0
    }
    #[inline]
    fn or_shl_assign(dst: &mut Self, val: Self, shift: u8) {
        *dst |= val << shift;
    }
    #[inline]
    fn one_shl(shift: u8) -> Self {
        1 << shift
    }
    #[inline]
    fn fast_pack(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
        bitpacking_wide::pack_wide(src, inpos, out, outpos, bit);
    }
    #[inline]
    fn fast_unpack(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8) {
        bitpacking_wide::unpack_wide(src, inpos, out, outpos, bit);
    }
    #[inline]
    fn write_bitmap(bitmap: u64, out: &mut [u32]) {
        out[0] = bitmap as u32;
        out[1] = (bitmap >> 32) as u32;
    }
    #[inline]
    fn read_bitmap(input: &[u32], pos: u32) -> FastPForResult<u64> {
        let lo: u32 = input.get_val(pos)?;
        let hi_pos = pos.checked_add(1).ok_or(FastPForError::NotEnoughData)?;
        let hi: u32 = input.get_val(hi_pos)?;
        Ok(u64::from(lo) | (u64::from(hi) << 32))
    }
}
