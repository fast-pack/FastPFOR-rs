//! Shared data generators, codec helpers, and pre-computed fixtures used by
//! the Criterion benchmark (`fastpfor_benchmark.rs`), smoke tests
//! (`tests/benchmark_smoke.rs`), and targeted integration tests
//! (`tests/encode_paths.rs`).
//!
//! Loaded as a module via `#[path]`, so every item consumed from outside must
//! be `pub`. Each consumer uses a different subset, so dead-code is allowed
//! at module scope.

// This is an internal dev-only module; doc-comments on every field would add
// noise without benefit.
#![allow(dead_code, missing_docs)]

use core::ops::Range;
use std::marker::PhantomData;

#[allow(unused_imports)]
use fastpfor::{AnyLenCodec, BlockCodec, slice_to_blocks};
use fastpfor::{
    FastPFor128, FastPFor256, FastPForBlock128, FastPForBlock256, JustCopy, VariableByte,
};
use rand::rngs::StdRng;
use rand::{RngExt as _, SeedableRng};

const SEED: u64 = 456;

// ---------------------------------------------------------------------------
// Generic codec helpers
// ---------------------------------------------------------------------------

pub fn roundtrip<C: AnyLenCodec>(data: &[u32]) {
    let compressed = compress::<C>(data);
    let decompressed = decompress::<C>(&compressed, Some(data.len() as u32));
    assert_eq!(decompressed, data);
}

pub fn compress<C: AnyLenCodec>(data: &[u32]) -> Vec<u32> {
    let mut codec = C::default();
    let mut compressed = Vec::new();
    codec.encode(data, &mut compressed).unwrap();
    compressed
}

pub fn decompress<C: AnyLenCodec>(compressed: &Vec<u32>, expected_len: Option<u32>) -> Vec<u32> {
    let mut codec = C::default();
    let mut decompressed = Vec::new();
    codec
        .decode(&compressed, &mut decompressed, expected_len)
        .unwrap();
    decompressed
}

pub fn block_roundtrip<C: BlockCodec>(data: &[u32]) {
    let compressed = block_compress::<C>(data);
    let decompressed = block_decompress::<C>(&compressed, Some(data.len() as u32));
    assert_eq!(decompressed, data);
}

pub fn block_compress<C: BlockCodec>(data: &[u32]) -> Vec<u32> {
    let mut codec = C::default();
    let (blocks, remainder) = slice_to_blocks::<C>(data);
    assert_eq!(
        remainder.len(),
        0,
        "data length must be a multiple of block size"
    );
    let mut out = Vec::new();
    codec.encode_blocks(blocks, &mut out).unwrap();
    out
}

pub fn block_decompress<C: BlockCodec>(compressed: &[u32], expected_len: Option<u32>) -> Vec<u32> {
    let mut codec = C::default();
    let mut out = Vec::new();
    codec
        .decode_blocks(compressed, expected_len, &mut out)
        .unwrap();
    out
}

/// Interpret `data` as little-endian `u32` words (length must be a multiple of 4) and
/// run [`roundtrip`] for every any-length codec covered here.
pub fn roundtrip_all(data: &[u32]) {
    roundtrip::<VariableByte>(data);
    roundtrip::<JustCopy>(data);
    roundtrip::<FastPFor256>(data);
    roundtrip::<FastPFor128>(data);

    #[cfg(feature = "cpp")]
    {
        use fastpfor::cpp::*;
        roundtrip::<CppFastPFor128>(data);
    }
}

pub fn block_roundtrip_all(data: &[u32]) {
    block_roundtrip::<FastPForBlock256>(data);
    block_roundtrip::<FastPForBlock128>(data);
}

// ---------------------------------------------------------------------------
// Data generators (private — only used to build fixtures)
// ---------------------------------------------------------------------------

type DataGeneratorFn = fn(usize) -> Vec<u32>;

fn generate_uniform_data_from_range(size: usize, value_range: Range<u32>) -> Vec<u32> {
    let mut rng = StdRng::seed_from_u64(SEED);
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
    let mut rng = StdRng::seed_from_u64(SEED);
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
    let mut rng = StdRng::seed_from_u64(SEED);
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
    vec![SEED as u32; size]
}

fn generate_geometric_data(size: usize) -> Vec<u32> {
    (0..size).map(|i| 1u32 << (i % 30)).collect()
}

/// Patterns used by compression / decompression / roundtrip / block-size benchmarks.
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

/// Superset of `COMPRESS_PATTERNS`, also used by the compression-ratio benchmark.
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

// ---------------------------------------------------------------------------
// Pre-computed fixtures
// ---------------------------------------------------------------------------

/// One row of pre-computed data for compression / decompression benchmarks.
///
/// Parameterised by `C: BlockCodec` so the same struct works for both 128-
/// and 256-element block codecs.
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
///
/// Parameterised by `C: BlockCodec` — create one per codec to compare.
/// FIXME: deduplicate these two structs if possible
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
            compressed: block_compress::<C>(&original),
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
            compressed: block_compress::<C>(&original),
            original,
            n_blocks: block_count,
            _codec: PhantomData,
        }
    }
}

/// Build fixtures for every `COMPRESS_PATTERNS × block_counts` combination.
pub fn compress_fixtures<C: BlockCodec>(
    block_counts: &[usize],
) -> Vec<(usize, CompressFixture<C>)> {
    block_counts
        .iter()
        .flat_map(|&bc| {
            COMPRESS_PATTERNS
                .iter()
                .map(move |&(name, generator)| (bc, CompressFixture::<C>::new(name, generator, bc)))
        })
        .collect()
}

/// Build fixtures for every `ALL_PATTERNS` at a single block count.
pub fn ratio_fixtures<C: BlockCodec>(block_count: usize) -> Vec<CompressFixture<C>> {
    ALL_PATTERNS
        .iter()
        .map(|&(name, generator)| CompressFixture::<C>::new(name, generator, block_count))
        .collect()
}
