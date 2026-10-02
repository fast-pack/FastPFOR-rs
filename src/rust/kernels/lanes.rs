//! Interleaved bit packing: the block layout of the C++ `SIMDFastPFor` codec.
//!
//! A *group* is 128 values, viewed as a 128-bit vector of `LANES` lanes (4 lanes of `u32`, or 2 of `u64`).
//! Value `i` belongs to lane `i % LANES`, at row `i / LANES`. Each lane is bit-packed on its own at `bit` bits
//! per value, little-endian, into `bit` words of the lane width: row `r` occupies bits `[r*bit, (r+1)*bit)` of
//! the lane's stream. Word `w` of every lane is then stored as one 128-bit vector, so lane `l` of word `w`
//! sits at `u32` index `4*w + l` for `u32`, or at the pair `(4*w + 2*l, 4*w + 2*l + 1)` (low word first)
//! for `u64`. A group therefore always takes `4 * bit` `u32` words.
//!
//! The `u32` layout is byte-compatible with C++ `SIMD_fastpack_32`. C++ has no 64-bit counterpart:
//! the `u64` layout is this crate's natural extension of the same scheme.
//!
//! The kernels are written once over the small [`Lane`] vector trait and instantiated per [`Backend`]:
//! [`Portable`] plain arrays here, and the SSE2 or NEON vectors of the `wide` crate in `lanes_wide`.
#![expect(
    clippy::inline_always,
    reason = "the lane operations and rows must inline into the unrolled kernels, whatever the caller"
)]

use std::io::Cursor;

use crate::FastPForResult;
use crate::rust::integer_compression::fastpfor::FastPFor;
use crate::rust::integer_compression::fastpfor_int::FastPForInt;
use crate::rust::kernels::{InterleavedScalar, Kernels, private};

/// A 128-bit vector of unsigned lanes, with the handful of operations the kernels need.
///
/// Shift counts are always smaller than [`BITS`](Self::BITS).
pub trait Lane: Copy {
    /// The lane element type.
    type E: Copy;
    /// Bits per lane element.
    const BITS: u32;
    /// Lanes per vector: `128 / BITS`.
    const LANES: usize;

    /// Reads four consecutive `u32` words as one vector (little-endian within wider lanes).
    fn from_words(words: &[u32]) -> Self;
    /// Writes the vector as four consecutive `u32` words.
    fn to_words(self, out: &mut [u32]);
    /// Reads `LANES` consecutive elements.
    fn from_elems(src: &[Self::E]) -> Self;
    /// Writes `LANES` consecutive elements.
    fn to_elems(self, out: &mut [Self::E]);
    /// Every lane set to the low `bits` bits, `0 < bits <= BITS`.
    fn mask(bits: u32) -> Self;
    /// All lanes zero.
    fn zero() -> Self;
    /// Bitwise AND.
    fn and(self, rhs: Self) -> Self;
    /// Bitwise OR.
    fn or(self, rhs: Self) -> Self;
    /// Shifts every lane left by `s < BITS`.
    fn shl(self, s: u32) -> Self;
    /// Shifts every lane right by `s < BITS`.
    fn shr(self, s: u32) -> Self;
}

/// A set of [`Lane`] vector types, one per element width.
pub trait Backend {
    /// Vector of `u32` lanes.
    type V32: Lane<E = u32>;
    /// Vector of `u64` lanes.
    type V64: Lane<E = u64>;
}

/// Element types with interleaved kernels: [`u32`] and [`u64`].
pub trait LaneElem: Sized {
    /// Packs 128 values from `src[inpos..]` into `4 * bit` words at `out[outpos..]`.
    fn pack_group<B: Backend>(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8);
    /// Unpacks 128 values from `4 * bit` words at `input[inpos..]` into `out[outpos..]`.
    fn unpack_group<B: Backend>(
        input: &[u32],
        inpos: usize,
        out: &mut [Self],
        outpos: usize,
        bit: u8,
    );
}

// The row functions index with constants, so they are expanded once per row instead of looped:
// LLVM does not reliably unroll a 32- or 64-iteration body this size. A group has as many rows as
// the lane has bits, so `u32` expands 32 rows and `u64` expands 64, and each only for its own width.
macro_rules! for_rows {
    (32, $v:ty, $row:ident::<$b:ident> $args:tt) => {
        for_rows!(@rows $v, $row, $b, $args;
            0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)
    };
    (64, $v:ty, $row:ident::<$b:ident> $args:tt) => {
        for_rows!(@rows $v, $row, $b, $args;
            0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
            32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47
            48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63)
    };
    (@rows $v:ty, $row:ident, $b:ident, $args:tt; $($r:literal)*) => {
        $($row::<$v, $b, $r> $args;)*
    };
}

/// Unpacks row `R` (the `R`th value of every lane) at `B` bits.
#[inline(always)]
fn unpack_row<V: Lane, const B: usize, const R: usize>(input: &[u32], mask: V, out: &mut [V::E]) {
    let bits = V::BITS as usize;
    let (w, s) = (R * B / bits, R * B % bits);
    let lo = V::from_words(&input[4 * w..]).shr(s as u32);
    let value = if s + B > bits {
        lo.or(V::from_words(&input[4 * (w + 1)..]).shl((bits - s) as u32))
    } else {
        lo
    };
    value.and(mask).to_elems(&mut out[V::LANES * R..]);
}

/// Packs row `R` at `B` bits. `state` is the word being built and the bits carried into the next word.
#[inline(always)]
fn pack_row<V: Lane, const B: usize, const R: usize>(
    src: &[V::E],
    mask: V,
    out: &mut [u32],
    state: &mut (V, V),
) {
    let bits = V::BITS as usize;
    let (w, s) = (R * B / bits, R * B % bits);
    if R > 0 && w != (R - R.min(1)) * B / bits {
        // This row starts the next word: store the finished one. Words are written in order
        // so only two accumulators are live, instead of all `B` of them.
        state.0.to_words(&mut out[4 * (w - 1)..]);
        state.0 = state.1;
        state.1 = V::zero();
    }
    let value = V::from_elems(&src[V::LANES * R..]).and(mask);
    state.0 = state.0.or(value.shl(s as u32));
    if s + B > bits {
        state.1 = value.shr((bits - s) as u32);
    }
}

/// Defines the unpack and pack kernels for one group of 128 values at `B` bits, `0 < B <= V::BITS`,
/// with `$rows` rows (`V::BITS`). Packing masks the values to `B` bits.
macro_rules! group_kernels {
    ($rows:tt, $unpack:ident, $pack:ident) => {
        // Not `inline(always)`: the width dispatch would then hold a copy of every width's kernel, and at
        // opt-level 0 each copy keeps its own stack slots, enough to overflow the stack in debug builds.
        fn $unpack<V: Lane, const B: usize>(input: &[u32], out: &mut [V::E]) {
            let input = &input[..4 * B];
            let mask = V::mask(B as u32);
            for_rows!($rows, V, unpack_row::<B>(input, mask, out));
        }

        fn $pack<V: Lane, const B: usize>(src: &[V::E], out: &mut [u32]) {
            let out = &mut out[..4 * B];
            let mask = V::mask(B as u32);
            let mut state = (V::zero(), V::zero());
            for_rows!($rows, V, pack_row::<B>(src, mask, out, &mut state));
            state.0.to_words(&mut out[4 * (B - 1)..]);
        }
    };
}

group_kernels!(32, unpack_group_32, pack_group_32);
group_kernels!(64, unpack_group_64, pack_group_64);

macro_rules! dispatch_bits {
    ($bit:expr, $f:ident::<$v:ty> $args:tt; $($b:literal)*) => {
        match $bit {
            $($b => $f::<$v, $b> $args,)*
            _ => panic!("Unsupported bit width"),
        }
    };
}

fn pack_u32<V: Lane<E = u32>>(src: &[u32], out: &mut [u32], bit: u8) {
    dispatch_bits!(bit, pack_group_32::<V>(src, out);
        1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);
}

fn unpack_u32<V: Lane<E = u32>>(input: &[u32], out: &mut [u32], bit: u8) {
    dispatch_bits!(bit, unpack_group_32::<V>(input, out);
        1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);
}

fn pack_u64<V: Lane<E = u64>>(src: &[u64], out: &mut [u32], bit: u8) {
    dispatch_bits!(bit, pack_group_64::<V>(src, out);
        1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
        33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64);
}

fn unpack_u64<V: Lane<E = u64>>(input: &[u32], out: &mut [u64], bit: u8) {
    dispatch_bits!(bit, unpack_group_64::<V>(input, out);
        1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
        33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64);
}

impl LaneElem for u32 {
    #[inline]
    fn pack_group<B: Backend>(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
        if bit != 0 {
            pack_u32::<B::V32>(
                &src[inpos..inpos + 128],
                &mut out[outpos..outpos + 4 * usize::from(bit)],
                bit,
            );
        }
    }

    #[inline]
    fn unpack_group<B: Backend>(
        input: &[u32],
        inpos: usize,
        out: &mut [Self],
        outpos: usize,
        bit: u8,
    ) {
        let out = &mut out[outpos..outpos + 128];
        if bit == 0 {
            out.fill(0);
        } else {
            unpack_u32::<B::V32>(&input[inpos..inpos + 4 * usize::from(bit)], out, bit);
        }
    }
}

impl LaneElem for u64 {
    #[inline]
    fn pack_group<B: Backend>(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
        if bit != 0 {
            pack_u64::<B::V64>(
                &src[inpos..inpos + 128],
                &mut out[outpos..outpos + 4 * usize::from(bit)],
                bit,
            );
        }
    }

    #[inline]
    fn unpack_group<B: Backend>(
        input: &[u32],
        inpos: usize,
        out: &mut [Self],
        outpos: usize,
        bit: u8,
    ) {
        let out = &mut out[outpos..outpos + 128];
        if bit == 0 {
            out.fill(0);
        } else {
            unpack_u64::<B::V64>(&input[inpos..inpos + 4 * usize::from(bit)], out, bit);
        }
    }
}

/// Encodes one page in the interleaved layout using the kernels of backend `B`.
///
/// The 32-value kernel still handles the tail of each exception array, as in C++.
pub(super) fn encode_page_lanes<const N: usize, T: FastPForInt, K: Kernels, B: Backend>(
    codec: &mut FastPFor<N, T, K>,
    input: &[T],
    this_size: u32,
    input_offset: &mut Cursor<u32>,
    output: &mut [u32],
    output_offset: &mut Cursor<u32>,
) {
    codec.encode_page_interleaved_with(
        input,
        this_size,
        input_offset,
        output,
        output_offset,
        #[inline(always)]
        |src, inpos, out, outpos, bit| T::fast_pack(src, inpos, out, outpos, bit),
        #[inline(always)]
        |src, inpos, out, outpos, bit| T::pack_group::<B>(src, inpos, out, outpos, bit),
    );
}

/// Decodes one page in the interleaved layout using the kernels of backend `B`.
pub(super) fn decode_page_lanes<const N: usize, T: FastPForInt, K: Kernels, B: Backend>(
    codec: &mut FastPFor<N, T, K>,
    input: &[u32],
    input_offset: &mut Cursor<u32>,
    output: &mut [T],
    output_offset: &mut Cursor<u32>,
    this_size: u32,
) -> FastPForResult<()> {
    codec.decode_page_interleaved_with(
        input,
        input_offset,
        output,
        output_offset,
        this_size,
        #[inline(always)]
        |src, inpos, out, outpos, bit| T::fast_unpack(src, inpos, out, outpos, bit),
        #[inline(always)]
        |src, inpos, out, outpos, bit| T::unpack_group::<B>(src, inpos, out, outpos, bit),
    )
}

/// Portable backend: plain arrays, which the compiler vectorizes where the target allows.
pub struct Portable;

/// `u32` lanes as an array.
#[derive(Clone, Copy)]
pub struct P32([u32; 4]);

/// `u64` lanes as an array.
#[derive(Clone, Copy)]
pub struct P64([u64; 2]);

impl Backend for Portable {
    type V32 = P32;
    type V64 = P64;
}

macro_rules! elementwise {
    ($self:ident, $rhs:ident, $op:tt) => {
        Self(std::array::from_fn(|i| $self.0[i] $op $rhs.0[i]))
    };
}

impl Lane for P32 {
    type E = u32;
    const BITS: u32 = 32;
    const LANES: usize = 4;

    #[inline(always)]
    fn from_words(words: &[u32]) -> Self {
        Self::from_elems(words)
    }
    #[inline(always)]
    fn to_words(self, out: &mut [u32]) {
        self.to_elems(out);
    }
    #[inline(always)]
    fn from_elems(src: &[u32]) -> Self {
        Self(src[..4].try_into().expect("4-element subslice"))
    }
    #[inline(always)]
    fn to_elems(self, out: &mut [u32]) {
        out[..4].copy_from_slice(&self.0);
    }
    #[inline(always)]
    fn mask(bits: u32) -> Self {
        Self([u32::MAX >> (32 - bits); 4])
    }
    #[inline(always)]
    fn zero() -> Self {
        Self([0; 4])
    }
    #[inline(always)]
    fn and(self, rhs: Self) -> Self {
        elementwise!(self, rhs, &)
    }
    #[inline(always)]
    fn or(self, rhs: Self) -> Self {
        elementwise!(self, rhs, |)
    }
    #[inline(always)]
    fn shl(self, s: u32) -> Self {
        Self(self.0.map(|x| x << s))
    }
    #[inline(always)]
    fn shr(self, s: u32) -> Self {
        Self(self.0.map(|x| x >> s))
    }
}

impl Lane for P64 {
    type E = u64;
    const BITS: u32 = 64;
    const LANES: usize = 2;

    #[inline(always)]
    fn from_words(words: &[u32]) -> Self {
        Self(std::array::from_fn(|i| {
            u64::from(words[2 * i]) | (u64::from(words[2 * i + 1]) << 32)
        }))
    }
    #[inline(always)]
    fn to_words(self, out: &mut [u32]) {
        for (i, x) in self.0.into_iter().enumerate() {
            out[2 * i] = x as u32;
            out[2 * i + 1] = (x >> 32) as u32;
        }
    }
    #[inline(always)]
    fn from_elems(src: &[u64]) -> Self {
        Self(src[..2].try_into().expect("2-element subslice"))
    }
    #[inline(always)]
    fn to_elems(self, out: &mut [u64]) {
        out[..2].copy_from_slice(&self.0);
    }
    #[inline(always)]
    fn mask(bits: u32) -> Self {
        Self([u64::MAX >> (64 - bits); 2])
    }
    #[inline(always)]
    fn zero() -> Self {
        Self([0; 2])
    }
    #[inline(always)]
    fn and(self, rhs: Self) -> Self {
        elementwise!(self, rhs, &)
    }
    #[inline(always)]
    fn or(self, rhs: Self) -> Self {
        elementwise!(self, rhs, |)
    }
    #[inline(always)]
    fn shl(self, s: u32) -> Self {
        Self(self.0.map(|x| x << s))
    }
    #[inline(always)]
    fn shr(self, s: u32) -> Self {
        Self(self.0.map(|x| x >> s))
    }
}

impl private::PageCodec for InterleavedScalar {
    fn encode_page<const N: usize, T: FastPForInt>(
        codec: &mut FastPFor<N, T, Self>,
        input: &[T],
        this_size: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) {
        encode_page_lanes::<N, T, Self, Portable>(
            codec,
            input,
            this_size,
            input_offset,
            output,
            output_offset,
        );
    }

    fn decode_page<const N: usize, T: FastPForInt>(
        codec: &mut FastPFor<N, T, Self>,
        input: &[u32],
        input_offset: &mut Cursor<u32>,
        output: &mut [T],
        output_offset: &mut Cursor<u32>,
        this_size: u32,
    ) -> FastPForResult<()> {
        decode_page_lanes::<N, T, Self, Portable>(
            codec,
            input,
            input_offset,
            output,
            output_offset,
            this_size,
        )
    }
}

#[cfg(test)]
pub(super) mod tests {
    use rand::rngs::StdRng;
    use rand::{RngExt as _, SeedableRng as _};

    use super::*;

    /// Packs 128 values bit by bit straight from the layout description, independent of the kernels.
    fn model_pack(values: &[u64], elem_bits: usize, bit: usize) -> Vec<u32> {
        let lanes = 128 / elem_bits;
        let mut streams = vec![vec![0u64; bit]; lanes];
        for (i, &v) in values.iter().enumerate() {
            let (lane, row) = (i % lanes, i / lanes);
            for t in 0..bit {
                let pos = row * bit + t;
                streams[lane][pos / elem_bits] |= ((v >> t) & 1) << (pos % elem_bits);
            }
        }
        let mut out = vec![0u32; 4 * bit];
        for w in 0..bit {
            for (lane, stream) in streams.iter().enumerate() {
                if elem_bits == 32 {
                    out[4 * w + lane] = stream[w] as u32;
                } else {
                    out[4 * w + 2 * lane] = stream[w] as u32;
                    out[4 * w + 2 * lane + 1] = (stream[w] >> 32) as u32;
                }
            }
        }
        out
    }

    fn masked(v: u64, bit: usize) -> u64 {
        if bit == 64 {
            v
        } else {
            v & ((1u64 << bit) - 1)
        }
    }

    /// Checks one backend against the model for every width, with unmasked high garbage on pack.
    pub(crate) fn check_backend<B: Backend>() {
        let mut rng = StdRng::seed_from_u64(7);
        for bit in 0..=32usize {
            let values: Vec<u32> = (0..128).map(|_| rng.random()).collect();
            let wide: Vec<u64> = values.iter().map(|&v| masked(u64::from(v), bit)).collect();
            let expected = model_pack(&wide, 32, bit);

            // Offsets are non-zero, and the sentinel proves nothing is written outside the group.
            let mut packed = vec![0xDEAD_BEEF; 3 + 4 * bit + 5];
            u32::pack_group::<B>(&values, 0, &mut packed, 3, bit as u8);
            assert_eq!(&packed[3..3 + 4 * bit], &expected[..], "u32 pack bit={bit}");
            assert!(
                packed[..3]
                    .iter()
                    .chain(&packed[3 + 4 * bit..])
                    .all(|&w| w == 0xDEAD_BEEF)
            );

            let mut back = vec![u32::MAX; 128 + 9];
            u32::unpack_group::<B>(&packed, 3, &mut back, 9, bit as u8);
            let want: Vec<u32> = wide.iter().map(|&v| v as u32).collect();
            assert_eq!(&back[9..], &want[..], "u32 unpack bit={bit}");
            assert!(back[..9].iter().all(|&w| w == u32::MAX));
        }
        for bit in 0..=64usize {
            let values: Vec<u64> = (0..128).map(|_| rng.random()).collect();
            let want: Vec<u64> = values.iter().map(|&v| masked(v, bit)).collect();
            let expected = model_pack(&want, 64, bit);

            let mut packed = vec![0xDEAD_BEEF; 3 + 4 * bit + 5];
            u64::pack_group::<B>(&values, 0, &mut packed, 3, bit as u8);
            assert_eq!(&packed[3..3 + 4 * bit], &expected[..], "u64 pack bit={bit}");
            assert!(
                packed[..3]
                    .iter()
                    .chain(&packed[3 + 4 * bit..])
                    .all(|&w| w == 0xDEAD_BEEF)
            );

            let mut back = vec![u64::MAX; 128 + 9];
            u64::unpack_group::<B>(&packed, 3, &mut back, 9, bit as u8);
            assert_eq!(&back[9..], &want[..], "u64 unpack bit={bit}");
            assert!(back[..9].iter().all(|&w| w == u64::MAX));
        }
    }

    #[test]
    fn portable_matches_model() {
        check_backend::<Portable>();
    }
}
