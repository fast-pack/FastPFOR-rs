//! Compatibility tests between the SIMD kernels and the scalar implementation.

#![cfg(feature = "rust")]
#![allow(missing_docs, clippy::unwrap_used)]
#![allow(deprecated, reason = "tests the deprecated names")]

#[path = "../src/test_utils.rs"]
mod test_utils;

use std::fmt::Debug;

use fastpfor::{
    AnyLenCodec, BlockCodec, FastPFor128, FastPFor256, FastPForBlock128, FastPForBlock256,
    FastPForBlockWide128, FastPForBlockWide256, FastPForSimd128, FastPForSimd256,
    FastPForSimdBlock128, FastPForSimdBlock256, FastPForSimdBlockWide128, FastPForSimdBlockWide256,
    FastPForSimdWide128, FastPForSimdWide256, FastPForWide128, FastPForWide256, slice_to_blocks,
};
use rand::rngs::StdRng;
use rand::{RngExt as _, SeedableRng as _};
use test_utils::{RNG_SEED, get_test_cases, test_input_sizes};

trait Value: Copy + Debug + PartialEq + Default {
    const BITS: u32;
    fn from_u64(v: u64) -> Self;
}

#[allow(
    clippy::use_self,
    reason = "BITS is a bit count, not the Self element type"
)]
impl Value for u32 {
    const BITS: u32 = 32;
    fn from_u64(v: u64) -> Self {
        v as Self
    }
}

impl Value for u64 {
    const BITS: u32 = 64;
    fn from_u64(v: u64) -> Self {
        v
    }
}

fn random_bits(rng: &mut StdRng, bits: u32) -> u64 {
    match bits {
        0 => 0,
        64 => rng.random(),
        b => rng.random::<u64>() & ((1u64 << b) - 1),
    }
}

fn pfor_like<T: Value>(rng: &mut StdRng, blocks: usize, block: usize, tail: usize) -> Vec<T> {
    let mut data = Vec::with_capacity(blocks * block + tail);
    for _ in 0..blocks {
        let base = rng.random_range(0..=T::BITS);
        let high = rng.random_range(base..=T::BITS);
        let exceptions = rng.random_range(0..=block / 4);
        let start = data.len();
        data.extend((0..block).map(|_| T::from_u64(random_bits(rng, base))));
        for _ in 0..exceptions {
            let pos = start + rng.random_range(0..block);
            data[pos] = T::from_u64(random_bits(rng, high));
        }
    }
    data.extend((0..tail).map(|_| T::from_u64(random_bits(rng, 20))));
    data
}

fn every_width<T: Value>(block: usize) -> Vec<T> {
    let mut rng = StdRng::seed_from_u64(RNG_SEED);
    let mut data = Vec::new();
    for base in 0..=T::BITS {
        for high in base..=T::BITS {
            let start = data.len();
            data.extend((0..block).map(|_| T::from_u64(random_bits(&mut rng, base))));
            for k in 0..=(high - base) as usize % 7 {
                data[start + (k * 37) % block] = T::from_u64(random_bits(&mut rng, high));
            }
        }
    }
    data
}

fn datasets<T: Value>(block: usize) -> Vec<Vec<T>> {
    let mut rng = StdRng::seed_from_u64(RNG_SEED);
    let mut sets = vec![Vec::new(), every_width(block)];
    for (blocks, tail) in [(1, 0), (3, 17), (8, block - 1), (600, 5)] {
        sets.push(pfor_like(&mut rng, blocks, block, tail));
    }
    sets
}

fn encode<C: AnyLenCodec>(codec: &mut C, data: &[C::Elem]) -> Vec<u32> {
    let mut out = Vec::new();
    codec.encode(data, &mut out).unwrap();
    out
}

fn decode<C: AnyLenCodec>(codec: &mut C, data: &[u32]) -> Result<Vec<C::Elem>, String> {
    let mut out = Vec::new();
    codec
        .decode(data, &mut out, None)
        .map(|()| out)
        .map_err(|e| format!("{e:?}"))
}

fn assert_compatible<ScalarCodec, SimdCodec, T>(data: &[T])
where
    T: Value,
    ScalarCodec: AnyLenCodec<Elem = T>,
    SimdCodec: AnyLenCodec<Elem = T>,
{
    let (mut scalar, mut simd) = (ScalarCodec::default(), SimdCodec::default());
    let scalar_enc = encode(&mut scalar, data);
    let simd_enc = encode(&mut simd, data);
    assert_eq!(
        simd_enc,
        scalar_enc,
        "encoded bytes differ, len={}",
        data.len()
    );
    assert_eq!(decode(&mut simd, &scalar_enc).unwrap(), data);
    assert_eq!(decode(&mut scalar, &simd_enc).unwrap(), data);
    assert_eq!(decode(&mut simd, &simd_enc).unwrap(), data);
}

fn assert_block_compatible<ScalarCodec, SimdCodec, T>(data: &[T])
where
    T: Value,
    ScalarCodec: BlockCodec<Elem = T>,
    SimdCodec: BlockCodec<Elem = T, Block = ScalarCodec::Block>,
{
    let (blocks, _) = slice_to_blocks::<ScalarCodec>(data);
    let (mut scalar, mut simd) = (ScalarCodec::default(), SimdCodec::default());
    let mut scalar_enc = Vec::new();
    scalar.encode_blocks(blocks, &mut scalar_enc).unwrap();
    let mut simd_enc = Vec::new();
    simd.encode_blocks(blocks, &mut simd_enc).unwrap();
    assert_eq!(simd_enc, scalar_enc);
    let n = Some(u32::try_from(blocks.len() * ScalarCodec::size()).unwrap());
    let mut simd_dec = Vec::new();
    let simd_used = simd.decode_blocks(&scalar_enc, n, &mut simd_dec).unwrap();
    let mut scalar_dec = Vec::new();
    let scalar_used = scalar.decode_blocks(&simd_enc, n, &mut scalar_dec).unwrap();
    assert_eq!(simd_used, scalar_used);
    assert_eq!(simd_dec, scalar_dec);
    assert_eq!(&simd_dec[..], &data[..simd_dec.len()]);
}

fn assert_corruption_agrees<ScalarCodec, SimdCodec, T>(data: &[T], seed: u64)
where
    T: Value,
    ScalarCodec: AnyLenCodec<Elem = T>,
    SimdCodec: AnyLenCodec<Elem = T>,
{
    let mut rng = StdRng::seed_from_u64(seed);
    let clean = encode(&mut ScalarCodec::default(), data);
    if clean.is_empty() {
        return;
    }
    for _ in 0..200 {
        let mut bad = clean.clone();
        for _ in 0..rng.random_range(1..=3) {
            let i = rng.random_range(0..bad.len());
            match rng.random_range(0..3) {
                0 => bad[i] ^= 1 << rng.random_range(0..32),
                1 => bad[i] = rng.random(),
                _ => bad.truncate(i),
            }
            if bad.is_empty() {
                break;
            }
        }
        assert_eq!(
            decode(&mut SimdCodec::default(), &bad),
            decode(&mut ScalarCodec::default(), &bad)
        );
    }
}

fn check_all<ScalarCodec, SimdCodec, ScalarBlocks, SimdBlocks, T>(block: usize)
where
    T: Value,
    ScalarCodec: AnyLenCodec<Elem = T>,
    SimdCodec: AnyLenCodec<Elem = T>,
    ScalarBlocks: BlockCodec<Elem = T>,
    SimdBlocks: BlockCodec<Elem = T, Block = ScalarBlocks::Block>,
{
    for (i, data) in datasets::<T>(block).iter().enumerate() {
        assert_compatible::<ScalarCodec, SimdCodec, T>(data);
        assert_block_compatible::<ScalarBlocks, SimdBlocks, T>(data);
        if data.len() < 20_000 {
            assert_corruption_agrees::<ScalarCodec, SimdCodec, T>(data, i as u64);
        }
    }
}

#[test]
fn simd128_matches_scalar() {
    check_all::<FastPFor128, FastPForSimd128, FastPForBlock128, FastPForSimdBlock128, u32>(128);
}

#[test]
fn simd256_matches_scalar() {
    check_all::<FastPFor256, FastPForSimd256, FastPForBlock256, FastPForSimdBlock256, u32>(256);
}

#[test]
fn simd_wide128_matches_scalar() {
    check_all::<
        FastPForWide128,
        FastPForSimdWide128,
        FastPForBlockWide128,
        FastPForSimdBlockWide128,
        u64,
    >(128);
}

#[test]
fn simd_wide256_matches_scalar() {
    check_all::<
        FastPForWide256,
        FastPForSimdWide256,
        FastPForBlockWide256,
        FastPForSimdBlockWide256,
        u64,
    >(256);
}

#[test]
fn simd_matches_scalar_on_shared_cases() {
    for n in test_input_sizes() {
        for data in get_test_cases(n + 37) {
            assert_compatible::<FastPFor128, FastPForSimd128, u32>(&data);
            assert_compatible::<FastPFor256, FastPForSimd256, u32>(&data);
        }
    }
}

#[test]
fn simd_reuses_state_like_scalar() {
    let mut rng = StdRng::seed_from_u64(RNG_SEED);
    let (mut scalar, mut simd) = (FastPFor128::default(), FastPForSimd128::default());
    for _ in 0..20 {
        let blocks = rng.random_range(0..40);
        let tail = rng.random_range(0..128);
        let data = pfor_like::<u32>(&mut rng, blocks, 128, tail);
        let enc = encode(&mut scalar, &data);
        assert_eq!(encode(&mut simd, &data), enc);
        assert_eq!(decode(&mut simd, &enc).unwrap(), data);
    }
}

#[cfg(feature = "__testing")]
#[test]
fn simd_fallback_matches_scalar() {
    fastpfor::__testing::with_simd_fallback(|| {
        check_all::<FastPFor128, FastPForSimd128, FastPForBlock128, FastPForSimdBlock128, u32>(128);
        check_all::<
            FastPForWide256,
            FastPForSimdWide256,
            FastPForBlockWide256,
            FastPForSimdBlockWide256,
            u64,
        >(256);
    });
}
