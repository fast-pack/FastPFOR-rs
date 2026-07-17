//! Element-width abstraction shared by the 32- and 64-bit `FastPFOR` codecs.
//!
//! The block-splitting, best-bit search, exception handling, and metadata layout are
//! identical for `u32` and `u64`; only the element width differs.
//! [`FastPForInt`] abstracts the width-specific pieces so a single [`FastPFor`](super::fastpfor::FastPFor)
//! implements the algorithm once.
//! `u32` keeps its hand-unrolled bit-packing kernels; `u64` uses the generic wide packer.

use crate::helpers::GetWithErr;
use crate::rust::integer_compression::{bitpacking, bitpacking_wide, bitunpacking};
use crate::{FastPForError, FastPForResult};

/// Element type of a `FastPFOR` stream: [`u32`] or [`u64`].
///
/// Implementors supply the width-specific operations the engine needs.
/// The exception bitmap spans [`BITMAP_WORDS`](Self::BITMAP_WORDS) output words.
pub trait FastPForInt: Copy + 'static {
    /// Bit width of the element: 32 or 64.
    const WIDTH: u8;
    /// Output words occupied by the exception bitmap: 1 for `u32`, 2 for `u64`.
    const BITMAP_WORDS: u32;
    /// The zero value.
    const ZERO: Self;

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

    fn significant_bits(self) -> u8 {
        (32 - self.leading_zeros()) as u8
    }
    fn shr(self, n: u8) -> Self {
        self >> n
    }
    fn is_zero(self) -> bool {
        self == 0
    }
    fn or_shl_assign(dst: &mut Self, val: Self, shift: u8) {
        *dst |= val << shift;
    }
    fn one_shl(shift: u8) -> Self {
        1 << shift
    }
    fn fast_pack(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
        bitpacking::fast_pack(src, inpos, out, outpos, bit);
    }
    fn fast_unpack(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8) {
        bitunpacking::fast_unpack(src, inpos, out, outpos, bit);
    }
    fn write_bitmap(bitmap: u64, out: &mut [u32]) {
        out[0] = bitmap as u32;
    }
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

    fn significant_bits(self) -> u8 {
        (64 - self.leading_zeros()) as u8
    }
    fn shr(self, n: u8) -> Self {
        self >> n
    }
    fn is_zero(self) -> bool {
        self == 0
    }
    fn or_shl_assign(dst: &mut Self, val: Self, shift: u8) {
        *dst |= val << shift;
    }
    fn one_shl(shift: u8) -> Self {
        1 << shift
    }
    fn fast_pack(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
        bitpacking_wide::pack_wide(src, inpos, out, outpos, bit);
    }
    fn fast_unpack(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8) {
        bitpacking_wide::unpack_wide(src, inpos, out, outpos, bit);
    }
    fn write_bitmap(bitmap: u64, out: &mut [u32]) {
        out[0] = bitmap as u32;
        out[1] = (bitmap >> 32) as u32;
    }
    fn read_bitmap(input: &[u32], pos: u32) -> FastPForResult<u64> {
        let lo: u32 = input.get_val(pos)?;
        let hi_pos = pos.checked_add(1).ok_or(FastPForError::NotEnoughData)?;
        let hi: u32 = input.get_val(hi_pos)?;
        Ok(u64::from(lo) | (u64::from(hi) << 32))
    }
}
