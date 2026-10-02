use std::fmt::Debug;
use std::io::Cursor;

use crate::FastPForResult;
use crate::rust::integer_compression::fastpfor::FastPFor;
use crate::rust::integer_compression::fastpfor_int::FastPForInt;

#[cfg(target_arch = "x86_64")]
mod avx2;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod neon;
mod portable;

/// Bit-packing kernels used by [`FastPFor`]: [`Scalar`] or [`Simd`]. Sealed.
pub trait Kernels: private::PageCodec + Debug + 'static {}

/// Portable scalar kernels.
#[derive(Debug, Clone, Copy, Default)]
pub struct Scalar;

/// SIMD kernels producing byte-identical output to [`Scalar`]: AVX2 on `x86_64` when detected
/// at runtime, NEON on `aarch64`, and [`Scalar`] otherwise.
#[derive(Debug, Clone, Copy, Default)]
pub struct Simd;

impl Kernels for Scalar {}
impl Kernels for Simd {}

#[cfg(feature = "__testing")]
thread_local! {
    static FORCE_FALLBACK: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Runs `f` with [`Simd`] skipping runtime-detected kernels (AVX2) on the current thread.
#[cfg(feature = "__testing")]
pub fn with_simd_fallback<R>(f: impl FnOnce() -> R) -> R {
    let previous = FORCE_FALLBACK.replace(true);
    let result = f();
    FORCE_FALLBACK.set(previous);
    result
}

pub(crate) mod private {
    use super::portable::{decode_page_scalar, encode_page_scalar};
    use super::{Cursor, FastPFor, FastPForInt, FastPForResult, Kernels};

    /// Encodes and decodes one page; defaults to the portable scalar kernels.
    pub trait PageCodec: Sized {
        fn encode_page<const N: usize, T: FastPForInt>(
            codec: &mut FastPFor<N, T, Self>,
            input: &[T],
            this_size: u32,
            input_offset: &mut Cursor<u32>,
            output: &mut [u32],
            output_offset: &mut Cursor<u32>,
        ) where
            Self: Kernels,
        {
            encode_page_scalar(codec, input, this_size, input_offset, output, output_offset);
        }

        fn decode_page<const N: usize, T: FastPForInt>(
            codec: &mut FastPFor<N, T, Self>,
            input: &[u32],
            input_offset: &mut Cursor<u32>,
            output: &mut [T],
            output_offset: &mut Cursor<u32>,
            this_size: u32,
        ) -> FastPForResult<()>
        where
            Self: Kernels,
        {
            decode_page_scalar(codec, input, input_offset, output, output_offset, this_size)
        }
    }
}

#[cfg(target_arch = "x86_64")]
pub(crate) use avx2::Avx2Int as SimdInt;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub(crate) use neon::NeonInt as SimdInt;
#[cfg(not(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_feature = "neon")
)))]
pub(crate) use portable::fallback::SimdInt;
