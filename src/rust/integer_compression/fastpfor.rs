use std::io::Cursor;

use bytemuck::cast_slice;

use crate::helpers::AsUsize;
use crate::rust::integer_compression::fastpfor_engine::FastPForEngine;
use crate::{BlockCodec, FastPForError, FastPForResult};

mod sealed {
    /// Sealed marker trait: only `[u32; 128]` and `[u32; 256]` are valid `FastPFor` block arrays.
    ///
    /// This is intentionally private so that users cannot implement it for other sizes,
    /// preventing instantiation of `FastPFor<N>` for unsupported `N` at compile time.
    pub trait BlockSize: bytemuck::Pod {}
    impl BlockSize for [u32; 128] {}
    impl BlockSize for [u32; 256] {}
}

/// Default page size in number of integers.
const DEFAULT_PAGE_SIZE: u32 = 65536;

/// Type alias for [`FastPFor`] with 128-element blocks.
pub type FastPForBlock128 = FastPFor<128>;

/// Type alias for [`FastPFor`] with 256-element blocks.
pub type FastPForBlock256 = FastPFor<256>;

/// Fast Patched Frame-of-Reference ([FastPFOR](https://github.com/lemire/FastPFor)) codec.
///
/// `N` is the block size (128 or 256 values per block). This struct implements
/// [`BlockCodec`] with `Block = [u32; N]`, giving compile-time guarantees that
/// only correctly-sized blocks are accepted.
///
/// Use [`FastPForBlock128`] or [`FastPForBlock256`] as convenient type aliases.
///
/// To compress arbitrary-length data (including a sub-block remainder),
/// wrap this in a [`CompositeCodec`](crate::CompositeCodec):
///
/// ```
/// # use fastpfor::{FastPFor256, AnyLenCodec};
/// # let data = [];
/// # let mut out = vec![];
/// let mut codec = FastPFor256::default();
/// codec.encode(&data, &mut out).unwrap();
/// ```
#[derive(Debug)]
pub struct FastPFor<const N: usize> {
    engine: FastPForEngine<N, u32>,
}

impl<const N: usize> Default for FastPFor<N>
where
    [u32; N]: sealed::BlockSize,
{
    fn default() -> Self {
        Self::new(DEFAULT_PAGE_SIZE)
            .expect("DEFAULT_PAGE_SIZE is a multiple of all valid block sizes")
    }
}

impl<const N: usize> FastPFor<N> {
    /// Creates a new codec with the given page size.
    ///
    /// Returns an error if `page_size` is not a multiple of the block size.
    /// Use [`Default`] for the default page size.
    pub fn new(page_size: u32) -> FastPForResult<Self> {
        if page_size % N as u32 != 0 {
            return Err(FastPForError::InvalidPageSize {
                page_size,
                block_size: N as u32,
            });
        }
        Ok(Self {
            engine: FastPForEngine::new(page_size),
        })
    }
}

impl<const N: usize> BlockCodec for FastPFor<N>
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
        self.engine.compress_blocks(
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

        self.engine.decode_headless_blocks(
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
        // Empty input encodes to length header [0] and decodes cleanly.
        let enc = block_compress::<FastPForBlock256>(&[]).unwrap();
        assert_eq!(enc, [0]);
        let dec = block_decompress::<FastPForBlock256>(&enc, Some(0)).unwrap();
        assert!(dec.is_empty());
    }

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

    #[test]
    fn uncompress_zero_input_length_err() {
        // Truly empty input (no header word at all) is invalid.
        block_decompress::<FastPForBlock256>(&[], None).unwrap_err();
    }

    #[test]
    fn headless_uncompress_zero_inlength_128_ok() {
        FastPForBlock128::default()
            .engine
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
                .engine
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
        let input = vec![0u32];
        let out = block_decompress::<FastPForBlock256>(&input, None).unwrap();
        assert!(out.is_empty());
    }
}
