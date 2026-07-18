//! Public any-length `FastPFOR` codecs.
//!
//! [`FastPFor128`]/[`FastPFor256`] compress `u32`; [`FastPForWide128`]/[`FastPForWide256`] compress `u64`.
//! Each is one [`CompositeCodec`]: the width-generic block engine plus a variable-byte tail for the remainder.

use crate::FastPForResult;
use crate::codec::{AnyLenCodec, BlockCodec64};
use crate::rust::VariableByte;
use crate::rust::composite::CompositeCodec;
use crate::rust::integer_compression::fastpfor::{FastPFor, sealed};
use crate::rust::integer_compression::fastpfor_int::FastPForInt;

/// Any-length `FastPFOR` codec over `N`-value blocks of width `T` ([`u32`] or [`u64`]).
///
/// A single [`CompositeCodec`] pairing the width-generic block engine with a [`VariableByte`] tail.
/// Instantiate through the [`FastPFor128`]/[`FastPForWide128`] aliases.
#[derive(Debug)]
pub struct FastPForCodec<const N: usize, T: FastPForInt>
where
    [T; N]: sealed::BlockSize,
    VariableByte<T>: AnyLenCodec<Elem = T>,
{
    inner: CompositeCodec<FastPFor<N, T>, VariableByte<T>>,
}

// Hand-written (not derived) so `default()` needs no `T: Default` bound;
// the tail's `AnyLenCodec: Default` supertrait already guarantees it.
impl<const N: usize, T: FastPForInt> Default for FastPForCodec<N, T>
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

impl<const N: usize, T: FastPForInt> AnyLenCodec for FastPForCodec<N, T>
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
impl<const N: usize> BlockCodec64 for FastPForCodec<N, u64>
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
pub type FastPFor128 = FastPForCodec<128, u32>;

/// Any-length `u32` `FastPFOR` codec with 256-value blocks.
pub type FastPFor256 = FastPForCodec<256, u32>;

/// Any-length `u64` `FastPFOR` codec with 128-value blocks.
pub type FastPForWide128 = FastPForCodec<128, u64>;

/// Any-length `u64` `FastPFOR` codec with 256-value blocks.
pub type FastPForWide256 = FastPForCodec<256, u64>;

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
