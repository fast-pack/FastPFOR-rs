use std::io::Cursor;

use bytemuck::cast_slice;

use crate::helpers::AsUsize;
use crate::rust::integer_compression::fastpfor::sealed;
use crate::{BlockCodec, FastPFor, FastPForError, FastPForResult};

/// Type alias for [`FastPFor`] with 128-element `u32` blocks.
pub type FastPForBlock128 = FastPFor<128, u32>;

/// Type alias for [`FastPFor`] with 256-element `u32` blocks.
pub type FastPForBlock256 = FastPFor<256, u32>;

impl<const N: usize> BlockCodec for FastPFor<N, u32>
where
    [u32; N]: sealed::BlockSize,
{
    type Block = [u32; N];

    fn encode_blocks(&mut self, blocks: &[Self::Block], out: &mut Vec<u32>) -> FastPForResult<()> {
        let n_values = (blocks.len() * N) as u32;
        if blocks.is_empty() {
            out.push(n_values);
            return Ok(());
        }
        let flat: &[u32] = cast_slice(blocks);

        let capacity = flat.len() * 2 + 1024;
        let start = out.len();
        // Reserve slot for the length header, then space for compressed data.
        out.resize(start + 1 + capacity, 0);

        let mut in_off = Cursor::new(0u32);
        let mut out_off = Cursor::new(0u32);

        // Write length header then compress.
        out[start] = n_values;
        self.compress_blocks(
            flat,
            n_values,
            &mut in_off,
            &mut out[start + 1..],
            &mut out_off,
        );

        let written = 1 + out_off.position() as usize;
        out.truncate(start + written);
        Ok(())
    }

    fn decode_blocks(
        &mut self,
        input: &[u32],
        expected_len: Option<u32>,
        out: &mut Vec<u32>,
    ) -> FastPForResult<usize> {
        let Some((&block_n_values, rest)) = input.split_first() else {
            return Err(FastPForError::NotEnoughData);
        };
        if block_n_values % N as u32 != 0 {
            return Err(FastPForError::NotEnoughData);
        }
        if let Some(expected) = expected_len {
            if block_n_values != expected {
                return Err(FastPForError::DecodedCountMismatch {
                    actual: block_n_values.as_usize(),
                    expected: expected.as_usize(),
                });
            }
        } else {
            let max = Self::max_decompressed_len(input.len());
            if block_n_values.as_usize() > max {
                return Err(FastPForError::NotEnoughData);
            }
        }
        let n_blocks = block_n_values as usize / N;
        if n_blocks == 0 {
            return Ok(1);
        }
        let start = out.len();
        out.resize(start + n_blocks * N, 0);

        let mut in_off = Cursor::new(0u32);
        let mut out_off = Cursor::new(0u32);

        self.decode_headless_blocks(
            rest,
            block_n_values,
            &mut in_off,
            &mut out[start..],
            &mut out_off,
        )?;

        let written = out_off.position() as usize;
        if written != n_blocks * N {
            out.truncate(start + written);
        }
        // +1 for the header word (block_n_values) that precedes `rest`.
        Ok(1 + in_off.position() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{block_compress, block_decompress, block_roundtrip};

    #[test]
    fn fastpfor_test() {
        let mut data = vec![0u32; 256];
        data[126] = u32::MAX;
        block_roundtrip::<FastPForBlock256>(&data);
    }

    #[test]
    fn fastpfor_test_128() {
        let mut data = vec![0u32; 128];
        data[126] = u32::MAX;
        block_roundtrip::<FastPForBlock128>(&data);
    }

    #[test]
    fn test_empty_blocks_ok() {
        // Empty input encodes to length header [0] (matches C++ FastPFor) and decodes cleanly.
        let enc = block_compress::<FastPForBlock256>(&[]).unwrap();
        assert_eq!(enc, [0]);
        let dec = block_decompress::<FastPForBlock256>(&enc, Some(0)).unwrap();
        assert!(dec.is_empty());
    }

    // Tests ported from C++
    #[test]
    fn test_constant_sequence() {
        block_roundtrip::<FastPForBlock128>(&vec![42u32; 65536]);
    }

    #[test]
    fn test_alternating_sequence() {
        let data: Vec<_> = (0..65536u32).map(|i| u32::from(i % 2 != 0)).collect();
        block_roundtrip::<FastPForBlock128>(&data);
    }

    #[test]
    fn test_large_numbers() {
        let data: Vec<u32> = (0..65536u32).map(|i| i + (1u32 << 30)).collect();
        block_roundtrip::<FastPForBlock128>(&data);
    }

    #[test]
    fn cursor_api_roundtrip() {
        block_roundtrip::<FastPForBlock256>(&vec![42u32; 256]);
    }

    #[test]
    fn headless_compress_unfit_pagesize() {
        // 640 values with 128-block codec spans two pages (512 + 128), exercising the loop.
        let input: Vec<u32> = (0..640u32).collect();
        block_roundtrip::<FastPForBlock128>(&input);
    }

    #[test]
    fn exception_value_vector_resizes() {
        // Alternating large/small values trigger exception-buffer resizing across pages.
        let input: Vec<u32> = (0..1024u32)
            .map(|i| if i % 2 == 0 { 1 << 30 } else { 3 })
            .collect();
        block_roundtrip::<FastPForBlock128>(&input);
    }

    // ── Error / edge tests not covered by `tests/decode_validation.rs` ─────
    //
    // `AnyLenCodec::decode` treats an empty slice as tail-only and succeeds; an empty
    // `decode_blocks` input is still invalid. Headless decode is internal-only.

    #[test]
    fn uncompress_zero_input_length_err() {
        // Truly empty input (no header word at all) is invalid — C++ would crash reading *in.
        block_decompress::<FastPForBlock256>(&[], None).unwrap_err();
    }

    #[test]
    fn headless_uncompress_zero_inlength_128_ok() {
        FastPForBlock128::default()
            .decode_headless_blocks(
                &[],
                0,
                &mut Cursor::new(0u32),
                &mut [],
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
        let compressed = block_compress::<FastPForBlock256>(&data).unwrap();

        let mut padded = vec![0u32];
        padded.extend_from_slice(&compressed);
        padded[2] = u32::MAX;
        let out_length = padded[1];
        assert!(
            FastPForBlock256::default()
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
    fn decode_index1_branch_valid() {
        let mut data = vec![1u32; 256];
        data[0] = 3;
        block_roundtrip::<FastPForBlock256>(&data);
    }

    /// `decode_blocks` with `expected_len: None` and header=0 returns `Ok` with empty output.
    #[test]
    fn decode_blocks_header_only_input() {
        // Input with just the length header [0]: no blocks to decode.
        let input = vec![0u32];
        let out = block_decompress::<FastPForBlock256>(&input, None).unwrap();
        assert!(out.is_empty());
    }
}
