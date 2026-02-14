# Fuzzing FastPFOR

This directory contains fuzz tests for the FastPFOR compression codec to find bugs, panics, and data corruption issues.

## Fuzz Targets

### Oracle-Based Fuzzers (Recommended)

These fuzzers compare the Rust implementation against the C++ reference implementation to find behavioral differences.

**`rust_compress_oracle`** - Compares Rust and C++ compression outputs
- Generates random input data
- Compresses with both Rust and C++ implementations
- Verifies both produce **identical compressed output**
- Finds discrepancies in compression behavior between implementations

**`rust_decompress_oracle`** - Compares Rust and C++ decompression outputs
- Generates random input data
- Compresses with C++ implementation (known good)
- Decompresses with both Rust and C++ implementations
- Verifies both produce **identical decompressed output**
- Ensures Rust can decompress C++-compressed data correctly
- Validates both implementations recover the original input

### Roundtrip Fuzzer

**`cpp_roundtrip`** - Tests C++ implementation roundtrip
- Compress → decompress → verify cycle
- Validates the reference C++ implementation
- Finds data corruption, crashes, and panics in the C++ FFI

## Why Oracle-Based Fuzzing?

Oracle fuzzing is the gold standard for verifying a reimplementation:

1. **Detects behavioral differences** - The Rust implementation should produce byte-for-byte identical output to the C++ reference
2. **Catches subtle bugs** - Even if both implementations round-trip correctly in isolation, they might compress differently
3. **Validates compatibility** - Ensures Rust can decompress C++ data and vice versa
4. **No false positives** - If outputs differ, it's a real bug (not just random data causing a crash)

## Why Fuzz FastPFOR?

The FastPFOR codec is a core compression algorithm. Fuzzing helps catch:
- Implementation discrepancies between Rust and C++
- Data corruption during compress/decompress cycles
- Panics on edge case inputs
- Buffer overflows or underflows
- Incorrect handling of different block sizes (128 vs 256)
- Issues with boundary conditions (empty data, very large values, etc.)

## Known Limitations

### Block Size Requirements

FastPFOR requires input lengths to be multiples of the block size (128 or 256). The fuzzers currently skip inputs that don't meet this requirement. Both implementations:
- Silently process only complete blocks
- Drop data that doesn't fit into complete blocks
- Should either handle partial blocks or return clear errors

This is a known limitation documented in the original C++ implementation.

## Prerequisites

Install cargo-fuzz and switch to nightly Rust:

```bash
cargo install cargo-fuzz
rustup install nightly
```

## Running the Fuzzers

Run the oracle-based compression fuzzer (recommended):

```bash
cd fuzz
cargo +nightly fuzz run rust_compress_oracle
```

Run the oracle-based decompression fuzzer:

```bash
cargo +nightly fuzz run rust_decompress_oracle
```

Run the C++ roundtrip fuzzer:

```bash
cargo +nightly fuzz run cpp_roundtrip
```

Run for a specific duration (e.g., 60 seconds):

```bash
cargo +nightly fuzz run rust_compress_oracle -- -max_total_time=60
```

Run with specific number of iterations:

```bash
cargo +nightly fuzz run rust_compress_oracle -- -runs=1000
```

## What the Oracle Fuzzers Test

1. Generate random sequences of u32 integers
2. Randomly select block size (128 or 256)
3. Truncate input to block size multiple
4. **Compress:** Compare Rust vs C++ compressed outputs (byte-for-byte)
5. **Decompress:** Compare Rust vs C++ decompressed outputs (element-by-element)
6. Verify both implementations recover the original input

This ensures the Rust implementation is **behaviorally equivalent** to the C++ reference.

## If a Crash Is Found

Crashes are saved to `fuzz/artifacts/<target_name>/`. To reproduce:

```bash
cargo +nightly fuzz run <target_name> fuzz/artifacts/<target_name>/crash-<hash>
```

For example:

```bash
cargo +nightly fuzz run rust_compress_oracle fuzz/artifacts/rust_compress_oracle/crash-abc123
```

## Interpreting Failures

### Compression Oracle Failure

If `rust_compress_oracle` fails, it means:
- Rust and C++ produce different compressed output for the same input
- This is a **critical bug** - the implementations are not compatible
- Compressed data from Rust may not decompress correctly with C++

### Decompression Oracle Failure

If `rust_decompress_oracle` fails, it means:
- Rust and C++ produce different output when decompressing the same data
- This is a **critical bug** - Rust cannot correctly read C++-compressed data
- Or both implementations fail to recover the original input

### Roundtrip Failure

If `cpp_roundtrip` fails, it means:
- The C++ reference implementation has a bug
- Or the FFI bindings are incorrect
- This should be rare as the C++ implementation is mature