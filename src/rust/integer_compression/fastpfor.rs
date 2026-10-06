use std::cmp::min;
use std::io::Cursor;
use std::marker::PhantomData;

use bytemuck::cast_slice;

use crate::helpers::{GetWithErr, greatest_multiple};
use crate::rust::cursor::IncrementCursor;
use crate::rust::integer_compression::fastpfor_int::FastPForInt;
use crate::rust::kernels::{Auto, Kernels, Layout};
use crate::{FastPForError, FastPForResult};

pub(crate) mod sealed {
    /// Sealed marker trait: only valid `[T; N]` block arrays are accepted by `FastPForBlock`.
    ///
    /// This is intentionally private so that users cannot implement it for other sizes,
    /// preventing instantiation of `FastPForBlock` for unsupported `N`/`T` at compile time.
    pub trait BlockSize: bytemuck::Pod {}
    impl BlockSize for [u32; 128] {}
    impl BlockSize for [u32; 256] {}
    impl BlockSize for [u64; 128] {}
    impl BlockSize for [u64; 256] {}
}

/// Overhead cost (in bits) for storing each exception's position in the block
const OVERHEAD_OF_EACH_EXCEPT: u32 = 8;

/// Values per group in the interleaved layout.
const GROUP: usize = 128;

/// Output words per packed bit for one interleaved group (`GROUP` values at `bit` bits).
const GROUP_WORDS_PER_BIT: u32 = GROUP as u32 / 32;

/// Default page size in number of integers.
const DEFAULT_PAGE_SIZE: u32 = 65536;

/// Elements by which [`grow_to`] lengthens an output vector at a time: few enough (16 KiB of `u32`)
/// to still be in the L1 cache when the kernels overwrite them.
const GROW_STEP: usize = 4096;

/// Makes `output` at least `len` elements long, padding it with zeros [`GROW_STEP`] elements at a time.
///
/// The encoder and decoder write into the output in place, so it must be initialized first. Zero-filling
/// it in small steps just ahead of the writes is much cheaper than zero-filling it all up front: a whole
/// output does not fit in the cache, so every element would be written to memory twice.
#[inline]
fn grow_to<E: Copy>(output: &mut Vec<E>, len: usize, zero: E) {
    if output.len() < len {
        output.resize(len.max(output.len() + GROW_STEP), zero);
    }
}

pub(crate) trait PackFn<T>: Fn(&[T], usize, &mut [u32], usize, u8) + Copy {}
impl<T, F: Fn(&[T], usize, &mut [u32], usize, u8) + Copy> PackFn<T> for F {}

pub(crate) trait UnpackFn<T>: Fn(&[u32], usize, &mut [T], usize, u8) + Copy {}
impl<T, F: Fn(&[u32], usize, &mut [T], usize, u8) + Copy> UnpackFn<T> for F {}

/// Block codec of Fast Patched Frame-of-Reference ([FastPFOR](https://github.com/lemire/FastPFor)).
///
/// - `L`, the wire [`Layout`]: [`Sequential`](crate::Sequential) (C++ `FastPFor`) or
///   [`Interleaved`](crate::Interleaved) (C++ `SIMDFastPFor`). The two are **not** interchangeable.
/// - `T`, the element type: [`u32`] or [`u64`].
/// - `N`, the block size: 128 or 256 values.
/// - `K`, the [`Kernels`]: [`Auto`] (the default) or [`Portable`](crate::Portable). They only affect speed:
///   both write the same bytes.
///
/// This codec only accepts whole blocks, with compile-time block sizes through
/// [`BlockCodec`](crate::BlockCodec). Most users want the any-length codecs instead, such as
/// [`FastPForSequential32x128`](crate::FastPForSequential32x128), which add a variable-byte tail
/// for the values after the last whole block.
///
/// The per-block scratch buffers are sized exactly for `T` (`T::WIDTH + 1` buckets) via the
/// sealed `FastPForInt` trait, so the bucket count is neither wasted nor part of this
/// type's signature.
///
/// ```
/// # use fastpfor::{BlockCodec, FastPForBlock, Sequential, slice_to_blocks};
/// let data: Vec<u32> = (0..512).collect();
/// let mut codec = FastPForBlock::<Sequential, u32, 256>::default();
/// let (blocks, _remainder) = slice_to_blocks::<FastPForBlock<Sequential, u32, 256>>(&data);
/// let mut out = Vec::new();
/// codec.encode_blocks(blocks, &mut out).unwrap();
/// ```
#[derive(Debug)]
pub struct FastPForBlock<L: Layout, T: FastPForInt, const N: usize, K: Kernels = Auto> {
    /// Exception values indexed by bit width difference
    exception_buffers: T::ExceptionBuffers,
    /// Metadata buffer for encoding/decoding
    bytes_container: Vec<u8>,
    /// Maximum integers per page
    page_size: u32,
    /// Position trackers for exception arrays
    data_pointers: T::DataPointers,
    /// Frequency count for each bit width:
    /// `freqs[i]` = count of values needing exactly i bits
    freqs: T::Freqs,
    /// Optimal number of bits chosen for the current block
    optimal_bits: u8,
    /// Number of exceptions that don't fit in the optimal bit width
    exception_count: u8,
    /// Maximum bit width required for any value in the block
    max_bits: u8,
    marker: PhantomData<(L, K)>,
}

impl<L: Layout, T: FastPForInt, const N: usize, K: Kernels> Default for FastPForBlock<L, T, N, K> {
    fn default() -> Self {
        Self::new(DEFAULT_PAGE_SIZE)
            .expect("DEFAULT_PAGE_SIZE is a multiple of all valid block sizes")
    }
}

impl<L: Layout, T: FastPForInt, const N: usize, K: Kernels> FastPForBlock<L, T, N, K> {
    /// Creates a codec with the given page size.
    ///
    /// Returns an error if `page_size` is not a multiple of the block size.
    /// Use [`Default`] for the default page size.
    pub fn new(page_size: u32) -> FastPForResult<Self> {
        if !page_size.is_multiple_of(N as u32) {
            return Err(FastPForError::InvalidPageSize {
                page_size,
                block_size: N as u32,
            });
        }
        Ok(Self {
            bytes_container: Vec::with_capacity((3 * page_size / N as u32 + page_size) as usize),
            page_size,
            exception_buffers: T::new_exception_buffers(),
            data_pointers: T::new_data_pointers(),
            freqs: T::new_freqs(),
            optimal_bits: 0,
            exception_count: 0,
            max_bits: 0,
            marker: PhantomData,
        })
    }

    pub(crate) fn compress_blocks(
        &mut self,
        input: &[T],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut Vec<u32>,
        output_offset: &mut Cursor<u32>,
    ) {
        let inlength = greatest_multiple(input_length, N as u32);
        let final_inpos = input_offset.position() as u32 + inlength;
        while input_offset.position() as u32 != final_inpos {
            let this_size = min(self.page_size, final_inpos - input_offset.position() as u32);
            L::encode_page(self, input, this_size, input_offset, output, output_offset);
        }
    }

    pub(crate) fn decode_headless_blocks(
        &mut self,
        input: &[u32],
        inlength: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut Vec<T>,
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        let mynvalue = greatest_multiple(inlength, N as u32);
        let final_out = output_offset.position() as u32 + mynvalue;
        while output_offset.position() as u32 != final_out {
            let this_size = min(self.page_size, final_out - output_offset.position() as u32);
            L::decode_page(self, input, input_offset, output, output_offset, this_size)?;
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
    #[expect(
        clippy::inline_always,
        reason = "the kernel must inline into the page, including inside #[target_feature] callers"
    )]
    #[inline(always)]
    pub(crate) fn encode_page_with(
        &mut self,
        input: &[T],
        this_size: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut Vec<u32>,
        output_offset: &mut Cursor<u32>,
        pack: impl PackFn<T>,
    ) {
        self.encode_page_layout::<false>(
            input,
            this_size,
            input_offset,
            output,
            output_offset,
            pack,
            pack,
        );
    }

    /// Like [`encode_page_with`](Self::encode_page_with), but in the interleaved `SIMDFastPFor` layout:
    /// blocks and the bulk of each exception array are packed 128 values at a time by `pack_group`,
    /// and `pack` (32 values) only handles the tail of an exception array.
    #[expect(
        clippy::inline_always,
        reason = "the kernel must inline into the page, including inside #[target_feature] callers"
    )]
    #[expect(clippy::too_many_arguments, reason = "the page state plus two kernels")]
    #[inline(always)]
    pub(crate) fn encode_page_interleaved_with(
        &mut self,
        input: &[T],
        this_size: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut Vec<u32>,
        output_offset: &mut Cursor<u32>,
        pack: impl PackFn<T>,
        pack_group: impl PackFn<T>,
    ) {
        self.encode_page_layout::<true>(
            input,
            this_size,
            input_offset,
            output,
            output_offset,
            pack,
            pack_group,
        );
    }

    /// Shared page encoder. `INTERLEAVED` selects the layout: when `false`, `pack_group` is unused.
    #[expect(
        clippy::inline_always,
        reason = "the kernel must inline into the page, including inside #[target_feature] callers"
    )]
    #[expect(clippy::too_many_arguments, reason = "the page state plus two kernels")]
    #[expect(clippy::too_many_lines)]
    #[inline(always)]
    fn encode_page_layout<const INTERLEAVED: bool>(
        &mut self,
        input: &[T],
        this_size: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut Vec<u32>,
        output_offset: &mut Cursor<u32>,
        pack: impl PackFn<T>,
        pack_group: impl PackFn<T>,
    ) {
        let header_pos = output_offset.position() as usize;
        grow_to(output, header_pos + 1, 0);
        output_offset.increment();
        let mut tmp_output_offset = output_offset.position() as u32;

        // Data pointers to 0
        self.data_pointers.as_mut().fill(0);
        self.bytes_container.clear();

        let mut tmp_input_offset = input_offset.position() as u32;
        let final_input_offset = tmp_input_offset + this_size - N as u32;
        while tmp_input_offset <= final_input_offset {
            let block: &[T; N] = input[tmp_input_offset as usize..][..N]
                .try_into()
                .expect("N-value block");
            self.best_bit_from_data::<INTERLEAVED>(block);
            self.bytes_container.push(self.optimal_bits);
            self.bytes_container.push(self.exception_count);
            if self.exception_count > 0 {
                self.bytes_container.push(self.max_bits);
                let index = usize::from(self.max_bits - self.optimal_bits);
                let needed = self.data_pointers[index] + usize::from(self.exception_count);
                if needed > self.exception_buffers[index].len() {
                    // Grow to the next multiple of 32 above 2×needed, to amortize resizes.
                    let new_cap = needed.saturating_mul(2).next_multiple_of(32);
                    self.exception_buffers[index].resize(new_cap, T::zero());
                }
                let exceptions = &mut self.exception_buffers[index];
                let mut pointer = self.data_pointers[index];
                for (k, &value) in block.iter().enumerate() {
                    let high = value >> usize::from(self.optimal_bits);
                    if high != T::zero() {
                        self.bytes_container.push(k as u8);
                        exceptions[pointer] = high;
                        pointer += 1;
                    }
                }
                self.data_pointers[index] = pointer;
            }
            let block_words = u32::from(self.optimal_bits) * (N as u32 / 32);
            grow_to(output, (tmp_output_offset + block_words) as usize, 0);
            if INTERLEAVED {
                for k in (0..N as u32).step_by(GROUP) {
                    pack_group(
                        input,
                        (tmp_input_offset + k) as usize,
                        output,
                        tmp_output_offset as usize,
                        self.optimal_bits,
                    );
                    tmp_output_offset += GROUP_WORDS_PER_BIT * u32::from(self.optimal_bits);
                }
            } else {
                for k in (0..N as u32).step_by(32) {
                    pack(
                        input,
                        (tmp_input_offset + k) as usize,
                        output,
                        tmp_output_offset as usize,
                        self.optimal_bits,
                    );
                    tmp_output_offset += u32::from(self.optimal_bits);
                }
            }
            tmp_input_offset += N as u32;
        }
        input_offset.set_position(u64::from(tmp_input_offset));
        output[header_pos] = tmp_output_offset - header_pos as u32;
        let byte_size = self.bytes_container.len();
        while (self.bytes_container.len() & 3) != 0 {
            self.bytes_container.push(0);
        }
        // Room for the metadata, the bitmap, and the size of each exception array.
        grow_to(
            output,
            tmp_output_offset as usize
                + 1
                + self.bytes_container.len() / 4
                + T::BITMAP_WORDS as usize
                + usize::from(T::WIDTH),
            0,
        );
        // Output should have 3 position as 4
        output[tmp_output_offset as usize] = byte_size as u32;
        tmp_output_offset += 1;
        let how_many_ints = self.bytes_container.len() / 4;
        // Match C++ memcpy: copy metadata bytes as u32s in one shot (native byte order).
        let meta_u32s: &[u32] = cast_slice(&self.bytes_container);
        output[tmp_output_offset as usize..][..how_many_ints]
            .copy_from_slice(&meta_u32s[..how_many_ints]);
        tmp_output_offset += how_many_ints as u32;
        // Exception bitmap: one bit per bit-width bucket, written as `T::BITMAP_WORDS` words.
        let mut bitmap = T::zero();
        for k in 2..=usize::from(T::WIDTH) {
            if self.data_pointers[k] != 0 {
                bitmap |= T::one() << (k - 1);
            }
        }
        T::write_bitmap(bitmap, &mut output[tmp_output_offset as usize..]);
        tmp_output_offset += T::BITMAP_WORDS;

        for k in 2..=usize::from(T::WIDTH) {
            if self.data_pointers[k] != 0 {
                output[tmp_output_offset as usize] = self.data_pointers[k] as u32;
                tmp_output_offset += 1;
                // Whole interleaved groups (interleaved layout only), then 32-value groups.
                let used = self.data_pointers[k];
                let groups = if INTERLEAVED { used / GROUP } else { 0 };
                let words = groups * GROUP_WORDS_PER_BIT as usize * k
                    + (used - groups * GROUP).div_ceil(32) * k;
                grow_to(output, tmp_output_offset as usize + words, 0);
                // The last group of 32 is packed whole; zero its unused slots (as C++ does via
                // `resize`) so stale values from earlier pages or calls don't leak into the output.
                self.exception_buffers[k][used..used.next_multiple_of(32)].fill(T::zero());
                let mut j = 0;
                if INTERLEAVED {
                    while j + GROUP <= self.data_pointers[k] {
                        pack_group(
                            &self.exception_buffers[k],
                            j,
                            output,
                            tmp_output_offset as usize,
                            k as u8,
                        );
                        tmp_output_offset += GROUP_WORDS_PER_BIT * k as u32;
                        j += GROUP;
                    }
                }
                while j < self.data_pointers[k] {
                    pack(
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
    ///
    /// The standard layout discounts exceptions that are a single bit wider than the packed width,
    /// since they need no exception array. The interleaved layout (`INTERLEAVED`) does not,
    /// matching the C++ `SIMDFastPFor` encoder.
    fn best_bit_from_data<const INTERLEAVED: bool>(&mut self, block: &[T; N]) {
        // Four histograms filled in turn: neighboring values often have the same width, and
        // incrementing a single counter would make each value wait for the previous one's store.
        let mut freqs = [
            T::new_freqs(),
            T::new_freqs(),
            T::new_freqs(),
            T::new_freqs(),
        ];
        for values in block.as_chunks::<4>().0 {
            for (f, value) in freqs.iter_mut().zip(values) {
                f[usize::from(value.significant_bits())] += 1;
            }
        }
        for (k, total) in self.freqs.as_mut().iter_mut().enumerate() {
            *total = freqs[0][k] + freqs[1][k] + freqs[2][k] + freqs[3][k];
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
            if diff == 1 && !INTERLEAVED {
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
    #[expect(
        clippy::inline_always,
        reason = "the kernel must inline into the page, including inside #[target_feature] callers"
    )]
    #[inline(always)]
    pub(crate) fn decode_page_with(
        &mut self,
        input: &[u32],
        input_offset: &mut Cursor<u32>,
        output: &mut Vec<T>,
        output_offset: &mut Cursor<u32>,
        this_size: u32,
        unpack: impl UnpackFn<T>,
    ) -> FastPForResult<()> {
        self.decode_page_layout::<false>(
            input,
            input_offset,
            output,
            output_offset,
            this_size,
            unpack,
            unpack,
        )
    }

    /// Like [`decode_page_with`](Self::decode_page_with), but for the interleaved `SIMDFastPFor` layout.
    /// See [`encode_page_interleaved_with`](Self::encode_page_interleaved_with).
    #[expect(
        clippy::inline_always,
        reason = "the kernel must inline into the page, including inside #[target_feature] callers"
    )]
    #[expect(clippy::too_many_arguments, reason = "the page state plus two kernels")]
    #[inline(always)]
    pub(crate) fn decode_page_interleaved_with(
        &mut self,
        input: &[u32],
        input_offset: &mut Cursor<u32>,
        output: &mut Vec<T>,
        output_offset: &mut Cursor<u32>,
        this_size: u32,
        unpack: impl UnpackFn<T>,
        unpack_group: impl UnpackFn<T>,
    ) -> FastPForResult<()> {
        self.decode_page_layout::<true>(
            input,
            input_offset,
            output,
            output_offset,
            this_size,
            unpack,
            unpack_group,
        )
    }

    /// Shared page decoder. `INTERLEAVED` selects the layout: when `false`, `unpack_group` is unused.
    #[expect(clippy::too_many_lines)]
    #[expect(clippy::too_many_arguments, reason = "the page state plus two kernels")]
    #[expect(
        clippy::inline_always,
        reason = "the kernel must inline into the page, including inside #[target_feature] callers"
    )]
    #[inline(always)]
    fn decode_page_layout<const INTERLEAVED: bool>(
        &mut self,
        input: &[u32],
        input_offset: &mut Cursor<u32>,
        output: &mut Vec<T>,
        output_offset: &mut Cursor<u32>,
        this_size: u32,
        unpack: impl UnpackFn<T>,
        unpack_group: impl UnpackFn<T>,
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

        let bitmap = T::read_bitmap(input, inexcept)?;
        inexcept = inexcept
            .checked_add(T::BITMAP_WORDS)
            .ok_or(FastPForError::NotEnoughData)?;

        let mut exception_counts = T::new_data_pointers();
        for k in 2..=u32::from(T::WIDTH) {
            if bitmap & (T::one() << (k - 1) as usize) != T::zero() {
                let size = input.get_val(inexcept)?;
                inexcept = inexcept
                    .checked_add(1)
                    .ok_or(FastPForError::NotEnoughData)?;
                // Reject adversarial inputs: exceptions can't exceed the page size.
                if size > self.page_size {
                    return Err(FastPForError::NotEnoughData);
                }
                exception_counts[k as usize] = size as usize;
                // Ensure the buffer is large enough for `size` values, rounded up
                // to the next group of 32 for the bitunpacking calls.
                let rounded_up = size.next_multiple_of(32) as usize;
                if self.exception_buffers[k as usize].len() < rounded_up {
                    self.exception_buffers[k as usize].resize(rounded_up, T::zero());
                }
                let mut j: u32 = 0;
                if INTERLEAVED {
                    // Whole interleaved groups first, then the 32-value groups below handle the rest.
                    while j.checked_add(GROUP as u32).is_some_and(|jg| jg <= size) {
                        let words = GROUP_WORDS_PER_BIT * k; // safe: k <= 64
                        if inexcept.checked_add(words).is_none_or(|ie| ie > n) {
                            return Err(FastPForError::NotEnoughData);
                        }
                        unpack_group(
                            input,
                            inexcept as usize,
                            &mut self.exception_buffers[k as usize],
                            j as usize,
                            k as u8,
                        );
                        inexcept += words; // safe: checked above
                        j += GROUP as u32; // safe: loop guard checked j + GROUP <= size
                    }
                }
                // Process full groups directly from input
                while j.checked_add(32).is_some_and(|j32| j32 <= size)
                    && inexcept.checked_add(k).is_some_and(|ie| ie <= n)
                {
                    unpack(
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
                    unpack(
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

        self.data_pointers.as_mut().fill(0);
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

            let block_start = tmp_output_offset as usize;
            grow_to(output, block_start + N, T::zero());
            let block: &mut [T; N] = (&mut output[block_start..block_start + N])
                .try_into()
                .expect("N-value block");
            let (group, words_per_group) = if INTERLEAVED {
                (GROUP, GROUP_WORDS_PER_BIT * u32::from(bits))
            } else {
                (32, u32::from(bits))
            };
            let block_words = words_per_group as usize * (N / group);
            let in_start = tmp_input_offset as usize;
            if in_start
                .checked_add(block_words)
                .is_none_or(|end| end > input.len())
            {
                return Err(FastPForError::NotEnoughData);
            }
            for k in (0..N).step_by(group) {
                let in_pos = in_start + k / group * words_per_group as usize;
                if INTERLEAVED {
                    unpack_group(input, in_pos, block, k, bits);
                } else {
                    unpack(input, in_pos, block, k, bits);
                }
            }
            tmp_input_offset += block_words as u32; // safe: checked against input.len() above

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
                let count = usize::from(num_exceptions);
                let positions = input_bytes
                    .get(byte_pos..byte_pos + count)
                    .ok_or(FastPForError::NotEnoughData)?;
                byte_pos += count;
                if index == 1 {
                    for &pos in positions {
                        let value = block
                            .get_mut(usize::from(pos))
                            .ok_or(FastPForError::NotEnoughData)?;
                        *value |= T::one() << usize::from(bits);
                    }
                } else {
                    let first = self.data_pointers[index];
                    let end = first + count;
                    // Reject exceptions beyond the array's declared size, even if a previous
                    // call left a longer buffer behind.
                    if end > exception_counts[index] {
                        return Err(FastPForError::NotEnoughData);
                    }
                    let values = &self.exception_buffers[index][first..end];
                    for (&pos, &except_value) in positions.iter().zip(values) {
                        let value = block
                            .get_mut(usize::from(pos))
                            .ok_or(FastPForError::NotEnoughData)?;
                        *value |= except_value << usize::from(bits);
                    }
                    self.data_pointers[index] = end;
                }
            }
            tmp_output_offset += N as u32;
        }
        // Drop the zeros `grow_to` added past the last block.
        output.truncate(tmp_output_offset as usize);
        output_offset.set_position(u64::from(tmp_output_offset));
        input_offset.set_position(u64::from(inexcept));
        Ok(())
    }
}
