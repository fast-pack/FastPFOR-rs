//! Shared data generators, codec helpers, and pre-computed fixtures used by
//! both the Criterion benchmark (`fastpfor_benchmark.rs`) and the smoke-test
//! suite (`tests/benchmark_smoke.rs`).
//!
//! Loaded as a module via `#[path]` in both consumers, so every item consumed
//! from outside must be `pub`.

// This is an internal dev-only module; doc-comments on every field would add
// noise without benefit.
#![allow(missing_docs)]

use core::ops::Range;
use std::marker::PhantomData;

#[allow(unused_imports)]
use fastpfor::{AnyLenCodec, BlockCodec, slice_to_blocks};
use rand::rngs::StdRng;
use rand::{RngExt as _, SeedableRng};

const SEED: u64 = 456;

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
// Generic codec helpers
// ---------------------------------------------------------------------------

/// Compress `data` with codec `C`, appending to `out` (which is cleared first).
///
/// Only the block-aligned prefix of `data` is compressed; any sub-block
/// remainder is silently dropped, matching what the benchmarks measure.
pub fn compress<C: BlockCodec + Default>(data: &[u32], out: &mut Vec<u32>) {
    let mut codec = C::default();
    let (blocks, _remainder) = slice_to_blocks::<C>(data);
    out.clear();
    codec.encode_blocks(blocks, out).unwrap();
}

/// Decompress `n_blocks` blocks of codec `C` from `compressed` into `out`
/// (cleared first), returning the number of elements written.
#[allow(dead_code)] // used by smoke tests; benches use codec directly
pub fn decompress<C: BlockCodec + Default>(
    compressed: &[u32],
    n_blocks: usize,
    out: &mut Vec<u32>,
) -> usize {
    let mut codec = C::default();
    out.clear();
    let expected_values = n_blocks * C::size();
    codec
        .decode_blocks(
            compressed,
            Some(u32::try_from(expected_values).expect("expected_values fits in u32")),
            out,
        )
        .unwrap();
    out.len()
}

/// Decompress with any-length codec `C`, using `expected_len` for validation/pre-allocation.
#[allow(dead_code)] // used by smoke_cpp_vs_rust
pub fn decompress_anylen<C: AnyLenCodec + Default>(
    compressed: &[u32],
    expected_len: usize,
    out: &mut Vec<u32>,
) -> usize {
    let mut codec = C::default();
    out.clear();
    codec
        .decode(
            compressed,
            out,
            Some(u32::try_from(expected_len).expect("expected_len fits in u32")),
        )
        .unwrap();
    out.len()
}

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
    pub data: Vec<u32>,
    /// Pre-compressed form, ready for decompression benchmarks.
    pub compressed: Vec<u32>,
    /// Number of blocks in `data`.
    pub n_blocks: usize,
    _codec: PhantomData<C>,
}

impl<C: BlockCodec + Default> CompressFixture<C> {
    fn new(name: &'static str, generator: DataGeneratorFn, block_count: usize) -> Self {
        let data = generator(block_count * C::size());
        // Data is already exactly block_count * blen elements; no trimming needed.
        let mut compressed = Vec::new();
        compress::<C>(&data, &mut compressed);
        Self {
            name,
            data,
            compressed,
            n_blocks: block_count,
            _codec: PhantomData,
        }
    }
}

/// Build fixtures for every `COMPRESS_PATTERNS × block_counts` combination.
pub fn compress_fixtures<C: BlockCodec + Default>(
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
pub fn ratio_fixtures<C: BlockCodec + Default>(block_count: usize) -> Vec<CompressFixture<C>> {
    ALL_PATTERNS
        .iter()
        .map(|&(name, generator)| CompressFixture::<C>::new(name, generator, block_count))
        .collect()
}

/// One row for the block-size comparison benchmark.
///
/// Parameterised by `C: BlockCodec` — create one per codec to compare.
pub struct BlockSizeFixture<C: BlockCodec> {
    pub data: Vec<u32>,
    pub compressed: Vec<u32>,
    pub n_blocks: usize,
    _codec: PhantomData<C>,
}

impl<C: BlockCodec + Default> BlockSizeFixture<C> {
    pub fn new(block_count: usize) -> Self {
        let data = generate_uniform_data_small_value_distribution(block_count * C::size());
        let mut compressed = Vec::new();
        compress::<C>(&data, &mut compressed);
        Self {
            data,
            compressed,
            n_blocks: block_count,
            _codec: PhantomData,
        }
    }
}
