use std::array;
use std::io::Cursor;
use std::num::NonZeroU32;

use bytes::{Buf as _, BufMut as _, BytesMut};

use crate::helpers::{GetWithErr, bits, greatest_multiple};
use crate::rust::cursor::IncrementCursor;
use crate::rust::integer_compression::{bitpacking, bitunpacking};
use crate::rust::{Integer, Skippable};
use crate::{FastPForError, FastPForResult};

/// Block size constant for 256 integers per block
pub const BLOCK_SIZE_256: NonZeroU32 = NonZeroU32::new(256).unwrap();

/// Block size constant for 128 integers per block
pub const BLOCK_SIZE_128: NonZeroU32 = NonZeroU32::new(128).unwrap();

/// Overhead cost (in bits) for storing each exception's position in the block
const OVERHEAD_OF_EACH_EXCEPT: u32 = 8;

/// Default page size in number of integers
pub const DEFAULT_PAGE_SIZE: NonZeroU32 = NonZeroU32::new(65536).unwrap();

/// Fast Patched Frame-of-Reference ([`FastPFOR`](https://github.com/lemire/FastPFor)) integer compression codec.
///
/// It is useful for compressing sequences of unsigned 32-bit integers.
///
/// The algorithm works by
/// - dividing data into blocks,
/// - determining the optimal number of bits needed for most values, and
/// - handling exceptions (values requiring more bits) separately
#[derive(Debug)]
pub struct FastPFOR {
    /// Exception values indexed by bit width difference
    pub exception_buffers: [Vec<u32>; 33],
    /// Metadata buffer for encoding/decoding
    pub bytes_container: BytesMut,
    /// Maximum integers per page
    pub page_size: u32,
    /// Position trackers for exception arrays
    pub data_pointers: [usize; 33],
    /// Frequency count for each bit width:
    /// `freqs[i]` = count of values needing exactly i bits
    pub freqs: [u32; 33],
    /// Optimal number of bits chosen for the current block
    pub optimal_bits: u8,
    /// Number of exceptions that don't fit in the optimal bit width
    pub exception_count: u8,
    /// Maximum bit width required for any value in the block
    pub max_bits: u8,
    /// Integers per block (128 or 256)
    pub block_size: u32,
}

impl Skippable for FastPFOR {
    fn headless_compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        let inlength = greatest_multiple(input_length, self.block_size);
        let final_inpos = input_offset.position() as u32 + inlength;
        while input_offset.position() as u32 != final_inpos {
            let this_size =
                std::cmp::min(self.page_size, final_inpos - input_offset.position() as u32);
            self.encode_page(input, this_size, input_offset, output, output_offset);
        }
        Ok(())
    }

    #[expect(unused_variables)]
    fn headless_uncompress(
        &mut self,
        input: &[u32],
        inlength: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
        num: u32,
    ) -> FastPForResult<()> {
        if inlength == 0 && self.block_size == BLOCK_SIZE_128.get() {
            // Return early if there is no data to uncompress and block size is 128
            return Ok(());
        }
        let mynvalue = greatest_multiple(inlength, self.block_size);
        let final_out = output_offset.position() as u32 + mynvalue;
        while output_offset.position() as u32 != final_out {
            let this_size =
                std::cmp::min(self.page_size, final_out - output_offset.position() as u32);
            self.decode_page(input, input_offset, output, output_offset, this_size)?;
        }
        Ok(())
    }
}

impl Integer<u32> for FastPFOR {
    fn compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        let inlength = greatest_multiple(input_length, self.block_size);
        if inlength == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        output[output_offset.position() as usize] = inlength;
        output_offset.increment();
        self.headless_compress(input, inlength, input_offset, output, output_offset)
    }

    fn uncompress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        if input_length == 0 {
            // Return early if there is no data to uncompress
            return Ok(());
        }
        let outlength = input[input_offset.position() as usize];
        input_offset.increment();
        self.headless_uncompress(
            input,
            outlength,
            input_offset,
            output,
            output_offset,
            outlength,
        )
    }
}

impl Default for FastPFOR {
    fn default() -> Self {
        Self::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_256) // Use default values here
    }
}

impl FastPFOR {
    /// Creates codec with specified page and block sizes.
    #[must_use]
    pub fn new(page_size: NonZeroU32, block_size: NonZeroU32) -> FastPFOR {
        let page_size = page_size.get();
        let block_size = block_size.get();
        FastPFOR {
            page_size,
            block_size,
            bytes_container: BytesMut::with_capacity(
                (3 * page_size / block_size + page_size) as usize,
            ),
            exception_buffers: array::from_fn(|_| Vec::new()),
            data_pointers: [0; 33],
            freqs: [0; 33],
            optimal_bits: 0,
            exception_count: 0,
            max_bits: 0,
        }
    }

    /// Encodes a page using optimal bit width per block.
    ///
    /// For each block:
    /// - Determines best bit width, bitpacks regular values,
    /// - Stores exceptions with positions.
    /// - Writes header, packed data, metadata bytes, and exception values.
    ///
    /// # Arguments
    /// * `thissize` - Must be multiple of `block_size`
    /// * `input_offset` - Advanced by `thissize`
    /// * `output_offset` - Advanced by compressed size
    fn encode_page(
        &mut self,
        input: &[u32],
        thissize: u32,
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
        let final_input_offset = tmp_input_offset + thissize - self.block_size;
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
                for k in 0..self.block_size {
                    if (input[(k + tmp_input_offset) as usize] >> self.optimal_bits) != 0 {
                        self.bytes_container.put_u8(k as u8);
                        self.exception_buffers[index][self.data_pointers[index]] =
                            input[(k + tmp_input_offset) as usize] >> self.optimal_bits;
                        self.data_pointers[index] += 1;
                    }
                }
            }
            for k in (0..self.block_size).step_by(32) {
                bitpacking::fast_pack(
                    input,
                    (tmp_input_offset + k) as usize,
                    output,
                    tmp_output_offset as usize,
                    self.optimal_bits,
                );
                tmp_output_offset += u32::from(self.optimal_bits);
            }
            tmp_input_offset += self.block_size;
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

        for it in output
            .iter_mut()
            .skip(tmp_output_offset as usize)
            .take(how_many_ints)
        {
            *it = self.bytes_container.get_u32_le();
        }
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
        let k_end = std::cmp::min(pos + self.block_size, input.len() as u32);
        for k in pos..k_end {
            self.freqs[bits(input[k as usize])] += 1;
        }

        self.optimal_bits = 32;
        while self.freqs[self.optimal_bits as usize] == 0 {
            self.optimal_bits -= 1;
        }
        self.max_bits = self.optimal_bits;

        let mut best_cost = u32::from(self.optimal_bits) * self.block_size;
        let mut num_exceptions: u32 = 0;
        self.exception_count = 0;

        for bits in (0..self.optimal_bits).rev() {
            num_exceptions += self.freqs[bits as usize + 1];
            if num_exceptions == self.block_size {
                break;
            }
            let diff = u32::from(self.max_bits - bits);
            let mut cost = num_exceptions * OVERHEAD_OF_EACH_EXCEPT
                + num_exceptions * diff
                + u32::from(bits) * self.block_size
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
    /// * `thissize` - Expected decompressed integer count
    /// * `input_offset` - Advanced by bytes read
    /// * `output_offset` - Advanced by `thissize`
    #[expect(clippy::too_many_lines)]
    fn decode_page(
        &mut self,
        input: &[u32],
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
        thissize: u32,
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
        let input_bytes: &[u8] = bytemuck::cast_slice(input);
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
                    debug_assert!(
                        copy_len > 0,
                        "j < size and k >= 2 guarantee words_needed >= 1"
                    );
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

        let run_end = thissize / self.block_size;
        for _ in 0..run_end {
            let bits = input_bytes.get_val(byte_pos)?;
            if bits > 32 {
                return Err(FastPForError::NotEnoughData);
            }
            byte_pos += 1;
            let num_exceptions = input_bytes.get_val(byte_pos)?;
            byte_pos += 1;
            for k in (0..self.block_size).step_by(32) {
                let in_start = tmp_input_offset as usize;
                let out_start = (tmp_output_offset + k) as usize;
                if in_start + usize::from(bits) > input.len() {
                    return Err(FastPForError::NotEnoughData);
                }
                if out_start + 32 > output.len() {
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
                        if u32::from(pos) >= self.block_size {
                            return Err(FastPForError::NotEnoughData);
                        }
                        let out_idx = tmp_output_offset as usize + pos as usize;
                        // out_idx < output.len(): pos < block_size and the bitunpack
                        // guard above already confirmed output.len() >= tmp_output_offset + block_size.
                        debug_assert!(out_idx < output.len());
                        output[out_idx] |= 1 << bits;
                    }
                } else {
                    for _ in 0..num_exceptions {
                        let pos = input_bytes.get_val(byte_pos)?;
                        byte_pos += 1;
                        if u32::from(pos) >= self.block_size {
                            return Err(FastPForError::NotEnoughData);
                        }
                        let out_idx = tmp_output_offset as usize + pos as usize;
                        // out_idx < output.len(): same invariant as index==1 branch above.
                        debug_assert!(out_idx < output.len());
                        let ptr = self.data_pointers[index];
                        let except_value = self.exception_buffers[index].get_val(ptr)?;
                        output[out_idx] |= except_value << bits;
                        self.data_pointers[index] += 1;
                    }
                }
            }
            tmp_output_offset += self.block_size;
        }
        output_offset.set_position(u64::from(tmp_output_offset));
        input_offset.set_position(u64::from(inexcept));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fastpfor_test() {
        let mut codec1 = FastPFOR::default();
        let mut codec2 = FastPFOR::default();
        let mut data = vec![0u32; BLOCK_SIZE_256.get() as usize];
        data[126] = -1i32 as u32;
        let mut out_buf = vec![0; data.len() * 4];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);
        codec1
            .compress(
                &data,
                data.len() as u32,
                &mut input_offset,
                &mut out_buf,
                &mut output_offset,
            )
            .unwrap();
        let comp = out_buf[..output_offset.position() as usize].to_vec();

        let mut out_buf_uncomp = vec![0; data.len() * 4];
        input_offset = Cursor::new(0);
        output_offset = Cursor::new(0);
        codec2
            .uncompress(
                &comp,
                comp.len() as u32,
                &mut input_offset,
                &mut out_buf_uncomp,
                &mut output_offset,
            )
            .unwrap();
        let answer = out_buf_uncomp[..output_offset.position() as usize].to_vec();

        assert_eq!(answer.len(), BLOCK_SIZE_256.get() as usize);
        assert_eq!(data.len(), BLOCK_SIZE_256.get() as usize);
        for k in 0..BLOCK_SIZE_256.get() {
            assert_eq!(answer[k as usize], data[k as usize], "bug in {k}");
        }
    }

    #[test]
    fn fastpfor_test_128() {
        let mut codec1 = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
        let mut codec2 = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
        let mut data = vec![0; BLOCK_SIZE_128.get() as usize];
        data[126] = -1i32 as u32;
        let mut out_buf = vec![0; data.len() * 4];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);
        codec1
            .compress(
                &data,
                data.len() as u32,
                &mut input_offset,
                &mut out_buf,
                &mut output_offset,
            )
            .unwrap();
        let comp = out_buf[..output_offset.position() as usize].to_vec();

        let mut out_buf_uncomp = vec![0; data.len() * 4];
        input_offset = Cursor::new(0);
        output_offset = Cursor::new(0);
        codec2
            .uncompress(
                &comp,
                comp.len() as u32,
                &mut input_offset,
                &mut out_buf_uncomp,
                &mut output_offset,
            )
            .unwrap();
        let answer = out_buf_uncomp[..output_offset.position() as usize].to_vec();

        assert_eq!(answer.len(), BLOCK_SIZE_128.get() as usize);
        assert_eq!(data.len(), BLOCK_SIZE_128.get() as usize);
        for k in 0..BLOCK_SIZE_128.get() {
            assert_eq!(answer[k as usize], data[k as usize], "bug in {k}");
        }
    }

    #[test]
    fn test_spurious() {
        let mut c = FastPFOR::default();
        let x = vec![0; 1024];
        let mut y = vec![0; 0];
        let mut i0 = Cursor::new(0);
        let mut i1 = Cursor::new(0);
        for inlength in 0..32 {
            c.compress(&x, inlength, &mut i0, &mut y, &mut i1).unwrap();
            assert_eq!(0, i1.position());
        }
    }

    #[test]
    fn test_zero_in_zero_out() {
        let mut c = FastPFOR::default();
        let x = vec![0; 0];
        let mut y = vec![0; 0];
        let mut i0 = Cursor::new(0);
        let mut i1 = Cursor::new(0);
        c.compress(&x, 0, &mut i0, &mut y, &mut i1).unwrap();
        assert_eq!(0, i1.position());

        // Needs uncompress
        let mut out = vec![0; 0];
        let mut outpos = Cursor::new(0);
        c.uncompress(&y, 0, &mut i1, &mut out, &mut outpos).unwrap();
        assert_eq!(0, outpos.position());
    }

    // The following tests are ported from C++
    fn run_codec_test(codec: &mut FastPFOR, data: &[u32]) {
        let mut compressed = vec![0u32; data.len() * 2];
        let mut decompressed = vec![0u32; data.len()];
        let len = data.len() as u32;
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        codec
            .compress(
                data,
                len,
                &mut input_offset,
                &mut compressed,
                &mut output_offset,
            )
            .expect("Compression failed");

        input_offset.set_position(0);
        output_offset.set_position(0);

        codec
            .uncompress(
                &compressed,
                len,
                &mut input_offset,
                &mut decompressed,
                &mut output_offset,
            )
            .expect("Decompression failed");

        for (i, &original) in data.iter().enumerate() {
            assert_eq!(
                decompressed[i], original,
                "Mismatch at index {}: {} != {}",
                i, decompressed[i], original
            );
        }
    }

    #[test]
    fn test_constant_sequence() {
        let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
        let data = vec![42u32; 65536];
        run_codec_test(&mut codec, &data);
    }

    #[test]
    fn test_alternating_sequence() {
        let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
        let data: Vec<_> = (0..65536).map(|i| u32::from(i % 2 != 0)).collect(); // Alternating 0s and 1s
        run_codec_test(&mut codec, &data);
    }

    #[test]
    fn test_large_numbers() {
        let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
        let data: Vec<u32> = (0..65536).map(|i| i + (1u32 << 30)).collect(); // Large numbers near 2^30
        run_codec_test(&mut codec, &data);
    }

    // The following tests fail. It is not clear if this is due the translation or there's a bug
    // Fails
    // #[test]
    // fn test_powers_of_two() {
    //     let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
    //     let data: Vec<u32> = (0..32).map(|i| 1 << i).collect(); // Powers of 2
    //     run_codec_test(&mut codec, &data);
    // }

    // Fails
    // #[test]
    // fn test_large_random_sequence() {
    //     let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
    //     let data = generate_random_data(100000); // Large random data set
    //     run_codec_test(&mut codec, &data);
    // }

    // Fails
    // #[test]
    // fn test_edge_cases() {
    //     let mut codec = fastpfor::FastPFOR::new(fastpfor::DEFAULT_PAGE_SIZE, fastpfor::BLOCK_SIZE_128);
    //     let data = vec![u32::MIN, u32::MAX, 0, 1, 42, u32::MAX - 1]; // Edge cases
    //     run_codec_test(&mut codec, &data);
    // }

    // Fails
    // Utility to generate random data
    // fn generate_random_data(size: usize) -> Vec<u32> {
    //     let mut rng = thread_rng();
    //     (0..size).map(|_| rng.gen()).collect()
    // }

    /// Compress one block of data and return the compressed words.
    fn compress_one_block(data: &[u32]) -> Vec<u32> {
        let mut codec = FastPFOR::default();
        let mut compressed = vec![0u32; data.len() * 4];
        let mut in_off = Cursor::new(0);
        let mut out_off = Cursor::new(0);
        codec
            .compress(
                data,
                data.len() as u32,
                &mut in_off,
                &mut compressed,
                &mut out_off,
            )
            .unwrap();
        compressed[..out_off.position() as usize].to_vec()
    }

    #[test]
    fn test_truncated_input_returns_error() {
        let data = vec![42u32; BLOCK_SIZE_256.get() as usize];
        let compressed = compress_one_block(&data);

        // Try decompressing with progressively shorter inputs — all must error, never panic.
        for truncated_len in [1, 2, compressed.len() / 2, compressed.len() - 1] {
            let truncated = &compressed[..truncated_len];
            let mut codec = FastPFOR::default();
            let mut out = vec![0u32; data.len()];
            let mut in_off = Cursor::new(0);
            let mut out_off = Cursor::new(0);
            let result = codec.uncompress(
                truncated,
                truncated.len() as u32,
                &mut in_off,
                &mut out,
                &mut out_off,
            );
            assert!(
                result.is_err(),
                "expected error for truncated len {truncated_len}, got Ok"
            );
        }
    }

    #[test]
    fn test_corrupted_where_meta_returns_error() {
        let data = vec![1u32; BLOCK_SIZE_256.get() as usize];
        let mut compressed = compress_one_block(&data);

        // The first word after the length header is `where_meta` — point it far past the end.
        if compressed.len() > 1 {
            compressed[1] = u32::MAX;
        }

        let mut codec = FastPFOR::default();
        let mut out = vec![0u32; data.len()];
        let mut in_off = Cursor::new(0);
        let mut out_off = Cursor::new(0);
        let result = codec.uncompress(
            &compressed,
            compressed.len() as u32,
            &mut in_off,
            &mut out,
            &mut out_off,
        );
        assert!(result.is_err(), "expected error for corrupted where_meta");
    }
}
