//! Public any-length `FastPFOR` codecs.
//!
//! [`FastPFor128`]/[`FastPFor256`] compress `u32`; [`FastPForWide128`]/[`FastPForWide256`] compress `u64`.
//! Each is one [`CompositeCodec`]: the width-generic block engine plus a variable-byte tail for the remainder.

use crate::FastPForResult;
use crate::codec::{AnyLenCodec, BlockCodec64};
use crate::rust::VariableByte;
use crate::rust::composite::CompositeCodec;
use crate::rust::integer_compression::fastpfor::{FastPFor, FastPForBlock, sealed};
use crate::rust::integer_compression::fastpfor_int::FastPForInt;
use crate::rust::kernels::{Auto, Kernels, Scalar, Sequential, Simd};

/// Any-length `FastPFOR` codec over `N`-value blocks of width `T` ([`u32`] or [`u64`]).
///
/// A single [`CompositeCodec`] pairing the width-generic block engine with a [`VariableByte`] tail.
/// `K` selects the bit-packing [`Kernels`]; all choices produce byte-identical output.
/// Instantiate through the [`FastPFor128`]/[`FastPForWide128`]/[`FastPForSimd128`] aliases.
#[derive(Debug)]
pub struct FastPForCodec<const N: usize, T: FastPForInt, K: Kernels = Scalar>
where
    [T; N]: sealed::BlockSize,
    VariableByte<T>: AnyLenCodec<Elem = T>,
{
    inner: CompositeCodec<FastPFor<N, T, K>, VariableByte<T>>,
}

// Hand-written (not derived) so `default()` needs no `T: Default` bound;
// the tail's `AnyLenCodec: Default` supertrait already guarantees it.
impl<const N: usize, T: FastPForInt, K: Kernels> Default for FastPForCodec<N, T, K>
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

impl<const N: usize, T: FastPForInt, K: Kernels> AnyLenCodec for FastPForCodec<N, T, K>
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
impl<const N: usize, K: Kernels> BlockCodec64 for FastPForCodec<N, u64, K>
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

/// Any-length `u32` `FastPFOR` codec with 128-value blocks.
#[deprecated(since = "0.9.2", note = "renamed to `FastPForSequential32x128`")]
pub type FastPFor128 = FastPForCodec<128, u32>;

/// Any-length `u32` `FastPFOR` codec with 256-value blocks.
#[deprecated(since = "0.9.2", note = "renamed to `FastPForSequential32x256`")]
pub type FastPFor256 = FastPForCodec<256, u32>;

/// Any-length `u64` `FastPFOR` codec with 128-value blocks.
#[deprecated(since = "0.9.2", note = "renamed to `FastPForSequential64x128`")]
pub type FastPForWide128 = FastPForCodec<128, u64>;

/// Any-length `u64` `FastPFOR` codec with 256-value blocks.
#[deprecated(since = "0.9.2", note = "renamed to `FastPForSequential64x256`")]
pub type FastPForWide256 = FastPForCodec<256, u64>;

/// [`FastPFor128`] using [`Simd`] kernels; byte-compatible with it.
#[deprecated(since = "0.9.2", note = "use `FastPForSequential32x128`")]
pub type FastPForSimd128 = FastPForCodec<128, u32, Simd>;

/// [`FastPFor256`] using [`Simd`] kernels; byte-compatible with it.
#[deprecated(since = "0.9.2", note = "use `FastPForSequential32x256`")]
pub type FastPForSimd256 = FastPForCodec<256, u32, Simd>;

/// [`FastPForWide128`] using [`Simd`] kernels; byte-compatible with it.
#[deprecated(since = "0.9.2", note = "use `FastPForSequential64x128`")]
pub type FastPForSimdWide128 = FastPForCodec<128, u64, Simd>;

/// [`FastPForWide256`] using [`Simd`] kernels; byte-compatible with it.
#[deprecated(since = "0.9.2", note = "use `FastPForSequential64x256`")]
pub type FastPForSimdWide256 = FastPForCodec<256, u64, Simd>;

/// The `u32`, 128-value block codec under its 0.10 name. Byte-identical to the C++ `FastPFor<4>` codec.
///
/// Uses the [`Auto`] kernels: SIMD where available, with the same output as the portable ones.
pub type FastPForSequential32x128 = FastPForCodec<128, u32, Auto>;

/// The `u32`, 256-value block codec under its 0.10 name. Byte-identical to the C++ `FastPFor<8>` codec.
///
/// Uses the [`Auto`] kernels: SIMD where available, with the same output as the portable ones.
pub type FastPForSequential32x256 = FastPForCodec<256, u32, Auto>;

/// The `u64`, 128-value block codec under its 0.10 name. Byte-identical to the 64-bit path of the
/// C++ `FastPFor<4>` codec.
///
/// Uses the [`Auto`] kernels: SIMD where available, with the same output as the portable ones.
pub type FastPForSequential64x128 = FastPForCodec<128, u64, Auto>;

/// The `u64`, 256-value block codec under its 0.10 name. Byte-identical to the 64-bit path of the
/// C++ `FastPFor<8>` codec.
///
/// Uses the [`Auto`] kernels: SIMD where available, with the same output as the portable ones.
pub type FastPForSequential64x256 = FastPForCodec<256, u64, Auto>;

/// The block codec of [`FastPForSequential32x128`], for whole blocks only.
pub type FastPForSequentialBlock32x128 = FastPForBlock<Sequential, u32, 128>;

/// The block codec of [`FastPForSequential32x256`], for whole blocks only.
pub type FastPForSequentialBlock32x256 = FastPForBlock<Sequential, u32, 256>;

/// The block codec of [`FastPForSequential64x128`], for whole blocks only.
pub type FastPForSequentialBlock64x128 = FastPForBlock<Sequential, u64, 128>;

/// The block codec of [`FastPForSequential64x256`], for whole blocks only.
pub type FastPForSequentialBlock64x256 = FastPForBlock<Sequential, u64, 256>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn narrow_codec_roundtrips_u32() {
        let mut codec = FastPFor256::default();
        let data: Vec<u32> = (0..600).collect();
        let mut enc = Vec::new();
        codec.encode(&data, &mut enc).unwrap();
        let mut dec = Vec::new();
        codec.decode(&enc, &mut dec, None).unwrap();
        assert_eq!(dec, data);
    }

    #[test]
    fn wide_codec_roundtrips_u64() {
        let mut codec = FastPForWide256::default();
        let data: Vec<u64> = (0..600).map(|i| i * 1_000_000_000).collect();
        let mut enc = Vec::new();
        codec.encode(&data, &mut enc).unwrap();
        let mut dec = Vec::new();
        codec.decode(&enc, &mut dec, None).unwrap();
        assert_eq!(dec, data);
    }
}
