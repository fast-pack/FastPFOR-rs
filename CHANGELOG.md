# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.1](https://github.com/fast-pack/FastPFOR-rs/compare/v0.9.0...v0.9.1) - 2026-10-02

### Added

- add Simd kernels for FastPFor (AVX2 and NEON) ([#96](https://github.com/fast-pack/FastPFOR-rs/pull/96))
- native pure-Rust u64 FastPFOR codec ([#85](https://github.com/fast-pack/FastPFOR-rs/pull/85))

### Fixed

- check decode_blocks expected_len against the input size ([#100](https://github.com/fast-pack/FastPFOR-rs/pull/100))
- reject exception lookups beyond the page's declared count ([#97](https://github.com/fast-pack/FastPFOR-rs/pull/97))
- zero the exception buffer tail before packing ([#94](https://github.com/fast-pack/FastPFOR-rs/pull/94))

### Other

- make code backwards compat with 0.9 ([#103](https://github.com/fast-pack/FastPFOR-rs/pull/103))
- make docs.rs build ([#102](https://github.com/fast-pack/FastPFOR-rs/pull/102))
- add release and dup-detection to CI ([#101](https://github.com/fast-pack/FastPFOR-rs/pull/101))
- share u64 narrowing between SIMD kernels and clarify names ([#99](https://github.com/fast-pack/FastPFOR-rs/pull/99))
- unroll scalar u64 bit-packing kernels per bit width ([#98](https://github.com/fast-pack/FastPFOR-rs/pull/98))
- bump MSRV to 1.87 and use is_multiple_of ([#95](https://github.com/fast-pack/FastPFOR-rs/pull/95))
- *(deps)* bump the all-actions-version-updates group across 1 directory with 4 updates ([#93](https://github.com/fast-pack/FastPFOR-rs/pull/93))
- [pre-commit.ci] pre-commit autoupdate ([#90](https://github.com/fast-pack/FastPFOR-rs/pull/90))
- *(deps)* bump the all-actions-version-updates group with 2 updates ([#88](https://github.com/fast-pack/FastPFOR-rs/pull/88))
- *(deps)* bump taiki-e/install-action from 2.82.0 to 2.83.1 in the all-actions-version-updates group ([#87](https://github.com/fast-pack/FastPFOR-rs/pull/87))
- many org cleanups, new err on bad input ([#86](https://github.com/fast-pack/FastPFOR-rs/pull/86))
- *(deps)* bump the all-actions-version-updates group across 1 directory with 2 updates ([#84](https://github.com/fast-pack/FastPFOR-rs/pull/84))
- *(deps)* improve supply chain security ([#82](https://github.com/fast-pack/FastPFOR-rs/pull/82))
- *(deps)* bump codecov/codecov-action from 5 to 6 in the all-actions-version-updates group ([#80](https://github.com/fast-pack/FastPFOR-rs/pull/80))
- Enable `clippy::use_self` at workspace level and align codec implementations ([#79](https://github.com/fast-pack/FastPFOR-rs/pull/79))
