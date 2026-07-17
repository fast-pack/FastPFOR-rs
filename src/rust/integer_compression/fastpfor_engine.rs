//! Width-generic `FastPFOR` page engine shared by the 32- and 64-bit codecs.
//!
//! The block-splitting, best-bit search, exception handling, and metadata layout are
//! identical for `u32` and `u64`; only the element width differs.
//! [`FastPForInt`] abstracts the width-specific pieces so a single [`FastPForEngine`]
//! implements the algorithm once.
//! `u32` keeps its hand-unrolled bit-packing kernels; `u64` uses the generic wide packer.

use std::array;
use std::cmp::min;
use std::io::Cursor;

use bytemuck::cast_slice;
use bytes::{Buf as _, BufMut as _, BytesMut};

use crate::helpers::{GetWithErr, greatest_multiple};
use crate::rust::cursor::IncrementCursor;
use crate::rust::integer_compression::{bitpacking, bitpacking_wide, bitunpacking};
use crate::{FastPForError, FastPForResult};

/// Overhead cost (in bits) for storing each exception's position in the block.
const OVERHEAD_OF_EACH_EXCEPT: u32 = 8;

/// One frequency/exception bucket per possible bit width, up to the widest supported (`u64`).
const WIDTHS: usize = 65;

/// Element type of a `FastPFOR` stream: [`u32`] or [`u64`].
///
/// Implementors supply the width-specific operations the engine needs.
/// The exception bitmap spans [`BITMAP_WORDS`](Self::BITMAP_WORDS) output words.
pub trait FastPForInt: Copy + 'static {
    /// Bit width of the element: 32 or 64.
    const WIDTH: u8;
    /// Output words occupied by the exception bitmap: 1 for `u32`, 2 for `u64`.
    const BITMAP_WORDS: u32;
    /// The zero value.
    const ZERO: Self;

    /// Number of significant bits, i.e. `WIDTH - leading_zeros` (0 for a zero value).
    fn significant_bits(self) -> u8;
    /// Logical right shift by `n`, where `n < WIDTH`.
    fn shr(self, n: u8) -> Self;
    /// Whether the value is zero.
    fn is_zero(self) -> bool;
    /// `*dst |= val << shift`, where `shift < WIDTH`.
    fn or_shl_assign(dst: &mut Self, val: Self, shift: u8);
    /// `1 << shift`, where `shift < WIDTH`.
    fn one_shl(shift: u8) -> Self;

    /// Pack 32 values at `bit` bits each into `out`.
    fn fast_pack(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8);
    /// Unpack 32 values at `bit` bits each from `src`.
    fn fast_unpack(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8);

    /// Write the exception bitmap as [`BITMAP_WORDS`](Self::BITMAP_WORDS) words at `out`.
    fn write_bitmap(bitmap: u64, out: &mut [u32]);
    /// Read the exception bitmap from [`BITMAP_WORDS`](Self::BITMAP_WORDS) words at `pos`.
    fn read_bitmap(input: &[u32], pos: u32) -> FastPForResult<u64>;
}

#[allow(
    clippy::use_self,
    reason = "u32 literals here are stream words, not the Self element type"
)]
impl FastPForInt for u32 {
    const WIDTH: u8 = 32;
    const BITMAP_WORDS: u32 = 1;
    const ZERO: Self = 0;

    fn significant_bits(self) -> u8 {
        (32 - self.leading_zeros()) as u8
    }
    fn shr(self, n: u8) -> Self {
        self >> n
    }
    fn is_zero(self) -> bool {
        self == 0
    }
    fn or_shl_assign(dst: &mut Self, val: Self, shift: u8) {
        *dst |= val << shift;
    }
    fn one_shl(shift: u8) -> Self {
        1 << shift
    }
    fn fast_pack(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
        bitpacking::fast_pack(src, inpos, out, outpos, bit);
    }
    fn fast_unpack(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8) {
        bitunpacking::fast_unpack(src, inpos, out, outpos, bit);
    }
    fn write_bitmap(bitmap: u64, out: &mut [u32]) {
        out[0] = bitmap as u32;
    }
    fn read_bitmap(input: &[u32], pos: u32) -> FastPForResult<u64> {
        let word: u32 = input.get_val(pos)?;
        Ok(u64::from(word))
    }
}

#[allow(
    clippy::use_self,
    reason = "u32 literals here are stream words, not the Self element type"
)]
impl FastPForInt for u64 {
    const WIDTH: u8 = 64;
    const BITMAP_WORDS: u32 = 2;
    const ZERO: Self = 0;

    fn significant_bits(self) -> u8 {
        (64 - self.leading_zeros()) as u8
    }
    fn shr(self, n: u8) -> Self {
        self >> n
    }
    fn is_zero(self) -> bool {
        self == 0
    }
    fn or_shl_assign(dst: &mut Self, val: Self, shift: u8) {
        *dst |= val << shift;
    }
    fn one_shl(shift: u8) -> Self {
        1 << shift
    }
    fn fast_pack(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
        bitpacking_wide::pack_wide(src, inpos, out, outpos, bit);
    }
    fn fast_unpack(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8) {
        bitpacking_wide::unpack_wide(src, inpos, out, outpos, bit);
    }
    fn write_bitmap(bitmap: u64, out: &mut [u32]) {
        out[0] = bitmap as u32;
        out[1] = (bitmap >> 32) as u32;
    }
    fn read_bitmap(input: &[u32], pos: u32) -> FastPForResult<u64> {
        let lo: u32 = input.get_val(pos)?;
        let hi_pos = pos.checked_add(1).ok_or(FastPForError::NotEnoughData)?;
        let hi: u32 = input.get_val(hi_pos)?;
        Ok(u64::from(lo) | (u64::from(hi) << 32))
    }
}

/// Shared `FastPFOR` scratch state and page codec for a block size `N` and element type `T`.
#[derive(Debug)]
pub struct FastPForEngine<const N: usize, T: FastPForInt> {
    /// Exception values grouped by `max_bits - optimal_bits`.
    exception_buffers: [Vec<T>; WIDTHS],
    /// Per-block metadata (bit widths, exception counts, positions).
    bytes_container: BytesMut,
    /// Maximum integers per page.
    page_size: u32,
    /// Write positions into `exception_buffers`.
    data_pointers: [usize; WIDTHS],
    /// Count of values needing exactly `i` bits.
    freqs: [u32; WIDTHS],
    /// Chosen bit width for the current block.
    optimal_bits: u8,
    /// Exceptions that exceed `optimal_bits`.
    exception_count: u8,
    /// Widest value in the current block.
    max_bits: u8,
}

impl<const N: usize, T: FastPForInt> FastPForEngine<N, T> {
    pub fn new(page_size: u32) -> Self {
        Self {
            bytes_container: BytesMut::with_capacity(
                (3 * page_size / N as u32 + page_size) as usize,
            ),
            page_size,
            exception_buffers: array::from_fn(|_| Vec::new()),
            data_pointers: [0; WIDTHS],
            freqs: [0; WIDTHS],
            optimal_bits: 0,
            exception_count: 0,
            max_bits: 0,
        }
    }

    pub fn compress_blocks(
        &mut self,
        input: &[T],
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

    pub fn decode_headless_blocks(
        &mut self,
        input: &[u32],
        inlength: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [T],
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
        input: &[T],
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
                    self.exception_buffers[index].resize(new_cap, T::ZERO);
                }
                for k in 0..N as u32 {
                    let value = input[(k + tmp_input_offset) as usize];
                    if !value.shr(self.optimal_bits).is_zero() {
                        self.bytes_container.put_u8(k as u8);
                        self.exception_buffers[index][self.data_pointers[index]] =
                            value.shr(self.optimal_bits);
                        self.data_pointers[index] += 1;
                    }
                }
            }
            for k in (0..N as u32).step_by(32) {
                T::fast_pack(
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
        for k in 2..=usize::from(T::WIDTH) {
            if self.data_pointers[k] != 0 {
                bitmap |= 1u64 << (k - 1);
            }
        }
        T::write_bitmap(bitmap, &mut output[tmp_output_offset as usize..]);
        tmp_output_offset += T::BITMAP_WORDS;

        for k in 2..=usize::from(T::WIDTH) {
            if self.data_pointers[k] != 0 {
                output[tmp_output_offset as usize] = self.data_pointers[k] as u32;
                tmp_output_offset += 1;
                let mut j = 0;
                while j < self.data_pointers[k] {
                    T::fast_pack(
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

    fn best_bit_from_data(&mut self, input: &[T], pos: u32) {
        self.freqs.fill(0);
        let k_end = min(pos + N as u32, input.len() as u32);
        for k in pos..k_end {
            self.freqs[usize::from(input[k as usize].significant_bits())] += 1;
        }

        self.optimal_bits = T::WIDTH;
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
        output: &mut [T],
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

        let bitmap = T::read_bitmap(input, inexcept)?;
        inexcept = inexcept
            .checked_add(T::BITMAP_WORDS)
            .ok_or(FastPForError::NotEnoughData)?;

        for k in 2..=u32::from(T::WIDTH) {
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
                    self.exception_buffers[k as usize].resize(rounded_up, T::ZERO);
                }
                let mut j: u32 = 0;
                while j.checked_add(32).is_some_and(|j32| j32 <= size)
                    && inexcept.checked_add(k).is_some_and(|ie| ie <= n)
                {
                    T::fast_unpack(
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
                    let mut tail_buf = [0u32; 64];
                    if copy_len == 0 {
                        return Err(FastPForError::NotEnoughData);
                    }
                    let start = inexcept as usize;
                    let src = input
                        .get(start..start + copy_len)
                        .ok_or(FastPForError::NotEnoughData)?;
                    tail_buf[..copy_len].copy_from_slice(src);
                    T::fast_unpack(
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
            if bits > T::WIDTH {
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
                T::fast_unpack(input, in_start, output, out_start, bits);
                tmp_input_offset += u32::from(bits);
            }
            if num_exceptions > 0 {
                let maxbits = input_bytes.get_val(byte_pos)?;
                byte_pos += 1;
                let index = maxbits
                    .checked_sub(bits)
                    .ok_or(FastPForError::NotEnoughData)?;
                if maxbits > T::WIDTH || index == 0 || index > T::WIDTH {
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
                        T::or_shl_assign(&mut output[out_idx], T::one_shl(bits), 0);
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
                        T::or_shl_assign(&mut output[out_idx], except_value, bits);
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
