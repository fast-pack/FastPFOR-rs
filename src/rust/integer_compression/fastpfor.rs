use std::array;
use std::cmp::min;
use std::io::Cursor;

use bytemuck::cast_slice;
use bytes::{Buf as _, BufMut as _, BytesMut};

use crate::helpers::{AsUsize, GetWithErr, bits, greatest_multiple};
use crate::rust::cursor::IncrementCursor;
use crate::rust::integer_compression::{bitpacking, bitunpacking};
use crate::{BlockCodec, FastPForError, FastPForResult};

/// Overhead cost (in bits) for storing each exception's position in the block
const OVERHEAD_OF_EACH_EXCEPT: u32 = 8;

/// Default page size in number of integers (64 KiB / 4 bytes = 16 Ki integers).
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
    /// Exception values indexed by bit width difference
    exception_buffers: [Vec<u32>; 33],
    /// Metadata buffer for encoding/decoding
    bytes_container: BytesMut,
    /// Maximum integers per page
    page_size: u32,
    /// Position trackers for exception arrays
    data_pointers: [usize; 33],
    /// Frequency count for each bit width:
    /// `freqs[i]` = count of values needing exactly i bits
    freqs: [u32; 33],
    /// Optimal number of bits chosen for the current block
    optimal_bits: u8,
    /// Number of exceptions that don't fit in the optimal bit width
    exception_count: u8,
    /// Maximum bit width required for any value in the block
    max_bits: u8,
}

impl<const N: usize> Default for FastPFor<N>
where
    [u32; N]: bytemuck::Pod,
{
    fn default() -> Self {
        Self::create(DEFAULT_PAGE_SIZE)
            .expect("DEFAULT_PAGE_SIZE is a multiple of all valid block sizes")
    }
}

impl FastPFor<128> {
    /// Creates a new `FastPForBlock128` codec with the given page size.
    ///
    /// Returns an error if `page_size` is not a multiple of 128.
    /// Use [`Default`] for the default page size.
    pub fn new(page_size: u32) -> FastPForResult<Self> {
        Self::create(page_size)
    }
}

impl FastPFor<256> {
    /// Creates a new `FastPForBlock256` codec with the given page size.
    ///
    /// Returns an error if `page_size` is not a multiple of 256.
    /// Use [`Default`] for the default page size.
    pub fn new(page_size: u32) -> FastPForResult<Self> {
        Self::create(page_size)
    }
}

impl<const N: usize> FastPFor<N> {
    fn create(page_size: u32) -> FastPForResult<Self> {
        if page_size % N as u32 != 0 {
            return Err(FastPForError::InvalidPageSize {
                page_size,
                block_size: N as u32,
            });
        }
        Ok(FastPFor {
            bytes_container: BytesMut::with_capacity(
                (3 * page_size / N as u32 + page_size) as usize,
            ),
            page_size,
            exception_buffers: array::from_fn(|_| Vec::new()),
            data_pointers: [0; 33],
            freqs: [0; 33],
            optimal_bits: 0,
            exception_count: 0,
            max_bits: 0,
        })
    }

    fn compress_blocks(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) {
        let inlength = greatest_multiple(input_length, N as u32);
        let final_inpos = input_offset.position() as u32 + inlength;
        while input_offset.position() as u32 != final_inpos {
            let this_size = min(self.page_size, final_inpos - input_offset.position() as u32);
            self.encode_page(input, this_size, input_offset, output, output_offset);
        }
    }

    fn decode_headless_blocks(
        &mut self,
        input: &[u32],
        inlength: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        let mynvalue = greatest_multiple(inlength, N as u32);
        let final_out = output_offset.position() as u32 + mynvalue;
        while output_offset.position() as u32 != final_out {
            let this_size = min(self.page_size, final_out - output_offset.position() as u32);
            self.decode_page(input, input_offset, output, output_offset, this_size)?;
        }
        Ok(())
    }

    /// Encodes a page using optimal bit width per block.
    ///
    /// For each block:
    /// - Determines best bit width, bitpacks regular values,
    /// - Stores exceptions with positions.
    /// - Writes header, packed data, metadata bytes, and exception values.
    ///
    /// # Arguments
    /// * `this_size` - Must be multiple of `block_size`
    /// * `input_offset` - Advanced by `this_size`
    /// * `output_offset` - Advanced by compressed size
    fn encode_page(
        &mut self,
        input: &[u32],
        this_size: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) {
        let header_pos = output_offset.position() as usize;
        output_offset.increment();
        let mut tmp_output_offset = output_offset.position() as u32;

        // Data pointers to 0
        self.data_pointers.fill(0);
        self.bytes_container.clear();

        let mut tmp_input_offset = input_offset.position() as u32;
        let final_input_offset = tmp_input_offset + this_size - N as u32;
        while tmp_input_offset <= final_input_offset {
            self.best_bit_from_data(input, tmp_input_offset);
            self.bytes_container.put_u8(self.optimal_bits);
            self.bytes_container.put_u8(self.exception_count);
            if self.exception_count > 0 {
                self.bytes_container.put_u8(self.max_bits);
                let index = usize::from(self.max_bits - self.optimal_bits);
                let needed = self.data_pointers[index] + usize::from(self.exception_count);
                if needed > self.exception_buffers[index].len() {
                    // Grow to the next multiple of 32 above 2×needed, to amortize resizes.
                    let new_cap = needed.saturating_mul(2).next_multiple_of(32);
                    self.exception_buffers[index].resize(new_cap, 0);
                }
                for k in 0..N as u32 {
                    if (input[(k + tmp_input_offset) as usize] >> self.optimal_bits) != 0 {
                        self.bytes_container.put_u8(k as u8);
                        self.exception_buffers[index][self.data_pointers[index]] =
                            input[(k + tmp_input_offset) as usize] >> self.optimal_bits;
                        self.data_pointers[index] += 1;
                    }
                }
            }
            for k in (0..N as u32).step_by(32) {
                bitpacking::fast_pack(
                    input,
                    (tmp_input_offset + k) as usize,
                    output,
                    tmp_output_offset as usize,
                    self.optimal_bits,
                );
                tmp_output_offset += u32::from(self.optimal_bits);
            }
            tmp_input_offset += N as u32;
        }
        input_offset.set_position(u64::from(tmp_input_offset));
        output[header_pos] = tmp_output_offset - header_pos as u32;
        let byte_size = self.bytes_container.len();
        while (self.bytes_container.len() & 3) != 0 {
            self.bytes_container.put_u8(0);
        }
        // Output should have 3 position as 4
        output[tmp_output_offset as usize] = byte_size as u32;
        tmp_output_offset += 1;
        let how_many_ints = self.bytes_container.len() / 4;
        // Match C++ memcpy: copy metadata bytes as u32s in one shot (native byte order).
        let meta_u32s: &[u32] = cast_slice(self.bytes_container.chunk());
        output[tmp_output_offset as usize..][..how_many_ints]
            .copy_from_slice(&meta_u32s[..how_many_ints]);
        tmp_output_offset += how_many_ints as u32;
        let mut bitmap = 0;
        for k in 2..=32 {
            if self.data_pointers[k] != 0 {
                bitmap |= 1 << (k - 1);
            }
        }
        output[tmp_output_offset as usize] = bitmap;
        tmp_output_offset += 1;

        for k in 2..=32 {
            if self.data_pointers[k] != 0 {
                output[tmp_output_offset as usize] = self.data_pointers[k] as u32;
                tmp_output_offset += 1;
                let mut j = 0;
                while j < self.data_pointers[k] {
                    bitpacking::fast_pack(
                        &self.exception_buffers[k],
                        j,
                        output,
                        tmp_output_offset as usize,
                        k as u8,
                    );
                    tmp_output_offset += k as u32;
                    j += 32;
                }

                // Overflow adjustment
                let overflow = j as u32 - self.data_pointers[k] as u32;
                tmp_output_offset -= (overflow * k as u32) / 32;
            }
        }
        output_offset.set_position(u64::from(tmp_output_offset));
    }

    /// Computes optimal bit width minimizing total storage cost.
    ///
    /// Analyzes frequency distribution to balance regular value bits against exception overhead.
    fn best_bit_from_data(&mut self, input: &[u32], pos: u32) {
        self.freqs.fill(0);
        let k_end = min(pos + N as u32, input.len() as u32);
        for k in pos..k_end {
            self.freqs[bits(input[k as usize])] += 1;
        }

        self.optimal_bits = 32;
        while self.freqs[self.optimal_bits as usize] == 0 {
            self.optimal_bits -= 1;
        }
        self.max_bits = self.optimal_bits;

        let mut best_cost = u32::from(self.optimal_bits) * N as u32;
        let mut num_exceptions: u32 = 0;
        self.exception_count = 0;

        for bits in (0..self.optimal_bits).rev() {
            num_exceptions += self.freqs[bits as usize + 1];
            if num_exceptions == N as u32 {
                break;
            }
            let diff = u32::from(self.max_bits - bits);
            let mut cost = num_exceptions * OVERHEAD_OF_EACH_EXCEPT
                + num_exceptions * diff
                + u32::from(bits) * N as u32
                + 8;
            if diff == 1 {
                cost -= num_exceptions;
            }
            if cost < best_cost {
                best_cost = cost;
                self.optimal_bits = bits;
                self.exception_count = num_exceptions as u8;
            }
        }
    }

    /// Decodes a compressed page.
    ///
    /// Reads header to locate exception data, loads exceptions by bit width,
    /// unpacks regular values per block, patches in exceptions by position.
    ///
    /// # Arguments
    /// * `this_size` - Expected decompressed integer count
    /// * `input_offset` - Advanced by bytes read
    /// * `output_offset` - Advanced by `this_size`
    #[expect(clippy::too_many_lines)]
    fn decode_page(
        &mut self,
        input: &[u32],
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
        this_size: u32,
    ) -> FastPForResult<()> {
        let n = u32::try_from(input.len())
            .map_err(|_| FastPForError::InvalidInputLength(input.len()))?;

        let init_pos =
            u32::try_from(input_offset.position()).map_err(|_| FastPForError::NotEnoughData)?;
        let where_meta = input.get_val(init_pos)?;
        input_offset.increment();
        let mut inexcept = init_pos
            .checked_add(where_meta)
            .ok_or(FastPForError::NotEnoughData)?;
        let bytesize = input.get_val(inexcept)?;
        inexcept = inexcept
            .checked_add(1)
            .ok_or(FastPForError::NotEnoughData)?;
        // Point a byte cursor directly at the metadata region in `input`,
        // mirrors C++ `const uint8_t *bytep = reinterpret_cast<const uint8_t *>(inexcept)`.
        // The C++ encoder uses a raw `memcpy` of bytes into the u32 output (no endian
        // conversion), and the decoder does a raw reinterpret_cast back -- both native byte
        // order. `cast_slice` is the exact Rust equivalent: a safe, zero-copy native view.
        let input_bytes: &[u8] = cast_slice(input);
        let mut byte_pos = (inexcept as usize)
            .checked_mul(4)
            .filter(|&bp| bp <= input_bytes.len())
            .ok_or(FastPForError::NotEnoughData)?;
        let length = bytesize.div_ceil(4);
        inexcept = inexcept
            .checked_add(length)
            .ok_or(FastPForError::NotEnoughData)?;

        let bitmap = input.get_val(inexcept)?;
        inexcept = inexcept
            .checked_add(1)
            .ok_or(FastPForError::NotEnoughData)?;

        for k in 2..=32 {
            if (bitmap & (1 << (k - 1))) != 0 {
                let size = input.get_val(inexcept)?;
                inexcept = inexcept
                    .checked_add(1)
                    .ok_or(FastPForError::NotEnoughData)?;
                // Reject adversarial inputs: exceptions can't exceed the page size.
                if size > self.page_size {
                    return Err(FastPForError::NotEnoughData);
                }
                // Ensure the buffer is large enough for `size` values, rounded up
                // to the next group of 32 for the bitunpacking calls.
                let rounded_up = size.next_multiple_of(32) as usize;
                if self.exception_buffers[k as usize].len() < rounded_up {
                    self.exception_buffers[k as usize].resize(rounded_up, 0);
                }
                let mut j: u32 = 0;
                // Process full groups directly from input
                while j.checked_add(32).is_some_and(|j32| j32 <= size)
                    && inexcept.checked_add(k).is_some_and(|ie| ie <= n)
                {
                    bitunpacking::fast_unpack(
                        input,
                        inexcept as usize,
                        &mut self.exception_buffers[k as usize],
                        j as usize,
                        k as u8,
                    );
                    inexcept += k; // safe: loop guard checked inexcept + k <= n <= u32::MAX
                    j += 32; // safe: loop guard checked j + 32 <= size
                }
                // Handle the final partial group using a stack buffer (mirrors C++ buffer[PACKSIZE*2])
                if j < size {
                    let words_needed = (size - j) // safe: j < size
                        .saturating_mul(k)
                        .div_ceil(32);
                    let avail = n - inexcept.min(n);
                    if avail < words_needed {
                        return Err(FastPForError::NotEnoughData);
                    }
                    let copy_len = words_needed as usize;
                    let mut tail_buf = [0u32; 64];
                    if copy_len == 0 {
                        return Err(FastPForError::NotEnoughData);
                    }
                    let start = inexcept as usize;
                    let src = input
                        .get(start..start + copy_len)
                        .ok_or(FastPForError::NotEnoughData)?;
                    tail_buf[..copy_len].copy_from_slice(src);
                    let tail_inpos = 0;
                    bitunpacking::fast_unpack(
                        &tail_buf,
                        tail_inpos,
                        &mut self.exception_buffers[k as usize],
                        j as usize,
                        k as u8,
                    );
                    inexcept += k;
                    j += 32;
                }
                let overflow = j - size;
                inexcept -= (overflow * k) / 32;
            }
        }

        self.data_pointers.fill(0);
        let mut tmp_output_offset = output_offset.position() as u32;
        let mut tmp_input_offset = input_offset.position() as u32;

        let run_end = this_size / N as u32;
        for _ in 0..run_end {
            let bits = input_bytes.get_val(byte_pos)?;
            if bits > 32 {
                return Err(FastPForError::NotEnoughData);
            }
            byte_pos += 1;
            let num_exceptions = input_bytes.get_val(byte_pos)?;
            byte_pos += 1;
            for k in (0..N as u32).step_by(32) {
                let in_start = tmp_input_offset as usize;
                let out_start = (tmp_output_offset + k) as usize;
                let in_end = in_start
                    .checked_add(usize::from(bits))
                    .ok_or(FastPForError::NotEnoughData)?;
                if in_end > input.len() {
                    return Err(FastPForError::NotEnoughData);
                }
                let out_end = out_start
                    .checked_add(32)
                    .ok_or(FastPForError::OutputBufferTooSmall)?;
                if out_end > output.len() {
                    return Err(FastPForError::OutputBufferTooSmall);
                }
                bitunpacking::fast_unpack(input, in_start, output, out_start, bits);
                tmp_input_offset += u32::from(bits);
            }
            if num_exceptions > 0 {
                let maxbits = input_bytes.get_val(byte_pos)?;
                byte_pos += 1;
                let index = maxbits
                    .checked_sub(bits)
                    .ok_or(FastPForError::NotEnoughData)?;
                if maxbits > 32 || index == 0 || index > 32 {
                    return Err(FastPForError::NotEnoughData);
                }
                let index = usize::from(index);
                if index == 1 {
                    for _ in 0..num_exceptions {
                        let pos = input_bytes.get_val(byte_pos)?;
                        byte_pos += 1;
                        if u32::from(pos) >= N as u32 {
                            return Err(FastPForError::NotEnoughData);
                        }
                        let out_idx = tmp_output_offset as usize + pos as usize;
                        if out_idx >= output.len() {
                            return Err(FastPForError::OutputBufferTooSmall);
                        }
                        output[out_idx] |= 1 << bits;
                    }
                } else {
                    for _ in 0..num_exceptions {
                        let pos = input_bytes.get_val(byte_pos)?;
                        byte_pos += 1;
                        if u32::from(pos) >= N as u32 {
                            return Err(FastPForError::NotEnoughData);
                        }
                        let out_idx = tmp_output_offset as usize + pos as usize;
                        if out_idx >= output.len() {
                            return Err(FastPForError::OutputBufferTooSmall);
                        }
                        let ptr = self.data_pointers[index];
                        let except_value = self.exception_buffers[index].get_val(ptr)?;
                        output[out_idx] |= except_value << bits;
                        self.data_pointers[index] += 1;
                    }
                }
            }
            tmp_output_offset += N as u32;
        }
        output_offset.set_position(u64::from(tmp_output_offset));
        input_offset.set_position(u64::from(inexcept));
        Ok(())
    }
}

impl<const N: usize> BlockCodec for FastPFor<N>
where
    [u32; N]: bytemuck::Pod,
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

    // ── Generic helpers ───────────────────────────────────────────────────────

    // ── Round-trip tests ──────────────────────────────────────────────────────

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
        assert!(
            FastPForBlock256::default()
                .decode_blocks(&[], None, &mut Vec::new())
                .is_err()
        );
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
        let compressed = block_compress::<FastPForBlock256>(&data).unwrap();
        let out = block_decompress::<FastPForBlock256>(&compressed, Some(256)).unwrap();
        assert_eq!(out, data);
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
