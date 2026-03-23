//! Shared data generators, codec helpers, and pre-computed fixtures used by
//! Criterion benchmarks, integration tests, and `#[cfg(test)]` unit tests in the
//! `fastpfor` crate.
//!
//! Loaded as a module via `#[path]` or as a normal child module, so every item
//! consumed from outside must be `pub`. Each consumer uses a different subset,
//! so dead-code is allowed at module scope.

// This is an internal dev-only module; doc-comments on every field would add
// noise without benefit.
#![allow(dead_code, missing_docs)]

#[cfg(feature = "cpp")]
use fastpfor::BlockCodec64;
#[allow(unused_imports)]
use fastpfor::{AnyLenCodec, BlockCodec, FastPForResult, slice_to_blocks};
#[cfg(feature = "rust")]
use fastpfor::{
    FastPFor128, FastPFor256, FastPForBlock128, FastPForBlock256, JustCopy, VariableByte,
};

pub const RNG_SEED: u64 = 456;

// ---------------------------------------------------------------------------
// Generic codec helpers
// ---------------------------------------------------------------------------

pub fn roundtrip<C: AnyLenCodec>(data: &[u32]) {
    roundtrip_expected::<C>(data, Some(data.len().try_into().unwrap()));
}

/// Encode `data` with a caller-owned codec, decode with `expected_len: None`, assert round-trip.
pub fn roundtrip_expected<E: AnyLenCodec>(data: &[u32], expected_len: Option<u32>) {
    roundtrip_full::<E, E>(data, expected_len);
}

/// Encode `data` with a caller-owned codec, decode with `expected_len: None`, assert round-trip.
pub fn roundtrip_full<E: AnyLenCodec, D: AnyLenCodec>(data: &[u32], expected_len: Option<u32>) {
    let mut encoder = E::default();
    let mut compressed = Vec::new();
    encoder.encode(data, &mut compressed).unwrap();

    let mut decoder = D::default();
    let mut decompressed = Vec::new();
    decoder
        .decode(&compressed, &mut decompressed, expected_len)
        .unwrap();
    assert_eq!(decompressed, data);
}

#[cfg(feature = "cpp")]
pub fn roundtrip64<C: BlockCodec64 + Default>(data: &[u64]) {
    let mut codec = C::default();
    let mut compressed = Vec::new();
    codec.encode64(data, &mut compressed).unwrap();
    let mut decoded = Vec::new();
    codec.decode64(&compressed, &mut decoded).unwrap();
    assert_eq!(decoded, data);
}

pub fn block_roundtrip<C: BlockCodec>(data: &[u32]) {
    let compressed = block_compress::<C>(data).unwrap();
    let decompressed = block_decompress::<C>(&compressed, Some(data.len() as u32)).unwrap();
    assert_eq!(decompressed, data);
}

pub fn compress<C: AnyLenCodec>(data: &[u32]) -> FastPForResult<Vec<u32>> {
    let mut compressed = Vec::new();
    C::default().encode(data, &mut compressed)?;
    Ok(compressed)
}

pub fn decompress<C: AnyLenCodec>(
    compressed: &[u32],
    expected_len: Option<u32>,
) -> FastPForResult<Vec<u32>> {
    let mut decompressed = Vec::new();
    C::default().decode(compressed, &mut decompressed, expected_len)?;
    Ok(decompressed)
}

pub fn block_compress<C: BlockCodec>(data: &[u32]) -> FastPForResult<Vec<u32>> {
    let (blocks, remainder) = slice_to_blocks::<C>(data);
    assert_eq!(
        remainder.len(),
        0,
        "data length must be a multiple of block size"
    );
    let mut out = Vec::new();
    C::default().encode_blocks(blocks, &mut out)?;
    Ok(out)
}

pub fn block_decompress<C: BlockCodec>(
    compressed: &[u32],
    expected_len: Option<u32>,
) -> FastPForResult<Vec<u32>> {
    let mut out = Vec::new();
    C::default().decode_blocks(compressed, expected_len, &mut out)?;
    Ok(out)
}

#[cfg(feature = "cpp")]
pub fn compress64<C: BlockCodec64 + Default>(data: &[u64]) -> FastPForResult<Vec<u32>> {
    let mut compressed = Vec::new();
    C::default().encode64(data, &mut compressed)?;
    Ok(compressed)
}

#[cfg(feature = "cpp")]
pub fn decompress64<C: BlockCodec64 + Default>(compressed: &[u32]) -> FastPForResult<Vec<u64>> {
    let mut out = Vec::new();
    C::default().decode64(compressed, &mut out)?;
    Ok(out)
}

/// Run [`roundtrip`] for every pure-Rust any-length codec covered here (and optionally C++).
#[cfg(feature = "rust")]
pub fn roundtrip_all(data: &[u32]) {
    roundtrip::<VariableByte>(data);
    roundtrip::<JustCopy>(data);
    roundtrip::<FastPFor256>(data);
    roundtrip::<FastPFor128>(data);

    #[cfg(feature = "cpp")]
    {
        use fastpfor::cpp::CppFastPFor128;
        roundtrip::<CppFastPFor128>(data);
    }
}

#[cfg(feature = "rust")]
pub fn block_roundtrip_all(data: &[u32]) {
    block_roundtrip::<FastPForBlock256>(data);
    block_roundtrip::<FastPForBlock128>(data);
}

/// Encode/decode round-trip using `CompositeCodec<B, T>` built from `B::default()` and `T::default()`.
///
/// `B` is the block codec; `T` is the any-length tail codec.
#[cfg(feature = "rust")]
pub fn roundtrip_composite<B, T>(data: &[u32])
where
    B: BlockCodec,
    T: AnyLenCodec,
{
    roundtrip::<fastpfor::CompositeCodec<B, T>>(data);
}

// ---------------------------------------------------------------------------
// Compatibility test helpers (used by integration tests)
// ---------------------------------------------------------------------------

/// Returns various input sizes to test codec behavior (multiples of 128).
pub fn test_input_sizes() -> Vec<usize> {
    (1..=8).map(|exp| (1usize << exp) * 128).collect()
}

/// Generates test data vectors of size `n` with various patterns.
pub fn get_test_cases(n: usize) -> Vec<Vec<u32>> {
    use rand::rngs::StdRng;
    use rand::{RngExt as _, SeedableRng as _};
    let mut rng = StdRng::seed_from_u64(RNG_SEED);

    vec![
        // Zeroes
        vec![0u32; n],
        // Same non-zero
        vec![14u32; n],
        // Ascending values
        (0..n).map(|i| i as u32).collect::<Vec<u32>>(),
        // Descending values
        (0..n).rev().map(|i| i as u32).collect::<Vec<u32>>(),
        // Bit-flipping pattern
        (0..n)
            .map(|i| ((i as u32) * 32) ^ ((i as u32) >> 1))
            .collect::<Vec<u32>>(),
        // Alternating large and small values
        (0..n)
            .map(|i| {
                let ui = i as u32;
                if ui % 2 == 0 { 1 << 30 } else { 3 }
            })
            .collect::<Vec<u32>>(),
        // Random u32 values
        (0..n)
            .map(|_| rng.random_range(0..(1 << 31)))
            .collect::<Vec<u32>>(),
        // Spike in the middle
        (0..n)
            .map(|i| if i == n / 2 { u32::MAX } else { 1 })
            .collect::<Vec<u32>>(),
        // An empty vector
        Vec::new(),
    ]
}

// ---------------------------------------------------------------------------
// Data generators + fixtures (Rust block codecs; benchmarks / smoke tests)
// ---------------------------------------------------------------------------

#[cfg(feature = "rust")]
mod rust_bench {
    use core::ops::Range;
    use std::marker::PhantomData;

    use rand::rngs::StdRng;
    use rand::{RngExt as _, SeedableRng};

    use super::{BlockCodec, RNG_SEED, block_compress};

    type DataGeneratorFn = fn(usize) -> Vec<u32>;

    fn generate_uniform_data_from_range(size: usize, value_range: Range<u32>) -> Vec<u32> {
        let mut rng = StdRng::seed_from_u64(RNG_SEED);
        (0..size)
            .map(|_| rng.random_range(value_range.clone()))
            .collect()
    }

    pub fn generate_uniform_data_small_value_distribution(size: usize) -> Vec<u32> {
        generate_uniform_data_from_range(size, 0..1000)
    }

    fn generate_uniform_data_large_value_distribution(size: usize) -> Vec<u32> {
        generate_uniform_data_from_range(size, 0..u32::MAX)
    }

    fn generate_clustered_data(size: usize) -> Vec<u32> {
        let mut rng = StdRng::seed_from_u64(RNG_SEED);
        let mut base = 0u32;
        (0..size)
            .map(|_| {
                if rng.random_bool(0.1) {
                    base = rng.random_range(0..1000);
                }
                base + rng.random_range(0..10)
            })
            .collect()
    }

    fn generate_sequential_data(size: usize) -> Vec<u32> {
        (0..size as u32).collect()
    }

    fn generate_sparse_data(size: usize) -> Vec<u32> {
        let mut rng = StdRng::seed_from_u64(RNG_SEED);
        (0..size)
            .map(|_| {
                if rng.random_bool(0.9) {
                    0
                } else {
                    rng.random()
                }
            })
            .collect()
    }

    fn generate_constant_data(size: usize) -> Vec<u32> {
        vec![RNG_SEED as u32; size]
    }

    fn generate_geometric_data(size: usize) -> Vec<u32> {
        (0..size).map(|i| 1u32 << (i % 30)).collect()
    }

    const COMPRESS_PATTERNS: &[(&str, DataGeneratorFn)] = &[
        (
            "uniform_small_value_distribution",
            generate_uniform_data_small_value_distribution,
        ),
        (
            "uniform_large_value_distribution",
            generate_uniform_data_large_value_distribution,
        ),
        ("clustered", generate_clustered_data),
        ("sequential", generate_sequential_data),
        ("sparse", generate_sparse_data),
    ];

    const ALL_PATTERNS: &[(&str, DataGeneratorFn)] = &[
        (
            "uniform_small_distribution",
            generate_uniform_data_small_value_distribution,
        ),
        (
            "uniform_large_distribution",
            generate_uniform_data_large_value_distribution,
        ),
        ("clustered", generate_clustered_data),
        ("sequential", generate_sequential_data),
        ("sparse", generate_sparse_data),
        ("constant", generate_constant_data),
        ("geometric", generate_geometric_data),
    ];

    /// One row of pre-computed data for compression / decompression benchmarks.
    pub struct CompressFixture<C: BlockCodec> {
        pub name: &'static str,
        /// Block-aligned uncompressed data (exactly `n_blocks * C::elements_per_block()` elements).
        pub original: Vec<u32>,
        /// Pre-compressed form, ready for decompression benchmarks.
        pub compressed: Vec<u32>,
        /// Number of blocks in `data`.
        pub n_blocks: usize,
        _codec: PhantomData<C>,
    }

    /// One row for the block-size comparison benchmark.
    pub struct BlockSizeFixture<C: BlockCodec> {
        pub compressed: Vec<u32>,
        pub original: Vec<u32>,
        pub n_blocks: usize,
        _codec: PhantomData<C>,
    }

    impl<C: BlockCodec> CompressFixture<C> {
        fn new(name: &'static str, generator: DataGeneratorFn, block_count: usize) -> Self {
            let original = generator(block_count * C::size());
            Self {
                name,
                compressed: block_compress::<C>(&original).unwrap(),
                original,
                n_blocks: block_count,
                _codec: PhantomData,
            }
        }
    }

    impl<C: BlockCodec> BlockSizeFixture<C> {
        pub fn new(block_count: usize) -> Self {
            let original = generate_uniform_data_small_value_distribution(block_count * C::size());
            Self {
                compressed: block_compress::<C>(&original).unwrap(),
                original,
                n_blocks: block_count,
                _codec: PhantomData,
            }
        }
    }

    pub fn compress_fixtures<C: BlockCodec>(
        block_counts: &[usize],
    ) -> Vec<(usize, CompressFixture<C>)> {
        block_counts
            .iter()
            .flat_map(|&bc| {
                COMPRESS_PATTERNS.iter().map(move |&(name, generator)| {
                    (bc, CompressFixture::<C>::new(name, generator, bc))
                })
            })
            .collect()
    }

    pub fn ratio_fixtures<C: BlockCodec>(block_count: usize) -> Vec<CompressFixture<C>> {
        ALL_PATTERNS
            .iter()
            .map(|&(name, generator)| CompressFixture::<C>::new(name, generator, block_count))
            .collect()
    }
}

#[cfg(feature = "rust")]
#[allow(unused_imports)]
// Re-exports for benches/integration tests; not every `#[path]` site uses all items.
pub use rust_bench::{
    BlockSizeFixture, CompressFixture, compress_fixtures,
    generate_uniform_data_small_value_distribution, ratio_fixtures,
};
