//! The interleaved codecs against the C++ `SIMDFastPFor`: identical bytes, and each decodes the other's output.

#![cfg(all(feature = "rust", feature = "cpp"))]
#![allow(missing_docs, clippy::unwrap_used)]

#[path = "../src/test_utils.rs"]
mod test_utils;

use fastpfor::cpp::{CppFastPFor128, CppFastPFor256, CppSimdFastPFor128, CppSimdFastPFor256};
use fastpfor::{
    AnyLenCodec, FastPForCodec, FastPForInterleaved32x128, FastPForInterleaved32x256,
    FastPForSequential32x128, FastPForSequential32x256, Interleaved, Portable,
};
use rand::rngs::StdRng;
use rand::{RngExt as _, SeedableRng as _};
use test_utils::{RNG_SEED, get_test_cases, test_input_sizes};

fn random_bits(rng: &mut StdRng, bits: u32) -> u32 {
    match bits {
        0 => 0,
        32 => rng.random(),
        b => rng.random::<u32>() & ((1u32 << b) - 1),
    }
}

/// Blocks of values of one width, a few of them replaced by wider values, plus a tail: the shape
/// PFOR is designed for, hitting every exception bucket and partial exception groups.
fn pfor_like(rng: &mut StdRng, blocks: usize, block: usize, tail: usize) -> Vec<u32> {
    let mut data = Vec::with_capacity(blocks * block + tail);
    for _ in 0..blocks {
        let base = rng.random_range(0..=32);
        let high = rng.random_range(base..=32);
        let start = data.len();
        data.extend((0..block).map(|_| random_bits(rng, base)));
        for _ in 0..rng.random_range(0..=block / 3) {
            data[start + rng.random_range(0..block)] = random_bits(rng, high);
        }
    }
    data.extend((0..tail).map(|_| random_bits(rng, 20)));
    data
}

fn datasets(block: usize) -> Vec<Vec<u32>> {
    let mut rng = StdRng::seed_from_u64(RNG_SEED);
    let mut sets = vec![Vec::new()];
    for n in test_input_sizes() {
        sets.extend(get_test_cases(n + 37));
    }
    // Several pages (the default page is 65536 values), and many small shapes.
    sets.push(pfor_like(&mut rng, 600, block, 5));
    for _ in 0..300 {
        let (blocks, tail) = (rng.random_range(1..12), rng.random_range(0..block));
        sets.push(pfor_like(&mut rng, blocks, block, tail));
    }
    sets
}

fn encode<C: AnyLenCodec<Elem = u32>>(data: &[u32]) -> Vec<u32> {
    let mut out = Vec::new();
    C::default().encode(data, &mut out).unwrap();
    out
}

fn decode<C: AnyLenCodec<Elem = u32>>(words: &[u32]) -> Vec<u32> {
    let mut out = Vec::new();
    C::default().decode(words, &mut out, None).unwrap();
    out
}

/// `R` (Rust, interleaved) must write the bytes of `C` (C++ `SIMDFastPFor`), and decode what `C` writes.
fn check_matches_cpp<R, C, S>(block: usize)
where
    R: AnyLenCodec<Elem = u32>,
    C: AnyLenCodec<Elem = u32>,
    S: AnyLenCodec<Elem = u32>,
{
    for (i, data) in datasets(block).iter().enumerate() {
        let (rust, cpp) = (encode::<R>(data), encode::<C>(data));
        assert_eq!(rust, cpp, "dataset {i} (len {}): bytes differ", data.len());
        assert_eq!(decode::<R>(&cpp), *data, "Rust decoding of C++ output");
        assert_eq!(decode::<C>(&rust), *data, "C++ decoding of Rust output");
        assert_eq!(
            encode::<S>(data),
            cpp,
            "dataset {i}: portable kernels differ"
        );
    }
}

/// The interleaved format with the portable kernels, to check them against C++ as well.
type PortableCodec<const N: usize> = FastPForCodec<Interleaved, u32, N, Portable>;

#[test]
fn interleaved128_matches_cpp_simdfastpfor() {
    check_matches_cpp::<FastPForInterleaved32x128, CppSimdFastPFor128, PortableCodec<128>>(128);
}

#[test]
fn interleaved256_matches_cpp_simdfastpfor() {
    check_matches_cpp::<FastPForInterleaved32x256, CppSimdFastPFor256, PortableCodec<256>>(256);
}

/// The two layouts must stay distinct: if `Interleaved` quietly fell back to the standard layout,
/// the byte comparison above would still pass wherever the C++ codecs happen to agree.
#[test]
fn interleaved_differs_from_standard_layout() {
    let mut rng = StdRng::seed_from_u64(RNG_SEED);
    let data = pfor_like(&mut rng, 20, 128, 0);
    assert_ne!(
        encode::<FastPForInterleaved32x128>(&data),
        encode::<FastPForSequential32x128>(&data)
    );
    assert_ne!(
        encode::<CppSimdFastPFor128>(&data),
        encode::<CppFastPFor128>(&data)
    );
    let data = pfor_like(&mut rng, 20, 256, 0);
    assert_ne!(
        encode::<FastPForInterleaved32x256>(&data),
        encode::<FastPForSequential32x256>(&data)
    );
    assert_ne!(
        encode::<CppSimdFastPFor256>(&data),
        encode::<CppFastPFor256>(&data)
    );
}
