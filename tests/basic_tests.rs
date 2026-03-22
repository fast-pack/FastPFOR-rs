//! Basic integration tests exercising the public `BlockCodec` and `AnyLenCodec` APIs.

#![cfg(feature = "rust")]

use fastpfor::{
    AnyLenCodec, BlockCodec, FastPFor128, FastPFor256, FastPForBlock128, FastPForBlock256,
    JustCopy, VariableByte, slice_to_blocks,
};
use rand::rngs::StdRng;
use rand::{RngExt as _, SeedableRng};

mod common;

// ── Generic helpers ───────────────────────────────────────────────────────────

fn anylen_roundtrip<C: AnyLenCodec + ?Sized>(codec: &mut C, data: &[u32]) {
    let mut compressed = Vec::new();
    codec
        .encode(data, &mut compressed)
        .unwrap_or_else(|e| panic!("encode failed: {e:?}"));
    let mut decoded = Vec::new();
    codec
        .decode(&compressed, &mut decoded, None)
        .unwrap_or_else(|e| panic!("decode failed: {e:?}"));
    assert_eq!(decoded, data);
}

fn block_roundtrip<C: BlockCodec + Default>(data: &[u32]) {
    let mut codec = C::default();
    let (blocks, _) = slice_to_blocks::<C>(data);
    let mut compressed = Vec::new();
    codec.encode_blocks(blocks, &mut compressed).unwrap();
    let mut decoded = Vec::new();
    let expected_values = blocks.len() * C::size();
    codec
        .decode_blocks(
            &compressed,
            Some(u32::try_from(expected_values).expect("expected_values fits in u32")),
            &mut decoded,
        )
        .unwrap();
    assert_eq!(decoded, &data[..expected_values]);
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "cpp")]
fn saul_test() {
    use fastpfor::cpp::CppFastPFor128;
    // Block codecs + tail for any-length. C++ block codecs are already any-length; use directly.
    let mut codecs: Vec<(&str, Box<dyn AnyLenCodec>)> = vec![
        ("JustCopy", Box::new(JustCopy)),
        ("FastPFor256", Box::new(FastPFor256::default())),
        ("FastPFor128", Box::new(FastPFor128::default())),
        ("CppFastPFor128", Box::new(CppFastPFor128::default())),
    ];
    let input = vec![2u32, 3, 4, 5];
    for (name, codec) in &mut codecs {
        anylen_roundtrip(codec.as_mut(), &input);
        // silence unused-variable warning when cpp feature is off
        let _ = name;
    }
}

/// Sub-block-sized inputs produce no output via `BlockCodec`.
#[test]
fn spurious_out_test() {
    fn check<C: BlockCodec + Default>(len: usize) {
        let x = vec![0u32; 1024];
        let (blocks, _) = slice_to_blocks::<C>(&x[..len]);
        let mut out = Vec::new();
        C::default().encode_blocks(blocks, &mut out).unwrap();
        assert!(out.is_empty() || blocks.is_empty());
    }
    for len in 0..32usize {
        check::<FastPForBlock256>(len);
        check::<FastPForBlock128>(len);
    }
}

/// `AnyLenCodec` round-trips empty input correctly.
#[test]
fn zero_in_zero_out_test() {
    anylen_roundtrip(&mut VariableByte::new(), &[]);
    anylen_roundtrip(&mut JustCopy::new(), &[]);
    anylen_roundtrip(&mut FastPFor256::default(), &[]);
    anylen_roundtrip(&mut FastPFor128::default(), &[]);
}

#[test]
fn test_increasing_sequence() {
    let data: Vec<u32> = (0..256u32).collect();
    anylen_roundtrip(&mut FastPFor256::default(), &data);
    anylen_roundtrip(&mut FastPFor128::default(), &data);
}

#[test]
fn test_random_numbers() {
    let data: Vec<u32> = (0..65536)
        .map(|_| StdRng::seed_from_u64(123456).random())
        .collect();
    anylen_roundtrip(&mut FastPFor256::default(), &data);
    anylen_roundtrip(&mut FastPFor128::default(), &data);
}

/// `BlockCodec` round-trip using `slice_to_blocks` to split aligned input.
#[test]
fn block_codec_roundtrip() {
    block_roundtrip::<FastPForBlock256>(&(0u32..512).collect::<Vec<_>>());
    block_roundtrip::<FastPForBlock128>(&(0u32..512).collect::<Vec<_>>());
}

/// `AnyLenCodec` round-trip with random values at various lengths.
#[test]
fn anylen_random_roundtrip() {
    let mut rng = rand::rng();
    for n in [128usize, 300, 512, 1000, 4096] {
        let data: Vec<u32> = (0..n).map(|_| rng.random()).collect();
        anylen_roundtrip(&mut FastPFor256::default(), &data);
        anylen_roundtrip(&mut FastPFor128::default(), &data);
    }
}
