//! Vector backend of the interleaved kernels, on the [`wide`] crate: SSE2 on `x86_64` and NEON on
//! `aarch64`, where both are part of the baseline, so there is no runtime detection and no `unsafe`.
#![expect(
    clippy::inline_always,
    reason = "the lane operations must inline into the unrolled kernels, whatever the caller"
)]

use std::io::Cursor;

use bytemuck::cast;
use wide::{u32x4, u64x2};

use crate::FastPForResult;
use crate::rust::integer_compression::fastpfor::FastPFor;
use crate::rust::integer_compression::fastpfor_int::FastPForInt;
use crate::rust::kernels::lanes::{Backend, Lane, decode_page_lanes, encode_page_lanes};
use crate::rust::kernels::{Interleaved, private};

/// The vector backend: `wide`'s 128-bit types.
pub struct Wide;

/// Four `u32` lanes in one vector register.
#[derive(Clone, Copy)]
pub struct W32(u32x4);

/// Two `u64` lanes in one vector register.
#[derive(Clone, Copy)]
pub struct W64(u64x2);

impl Backend for Wide {
    type V32 = W32;
    type V64 = W64;
}

impl Lane for W32 {
    type E = u32;
    const BITS: u32 = 32;
    const LANES: usize = 4;

    #[inline(always)]
    fn from_words(words: &[u32]) -> Self {
        Self::from_elems(words)
    }
    #[inline(always)]
    fn to_words(self, out: &mut [u32]) {
        self.to_elems(out);
    }
    #[inline(always)]
    fn from_elems(src: &[u32]) -> Self {
        Self(u32x4::new(src[..4].try_into().expect("4-element subslice")))
    }
    #[inline(always)]
    fn to_elems(self, out: &mut [u32]) {
        out[..4].copy_from_slice(&self.0.to_array());
    }
    #[inline(always)]
    fn mask(bits: u32) -> Self {
        Self(u32x4::splat(u32::MAX >> (32 - bits)))
    }
    #[inline(always)]
    fn zero() -> Self {
        Self(u32x4::splat(0))
    }
    #[inline(always)]
    fn and(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
    #[inline(always)]
    fn or(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
    #[inline(always)]
    fn shl(self, s: u32) -> Self {
        Self(self.0 << s)
    }
    #[inline(always)]
    fn shr(self, s: u32) -> Self {
        Self(self.0 >> s)
    }
}

impl Lane for W64 {
    type E = u64;
    const BITS: u32 = 64;
    const LANES: usize = 2;

    #[inline(always)]
    fn from_words(words: &[u32]) -> Self {
        // Both targets are little-endian here (see the module gate in `kernels`):
        // the low word of each lane comes first, as the layout requires.
        Self(cast::<[u32; 4], u64x2>(
            words[..4].try_into().expect("4-word subslice"),
        ))
    }
    #[inline(always)]
    fn to_words(self, out: &mut [u32]) {
        out[..4].copy_from_slice(&cast::<u64x2, [u32; 4]>(self.0));
    }
    #[inline(always)]
    fn from_elems(src: &[u64]) -> Self {
        Self(u64x2::new(src[..2].try_into().expect("2-element subslice")))
    }
    #[inline(always)]
    fn to_elems(self, out: &mut [u64]) {
        out[..2].copy_from_slice(&self.0.to_array());
    }
    #[inline(always)]
    fn mask(bits: u32) -> Self {
        Self(u64x2::splat(u64::MAX >> (64 - bits)))
    }
    #[inline(always)]
    fn zero() -> Self {
        Self(u64x2::splat(0))
    }
    #[inline(always)]
    fn and(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
    #[inline(always)]
    fn or(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
    #[inline(always)]
    fn shl(self, s: u32) -> Self {
        Self(self.0 << s)
    }
    #[inline(always)]
    fn shr(self, s: u32) -> Self {
        Self(self.0 >> s)
    }
}

/// [`Interleaved`] on `x86_64` (SSE2) and `aarch64` (NEON).
impl private::PageCodec for Interleaved {
    fn encode_page<const N: usize, T: FastPForInt>(
        codec: &mut FastPFor<N, T, Self>,
        input: &[T],
        this_size: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) {
        encode_page_lanes::<N, T, Self, Wide>(
            codec,
            input,
            this_size,
            input_offset,
            output,
            output_offset,
        );
    }

    fn decode_page<const N: usize, T: FastPForInt>(
        codec: &mut FastPFor<N, T, Self>,
        input: &[u32],
        input_offset: &mut Cursor<u32>,
        output: &mut [T],
        output_offset: &mut Cursor<u32>,
        this_size: u32,
    ) -> FastPForResult<()> {
        decode_page_lanes::<N, T, Self, Wide>(
            codec,
            input,
            input_offset,
            output,
            output_offset,
            this_size,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::kernels::lanes::tests::check_backend;

    #[test]
    fn wide_matches_model() {
        check_backend::<Wide>();
    }
}
