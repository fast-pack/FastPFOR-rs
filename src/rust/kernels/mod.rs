use std::fmt::Debug;
use std::io::Cursor;

use crate::FastPForResult;
use crate::rust::integer_compression::fastpfor::FastPFor;
use crate::rust::integer_compression::fastpfor_int::FastPForInt;

#[cfg(target_arch = "x86_64")]
mod avx2;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod neon;

/// Bit-packing kernels used by [`FastPFor`](crate::FastPFor): [`Scalar`] or [`Simd`]. Sealed.
pub trait Kernels: private::Sealed + Debug + 'static {}

/// Portable scalar kernels.
#[derive(Debug, Clone, Copy, Default)]
pub struct Scalar;

/// SIMD kernels producing byte-identical output to [`Scalar`]: AVX2 on `x86_64` when detected
/// at runtime, NEON on `aarch64`, and [`Scalar`] otherwise.
#[derive(Debug, Clone, Copy, Default)]
pub struct Simd;

impl Kernels for Scalar {}
impl Kernels for Simd {}

pub(crate) mod private {
    use super::{Cursor, FastPFor, FastPForInt, FastPForResult, Kernels};

    pub trait Sealed: Sized {
        fn encode_page<const N: usize, T: FastPForInt>(
            codec: &mut FastPFor<N, T, Self>,
            input: &[T],
            this_size: u32,
            input_offset: &mut Cursor<u32>,
            output: &mut [u32],
            output_offset: &mut Cursor<u32>,
        ) where
            Self: Kernels;

        fn decode_page<const N: usize, T: FastPForInt>(
            codec: &mut FastPFor<N, T, Self>,
            input: &[u32],
            input_offset: &mut Cursor<u32>,
            output: &mut [T],
            output_offset: &mut Cursor<u32>,
            this_size: u32,
        ) -> FastPForResult<()>
        where
            Self: Kernels;
    }
}

impl private::Sealed for Scalar {
    fn encode_page<const N: usize, T: FastPForInt>(
        codec: &mut FastPFor<N, T, Self>,
        input: &[T],
        this_size: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) {
        codec.encode_page_with(
            input,
            this_size,
            input_offset,
            output,
            output_offset,
            #[inline(always)]
            |src, inpos, out, outpos, bit| T::fast_pack(src, inpos, out, outpos, bit),
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
        codec.decode_page_with(
            input,
            input_offset,
            output,
            output_offset,
            this_size,
            #[inline(always)]
            |src, inpos, out, outpos, bit| T::fast_unpack(src, inpos, out, outpos, bit),
        )
    }
}

impl private::Sealed for Simd {
    fn encode_page<const N: usize, T: FastPForInt>(
        codec: &mut FastPFor<N, T, Self>,
        input: &[T],
        this_size: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) {
        #[cfg(target_arch = "x86_64")]
        if let Some(token) = avx2::Avx2::detect() {
            T::encode_page_avx2(
                token,
                codec,
                input,
                this_size,
                input_offset,
                output,
                output_offset,
            );
            return;
        }
        codec.encode_page_with(
            input,
            this_size,
            input_offset,
            output,
            output_offset,
            #[inline(always)]
            |src, inpos, out, outpos, bit| fallback_pack::<T>(src, inpos, out, outpos, bit),
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
        #[cfg(target_arch = "x86_64")]
        if let Some(token) = avx2::Avx2::detect() {
            return T::decode_page_avx2(
                token,
                codec,
                input,
                input_offset,
                output,
                output_offset,
                this_size,
            );
        }
        codec.decode_page_with(
            input,
            input_offset,
            output,
            output_offset,
            this_size,
            #[inline(always)]
            |src, inpos, out, outpos, bit| fallback_unpack::<T>(src, inpos, out, outpos, bit),
        )
    }
}

#[expect(clippy::inline_always, reason = "kernel must inline into the page")]
#[inline(always)]
fn fallback_pack<T: FastPForInt>(src: &[T], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    T::pack_neon(src, inpos, out, outpos, bit);
    #[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
    T::fast_pack(src, inpos, out, outpos, bit);
}

#[expect(clippy::inline_always, reason = "kernel must inline into the page")]
#[inline(always)]
fn fallback_unpack<T: FastPForInt>(
    src: &[u32],
    inpos: usize,
    out: &mut [T],
    outpos: usize,
    bit: u8,
) {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    T::unpack_neon(src, inpos, out, outpos, bit);
    #[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
    T::fast_unpack(src, inpos, out, outpos, bit);
}

#[cfg(target_arch = "x86_64")]
pub(crate) use avx2::Avx2Int as SimdInt;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub(crate) use neon::NeonInt as SimdInt;

#[cfg(not(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_feature = "neon")
)))]
pub trait SimdInt {}
#[cfg(not(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_feature = "neon")
)))]
impl SimdInt for u32 {}
#[cfg(not(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_feature = "neon")
)))]
impl SimdInt for u64 {}
