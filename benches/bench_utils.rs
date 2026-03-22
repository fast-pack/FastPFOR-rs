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
pub use std::io::Cursor;
use std::num::NonZeroU32;

#[cfg(feature = "cpp")]
use fastpfor::AnyLenCodec as _;
#[cfg(feature = "cpp")]
use fastpfor::cpp::CppFastPFor128;
pub use fastpfor::rust::{BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE, FastPFOR, Integer};
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
// Codec helpers
// ---------------------------------------------------------------------------

/// Compress `data` and return the compressed words.
pub fn compress_data(codec: &mut FastPFOR, data: &[u32]) -> Vec<u32> {
    let mut compressed = vec![0u32; data.len() * 2 + 1024];
    let mut input_offset = Cursor::new(0);
    let mut output_offset = Cursor::new(0);
    codec
        .compress(
            data,
            data.len() as u32,
            &mut input_offset,
            &mut compressed,
            &mut output_offset,
        )
        .unwrap();
    let len = output_offset.position() as usize;
    compressed.truncate(len);
    compressed
}

/// Decompress `compressed` into the caller-provided `decompressed` buffer and
/// return the number of elements written.
///
/// The buffer must be allocated outside the timed loop so that allocation cost
/// is not measured.
pub fn decompress_data(
    codec: &mut FastPFOR,
    compressed: &[u32],
    decompressed: &mut [u32],
) -> usize {
    let mut input_offset = Cursor::new(0);
    let mut output_offset = Cursor::new(0);
    codec
        .uncompress(
            compressed,
            compressed.len() as u32,
            &mut input_offset,
            decompressed,
            &mut output_offset,
        )
        .unwrap();
    output_offset.position() as usize
}

/// Pre-compress `data` with a specific `block_size` and return the compressed buffer.
fn prepare_compressed_data(data: &[u32], block_size: NonZeroU32) -> Vec<u32> {
    compress_data(&mut FastPFOR::new(DEFAULT_PAGE_SIZE, block_size), data)
}

// ---------------------------------------------------------------------------
// C++ helpers (compiled only when the `cpp` feature is active)
// ---------------------------------------------------------------------------

#[cfg(feature = "cpp")]
pub fn cpp_encode(codec: &mut CppFastPFor128, data: &[u32]) -> Vec<u32> {
    let mut out = Vec::new();
    codec.encode(data, &mut out).unwrap();
    out
}

#[cfg(feature = "cpp")]
pub fn cpp_decode(
    codec: &mut CppFastPFor128,
    compressed: &[u32],
    decompressed: &mut [u32],
) -> usize {
    let mut out = Vec::new();
    codec
        .decode(compressed, &mut out, Some(decompressed.len() as u32))
        .unwrap();
    decompressed.copy_from_slice(&out);
    out.len()
}

// ---------------------------------------------------------------------------
// Pre-computed fixtures
// ---------------------------------------------------------------------------

/// One row of pre-computed data for compression / decompression benchmarks.
pub struct CompressFixture {
    pub name: &'static str,
    pub data: Vec<u32>,
    /// Rust-compressed form (`BLOCK_SIZE_128`), ready for decompression benchmarks.
    pub rust_compressed: Vec<u32>,
}

impl CompressFixture {
    fn new(name: &'static str, generator: DataGeneratorFn, size: usize) -> Self {
        let data = generator(size);
        let rust_compressed = prepare_compressed_data(&data, BLOCK_SIZE_128);
        Self {
            name,
            data,
            rust_compressed,
        }
    }
}

/// Build fixtures for every `COMPRESS_PATTERNS × sizes` combination.
pub fn compress_fixtures(sizes: &[usize]) -> Vec<(usize, CompressFixture)> {
    sizes
        .iter()
        .flat_map(|&size| {
            COMPRESS_PATTERNS
                .iter()
                .map(move |&(name, generator)| (size, CompressFixture::new(name, generator, size)))
        })
        .collect()
}

/// Build fixtures for every `ALL_PATTERNS` at a single size.
pub fn ratio_fixtures(size: usize) -> Vec<CompressFixture> {
    ALL_PATTERNS
        .iter()
        .map(|&(name, generator)| CompressFixture::new(name, generator, size))
        .collect()
}

/// One row for the block-size benchmark.
pub struct BlockSizeFixture {
    pub block_size: NonZeroU32,
    pub data: Vec<u32>,
    pub compressed: Vec<u32>,
}

impl BlockSizeFixture {
    fn new(block_size: NonZeroU32, size: usize) -> Self {
        let data = generate_uniform_data_small_value_distribution(size);
        let compressed = prepare_compressed_data(&data, block_size);
        Self {
            block_size,
            data,
            compressed,
        }
    }
}

/// Build fixtures for both block sizes at a given `size`.
pub fn block_size_fixtures(size: usize) -> Vec<BlockSizeFixture> {
    [BLOCK_SIZE_128, BLOCK_SIZE_256]
        .iter()
        .map(|&bs| BlockSizeFixture::new(bs, size))
        .collect()
}

/// One row for the C++ vs Rust decode benchmark.
#[cfg(feature = "cpp")]
pub struct CppDecodeFixture {
    pub name: &'static str,
    pub cpp_compressed: Vec<u32>,
    pub rust_compressed: Vec<u32>,
    pub original_len: usize,
}

#[cfg(feature = "cpp")]
impl CppDecodeFixture {
    fn new(name: &'static str, generator: DataGeneratorFn, size: usize) -> Self {
        let data = generator(size);
        let mut codec = CppFastPFor128::new();
        let cpp_compressed = cpp_encode(&mut codec, &data);
        let rust_compressed = prepare_compressed_data(&data, BLOCK_SIZE_128);
        Self {
            name,
            cpp_compressed,
            rust_compressed,
            original_len: size,
        }
    }
}

/// Build C++ vs Rust decode fixtures for every `COMPRESS_PATTERNS × sizes` combination.
#[cfg(feature = "cpp")]
pub fn cpp_decode_fixtures(sizes: &[usize]) -> Vec<(usize, CppDecodeFixture)> {
    sizes
        .iter()
        .flat_map(|&size| {
            COMPRESS_PATTERNS
                .iter()
                .map(move |&(name, generator)| (size, CppDecodeFixture::new(name, generator, size)))
        })
        .collect()
}
