//! Public any-length `FastPFOR` codecs supporting both 32- and 64-bit integers.
//!
//! [`FastPFor128`] and [`FastPFor256`] are the primary entry points.
//! Each implements [`AnyLenCodec`] for `u32` and [`BlockCodec64`] for `u64`.
//! Aligned blocks are coded with `FastPFOR` and the sub-block remainder with variable-byte coding.

use crate::FastPForResult;
use crate::codec::{AnyLenCodec, BlockCodec64};
use crate::rust::VariableByte;
use crate::rust::composite::CompositeCodec;
use crate::rust::integer_compression::fastpfor::{FastPForBlock128, FastPForBlock256};
use crate::rust::integer_compression::fastpfor64::FastPForWide;

macro_rules! define_fastpfor {
    ($(#[$meta:meta])* $name:ident, $block:ty, $n:literal) => {
        $(#[$meta])*
        #[derive(Debug, Default)]
        pub struct $name {
            narrow: CompositeCodec<$block, VariableByte>,
            wide: FastPForWide<$n>,
        }

        impl AnyLenCodec for $name {
            fn encode(&mut self, input: &[u32], out: &mut Vec<u32>) -> FastPForResult<()> {
                self.narrow.encode(input, out)
            }

            fn decode(
                &mut self,
                input: &[u32],
                out: &mut Vec<u32>,
                expected_len: Option<u32>,
            ) -> FastPForResult<()> {
                self.narrow.decode(input, out, expected_len)
            }
        }

        impl BlockCodec64 for $name {
            fn encode64(&mut self, input: &[u64], out: &mut Vec<u32>) -> FastPForResult<()> {
                self.wide.encode64(input, out)
            }

            fn decode64(&mut self, input: &[u32], out: &mut Vec<u64>) -> FastPForResult<()> {
                self.wide.decode64(input, out)
            }
        }
    };
}

define_fastpfor! {
    /// Any-length `FastPFOR` codec with 128-value blocks.
    ///
    /// Compresses `u32` via [`AnyLenCodec`] and `u64` via [`BlockCodec64`].
    FastPFor128, FastPForBlock128, 128
}

define_fastpfor! {
    /// Any-length `FastPFOR` codec with 256-value blocks.
    ///
    /// Compresses `u32` via [`AnyLenCodec`] and `u64` via [`BlockCodec64`].
    FastPFor256, FastPForBlock256, 256
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_codec_handles_both_widths() {
        let mut codec = FastPFor256::default();

        let data32: Vec<u32> = (0..600).collect();
        let mut enc32 = Vec::new();
        codec.encode(&data32, &mut enc32).unwrap();
        let mut dec32 = Vec::new();
        codec.decode(&enc32, &mut dec32, None).unwrap();
        assert_eq!(dec32, data32);

        let data64: Vec<u64> = (0..600).map(|i| i * 1_000_000_000).collect();
        let mut enc64 = Vec::new();
        codec.encode64(&data64, &mut enc64).unwrap();
        let mut dec64 = Vec::new();
        codec.decode64(&enc64, &mut dec64).unwrap();
        assert_eq!(dec64, data64);
    }
}
