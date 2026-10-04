//! Public any-length `FastPFOR` codecs.
//!
//! [`FastPForCodec`] pairs the [`FastPForBlock`] engine with a [`VariableByte`] tail for the values after the
//! last whole block. The eight aliases name every combination of wire format, value width and block size,
//! as `FastPFor<Layout><value bits>x<block size>`, for example [`FastPForSequential32x128`]. Each has a
//! block-only counterpart, `FastPFor<Layout>Block<value bits>x<block size>`.

use crate::FastPForResult;
use crate::codec::{AnyLenCodec, BlockCodec64};
use crate::rust::VariableByte;
use crate::rust::composite::CompositeCodec;
use crate::rust::integer_compression::fastpfor::{FastPForBlock, sealed};
use crate::rust::integer_compression::fastpfor_int::FastPForInt;
use crate::rust::kernels::{Auto, Interleaved, Kernels, Layout, Sequential};

/// Any-length `FastPFOR` codec: a [`CompositeCodec`] of a [`FastPForBlock`] and a [`VariableByte`] tail.
///
/// The parameters are those of [`FastPForBlock`]: the wire [`Layout`] `L`, the element type `T`
/// ([`u32`] or [`u64`]), the block size `N` (128 or 256), and the [`Kernels`] `K`, which only affect
/// speed. Usually used through the aliases, such as [`FastPForSequential32x128`].
#[derive(Debug)]
pub struct FastPForCodec<L: Layout, T: FastPForInt, const N: usize, K: Kernels = Auto>
where
    [T; N]: sealed::BlockSize,
    VariableByte<T>: AnyLenCodec<Elem = T>,
{
    inner: CompositeCodec<FastPForBlock<L, T, N, K>, VariableByte<T>>,
}

// Hand-written (not derived) so `default()` needs no `T: Default` bound;
// the tail's `AnyLenCodec: Default` supertrait already guarantees it.
impl<L: Layout, T: FastPForInt, const N: usize, K: Kernels> Default for FastPForCodec<L, T, N, K>
where
    [T; N]: sealed::BlockSize,
    VariableByte<T>: AnyLenCodec<Elem = T>,
{
    fn default() -> Self {
        Self {
            inner: CompositeCodec::default(),
        }
    }
}

impl<L: Layout, T: FastPForInt, const N: usize, K: Kernels> AnyLenCodec
    for FastPForCodec<L, T, N, K>
where
    [T; N]: sealed::BlockSize,
    VariableByte<T>: AnyLenCodec<Elem = T>,
{
    type Elem = T;

    fn encode(&mut self, input: &[T], out: &mut Vec<u32>) -> FastPForResult<()> {
        self.inner.encode(input, out)
    }

    fn decode(
        &mut self,
        input: &[u32],
        out: &mut Vec<T>,
        expected_len: Option<u32>,
    ) -> FastPForResult<()> {
        self.inner.decode(input, out, expected_len)
    }
}

/// Compresses 64-bit integers through the shared [`BlockCodec64`] interface.
///
/// Lets the `u64` codecs be compared against the C++ codecs, which expose `u64` the same way.
impl<L: Layout, const N: usize, K: Kernels> BlockCodec64 for FastPForCodec<L, u64, N, K>
where
    [u64; N]: sealed::BlockSize,
{
    fn encode64(&mut self, input: &[u64], out: &mut Vec<u32>) -> FastPForResult<()> {
        self.inner.encode(input, out)
    }

    fn decode64(&mut self, input: &[u32], out: &mut Vec<u64>) -> FastPForResult<()> {
        self.inner.decode(input, out, None)
    }
}

/// [`Sequential`] format, `u32` values, 128-value blocks. Byte-identical to the C++ `FastPFor<4>`
/// any-length codec (`CppFastPFor128` with the `cpp` feature).
pub type FastPForSequential32x128 = FastPForCodec<Sequential, u32, 128>;

/// [`Sequential`] format, `u32` values, 256-value blocks. Byte-identical to the C++ `FastPFor<8>`
/// any-length codec (`CppFastPFor256` with the `cpp` feature).
pub type FastPForSequential32x256 = FastPForCodec<Sequential, u32, 256>;

/// [`Sequential`] format, `u64` values, 128-value blocks. Byte-identical to the 64-bit path of the
/// C++ `FastPFor<4>` codec (`encode64` of `CppFastPFor128`).
pub type FastPForSequential64x128 = FastPForCodec<Sequential, u64, 128>;

/// [`Sequential`] format, `u64` values, 256-value blocks. Byte-identical to the 64-bit path of the
/// C++ `FastPFor<8>` codec (`encode64` of `CppFastPFor256`).
pub type FastPForSequential64x256 = FastPForCodec<Sequential, u64, 256>;

/// [`Interleaved`] format, `u32` values, 128-value blocks. Byte-identical to the C++ `SIMDFastPFor<4>`
/// any-length codec (`CppSimdFastPFor128` with the `cpp` feature).
/// **Not** compatible with [`FastPForSequential32x128`].
#[doc(alias = "SIMDFastPFor")]
pub type FastPForInterleaved32x128 = FastPForCodec<Interleaved, u32, 128>;

/// [`Interleaved`] format, `u32` values, 256-value blocks. Byte-identical to the C++ `SIMDFastPFor<8>`
/// any-length codec (`CppSimdFastPFor256` with the `cpp` feature).
/// **Not** compatible with [`FastPForSequential32x256`].
#[doc(alias = "SIMDFastPFor")]
pub type FastPForInterleaved32x256 = FastPForCodec<Interleaved, u32, 256>;

/// [`Interleaved`] format, `u64` values, 128-value blocks. C++ has no 64-bit `SIMDFastPFor`: this
/// format is this crate's extension, with nothing in C++ to interoperate with.
/// **Not** compatible with [`FastPForSequential64x128`].
pub type FastPForInterleaved64x128 = FastPForCodec<Interleaved, u64, 128>;

/// [`Interleaved`] format, `u64` values, 256-value blocks; see [`FastPForInterleaved64x128`].
/// **Not** compatible with [`FastPForSequential64x256`].
pub type FastPForInterleaved64x256 = FastPForCodec<Interleaved, u64, 256>;

/// The block codec of [`FastPForSequential32x128`], for whole blocks only.
pub type FastPForSequentialBlock32x128 = FastPForBlock<Sequential, u32, 128>;

/// The block codec of [`FastPForSequential32x256`], for whole blocks only.
pub type FastPForSequentialBlock32x256 = FastPForBlock<Sequential, u32, 256>;

/// The block codec of [`FastPForSequential64x128`], for whole blocks only.
pub type FastPForSequentialBlock64x128 = FastPForBlock<Sequential, u64, 128>;

/// The block codec of [`FastPForSequential64x256`], for whole blocks only.
pub type FastPForSequentialBlock64x256 = FastPForBlock<Sequential, u64, 256>;

/// The block codec of [`FastPForInterleaved32x128`], for whole blocks only.
pub type FastPForInterleavedBlock32x128 = FastPForBlock<Interleaved, u32, 128>;

/// The block codec of [`FastPForInterleaved32x256`], for whole blocks only.
pub type FastPForInterleavedBlock32x256 = FastPForBlock<Interleaved, u32, 256>;

/// The block codec of [`FastPForInterleaved64x128`], for whole blocks only.
pub type FastPForInterleavedBlock64x128 = FastPForBlock<Interleaved, u64, 128>;

/// The block codec of [`FastPForInterleaved64x256`], for whole blocks only.
pub type FastPForInterleavedBlock64x256 = FastPForBlock<Interleaved, u64, 256>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn narrow_codec_roundtrips_u32() {
        let mut codec = FastPForSequential32x256::default();
        let data: Vec<u32> = (0..600).collect();
        let mut enc = Vec::new();
        codec.encode(&data, &mut enc).unwrap();
        let mut dec = Vec::new();
        codec.decode(&enc, &mut dec, None).unwrap();
        assert_eq!(dec, data);
    }

    #[test]
    fn wide_codec_roundtrips_u64() {
        let mut codec = FastPForSequential64x256::default();
        let data: Vec<u64> = (0..600).map(|i| i * 1_000_000_000).collect();
        let mut enc = Vec::new();
        codec.encode(&data, &mut enc).unwrap();
        let mut dec = Vec::new();
        codec.decode(&enc, &mut dec, None).unwrap();
        assert_eq!(dec, data);
    }
}
