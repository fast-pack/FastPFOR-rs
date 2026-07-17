//! 64-bit ([`u64`]) `FastPFOR` codec.
//!
//! This is the widened counterpart of the 32-bit [`FastPFor`](super::fastpfor::FastPFor)
//! codec: the algorithm and wire format are identical except that values, exceptions,
//! and the exception bitmap are 64 bits wide. It is byte-compatible with the C++
//! `CppFastPFor128` / `CppFastPFor256` `encode64` / `decode64` paths.
//!
//! Like the 32-bit codec, `FastPFOR` only handles complete blocks, so a
//! [`VariableByte`] tail encodes the sub-block remainder. [`FastPForWide`]
//! bundles both and implements [`BlockCodec64`].

use std::cmp::min;
use std::io::Cursor;

use bytemuck::{cast_slice, cast_slice_mut};
use bytes::{Buf as _, BufMut as _, BytesMut};

use crate::codec::default_max_decoded_len;
use crate::helpers::{AsUsize, GetWithErr, greatest_multiple};
use crate::rust::cursor::IncrementCursor;
use crate::rust::integer_compression::bitpacking_wide::{pack_wide, unpack_wide};
use crate::{BlockCodec64, FastPForError, FastPForResult};

/// Overhead cost (in bits) for storing each exception's position in the block.
const OVERHEAD_OF_EACH_EXCEPT: u32 = 8;

/// Default page size in number of integers.
const DEFAULT_PAGE_SIZE: u32 = 65536;

/// Number of frequency/exception buckets: one per possible bit width `0..=64`.
const WIDTHS: usize = 65;

/// [`FastPForWide`] with 128-value blocks.
pub type FastPForWide128 = FastPForWide<128>;

/// [`FastPForWide`] with 256-value blocks.
pub type FastPForWide256 = FastPForWide<256>;

fn bits64(value: u64) -> usize {
    64 - value.leading_zeros().as_usize()
}

/// 64-bit `FastPFOR` codec: `FastPFOR`-packed blocks plus a variable-byte tail.
///
/// `N` is the block size (128 or 256 values). Use [`FastPForWide128`] or
/// [`FastPForWide256`], or call [`BlockCodec64::encode64`] / [`decode64`](BlockCodec64::decode64).
#[derive(Debug)]
pub struct FastPForWide<const N: usize> {
    exception_buffers: [Vec<u64>; WIDTHS],
    bytes_container: BytesMut,
    page_size: u32,
    data_pointers: [usize; WIDTHS],
    freqs: [u32; WIDTHS],
    optimal_bits: u8,
    exception_count: u8,
    max_bits: u8,
}

impl<const N: usize> Default for FastPForWide<N> {
    fn default() -> Self {
        Self::new(DEFAULT_PAGE_SIZE)
    }
}

impl<const N: usize> FastPForWide<N> {
    fn new(page_size: u32) -> Self {
        Self {
            bytes_container: BytesMut::with_capacity(
                (3 * page_size / N as u32 + page_size) as usize,
            ),
            page_size,
            exception_buffers: std::array::from_fn(|_| Vec::new()),
            data_pointers: [0; WIDTHS],
            freqs: [0; WIDTHS],
            optimal_bits: 0,
            exception_count: 0,
            max_bits: 0,
        }
    }

    fn compress_blocks(
        &mut self,
        input: &[u64],
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
        output: &mut [u64],
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

    fn encode_page(
        &mut self,
        input: &[u64],
        this_size: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) {
        let header_pos = output_offset.position() as usize;
        output_offset.increment();
        let mut tmp_output_offset = output_offset.position() as u32;

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
                pack_wide(
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
        output[tmp_output_offset as usize] = byte_size as u32;
        tmp_output_offset += 1;
        let how_many_ints = self.bytes_container.len() / 4;
        let meta_u32s: &[u32] = cast_slice(self.bytes_container.chunk());
        output[tmp_output_offset as usize..][..how_many_ints]
            .copy_from_slice(&meta_u32s[..how_many_ints]);
        tmp_output_offset += how_many_ints as u32;

        let mut bitmap: u64 = 0;
        for k in 2..=64 {
            if self.data_pointers[k] != 0 {
                bitmap |= 1u64 << (k - 1);
            }
        }
        output[tmp_output_offset as usize] = bitmap as u32;
        output[tmp_output_offset as usize + 1] = (bitmap >> 32) as u32;
        tmp_output_offset += 2;

        for k in 2..=64 {
            if self.data_pointers[k] != 0 {
                output[tmp_output_offset as usize] = self.data_pointers[k] as u32;
                tmp_output_offset += 1;
                let mut j = 0;
                while j < self.data_pointers[k] {
                    pack_wide(
                        &self.exception_buffers[k],
                        j,
                        output,
                        tmp_output_offset as usize,
                        k as u8,
                    );
                    tmp_output_offset += k as u32;
                    j += 32;
                }
                let overflow = j as u32 - self.data_pointers[k] as u32;
                tmp_output_offset -= (overflow * k as u32) / 32;
            }
        }
        output_offset.set_position(u64::from(tmp_output_offset));
    }

    fn best_bit_from_data(&mut self, input: &[u64], pos: u32) {
        self.freqs.fill(0);
        let k_end = min(pos + N as u32, input.len() as u32);
        for k in pos..k_end {
            self.freqs[bits64(input[k as usize])] += 1;
        }

        self.optimal_bits = 64;
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

    #[expect(clippy::too_many_lines)]
    fn decode_page(
        &mut self,
        input: &[u32],
        input_offset: &mut Cursor<u32>,
        output: &mut [u64],
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
        let input_bytes: &[u8] = cast_slice(input);
        let mut byte_pos = (inexcept as usize)
            .checked_mul(4)
            .filter(|&bp| bp <= input_bytes.len())
            .ok_or(FastPForError::NotEnoughData)?;
        let length = bytesize.div_ceil(4);
        inexcept = inexcept
            .checked_add(length)
            .ok_or(FastPForError::NotEnoughData)?;

        let bitmap_lo = input.get_val(inexcept)?;
        let bitmap_hi = input.get_val(
            inexcept
                .checked_add(1)
                .ok_or(FastPForError::NotEnoughData)?,
        )?;
        let bitmap = u64::from(bitmap_lo) | (u64::from(bitmap_hi) << 32);
        inexcept = inexcept
            .checked_add(2)
            .ok_or(FastPForError::NotEnoughData)?;

        for k in 2..=64u32 {
            if (bitmap & (1u64 << (k - 1))) != 0 {
                let size = input.get_val(inexcept)?;
                inexcept = inexcept
                    .checked_add(1)
                    .ok_or(FastPForError::NotEnoughData)?;
                if size > self.page_size {
                    return Err(FastPForError::NotEnoughData);
                }
                let rounded_up = size.next_multiple_of(32) as usize;
                if self.exception_buffers[k as usize].len() < rounded_up {
                    self.exception_buffers[k as usize].resize(rounded_up, 0);
                }
                let mut j: u32 = 0;
                while j.checked_add(32).is_some_and(|j32| j32 <= size)
                    && inexcept.checked_add(k).is_some_and(|ie| ie <= n)
                {
                    unpack_wide(
                        input,
                        inexcept as usize,
                        &mut self.exception_buffers[k as usize],
                        j as usize,
                        k as u8,
                    );
                    inexcept += k;
                    j += 32;
                }
                if j < size {
                    let words_needed = (size - j).saturating_mul(k).div_ceil(32);
                    let avail = n - inexcept.min(n);
                    if avail < words_needed {
                        return Err(FastPForError::NotEnoughData);
                    }
                    let copy_len = words_needed as usize;
                    let mut tail_buf = [0u32; 128];
                    if copy_len == 0 {
                        return Err(FastPForError::NotEnoughData);
                    }
                    let start = inexcept as usize;
                    let src = input
                        .get(start..start + copy_len)
                        .ok_or(FastPForError::NotEnoughData)?;
                    tail_buf[..copy_len].copy_from_slice(src);
                    unpack_wide(
                        &tail_buf,
                        0,
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
            if bits > 64 {
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
                unpack_wide(input, in_start, output, out_start, bits);
                tmp_input_offset += u32::from(bits);
            }
            if num_exceptions > 0 {
                let maxbits = input_bytes.get_val(byte_pos)?;
                byte_pos += 1;
                let index = maxbits
                    .checked_sub(bits)
                    .ok_or(FastPForError::NotEnoughData)?;
                if maxbits > 64 || index == 0 || index > 64 {
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
                        output[out_idx] |= 1u64 << bits;
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

/// Variable-byte (LEB128 with high-bit terminator) encoding of the `u64` tail.
///
/// Matches the C++ `VariableByte::encodeToByteArray` layout: every byte but the
/// last carries 7 payload bits with the high bit clear; the final byte has its
/// high bit set. Output is padded with zero bytes to a whole number of `u32` words.
fn vbyte_encode64(input: &[u64], out: &mut Vec<u32>) {
    if input.is_empty() {
        return;
    }
    let start = out.len();
    let capacity = input.len() * 3 + 4;
    out.resize(start + capacity, 0);
    let bytes: &mut [u8] = cast_slice_mut(&mut out[start..]);
    let mut byte_pos = 0;
    for &value in input {
        let mut v = value;
        while v >= 0x80 {
            bytes[byte_pos] = (v as u8) & 0x7F;
            byte_pos += 1;
            v >>= 7;
        }
        bytes[byte_pos] = (v as u8) | 0x80;
        byte_pos += 1;
    }
    while byte_pos % 4 != 0 {
        bytes[byte_pos] = 0;
        byte_pos += 1;
    }
    out.truncate(start + byte_pos / 4);
}

/// Inverse of [`vbyte_encode64`]. Trailing zero padding decodes to no value
/// because a padding byte never has the high-bit terminator.
fn vbyte_decode64(input: &[u32], out: &mut Vec<u64>) -> FastPForResult<()> {
    if input.is_empty() {
        return Ok(());
    }
    let bytes: &[u8] = cast_slice(input);
    let byte_len = bytes.len();
    let mut byte_pos = 0;
    while byte_pos < byte_len {
        let mut v: u64 = 0;
        let mut shift = 0u32;
        loop {
            if byte_pos >= byte_len {
                return Ok(());
            }
            let c = bytes[byte_pos];
            byte_pos += 1;
            if shift >= 64 {
                return Err(FastPForError::NotEnoughData);
            }
            if c >= 0x80 {
                v |= u64::from(c & 0x7F) << shift;
                out.push(v);
                break;
            }
            v |= u64::from(c) << shift;
            shift += 7;
        }
    }
    Ok(())
}

impl<const N: usize> BlockCodec64 for FastPForWide<N> {
    fn encode64(&mut self, input: &[u64], out: &mut Vec<u32>) -> FastPForResult<()> {
        let rounded = (input.len() / N) * N;
        let n_values = rounded as u32;

        let start = out.len();
        if rounded == 0 {
            out.push(0);
        } else {
            let capacity = rounded * 3 + 1024;
            out.resize(start + 1 + capacity, 0);
            out[start] = n_values;

            let mut in_off = Cursor::new(0u32);
            let mut out_off = Cursor::new(0u32);
            self.compress_blocks(
                &input[..rounded],
                n_values,
                &mut in_off,
                &mut out[start + 1..],
                &mut out_off,
            );
            let written = 1 + out_off.position() as usize;
            out.truncate(start + written);
        }

        vbyte_encode64(&input[rounded..], out);
        Ok(())
    }

    fn decode64(&mut self, input: &[u32], out: &mut Vec<u64>) -> FastPForResult<()> {
        let Some((&block_n_values, rest)) = input.split_first() else {
            return Ok(());
        };
        if block_n_values % N as u32 != 0 {
            return Err(FastPForError::NotEnoughData);
        }
        if block_n_values.as_usize() > default_max_decoded_len(input.len()) {
            return Err(FastPForError::NotEnoughData);
        }
        let n_blocks = block_n_values.as_usize() / N;

        let consumed = if n_blocks == 0 {
            1
        } else {
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
            1 + in_off.position() as usize
        };

        let tail_input = input.get(consumed..).ok_or(FastPForError::NotEnoughData)?;
        vbyte_decode64(tail_input, out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip<const N: usize>(input: &[u64]) {
        let mut codec = FastPForWide::<N>::default();
        let mut encoded = Vec::new();
        codec.encode64(input, &mut encoded).unwrap();
        let mut decoded = Vec::new();
        codec.decode64(&encoded, &mut decoded).unwrap();
        assert_eq!(decoded, input, "roundtrip mismatch (N={N})");
    }

    #[test]
    fn empty() {
        roundtrip::<128>(&[]);
        roundtrip::<256>(&[]);
    }

    #[test]
    fn single_value() {
        roundtrip::<128>(&[42]);
        roundtrip::<256>(&[u64::MAX]);
    }

    #[test]
    fn sub_block_tail_only() {
        let data: Vec<u64> = (0..10).collect();
        roundtrip::<256>(&data);
    }

    #[test]
    fn exact_block() {
        let data: Vec<u64> = (0..128).collect();
        roundtrip::<128>(&data);
    }

    #[test]
    fn blocks_with_remainder() {
        let data: Vec<u64> = (0..600).collect();
        roundtrip::<256>(&data);
    }

    #[test]
    fn large_values_and_exceptions() {
        let data: Vec<u64> = (0..1024u32)
            .map(|i| if i % 7 == 0 { 1u64 << 60 } else { u64::from(i) })
            .collect();
        roundtrip::<128>(&data);
    }

    #[test]
    fn full_width_values() {
        let data: Vec<u64> = (0..256u32).map(|i| u64::MAX - u64::from(i)).collect();
        roundtrip::<256>(&data);
    }

    #[test]
    fn spans_multiple_pages() {
        let data: Vec<u64> = (0..70_000u64).map(|i| i.wrapping_mul(0x1_0001)).collect();
        roundtrip::<128>(&data);
    }

    #[cfg(feature = "cpp")]
    mod cpp_parity {
        use super::*;
        use crate::BlockCodec64;
        use crate::cpp::{CppFastPFor128, CppFastPFor256};

        fn cases() -> Vec<Vec<u64>> {
            vec![
                vec![],
                vec![42],
                vec![u64::MAX],
                (0..10).collect(),
                (0..128).collect(),
                (0..256).collect(),
                (0..600).collect(),
                (0..1024u32)
                    .map(|i| if i % 7 == 0 { 1u64 << 60 } else { u64::from(i) })
                    .collect(),
                (0..256u32).map(|i| u64::MAX - u64::from(i)).collect(),
                (0..5000u64).map(|i| i.wrapping_mul(0x1_0001)).collect(),
            ]
        }

        fn assert_parity<const N: usize>(rust: &mut FastPForWide<N>, cpp: &mut impl BlockCodec64) {
            for data in cases() {
                let mut rust_enc = Vec::new();
                rust.encode64(&data, &mut rust_enc).unwrap();
                let mut cpp_enc = Vec::new();
                cpp.encode64(&data, &mut cpp_enc).unwrap();
                assert_eq!(rust_enc, cpp_enc, "encode64 bytes differ for {data:?}");

                let mut rust_dec = Vec::new();
                rust.decode64(&cpp_enc, &mut rust_dec).unwrap();
                assert_eq!(rust_dec, data, "Rust failed to decode C++ output");

                let mut cpp_dec = Vec::new();
                cpp.decode64(&rust_enc, &mut cpp_dec).unwrap();
                assert_eq!(cpp_dec, data, "C++ failed to decode Rust output");
            }
        }

        #[test]
        fn parity_128() {
            assert_parity(&mut FastPForWide::<128>::default(), &mut CppFastPFor128::default());
        }

        #[test]
        fn parity_256() {
            assert_parity(&mut FastPForWide::<256>::default(), &mut CppFastPFor256::default());
        }
    }
}
