use std::fmt::Debug;
use std::io::Cursor;

use crate::FastPForResult;
use crate::rust::integer_compression::fastpfor::FastPForBlock;
use crate::rust::integer_compression::fastpfor_int::FastPForInt;

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
mod avx2;
mod lanes;
#[cfg(all(
    feature = "simd",
    any(
        target_arch = "x86_64",
        all(
            target_arch = "aarch64",
            target_feature = "neon",
            target_endian = "little"
        )
    )
))]
mod lanes_wide;
#[cfg(all(
    feature = "simd",
    target_arch = "aarch64",
    target_feature = "neon",
    target_endian = "little"
))]
mod neon;
mod portable;

pub(crate) use lanes::LaneElem;

/// Wire format of a `FastPFor` stream: [`Sequential`] or [`Interleaved`]. Sealed.
///
/// The formats are not interchangeable, and streams carry no format tag: decoding one as the other
/// is not detected, and silently returns wrong values. Choose one per data set.
pub trait Layout: private::PageLayout + Debug + 'static {}

/// The sequential format: the layout of the C++ `FastPFor` codec (`CppFastPFor128` / `CppFastPFor256`).
///
/// Each block is bit-packed 32 values at a time, as one continuous little-endian bitstream.
/// Byte-identical to C++ for both `u32` and `u64`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Sequential;

/// The interleaved format: the layout of the C++ `SIMDFastPFor` codec (`CppSimdFastPFor128` / `CppSimdFastPFor256`).
///
/// Each block is bit-packed 128 values at a time, interleaved over the lanes of a 128-bit vector:
/// value `i` goes to lane `i % 4` (`u32`) or `i % 2` (`u64`), and each lane is packed on its own.
/// The bulk of each exception array uses the same layout. The encoder's bit-width choice also differs
/// slightly from [`Sequential`], so the encoded sizes can differ by a word or two.
///
/// Byte-identical to C++ `SIMDFastPFor` for `u32`. C++ has no 64-bit `SIMDFastPFor`: the `u64` format
/// is this crate's extension of the same layout, with two 64-bit lanes (low word first).
#[derive(Debug, Clone, Copy, Default)]
pub struct Interleaved;

impl Layout for Sequential {}
impl Layout for Interleaved {}

/// Bit-packing kernels used by [`FastPForBlock`]: [`Auto`] or [`Portable`]. Sealed.
///
/// Kernels only affect speed: for a given [`Layout`], both write the same bytes.
pub trait Kernels: private::PageCodec + Debug + 'static {}

/// Portable kernels: plain Rust with no SIMD intrinsics and no `unsafe` code.
///
/// The compiler may still vectorize them for the target.
#[derive(Debug, Clone, Copy, Default)]
pub struct Portable;

/// The default kernels: the fastest implementation available in this build, byte-identical to [`Portable`].
///
/// - [`Sequential`] layout: AVX2 on `x86_64` when the CPU has it (detected at runtime), NEON on little-endian
///   `aarch64`.
/// - [`Interleaved`] layout: SSE2 on `x86_64` and NEON on little-endian `aarch64`, which those targets always have.
/// - Anywhere else, and in builds without the default `simd` cargo feature: [`Portable`].
///
/// Use [`Portable`] instead to always run the portable code, for example in tests and benchmarks.
#[derive(Debug, Clone, Copy, Default)]
pub struct Auto;

impl Kernels for Portable {}
impl Kernels for Auto {}

#[cfg(feature = "__testing")]
thread_local! {
    static FORCE_FALLBACK: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Runs `f` with [`Auto`] skipping its runtime-detected kernels (AVX2) on the current thread.
#[cfg(feature = "__testing")]
pub fn with_simd_fallback<R>(f: impl FnOnce() -> R) -> R {
    let previous = FORCE_FALLBACK.replace(true);
    let result = f();
    FORCE_FALLBACK.set(previous);
    result
}

/// An implementation of the page kernels, as reported by [`implementation`].
#[cfg(feature = "__testing")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Implementation {
    /// AVX2 intrinsics (`x86_64`, detected at runtime).
    Avx2,
    /// SSE2 vectors (`x86_64`).
    Sse2,
    /// NEON vectors (`aarch64`).
    Neon,
    /// The portable code.
    Portable,
}

/// The implementation that kernels `K` use for layout `L` on this target and CPU, so tests can assert that
/// [`Auto`] really runs SIMD code where it should, instead of comparing the portable code with itself.
///
/// Each kernel implementation reports its own choice next to the code that makes it, using the same checks.
#[cfg(feature = "__testing")]
#[must_use]
pub fn implementation<L: Layout, K: Kernels>() -> Implementation {
    L::implementation::<K>()
}

pub(crate) mod private {
    use super::portable::{decode_page_scalar, encode_page_scalar};
    use super::{
        Cursor, FastPForBlock, FastPForInt, FastPForResult, Interleaved, Kernels, Layout,
        Sequential, lanes,
    };

    /// Encodes and decodes one page in each layout. Defaults to the portable kernels.
    pub trait PageCodec: Sized {
        /// The implementation the sequential methods use; reported by [`super::implementation`].
        #[cfg(feature = "__testing")]
        fn sequential_implementation() -> super::Implementation {
            super::Implementation::Portable
        }

        /// The implementation the interleaved methods use; reported by [`super::implementation`].
        #[cfg(feature = "__testing")]
        fn interleaved_implementation() -> super::Implementation {
            super::Implementation::Portable
        }

        fn encode_sequential<L: Layout, T: FastPForInt, const N: usize>(
            codec: &mut FastPForBlock<L, T, N, Self>,
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

        fn decode_sequential<L: Layout, T: FastPForInt, const N: usize>(
            codec: &mut FastPForBlock<L, T, N, Self>,
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

        fn encode_interleaved<L: Layout, T: FastPForInt, const N: usize>(
            codec: &mut FastPForBlock<L, T, N, Self>,
            input: &[T],
            this_size: u32,
            input_offset: &mut Cursor<u32>,
            output: &mut [u32],
            output_offset: &mut Cursor<u32>,
        ) where
            Self: Kernels,
        {
            lanes::encode_page_lanes::<L, T, N, Self, lanes::Arrays>(
                codec,
                input,
                this_size,
                input_offset,
                output,
                output_offset,
            );
        }

        fn decode_interleaved<L: Layout, T: FastPForInt, const N: usize>(
            codec: &mut FastPForBlock<L, T, N, Self>,
            input: &[u32],
            input_offset: &mut Cursor<u32>,
            output: &mut [T],
            output_offset: &mut Cursor<u32>,
            this_size: u32,
        ) -> FastPForResult<()>
        where
            Self: Kernels,
        {
            lanes::decode_page_lanes::<L, T, N, Self, lanes::Arrays>(
                codec,
                input,
                input_offset,
                output,
                output_offset,
                this_size,
            )
        }
    }

    /// Routes a page to the kernels of its layout. Dispatching on the type, rather than on a
    /// constant, means only the layout's own kernels are instantiated.
    pub trait PageLayout: Sized {
        #[cfg(feature = "__testing")]
        fn implementation<K: Kernels>() -> super::Implementation;

        fn encode_page<T: FastPForInt, const N: usize, K: Kernels>(
            codec: &mut FastPForBlock<Self, T, N, K>,
            input: &[T],
            this_size: u32,
            input_offset: &mut Cursor<u32>,
            output: &mut [u32],
            output_offset: &mut Cursor<u32>,
        ) where
            Self: Layout;

        fn decode_page<T: FastPForInt, const N: usize, K: Kernels>(
            codec: &mut FastPForBlock<Self, T, N, K>,
            input: &[u32],
            input_offset: &mut Cursor<u32>,
            output: &mut [T],
            output_offset: &mut Cursor<u32>,
            this_size: u32,
        ) -> FastPForResult<()>
        where
            Self: Layout;
    }

    impl PageLayout for Sequential {
        #[cfg(feature = "__testing")]
        fn implementation<K: Kernels>() -> super::Implementation {
            K::sequential_implementation()
        }

        fn encode_page<T: FastPForInt, const N: usize, K: Kernels>(
            codec: &mut FastPForBlock<Self, T, N, K>,
            input: &[T],
            this_size: u32,
            input_offset: &mut Cursor<u32>,
            output: &mut [u32],
            output_offset: &mut Cursor<u32>,
        ) {
            K::encode_sequential(codec, input, this_size, input_offset, output, output_offset);
        }

        fn decode_page<T: FastPForInt, const N: usize, K: Kernels>(
            codec: &mut FastPForBlock<Self, T, N, K>,
            input: &[u32],
            input_offset: &mut Cursor<u32>,
            output: &mut [T],
            output_offset: &mut Cursor<u32>,
            this_size: u32,
        ) -> FastPForResult<()> {
            K::decode_sequential(codec, input, input_offset, output, output_offset, this_size)
        }
    }

    impl PageLayout for Interleaved {
        #[cfg(feature = "__testing")]
        fn implementation<K: Kernels>() -> super::Implementation {
            K::interleaved_implementation()
        }

        fn encode_page<T: FastPForInt, const N: usize, K: Kernels>(
            codec: &mut FastPForBlock<Self, T, N, K>,
            input: &[T],
            this_size: u32,
            input_offset: &mut Cursor<u32>,
            output: &mut [u32],
            output_offset: &mut Cursor<u32>,
        ) {
            K::encode_interleaved(codec, input, this_size, input_offset, output, output_offset);
        }

        fn decode_page<T: FastPForInt, const N: usize, K: Kernels>(
            codec: &mut FastPForBlock<Self, T, N, K>,
            input: &[u32],
            input_offset: &mut Cursor<u32>,
            output: &mut [T],
            output_offset: &mut Cursor<u32>,
            this_size: u32,
        ) -> FastPForResult<()> {
            K::decode_interleaved(codec, input, input_offset, output, output_offset, this_size)
        }
    }
}

/// Narrows 32 `u64` values to their low 32 bits and passes them to a 32-bit `pack32` kernel.
#[cfg(all(
    feature = "simd",
    any(
        target_arch = "x86_64",
        all(
            target_arch = "aarch64",
            target_feature = "neon",
            target_endian = "little"
        )
    )
))]
#[expect(clippy::inline_always, reason = "kernel must inline into the page")]
#[inline(always)]
fn pack_narrowed(src: &[u64], inpos: usize, pack32: impl FnOnce(&[u32])) {
    let mut narrow = [0u32; 32];
    for (n, &v) in narrow.iter_mut().zip(&src[inpos..inpos + 32]) {
        *n = v as u32;
    }
    pack32(&narrow);
}

/// Unpacks 32 values of at most 32 bits each through a 32-bit `unpack32` kernel into `out`.
#[cfg(all(
    feature = "simd",
    any(
        target_arch = "x86_64",
        all(
            target_arch = "aarch64",
            target_feature = "neon",
            target_endian = "little"
        )
    )
))]
#[expect(clippy::inline_always, reason = "kernel must inline into the page")]
#[inline(always)]
fn unpack_narrowed(out: &mut [u64], unpack32: impl FnOnce(&mut [u32])) {
    let mut narrow = [0u32; 32];
    unpack32(&mut narrow);
    for (o, v) in out[..32].iter_mut().zip(narrow) {
        *o = u64::from(v);
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
pub(crate) use avx2::Avx2Int as SimdInt;
#[cfg(all(
    feature = "simd",
    target_arch = "aarch64",
    target_feature = "neon",
    target_endian = "little"
))]
pub(crate) use neon::NeonInt as SimdInt;
#[cfg(not(all(
    feature = "simd",
    any(
        target_arch = "x86_64",
        all(
            target_arch = "aarch64",
            target_feature = "neon",
            target_endian = "little"
        )
    )
)))]
pub(crate) use portable::fallback::SimdInt;
