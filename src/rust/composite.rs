//! [`CompositeCodec`]: chains a [`BlockCodec`] for aligned blocks with an
//! [`AnyLenCodec`] for the sub-block remainder.
//!
//! Rust-only: combines Rust block codecs with Rust tail codecs. Do not wrap C++ codecs.

use crate::FastPForResult;
use crate::codec::{AnyLenCodec, BlockCodec, slice_to_blocks};
use crate::helpers::AsUsize;

/// Combines a block-oriented codec with an arbitrary-length tail codec.
///
/// `CompositeCodec<Blocks, Tail>` implements [`AnyLenCodec`]: it accepts any
/// input length, encodes the aligned prefix with `Blocks`, and the
/// sub-block remainder with `Tail`.
///
/// **Rust-only:** Use only with Rust codecs (e.g. `FastPForBlock<Sequential, u32, 256>`, `VariableByte`).
/// C++ block codecs are already any-length in the C++ library; use them directly.
///
/// # Wire format (matches C++ `CompositeCodec`)
///
/// ```text
/// [ Blocks encoded data... ] [ Tail encoded data... ]
/// ```
///
/// No composite-level header; the block codec's first word is its value count.
/// For tail-only input, C++ `FastPFor` writes 0, so we emit `[0][tail]`.
///
/// # Example
///
/// ```
/// use fastpfor::{AnyLenCodec, FastPForSequential32x256};
///
/// let data: Vec<u32> = (0..600).collect(); // 2 × 256 + 88 remainder
/// let mut codec = FastPForSequential32x256::default();
///
/// let mut encoded = Vec::new();
/// codec.encode(&data, &mut encoded).unwrap();
///
/// let mut decoded = Vec::new();
/// codec.decode(&encoded, &mut decoded, None).unwrap();
/// assert_eq!(decoded, data);
/// ```
#[derive(Debug)]
pub struct CompositeCodec<Blocks: BlockCodec, Tail: AnyLenCodec<Elem = Blocks::Elem>> {
    block: Blocks,
    tail: Tail,
}

impl<Blocks: BlockCodec, Tail: AnyLenCodec<Elem = Blocks::Elem>> Default
    for CompositeCodec<Blocks, Tail>
{
    fn default() -> Self {
        Self::new(Blocks::default(), Tail::default())
    }
}

impl<Blocks: BlockCodec, Tail: AnyLenCodec<Elem = Blocks::Elem>> CompositeCodec<Blocks, Tail> {
    /// Creates a new `CompositeCodec` from a block codec and a tail codec.
    pub fn new(block: Blocks, tail: Tail) -> Self {
        Self { block, tail }
    }
}

impl<Blocks: BlockCodec, Tail: AnyLenCodec<Elem = Blocks::Elem>> AnyLenCodec
    for CompositeCodec<Blocks, Tail>
{
    type Elem = Blocks::Elem;

    fn encode(&mut self, input: &[Self::Elem], out: &mut Vec<u32>) -> FastPForResult<()> {
        let (blocks, remainder) = slice_to_blocks::<Blocks>(input);
        // C++ CompositeCodec: concatenate block + tail. Block codec writes length header (0 when empty).
        self.block.encode_blocks(blocks, out)?;
        self.tail.encode(remainder, out)
    }

    /// Decode C++ format: `[block_data][tail_data]`. Block codec's first word = block value count.
    fn decode(
        &mut self,
        input: &[u32],
        out: &mut Vec<Self::Elem>,
        expected_len: Option<u32>,
    ) -> FastPForResult<()> {
        let start_len = out.len();
        let max = Self::max_decompressed_len(input.len());

        if let Some(expected) = expected_len {
            out.reserve(expected.is_valid_expected(max)?);
        }

        if input.is_empty() {
            // When input is empty, max_decompressed_len(0) == 0, so is_valid_expected
            // already rejected any expected_len > 0 above. No mismatch check needed.
            self.tail.decode(&[], out, None)?;
            return Ok(());
        }

        let block_expected = expected_len.map(|v| {
            let v = (v.as_usize() / Blocks::size()) * Blocks::size();
            u32::try_from(v).expect("block-aligned expected_len fits in u32")
        });

        let consumed = self.block.decode_blocks(input, block_expected, out)?;
        // Decoder is expected to return valid data
        let tail_input = &input[consumed..];
        self.tail.decode(tail_input, out, None)?;

        if let Some(n) = expected_len {
            (out.len() - start_len).is_decoded_mismatch(n)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::{FastPForBlock, JustCopy, Sequential, VariableByte};
    use crate::test_utils::{compress, decompress, roundtrip_composite, roundtrip_expected};
    use crate::{FastPForSequential32x128, FastPForSequential32x256};

    #[test]
    fn test_fastpfor256_vbyte_exact_two_blocks() {
        let data: Vec<u32> = (0..512).collect();
        roundtrip_composite::<FastPForBlock<Sequential, u32, 256>, VariableByte>(&data);
    }

    #[test]
    fn test_fastpfor256_vbyte_with_remainder() {
        let data: Vec<u32> = (0..600).collect();
        roundtrip_composite::<FastPForBlock<Sequential, u32, 256>, VariableByte>(&data);
    }

    #[test]
    fn test_fastpfor128_justcopy_with_remainder() {
        let data: Vec<u32> = (0..300).collect();
        roundtrip_composite::<FastPForBlock<Sequential, u32, 128>, JustCopy>(&data);
    }

    #[test]
    fn test_empty_input() {
        roundtrip_composite::<FastPForBlock<Sequential, u32, 256>, VariableByte>(&[]);
    }

    #[test]
    fn test_decode_truly_empty_input() {
        // Decoding a zero-length slice (not even a header word) must succeed with empty output.
        let out = decompress::<FastPForSequential32x256>(&[], None).unwrap();
        assert_eq!(out, []);
    }

    #[test]
    fn test_decode_empty_input_with_expected_zero() {
        // Empty input with expected_len=0 must succeed.
        let out = decompress::<FastPForSequential32x256>(&[], Some(0)).unwrap();
        assert_eq!(out, []);
    }

    /// Encoding empty input produces a single `[0]` header word — and Rust matches C++ exactly.
    #[test]
    fn test_empty_input_encodes_to_one_zero_word() {
        let rust128 = compress::<FastPForSequential32x128>(&[]).unwrap();
        assert_eq!(
            rust128,
            [0u32],
            "FastPForSequential32x128: empty input must produce [0], got {rust128:?}"
        );

        let rust256 = compress::<FastPForSequential32x256>(&[]).unwrap();
        assert_eq!(
            rust256,
            [0u32],
            "FastPForSequential32x256: empty input must produce [0], got {rust256:?}"
        );

        // Verify C++ produces identical output — both codecs agree on [0] for empty.
        #[cfg(feature = "cpp")]
        {
            use crate::cpp::{CppFastPFor128, CppFastPFor256};

            let cpp128 = compress::<CppFastPFor128>(&[]).unwrap();
            assert_eq!(
                cpp128, rust128,
                "CppFastPFor128 and FastPForSequential32x128 must agree on empty encoding"
            );

            let cpp256 = compress::<CppFastPFor256>(&[]).unwrap();
            assert_eq!(
                cpp256, rust256,
                "CppFastPFor256 and FastPForSequential32x256 must agree on empty encoding"
            );
        }
    }

    #[test]
    fn test_decode_empty_input_with_nonzero_expected_errors() {
        // Empty input: max_decompressed_len(0) == 0, so any expected_len > 0 fails
        // with ExpectedCountExceedsMax before decoding begins.
        decompress::<FastPForSequential32x256>(&[], Some(5)).unwrap_err();
    }

    #[test]
    fn test_decode_huge_n_blocks_header_returns_error() {
        // A corrupt header claiming ~1.6 M blocks must return an error rather
        // than attempting a multi-gigabyte allocation.
        // Regression: fuzzer found bytes [0x04, 0x35, 0x19] → u32 LE 0x00193504 = 1_651_460
        // fed to FastPForSequential32x256.decode caused an OOM via a ~2.5 GB Vec::resize.
        let input = &[0x0019_3504u32]; // n_blocks = 1_651_460, rest is empty
        decompress::<FastPForSequential32x256>(input, None).unwrap_err();
    }

    #[test]
    fn test_sub_block_only() {
        let data: Vec<u32> = (0..10).collect();
        roundtrip_composite::<FastPForBlock<Sequential, u32, 256>, VariableByte>(&data);
    }

    #[test]
    fn test_decode_with_expected_len() {
        let data: Vec<u32> = (0..600).collect();
        roundtrip_expected::<FastPForSequential32x256>(&data, Some(600));
    }

    #[test]
    fn test_decode_expected_len_mismatch_errors() {
        let data: Vec<u32> = (0..100).collect();
        let encoded = compress::<FastPForSequential32x256>(&data).unwrap();
        decompress::<FastPForSequential32x256>(&encoded, Some(50)).unwrap_err();
    }

    #[test]
    fn test_decode_expected_len_exceeds_max_errors() {
        let data: Vec<u32> = (0..10).collect();
        let encoded = compress::<FastPForSequential32x256>(&data).unwrap();
        let huge = (FastPForSequential32x256::max_decompressed_len(encoded.len()) + 1) as u32;
        decompress::<FastPForSequential32x256>(&encoded, Some(huge)).unwrap_err();
    }
}
