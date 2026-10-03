//! The names introduced in 0.9.2 ahead of 0.10 must mean the same as the names they replace.

#![cfg(feature = "rust")]
#![allow(missing_docs, clippy::unwrap_used)]
#![allow(deprecated, reason = "compares with the deprecated names")]

use std::any::TypeId;

use fastpfor::{
    AnyLenCodec, Auto, FastPFor, FastPFor128, FastPFor256, FastPForBlock, FastPForSequential32x128,
    FastPForSequential32x256, FastPForSequential64x128, FastPForSequential64x256,
    FastPForSequentialBlock32x128, FastPForSequentialBlock32x256, FastPForSequentialBlock64x128,
    FastPForSequentialBlock64x256, FastPForSimd128, FastPForSimd256, FastPForSimdBlock128,
    FastPForSimdBlock256, FastPForSimdBlockWide128, FastPForSimdBlockWide256, FastPForSimdWide128,
    FastPForSimdWide256, FastPForWide128, FastPForWide256, Portable, Scalar, Sequential, Simd,
};

fn same<A: 'static, B: 'static>() -> bool {
    TypeId::of::<A>() == TypeId::of::<B>()
}

/// The new names are aliases: the same types as before.
#[test]
fn new_names_are_the_same_types() {
    assert!(same::<Portable, Scalar>());
    assert!(same::<Auto, Simd>());
    assert!(same::<FastPForSequential32x128, FastPForSimd128>());
    assert!(same::<FastPForSequential32x256, FastPForSimd256>());
    assert!(same::<FastPForSequential64x128, FastPForSimdWide128>());
    assert!(same::<FastPForSequential64x256, FastPForSimdWide256>());
    assert!(same::<FastPForSequentialBlock32x128, FastPForSimdBlock128>());
    assert!(same::<FastPForSequentialBlock32x256, FastPForSimdBlock256>());
    assert!(same::<
        FastPForSequentialBlock64x128,
        FastPForSimdBlockWide128,
    >());
    assert!(same::<
        FastPForSequentialBlock64x256,
        FastPForSimdBlockWide256,
    >());
    assert!(same::<
        FastPForBlock<Sequential, u32, 128>,
        FastPFor<128, u32, Auto>,
    >());
    assert!(same::<
        FastPForBlock<Sequential, u64, 256, Portable>,
        FastPFor<256, u64, Scalar>,
    >());
}

fn encode<C: AnyLenCodec + Default>(data: &[C::Elem]) -> Vec<u32> {
    let mut out = Vec::new();
    C::default().encode(data, &mut out).unwrap();
    out
}

/// The new names default to the SIMD kernels, unlike `FastPFor128` and friends, but write the same bytes.
#[test]
fn new_names_write_the_same_bytes() {
    let data: Vec<u32> = (0..1000u32)
        .map(|i| i.wrapping_mul(2_654_435_761) >> (i % 20))
        .collect();
    assert_eq!(
        encode::<FastPForSequential32x128>(&data),
        encode::<FastPFor128>(&data)
    );
    assert_eq!(
        encode::<FastPForSequential32x256>(&data),
        encode::<FastPFor256>(&data)
    );
    let wide: Vec<u64> = data
        .iter()
        .map(|&v| (u64::from(v) << 24) | u64::from(v))
        .collect();
    assert_eq!(
        encode::<FastPForSequential64x128>(&wide),
        encode::<FastPForWide128>(&wide)
    );
    assert_eq!(
        encode::<FastPForSequential64x256>(&wide),
        encode::<FastPForWide256>(&wide)
    );
}
