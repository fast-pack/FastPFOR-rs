use std::io::Cursor;

use bytemuck::cast_slice;

use crate::helpers::AsUsize;
use crate::rust::integer_compression::fastpfor::{FastPForBlock, sealed};
use crate::rust::integer_compression::fastpfor_int::FastPForInt;
use crate::rust::kernels::{Kernels, Layout};
use crate::{BlockCodec, FastPForError, FastPForResult};

impl<L: Layout, T: FastPForInt, const N: usize, K: Kernels> BlockCodec for FastPForBlock<L, T, N, K>
where
    [T; N]: sealed::BlockSize,
{
    type Elem = T;
    type Block = [T; N];

    fn encode_blocks(&mut self, blocks: &[Self::Block], out: &mut Vec<u32>) -> FastPForResult<()> {
        let n_values = (blocks.len() * N) as u32;
        if blocks.is_empty() {
            out.push(n_values);
            return Ok(());
        }
        let flat: &[T] = cast_slice(blocks);

        // The encoder grows `out` as it writes, so only reserve room: compressed data is
        // usually smaller than its input.
        out.reserve(1 + size_of_val(flat) / 4 + 1024);
        out.push(n_values);
        let start = u32::try_from(out.len()).map_err(|_| FastPForError::OutputBufferTooSmall)?;
        let mut in_off = Cursor::new(0u32);
        let mut out_off = Cursor::new(0u32);
        out_off.set_position(u64::from(start));
        self.compress_blocks(flat, n_values, &mut in_off, out, &mut out_off);
        out.truncate(out_off.position() as usize);
        Ok(())
    }

    fn decode_blocks(
        &mut self,
        input: &[u32],
        expected_len: Option<u32>,
        out: &mut Vec<Self::Elem>,
    ) -> FastPForResult<usize> {
        let Some((&block_n_values, rest)) = input.split_first() else {
            return Err(FastPForError::NotEnoughData);
        };
        if block_n_values % N as u32 != 0 {
            return Err(FastPForError::NotEnoughData);
        }
        let max = Self::max_decompressed_len(input.len());
        if let Some(expected) = expected_len {
            expected.is_valid_expected(max)?;
            if block_n_values != expected {
                return Err(FastPForError::DecodedCountMismatch {
                    actual: block_n_values.as_usize(),
                    expected: expected.as_usize(),
                });
            }
        } else if block_n_values.as_usize() > max {
            return Err(FastPForError::NotEnoughData);
        }
        let n_blocks = block_n_values as usize / N;
        if n_blocks == 0 {
            return Ok(1);
        }
        // The decoder grows `out` as it writes, so only reserve room.
        out.reserve(n_blocks * N);
        let start = out.len();
        if u32::try_from(start + n_blocks * N).is_err() {
            return Err(FastPForError::OutputBufferTooSmall);
        }
        let mut in_off = Cursor::new(0u32);
        let mut out_off = Cursor::new(0u32);
        out_off.set_position(start as u64);

        self.decode_headless_blocks(rest, block_n_values, &mut in_off, out, &mut out_off)?;

        // `decode_headless_blocks` only returns `Ok` once it has filled every block.
        debug_assert_eq!(out.len(), start + n_blocks * N);
        // +1 for the header word (block_n_values) that precedes `rest`.
        Ok(1 + in_off.position() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::kernels::Sequential;

    type Seq128 = FastPForBlock<Sequential, u32, 128>;
    type Seq256 = FastPForBlock<Sequential, u32, 256>;
    use crate::test_utils::{block_compress, block_decompress, block_roundtrip};

    #[test]
    fn fastpfor_test() {
        let mut data = vec![0u32; 256];
        data[126] = u32::MAX;
        block_roundtrip::<Seq256>(&data);
    }

    #[test]
    fn fastpfor_test_128() {
        let mut data = vec![0u32; 128];
        data[126] = u32::MAX;
        block_roundtrip::<Seq128>(&data);
    }

    #[test]
    fn test_empty_blocks_ok() {
        // Empty input encodes to length header [0] (matches C++ FastPFor) and decodes cleanly.
        let enc = block_compress::<Seq256>(&[]).unwrap();
        assert_eq!(enc, [0]);
        assert_eq!(block_decompress::<Seq256>(&enc, Some(0)).unwrap(), []);
    }

    // Tests ported from C++
    #[test]
    fn test_constant_sequence() {
        block_roundtrip::<Seq128>(&vec![42u32; 65536]);
    }

    #[test]
    fn test_alternating_sequence() {
        let data: Vec<_> = (0..65536u32).map(|i| u32::from(i % 2 != 0)).collect();
        block_roundtrip::<Seq128>(&data);
    }

    #[test]
    fn test_large_numbers() {
        let data: Vec<u32> = (0..65536u32).map(|i| i + (1u32 << 30)).collect();
        block_roundtrip::<Seq128>(&data);
    }

    #[test]
    fn cursor_api_roundtrip() {
        block_roundtrip::<Seq256>(&vec![42u32; 256]);
    }

    #[test]
    fn headless_compress_unfit_pagesize() {
        // 640 values with 128-block codec spans two pages (512 + 128), exercising the loop.
        let input: Vec<u32> = (0..640u32).collect();
        block_roundtrip::<Seq128>(&input);
    }

    #[test]
    fn exception_value_vector_resizes() {
        // Alternating large/small values trigger exception-buffer resizing across pages.
        let input: Vec<u32> = (0..1024u32)
            .map(|i| if i % 2 == 0 { 1 << 30 } else { 3 })
            .collect();
        block_roundtrip::<Seq128>(&input);
    }

    // ── Error / edge tests not covered by `tests/decode_validation.rs` ─────
    //
    // `AnyLenCodec::decode` treats an empty slice as tail-only and succeeds; an empty
    // `decode_blocks` input is still invalid. Headless decode is internal-only.

    #[test]
    fn uncompress_zero_input_length_err() {
        // Truly empty input (no header word at all) is invalid — C++ would crash reading *in.
        block_decompress::<Seq256>(&[], None).unwrap_err();
    }

    #[test]
    fn headless_uncompress_zero_inlength_128_ok() {
        Seq128::default()
            .decode_headless_blocks(
                &[],
                0,
                &mut Cursor::new(0u32),
                &mut Vec::new(),
                &mut Cursor::new(0u32),
            )
            .expect("zero-length decompress must succeed");
    }

    #[test]
    fn decode_where_meta_overflow() {
        // `decode_headless_blocks` only: no `AnyLenCodec` entry point passes this layout.
        let data: Vec<u32> = (0..256u32)
            .map(|i| if i % 2 == 0 { 1u32 << 30 } else { 3 })
            .collect();
        let compressed = block_compress::<Seq256>(&data).unwrap();

        let mut padded = vec![0u32];
        padded.extend_from_slice(&compressed);
        padded[2] = u32::MAX;
        let out_length = padded[1];
        assert!(
            Seq256::default()
                .decode_headless_blocks(
                    &padded,
                    out_length,
                    &mut Cursor::new(1u32),
                    &mut vec![0u32; 320],
                    &mut Cursor::new(0u32),
                )
                .is_err()
        );
    }

    #[test]
    fn decode_rejects_exceptions_from_prior_calls() {
        let mut data = vec![0u32; 128];
        data[45] = 1 << 20;
        let mut codec = Seq128::default();
        let (blocks, _) = crate::slice_to_blocks::<Seq128>(&data);
        let mut words = Vec::new();
        codec.encode_blocks(blocks, &mut words).unwrap();
        let bytesize = words[2];
        let bitmap_at = 3 + bytesize.div_ceil(4) as usize;
        words[bitmap_at] = 0;

        let fresh = Seq128::default().decode_blocks(&words, None, &mut Vec::new());
        let reused = codec.decode_blocks(&words, None, &mut Vec::new());
        assert!(fresh.is_err());
        assert_eq!(format!("{reused:?}"), format!("{fresh:?}"));
    }

    #[test]
    fn decode_index1_branch_valid() {
        let mut data = vec![1u32; 256];
        data[0] = 3;
        block_roundtrip::<Seq256>(&data);
    }

    /// `decode_blocks` with `expected_len: None` and header=0 returns `Ok` with empty output.
    #[test]
    fn decode_blocks_header_only_input() {
        // Input with just the length header [0]: no blocks to decode.
        let input = vec![0u32];
        assert_eq!(block_decompress::<Seq256>(&input, None).unwrap(), []);
    }

    #[test]
    fn decode_blocks_expected_len_mismatch_errors() {
        let data = vec![7u32; 256];
        let compressed = block_compress::<Seq128>(&data).unwrap();
        let err = block_decompress::<Seq128>(&compressed, Some(128)).unwrap_err();
        assert_eq!(
            format!("{err:?}"),
            "DecodedCountMismatch { actual: 256, expected: 128 }"
        );
    }

    #[test]
    fn decode_blocks_header_exceeds_max_len_errors() {
        let input = vec![2048u32];
        let result = block_decompress::<Seq128>(&input, None);
        assert!(
            matches!(result, Err(FastPForError::NotEnoughData)),
            "expected NotEnoughData, got {result:?}"
        );
    }

    #[test]
    fn decode_blocks_expected_len_exceeds_max_len_errors() {
        let input = vec![4_286_056_704u32, 27_590_491];
        let max = Seq256::max_decompressed_len(input.len());
        let result = block_decompress::<Seq256>(&input, Some(4_286_056_704));
        assert!(
            matches!(
                result,
                Err(FastPForError::ExpectedCountExceedsMax { expected: 4_286_056_704, max: m })
                    if m == max
            ),
            "expected ExpectedCountExceedsMax, got {result:?}"
        );
    }

    #[test]
    fn encode_is_independent_of_prior_calls() {
        let noisy: Vec<u32> = (0..512u32)
            .map(|i| if i % 3 == 0 { 0xFFFF_FFF0 | i } else { 1 })
            .collect();
        let mut data = vec![1u32; 128];
        data[5] = u32::MAX;
        let fresh = block_compress::<Seq128>(&data).unwrap();

        let mut codec = Seq128::default();
        let (blocks, _) = crate::slice_to_blocks::<Seq128>(&noisy);
        codec.encode_blocks(blocks, &mut Vec::new()).unwrap();
        let noisy_enc = block_compress::<Seq128>(&noisy).unwrap();
        codec
            .decode_blocks(&noisy_enc, None, &mut Vec::new())
            .unwrap();
        let (blocks, _) = crate::slice_to_blocks::<Seq128>(&data);
        let mut reused = Vec::new();
        codec.encode_blocks(blocks, &mut reused).unwrap();
        assert_eq!(reused, fresh);
    }
}

#[cfg(test)]
mod interleaved_tests {
    use super::*;
    use crate::rust::kernels::Interleaved;
    use crate::test_utils::{block_compress, block_decompress, block_roundtrip};

    type Int128 = FastPForBlock<Interleaved, u32, 128>;
    type Int256 = FastPForBlock<Interleaved, u32, 256>;

    #[test]
    fn exception_roundtrip() {
        let mut data = vec![0u32; 128];
        data[126] = u32::MAX;
        block_roundtrip::<Int128>(&data);
        let mut data = vec![0u32; 256];
        data[126] = u32::MAX;
        block_roundtrip::<Int256>(&data);
    }

    #[test]
    fn multi_page_roundtrip() {
        block_roundtrip::<Int128>(&vec![42u32; 65536]);
        let data: Vec<u32> = (0..1024u32)
            .map(|i| if i % 2 == 0 { 1 << 30 } else { 3 })
            .collect();
        block_roundtrip::<Int128>(&data);
        block_roundtrip::<Int256>(&data);
    }

    #[test]
    fn empty_blocks_ok() {
        let enc = block_compress::<Int256>(&[]).unwrap();
        assert_eq!(enc, [0]);
        assert_eq!(block_decompress::<Int256>(&enc, Some(0)).unwrap(), []);
        block_decompress::<Int256>(&[], None).unwrap_err();
    }

    #[test]
    fn index1_exception_branch() {
        let mut data = vec![1u32; 256];
        data[0] = 3;
        block_roundtrip::<Int256>(&data);
    }

    #[test]
    fn expected_len_mismatch_errors() {
        let compressed = block_compress::<Int128>(&[7u32; 256]).unwrap();
        let err = block_decompress::<Int128>(&compressed, Some(128)).unwrap_err();
        assert_eq!(
            format!("{err:?}"),
            "DecodedCountMismatch { actual: 256, expected: 128 }"
        );
    }

    #[test]
    fn header_exceeds_max_len_errors() {
        let result = block_decompress::<Int128>(&[2048u32], None);
        assert!(matches!(result, Err(FastPForError::NotEnoughData)));
    }

    #[test]
    fn decode_rejects_exceptions_from_prior_calls() {
        let mut data = vec![0u32; 128];
        data[45] = 1 << 20;
        let mut codec = Int128::default();
        let (blocks, _) = crate::slice_to_blocks::<Int128>(&data);
        let mut words = Vec::new();
        codec.encode_blocks(blocks, &mut words).unwrap();
        let bitmap_at = 3 + words[2].div_ceil(4) as usize;
        words[bitmap_at] = 0;

        let fresh = Int128::default().decode_blocks(&words, None, &mut Vec::new());
        let reused = codec.decode_blocks(&words, None, &mut Vec::new());
        assert!(fresh.is_err());
        assert_eq!(format!("{reused:?}"), format!("{fresh:?}"));
    }

    #[test]
    fn encode_is_independent_of_prior_calls() {
        let noisy: Vec<u32> = (0..512u32)
            .map(|i| if i % 3 == 0 { 0xFFFF_FFF0 | i } else { 1 })
            .collect();
        let mut data = vec![1u32; 128];
        data[5] = u32::MAX;
        let fresh = block_compress::<Int128>(&data).unwrap();

        let mut codec = Int128::default();
        let (blocks, _) = crate::slice_to_blocks::<Int128>(&noisy);
        codec.encode_blocks(blocks, &mut Vec::new()).unwrap();
        let noisy_enc = block_compress::<Int128>(&noisy).unwrap();
        codec
            .decode_blocks(&noisy_enc, None, &mut Vec::new())
            .unwrap();
        let (blocks, _) = crate::slice_to_blocks::<Int128>(&data);
        let mut reused = Vec::new();
        codec.encode_blocks(blocks, &mut reused).unwrap();
        assert_eq!(reused, fresh);
    }
}
