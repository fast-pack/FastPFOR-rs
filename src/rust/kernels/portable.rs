use std::io::Cursor;

use crate::FastPForResult;
use crate::rust::integer_compression::fastpfor::FastPFor;
use crate::rust::integer_compression::fastpfor_int::FastPForInt;
use crate::rust::kernels::{Kernels, Scalar, private};

impl private::PageCodec for Scalar {}

pub(super) fn encode_page_scalar<const N: usize, T: FastPForInt, K: Kernels>(
    codec: &mut FastPFor<N, T, K>,
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

#[inline]
pub(super) fn decode_page_scalar<const N: usize, T: FastPForInt, K: Kernels>(
    codec: &mut FastPFor<N, T, K>,
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

/// Targets without SIMD kernels: [`Simd`](super::Simd) uses the scalar kernels.
#[cfg(not(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_feature = "neon")
)))]
pub(super) mod fallback {
    pub trait SimdInt {}
    impl SimdInt for u32 {}
    impl SimdInt for u64 {}

    impl super::private::PageCodec for crate::rust::kernels::Simd {}
}
