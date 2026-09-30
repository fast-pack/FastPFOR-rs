use bytemuck::cast;
use wide::{i8x16, u32x4};

use crate::rust::integer_compression::fastpfor_int::FastPForInt;

const ZERO: i8 = -1;

pub trait NeonInt: Sized {
    fn pack_neon(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8);
    fn unpack_neon(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8);
}

impl NeonInt for u32 {
    #[inline]
    fn pack_neon(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
        pack32(src, inpos, out, outpos, bit);
    }
    #[inline]
    fn unpack_neon(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8) {
        unpack32(src, inpos, out, outpos, bit);
    }
}

impl NeonInt for u64 {
    #[inline]
    fn pack_neon(src: &[Self], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
        if bit <= 32 {
            let narrow: [u32; 32] = std::array::from_fn(|i| src[inpos + i] as u32);
            pack32(&narrow, 0, out, outpos, bit);
        } else {
            <Self as FastPForInt>::fast_pack(src, inpos, out, outpos, bit);
        }
    }
    #[inline]
    fn unpack_neon(src: &[u32], inpos: usize, out: &mut [Self], outpos: usize, bit: u8) {
        if bit <= 32 {
            let mut narrow = [0u32; 32];
            unpack32(src, inpos, &mut narrow, 0, bit);
            for (o, v) in out[outpos..outpos + 32].iter_mut().zip(narrow) {
                *o = Self::from(v);
            }
        } else {
            <Self as FastPForInt>::fast_unpack(src, inpos, out, outpos, bit);
        }
    }
}

const fn select_word(idx: &mut [i8; 16], lane: usize, w: u32) {
    let mut b = 0;
    while b < 4 {
        idx[4 * lane + b] = (4 * w) as i8 + b as i8;
        b += 1;
    }
}

#[inline]
fn swizzle(v: u32x4, idx: [i8; 16]) -> u32x4 {
    cast(cast::<u32x4, i8x16>(v).swizzle(cast(idx)))
}

#[inline]
fn load4(src: &[u32], at: usize) -> u32x4 {
    u32x4::new(src[at..at + 4].try_into().expect("4-word subslice"))
}

struct Unpack32Layout {
    word_off: [usize; 8],
    lo: [[i8; 16]; 8],
    hi: [[i8; 16]; 8],
    shift_lo: [[u32; 4]; 8],
    shift_hi: [[u32; 4]; 8],
}

impl Unpack32Layout {
    const fn new(bit: u32) -> Self {
        let mut layout = Self {
            word_off: [0; 8],
            lo: [[ZERO; 16]; 8],
            hi: [[ZERO; 16]; 8],
            shift_lo: [[0; 4]; 8],
            shift_hi: [[0; 4]; 8],
        };
        let mut c = 0;
        while c < 8 {
            let start = 4 * c as u32 * bit;
            let word0 = start / 32;
            layout.word_off[c] = word0 as usize;
            let mut l = 0;
            while l < 4 {
                let offset = start + l as u32 * bit - word0 * 32;
                let (w, s) = (offset / 32, offset % 32);
                select_word(&mut layout.lo[c], l, w);
                layout.shift_lo[c][l] = s;
                if s + bit > 32 {
                    select_word(&mut layout.hi[c], l, w);
                    layout.shift_hi[c][l] = 32 - s;
                }
                l += 1;
            }
            c += 1;
        }
        layout
    }

    const fn words_read(&self) -> usize {
        self.word_off[7] + 5
    }
}

#[inline]
fn unpack32_bits<const B: u32>(src: &[u32], out: &mut [u32; 32]) {
    let layout = const { &Unpack32Layout::new(B) };
    let src = &src[..layout.words_read()];
    let mask = u32x4::splat((1u32 << B) - 1);
    for c in 0..8 {
        let w = layout.word_off[c];
        let lo = swizzle(load4(src, w), layout.lo[c]) >> u32x4::new(layout.shift_lo[c]);
        let hi = swizzle(load4(src, w + 1), layout.hi[c]) << u32x4::new(layout.shift_hi[c]);
        out[4 * c..4 * c + 4].copy_from_slice(&((lo | hi) & mask).to_array());
    }
}

#[inline]
fn unpack32_group<const B: u32>(src: &[u32], inpos: usize, out: &mut [u32; 32]) {
    let needed = const { Unpack32Layout::new(B).words_read() };
    if let Some(window) = src.get(inpos..inpos + needed) {
        unpack32_bits::<B>(window, out);
    } else {
        let mut padded = [0u32; 32 + 5];
        padded[..B as usize].copy_from_slice(&src[inpos..inpos + B as usize]);
        unpack32_bits::<B>(&padded, out);
    }
}

#[inline]
pub(crate) fn unpack32(src: &[u32], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
    let out: &mut [u32; 32] = (&mut out[outpos..outpos + 32])
        .try_into()
        .expect("32-value subslice");
    macro_rules! dispatch {
        ($($b:literal)*) => {
            match bit {
                0 => out.fill(0),
                $($b => unpack32_group::<$b>(src, inpos, out),)*
                32 => out.copy_from_slice(&src[inpos..inpos + 32]),
                _ => panic!("Unsupported bit width"),
            }
        };
    }
    dispatch!(1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31);
}

#[derive(Clone, Copy)]
struct Pack32Term {
    from0: [i8; 16],
    from1: [i8; 16],
    shift: [u32; 4],
}

struct Pack32Layout {
    rounds: usize,
    starts: [[Pack32Term; 2]; 8],
    straddle: [Pack32Term; 2],
}

impl Pack32Layout {
    const fn new(bit: u32) -> Self {
        let empty = Pack32Term {
            from0: [ZERO; 16],
            from1: [ZERO; 16],
            shift: [0; 4],
        };
        let mut layout = Self {
            rounds: 0,
            starts: [[empty; 2]; 8],
            straddle: [empty; 2],
        };
        let mut starters = [0usize; 8];
        let mut l = 0;
        while l < 8 {
            let offset = l as u32 * bit;
            let (q, s) = ((offset / 32) as usize, offset % 32);
            let r = starters[q];
            starters[q] += 1;
            if r + 1 > layout.rounds {
                layout.rounds = r + 1;
            }
            Self::set(&mut layout.starts[r][q / 4], q % 4, l, s);
            if s + bit > 32 {
                let q = q + 1;
                Self::set(&mut layout.straddle[q / 4], q % 4, l, 32 - s);
            }
            l += 1;
        }
        layout
    }

    const fn set(term: &mut Pack32Term, lane: usize, value: usize, shift: u32) {
        let from = if value < 4 {
            &mut term.from0
        } else {
            &mut term.from1
        };
        select_word(from, lane, (value % 4) as u32);
        term.shift[lane] = shift;
    }
}

#[inline]
fn select32(v0: u32x4, v1: u32x4, term: &Pack32Term) -> u32x4 {
    swizzle(v0, term.from0) | swizzle(v1, term.from1)
}

#[inline]
fn pack32_chunk<const B: u32, const J: usize>(src: &[u32; 32], out: &mut [u8]) {
    let layout = const { &Pack32Layout::new(B) };
    let regs = const { if B > 16 { 2 } else { 1 } };
    let mask = u32x4::splat((1u32 << B) - 1);
    let v0 = load4(src, 8 * J) & mask;
    let v1 = load4(src, 8 * J + 4) & mask;
    let mut acc = [u32x4::ZERO; 2];
    for (h, acc) in acc.iter_mut().enumerate().take(regs) {
        for starts in &layout.starts[..layout.rounds] {
            let term = &starts[h];
            *acc |= select32(v0, v1, term) << u32x4::new(term.shift);
        }
        let term = &layout.straddle[h];
        *acc |= select32(v0, v1, term) >> u32x4::new(term.shift);
    }
    let bytes: [u8; 32] = cast([acc[0].to_array(), acc[1].to_array()]);
    let off = const { J * B as usize };
    let len = const {
        let remaining = (4 - J) * B as usize;
        if remaining < 32 { remaining } else { 32 }
    };
    out[off..off + len].copy_from_slice(&bytes[..len]);
}

#[inline]
fn pack32_group<const B: u32>(src: &[u32], inpos: usize, out: &mut [u32]) {
    let src: &[u32; 32] = src[inpos..inpos + 32]
        .try_into()
        .expect("32-value subslice");
    let out: &mut [u8] = bytemuck::cast_slice_mut(&mut out[..B as usize]);
    pack32_chunk::<B, 0>(src, out);
    pack32_chunk::<B, 1>(src, out);
    pack32_chunk::<B, 2>(src, out);
    pack32_chunk::<B, 3>(src, out);
}

#[inline]
pub(crate) fn pack32(src: &[u32], inpos: usize, out: &mut [u32], outpos: usize, bit: u8) {
    let out = &mut out[outpos..];
    macro_rules! dispatch {
        ($($b:literal)*) => {
            match bit {
                0 => (),
                $($b => pack32_group::<$b>(src, inpos, out),)*
                32 => out[..32].copy_from_slice(&src[inpos..inpos + 32]),
                _ => panic!("Unsupported bit width"),
            }
        };
    }
    dispatch!(1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31);
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::{RngExt as _, SeedableRng as _};

    use super::*;
    use crate::rust::integer_compression::{bit_pack32, bit_pack64, bit_unpack32};

    const SENTINEL: u32 = 0xDEAD_BEEF;

    fn check_pack<T: Copy>(
        width: u8,
        random: impl Fn(&mut StdRng) -> T,
        scalar: fn(&[T], usize, &mut [u32], usize, u8),
        simd: fn(&[T], usize, &mut [u32], usize, u8),
    ) {
        let mut rng = StdRng::seed_from_u64(1);
        for bit in 0..=width {
            for (inpos, outpos) in [(0, 0), (3, 5), (32, 1)] {
                let src: Vec<T> = (0..inpos + 32).map(|_| random(&mut rng)).collect();
                let len = outpos + usize::from(bit) + 4;
                let mut expected = vec![SENTINEL; len];
                scalar(&src, inpos, &mut expected, outpos, bit);
                let mut actual = vec![SENTINEL; len];
                simd(&src, inpos, &mut actual, outpos, bit);
                assert_eq!(actual, expected, "bit={bit} inpos={inpos} outpos={outpos}");
            }
        }
    }

    fn check_unpack<T: Copy + Default + PartialEq + std::fmt::Debug>(
        width: u8,
        scalar: fn(&[u32], usize, &mut [T], usize, u8),
        simd: fn(&[u32], usize, &mut [T], usize, u8),
    ) {
        let mut rng = StdRng::seed_from_u64(2);
        for bit in 0..=width {
            for (inpos, extra, outpos) in [(0, 0, 0), (2, 1, 3), (5, 9, 32), (1, 80, 0)] {
                let src: Vec<u32> = (0..inpos + usize::from(bit) + extra)
                    .map(|_| rng.random())
                    .collect();
                let mut expected = vec![T::default(); outpos + 36];
                scalar(&src, inpos, &mut expected, outpos, bit);
                let mut actual = vec![T::default(); outpos + 36];
                simd(&src, inpos, &mut actual, outpos, bit);
                assert_eq!(actual, expected, "bit={bit} inpos={inpos} extra={extra}");
            }
        }
    }

    #[test]
    fn pack32_matches_scalar() {
        check_pack(
            32,
            rand::RngExt::random::<u32>,
            bit_pack32::fast_pack,
            pack32,
        );
    }

    #[test]
    fn unpack32_matches_scalar() {
        check_unpack(32, bit_unpack32::fast_unpack, unpack32);
    }

    #[test]
    fn pack64_matches_scalar() {
        check_pack(
            64,
            |r| r.random::<u64>(),
            bit_pack64::pack_wide,
            u64::pack_neon,
        );
    }

    #[test]
    fn unpack64_matches_scalar() {
        check_unpack(64, bit_pack64::unpack_wide, u64::unpack_neon);
    }
}
