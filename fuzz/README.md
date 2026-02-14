# Fuzzing FastPFOR

This directory contains fuzz tests for the FastPFOR compression codec to find bugs, panics, and data corruption issues.

## Why Fuzz FastPFOR?

The FastPFOR codec is a core compression algorithm. Fuzzing helps catch:
- Implementation discrepancies between Rust and C++
- Data corruption during compress/decompress cycles
- Panics on edge case inputs
- Buffer overflows or underflows
- Incorrect handling of different block sizes (128 vs 256)
- Issues with boundary conditions (empty data, very large values, etc.)

## Prerequisites

Install cargo-fuzz and switch to nightly Rust:

```bash
cargo install cargo-fuzz
rustup install nightly
```

## Running the Fuzzers

Run the oracle-based compression fuzzer:

```bash
cd fuzz
cargo +nightly fuzz run rust_compress_oracle
# or
cargo +nightly fuzz run rust_decompress_oracle
# or
cargo +nightly fuzz run rust_roundtrip_oracle
```

Run for a specific duration (e.g., 60 seconds):

```bash
cargo +nightly fuzz run rust_compress_oracle -- -max_total_time=60
```

Run with specific number of iterations:

```bash
cargo +nightly fuzz run rust_compress_oracle -- -runs=1000
```

## If a Crash Is Found

Crashes are saved to `fuzz/artifacts/<target_name>/`. To reproduce:

```bash
cargo +nightly fuzz run <target_name> fuzz/artifacts/<target_name>/crash-<hash>
```

For example:

```bash
cargo +nightly fuzz run rust_compress_oracle fuzz/artifacts/rust_compress_oracle/crash-abc123
```
