use std::arch::x86_64::{
    __m256i, _mm256_and_si256, _mm256_blendv_epi8, _mm256_extract_epi64, _mm256_or_si256,
    _mm256_permutevar8x32_epi32, _mm256_set1_epi32, _mm256_set1_epi64x, _mm256_setzero_si256,
    _mm256_sllv_epi32, _mm256_sllv_epi64, _mm256_srlv_epi32, _mm256_srlv_epi64,
};
use std::io::Cursor;

use bytemuck::cast;

use crate::FastPForResult;
use crate::rust::integer_compression::fastpfor::FastPFor;
use crate::rust::integer_compression::fastpfor_int::FastPForInt;
use crate::rust::kernels::Simd;

#[derive(Clone, Copy)]
pub struct Avx2(());

impl Avx2 {
    #[inline]
    pub fn detect() -> Option<Self> {
        #[cfg(feature = "__testing")]
        if super::fallback_forced() {
            return None;
        }
        std::is_x86_feature_detected!("avx2").then_some(Self(()))
    }
}

pub trait Avx2Int: Sized {
    fn encode_page_avx2<const N: usize>(
        token: Avx2,
        codec: &mut FastPFor<N, Self, Simd>,
        input: &[Self],
        this_size: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) where
        Self: FastPForInt;

    fn decode_page_avx2<const N: usize>(
        token: Avx2,
        codec: &mut FastPFor<N, Self, Simd>,
        input: &[u32],
        input_offset: &mut Cursor<u32>,
        output: &mut [Self],
        output_offset: &mut Cursor<u32>,
        this_size: u32,
    ) -> FastPForResult<()>
    where
        Self: FastPForInt;
}

impl Avx2Int for u32 {
    #[inline]
    fn encode_page_avx2<const N: usize>(
        _token: Avx2,
        codec: &mut FastPFor<N, Self, Simd>,
        input: &[Self],
        this_size: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) {
        #[expect(unsafe_code, reason = "calls the #[target_feature] AVX2 page kernel")]
        // SAFETY: the `Avx2` token proves the CPU supports AVX2.
        unsafe {
            encode_page_u32(codec, input, this_size, input_offset, output, output_offset);
        }
    }

    #[inline]
    fn decode_page_avx2<const N: usize>(
        _token: Avx2,
        codec: &mut FastPFor<N, Self, Simd>,
        input: &[u32],
        input_offset: &mut Cursor<u32>,
        output: &mut [Self],
        output_offset: &mut Cursor<u32>,
        this_size: u32,
    ) -> FastPForResult<()> {
        #[expect(unsafe_code, reason = "calls the #[target_feature] AVX2 page kernel")]
        // SAFETY: the `Avx2` token proves the CPU supports AVX2.
        unsafe {
            decode_page_u32(codec, input, input_offset, output, output_offset, this_size)
        }
    }
}

impl Avx2Int for u64 {
    #[inline]
    fn encode_page_avx2<const N: usize>(
        _token: Avx2,
        codec: &mut FastPFor<N, Self, Simd>,
        input: &[Self],
        this_size: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) {
        #[expect(unsafe_code, reason = "calls the #[target_feature] AVX2 page kernel")]
        // SAFETY: the `Avx2` token proves the CPU supports AVX2.
        unsafe {
            encode_page_u64(codec, input, this_size, input_offset, output, output_offset);
        }
    }

    #[inline]
    fn decode_page_avx2<const N: usize>(
        _token: Avx2,
        codec: &mut FastPFor<N, Self, Simd>,
        input: &[u32],
        input_offset: &mut Cursor<u32>,
        output: &mut [Self],
        output_offset: &mut Cursor<u32>,
        this_size: u32,
    ) -> FastPForResult<()> {
        #[expect(unsafe_code, reason = "calls the #[target_feature] AVX2 page kernel")]
        // SAFETY: the `Avx2` token proves the CPU supports AVX2.
        unsafe {
            decode_page_u64(codec, input, input_offset, output, output_offset, this_size)
        }
    }
}

macro_rules! layout {
    (let $name:ident = $layout:expr) => {
        #[cfg(not(feature = "__testing"))]
        let $name = const { &$layout };
        #[cfg(feature = "__testing")]
        let $name = &$layout;
    };
}

macro_rules! dispatch_width {
    ($bit:expr; $($b:literal)*; |$w:ident| $arm:expr; $($pat:pat => $e:expr,)*) => {
        match $bit {
            $($pat => $e,)*
            $($b => {
                const $w: u32 = $b;
                $arm
            })*
            _ => panic!("Unsupported bit width"),
        }
    };
}

macro_rules! unpack32_body {
    ($src:ident, $inpos:ident, $out:ident, $outpos:ident, $bit:ident) => {{
        let out: &mut [u32; 32] = (&mut $out[$outpos..$outpos + 32])
            .try_into()
            .expect("32-value subslice");
        dispatch_width!($bit; 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31; |W| unpack32_group::<W>($src, $inpos, out);
            0 => out.fill(0),
            32 => out.copy_from_slice(&$src[$inpos..$inpos + 32]),
        )
    }};
}

macro_rules! pack32_body {
    ($src:ident, $inpos:ident, $out:ident, $outpos:ident, $bit:ident) => {{
        let out = &mut $out[$outpos..];
        dispatch_width!($bit; 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31; |W| pack32_group::<W>($src, $inpos, out);
            0 => (),
            32 => out[..32].copy_from_slice(&$src[$inpos..$inpos + 32]),
        )
    }};
}

macro_rules! unpack64_body {
    ($src:ident, $inpos:ident, $out:ident, $outpos:ident, $bit:ident) => {{
        let out: &mut [u64; 32] = (&mut $out[$outpos..$outpos + 32])
            .try_into()
            .expect("32-value subslice");
        if $bit <= 32 {
            let mut narrow = [0u32; 32];
            let narrow_out = &mut narrow[..];
            let zero = 0;
            unpack32_body!($src, $inpos, narrow_out, zero, $bit);
            for (o, v) in out.iter_mut().zip(narrow) {
                *o = u64::from(v);
            }
        } else {
            dispatch_width!($bit; 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63; |W| unpack64_group::<W>($src, $inpos, out);
                64 => {
                    for (o, pair) in out.iter_mut().zip($src[$inpos..$inpos + 64].chunks_exact(2)) {
                        *o = u64::from(pair[0]) | (u64::from(pair[1]) << 32);
                    }
                },
            )
        }
    }};
}

macro_rules! pack64_body {
    ($src:ident, $inpos:ident, $out:ident, $outpos:ident, $bit:ident) => {{
        if $bit <= 32 {
            let narrow: [u32; 32] = std::array::from_fn(|i| $src[$inpos + i] as u32);
            let narrow_src = &narrow[..];
            let zero = 0;
            pack32_body!(narrow_src, zero, $out, $outpos, $bit);
        } else {
            let out = &mut $out[$outpos..];
            dispatch_width!($bit; 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63; |W| pack64_group::<W>($src, $inpos, out);
                64 => out[..64].copy_from_slice(bytemuck::cast_slice(&$src[$inpos..$inpos + 32])),
            )
        }
    }};
}

#[target_feature(enable = "avx2")]
fn encode_page_u32<const N: usize>(
    codec: &mut FastPFor<N, u32, Simd>,
    input: &[u32],
    this_size: u32,
    input_offset: &mut Cursor<u32>,
    output: &mut [u32],
    output_offset: &mut Cursor<u32>,
) {
    codec.encode_page_with(
        input,
        this_size,
        input_offset,
        output,
        output_offset,
        #[inline(always)]
        |src, inpos, out, outpos, bit| pack32_body!(src, inpos, out, outpos, bit),
    );
}

#[target_feature(enable = "avx2")]
fn decode_page_u32<const N: usize>(
    codec: &mut FastPFor<N, u32, Simd>,
    input: &[u32],
    input_offset: &mut Cursor<u32>,
    output: &mut [u32],
    output_offset: &mut Cursor<u32>,
    this_size: u32,
) -> FastPForResult<()> {
    codec.decode_page_with(
        input,
        input_offset,
        output,
        output_offset,
        this_size,
        #[inline(always)]
        |src, inpos, out, outpos, bit| unpack32_body!(src, inpos, out, outpos, bit),
    )
}

#[target_feature(enable = "avx2")]
fn encode_page_u64<const N: usize>(
    codec: &mut FastPFor<N, u64, Simd>,
    input: &[u64],
    this_size: u32,
    input_offset: &mut Cursor<u32>,
    output: &mut [u32],
    output_offset: &mut Cursor<u32>,
) {
    codec.encode_page_with(
        input,
        this_size,
        input_offset,
        output,
        output_offset,
        #[inline(always)]
        |src, inpos, out, outpos, bit| pack64_body!(src, inpos, out, outpos, bit),
    );
}

#[target_feature(enable = "avx2")]
fn decode_page_u64<const N: usize>(
    codec: &mut FastPFor<N, u64, Simd>,
    input: &[u32],
    input_offset: &mut Cursor<u32>,
    output: &mut [u64],
    output_offset: &mut Cursor<u32>,
    this_size: u32,
) -> FastPForResult<()> {
    codec.decode_page_with(
        input,
        input_offset,
        output,
        output_offset,
        this_size,
        #[inline(always)]
        |src, inpos, out, outpos, bit| unpack64_body!(src, inpos, out, outpos, bit),
    )
}

#[target_feature(enable = "avx2")]
#[inline]
fn store_partial(acc: __m256i, out: &mut [u8]) {
    let words = [
        _mm256_extract_epi64::<0>(acc) as u64,
        _mm256_extract_epi64::<1>(acc) as u64,
        _mm256_extract_epi64::<2>(acc) as u64,
        _mm256_extract_epi64::<3>(acc) as u64,
    ];
    let full = out.len() / 8;
    for (chunk, word) in out.chunks_exact_mut(8).zip(words) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
    let rest = out.len() % 8;
    if rest > 0 {
        out[8 * full..].copy_from_slice(&words[full].to_le_bytes()[..rest]);
    }
}

#[target_feature(enable = "avx2")]
#[inline]
fn load8(src: &[u32], at: usize) -> __m256i {
    cast::<[u32; 8], __m256i>(src[at..at + 8].try_into().expect("8-word subslice"))
}

struct Unpack32Layout {
    word_off: [usize; 4],
    perm: [[u32; 8]; 4],
    shift_lo: [[u32; 8]; 4],
    shift_hi: [[u32; 8]; 4],
}

impl Unpack32Layout {
    const fn new(bit: u32) -> Self {
        let mut t = Self {
            word_off: [0; 4],
            perm: [[0; 8]; 4],
            shift_lo: [[0; 8]; 4],
            shift_hi: [[0; 8]; 4],
        };
        let mut j = 0;
        while j < 4 {
            let start = 8 * j as u32 * bit;
            let word0 = start / 32;
            t.word_off[j] = word0 as usize;
            let mut l = 0;
            while l < 8 {
                let offset = start + l as u32 * bit - word0 * 32;
                t.perm[j][l] = offset / 32;
                t.shift_lo[j][l] = offset % 32;
                t.shift_hi[j][l] = 32 - offset % 32;
                l += 1;
            }
            j += 1;
        }
        t
    }

    const fn words_read(&self) -> usize {
        self.word_off[3] + 9
    }
}

#[target_feature(enable = "avx2")]
#[inline]
fn unpack32_bits<const B: u32>(src: &[u32], out: &mut [u32; 32]) {
    layout!(let layout = Unpack32Layout::new(B));
    let src = &src[..layout.words_read()];
    let mask = _mm256_set1_epi32(((1u32 << B) - 1) as i32);
    for j in 0..4 {
        let w = layout.word_off[j];
        let perm: __m256i = cast(layout.perm[j]);
        let lo = _mm256_srlv_epi32(
            _mm256_permutevar8x32_epi32(load8(src, w), perm),
            cast(layout.shift_lo[j]),
        );
        let hi = _mm256_sllv_epi32(
            _mm256_permutevar8x32_epi32(load8(src, w + 1), perm),
            cast(layout.shift_hi[j]),
        );
        let values = _mm256_and_si256(_mm256_or_si256(lo, hi), mask);
        out[8 * j..8 * j + 8].copy_from_slice(&cast::<__m256i, [u32; 8]>(values));
    }
}

#[target_feature(enable = "avx2")]
#[inline]
fn unpack32_group<const B: u32>(src: &[u32], inpos: usize, out: &mut [u32; 32]) {
    layout!(let layout = Unpack32Layout::new(B));
    let needed = layout.words_read();
    if let Some(window) = src.get(inpos..inpos + needed) {
        unpack32_bits::<B>(window, out);
    } else {
        let mut padded = [0u32; 32 + 9];
        padded[..B as usize].copy_from_slice(&src[inpos..inpos + B as usize]);
        unpack32_bits::<B>(&padded, out);
    }
}

struct Pack32Layout {
    rounds: usize,
    idx: [[u32; 8]; 8],
    shl: [[u32; 8]; 8],
    hidx: [u32; 8],
    shr: [u32; 8],
}

impl Pack32Layout {
    const fn new(bit: u32) -> Self {
        let mut t = Self {
            rounds: 0,
            idx: [[0; 8]; 8],
            shl: [[32; 8]; 8],
            hidx: [0; 8],
            shr: [32; 8],
        };
        let mut starters = [0usize; 8];
        let mut l = 0;
        while l < 8 {
            let offset = l as u32 * bit;
            let w = (offset / 32) as usize;
            let r = starters[w];
            t.idx[r][w] = l as u32;
            t.shl[r][w] = offset % 32;
            starters[w] += 1;
            if r + 1 > t.rounds {
                t.rounds = r + 1;
            }
            if offset % 32 + bit > 32 {
                t.hidx[w + 1] = l as u32;
                t.shr[w + 1] = 32 - offset % 32;
            }
            l += 1;
        }
        t
    }
}

#[target_feature(enable = "avx2")]
#[inline]
fn pack32_chunk<const B: u32, const J: usize>(src: &[u32; 32], out: &mut [u8]) {
    layout!(let layout = Pack32Layout::new(B));
    let mask = _mm256_set1_epi32(((1u32 << B) - 1) as i32);
    let v = _mm256_and_si256(load8(src, 8 * J), mask);
    let mut acc = _mm256_srlv_epi32(
        _mm256_permutevar8x32_epi32(v, cast(layout.hidx)),
        cast(layout.shr),
    );
    for r in 0..layout.rounds {
        acc = _mm256_or_si256(
            acc,
            _mm256_sllv_epi32(
                _mm256_permutevar8x32_epi32(v, cast(layout.idx[r])),
                cast(layout.shl[r]),
            ),
        );
    }
    let off = const { J * B as usize };
    let len = const {
        let remaining = (4 - J) * B as usize;
        if remaining < 32 { remaining } else { 32 }
    };
    if len == 32 {
        out[off..off + 32].copy_from_slice(&cast::<__m256i, [u8; 32]>(acc));
    } else {
        store_partial(acc, &mut out[off..off + len]);
    }
}

#[target_feature(enable = "avx2")]
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

struct Unpack64Layout {
    word_off: [usize; 8],
    perm_lo: [[u32; 8]; 8],
    perm_hi: [[u32; 8]; 8],
    shift_lo: [[u64; 4]; 8],
    shift_hi: [[u64; 4]; 8],
}

impl Unpack64Layout {
    const fn new(bit: u32) -> Self {
        let mut t = Self {
            word_off: [0; 8],
            perm_lo: [[0; 8]; 8],
            perm_hi: [[0; 8]; 8],
            shift_lo: [[0; 4]; 8],
            shift_hi: [[0; 4]; 8],
        };
        let mut c = 0;
        while c < 8 {
            let start = 4 * c as u32 * bit;
            let word0 = start / 32;
            t.word_off[c] = word0 as usize;
            let mut l = 0;
            while l < 4 {
                let offset = start + l as u32 * bit - word0 * 32;
                let w = offset / 32;
                t.perm_lo[c][2 * l] = w;
                t.perm_lo[c][2 * l + 1] = w + 1;
                t.perm_hi[c][2 * l] = w + 1;
                t.perm_hi[c][2 * l + 1] = w + 1;
                t.shift_lo[c][l] = (offset % 32) as u64;
                t.shift_hi[c][l] = 64 - (offset % 32) as u64;
                l += 1;
            }
            c += 1;
        }
        t
    }

    const fn words_read(&self) -> usize {
        self.word_off[7] + 9
    }
}

#[target_feature(enable = "avx2")]
#[inline]
fn unpack64_bits<const B: u32>(src: &[u32], out: &mut [u64; 32]) {
    layout!(let layout = Unpack64Layout::new(B));
    let src = &src[..layout.words_read()];
    let mask = _mm256_set1_epi64x(((1u64 << B) - 1) as i64);
    for c in 0..8 {
        let w = layout.word_off[c];
        let lo = _mm256_srlv_epi64(
            _mm256_permutevar8x32_epi32(load8(src, w), cast(layout.perm_lo[c])),
            cast(layout.shift_lo[c]),
        );
        let hi = _mm256_sllv_epi64(
            _mm256_permutevar8x32_epi32(load8(src, w + 1), cast(layout.perm_hi[c])),
            cast(layout.shift_hi[c]),
        );
        let values = _mm256_and_si256(_mm256_or_si256(lo, hi), mask);
        out[4 * c..4 * c + 4].copy_from_slice(&cast::<__m256i, [u64; 4]>(values));
    }
}

#[target_feature(enable = "avx2")]
#[inline]
fn unpack64_group<const B: u32>(src: &[u32], inpos: usize, out: &mut [u64; 32]) {
    layout!(let layout = Unpack64Layout::new(B));
    let needed = layout.words_read();
    if let Some(window) = src.get(inpos..inpos + needed) {
        unpack64_bits::<B>(window, out);
    } else {
        let mut padded = [0u32; 64 + 9];
        padded[..B as usize].copy_from_slice(&src[inpos..inpos + B as usize]);
        unpack64_bits::<B>(&padded, out);
    }
}

#[derive(Clone, Copy)]
struct Pack64Term {
    perm: [u32; 8],
    from_hi: [u64; 4],
    shift: [u64; 4],
}

struct Pack64Layout {
    starts: [[Pack64Term; 2]; 2],
    straddle: [Pack64Term; 2],
}

impl Pack64Layout {
    const fn new(bit: u32) -> Self {
        let empty = Pack64Term {
            perm: [0; 8],
            from_hi: [0; 4],
            shift: [64; 4],
        };
        let mut t = Self {
            starts: [[empty; 2]; 2],
            straddle: [empty; 2],
        };
        let mut starters = [0usize; 8];
        let mut l = 0;
        while l < 8 {
            let offset = l as u32 * bit;
            let q = (offset / 64) as usize;
            let shift = (offset % 64) as u64;
            let r = starters[q];
            starters[q] += 1;
            Self::set(&mut t.starts[r][q / 4], q % 4, l, shift);
            if offset % 64 + bit > 64 {
                let q = q + 1;
                Self::set(&mut t.straddle[q / 4], q % 4, l, 64 - shift);
            }
            l += 1;
        }
        t
    }

    const fn set(term: &mut Pack64Term, lane: usize, value: usize, shift: u64) {
        term.perm[2 * lane] = 2 * (value % 4) as u32;
        term.perm[2 * lane + 1] = 2 * (value % 4) as u32 + 1;
        term.from_hi[lane] = if value >= 4 { u64::MAX } else { 0 };
        term.shift[lane] = shift;
    }
}

#[target_feature(enable = "avx2")]
#[inline]
fn select64(v0: __m256i, v1: __m256i, term: &Pack64Term) -> __m256i {
    let perm: __m256i = cast(term.perm);
    _mm256_blendv_epi8(
        _mm256_permutevar8x32_epi32(v0, perm),
        _mm256_permutevar8x32_epi32(v1, perm),
        cast(term.from_hi),
    )
}

#[target_feature(enable = "avx2")]
#[inline]
fn pack64_chunk<const B: u32>(src: &[u64], out: &mut [u8]) {
    layout!(let layout = Pack64Layout::new(B));
    let mask = _mm256_set1_epi64x(((1u64 << B) - 1) as i64);
    let src: &[u64; 8] = src[..8].try_into().expect("8-value subslice");
    let v0 = _mm256_and_si256(
        cast::<[u64; 4], __m256i>(src[..4].try_into().expect("4-value subslice")),
        mask,
    );
    let v1 = _mm256_and_si256(
        cast::<[u64; 4], __m256i>(src[4..].try_into().expect("4-value subslice")),
        mask,
    );
    let mut acc = [_mm256_setzero_si256(); 2];
    for (h, acc) in acc.iter_mut().enumerate() {
        for starts in &layout.starts {
            let term = &starts[h];
            *acc = _mm256_or_si256(
                *acc,
                _mm256_sllv_epi64(select64(v0, v1, term), cast(term.shift)),
            );
        }
        let term = &layout.straddle[h];
        *acc = _mm256_or_si256(
            *acc,
            _mm256_srlv_epi64(select64(v0, v1, term), cast(term.shift)),
        );
    }
    let out = &mut out[..B as usize];
    out[..32].copy_from_slice(&cast::<__m256i, [u8; 32]>(acc[0]));
    store_partial(acc[1], &mut out[32..]);
}

#[target_feature(enable = "avx2")]
#[inline]
fn pack64_group<const B: u32>(src: &[u64], inpos: usize, out: &mut [u32]) {
    let src = &src[inpos..inpos + 32];
    let out: &mut [u8] = bytemuck::cast_slice_mut(&mut out[..B as usize]);
    for c in 0..4 {
        pack64_chunk::<B>(&src[8 * c..], &mut out[c * B as usize..]);
    }
}
