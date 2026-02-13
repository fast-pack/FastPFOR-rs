# Fuzzing FastPFOR

This directory contains a fuzz test for the FastPFOR compression codec to find bugs, panics, and data corruption issues.

## Why Fuzz FastPFOR?

The FastPFOR codec is the core compression algorithm. Fuzzing helps catch:
- Data corruption during compress/decompress roundtrips
- Panics on edge case inputs
- Buffer overflows or underflows
- Incorrect handling of different block sizes (128 vs 256)
- Issues with boundary conditions (empty data, very large values, etc.)

## Known Issues Found

The fuzzer has already discovered the following issues:

### Data Loss with Small Inputs (Issue #1)

**Input:** Single element array `[0]` with block size 128

**Issue:** FastPFOR silently drops data that doesn't fit into complete blocks. When the input length is less than the block size (128 or 256), `greatest_multiple(input_length, block_size)` returns 0, causing the codec to:
1. Write 0 to the output header
2. Skip compression entirely
3. On decompression, return 0 elements instead of the original input

**Expected:** Should either:
- Compress partial blocks correctly, or
- Return an error indicating input is too small, or
- Document this limitation clearly

This is a **data corruption bug** where the codec claims success but loses data.

## Prerequisites

Install cargo-fuzz and switch to nightly Rust:

```bash
cargo install cargo-fuzz
rustup install nightly
```

## Running the Fuzzer

```bash
cd fuzz
cargo +nightly fuzz run fastpfor_roundtrip
```

Run for a specific duration (e.g., 60 seconds):

```bash
cargo +nightly fuzz run fastpfor_roundtrip -- -max_total_time=60
```

## What It Tests

The fuzzer:
1. Generates random sequences of u32 integers
2. Randomly selects block size (128 or 256)
3. Compresses the data with FastPFOR
4. Decompresses the result
5. Verifies the output matches the original input exactly

This ensures the codec is lossless and doesn't corrupt data under any input pattern.

## If a Crash Is Found

Crashes are saved to `fuzz/artifacts/fastpfor_roundtrip/`. To reproduce:

```bash
cargo +nightly fuzz run fastpfor_roundtrip fuzz/artifacts/fastpfor_roundtrip/crash-<hash>
```
