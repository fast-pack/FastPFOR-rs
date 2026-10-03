#![deny(
    unsafe_code,
    reason = "x86 SIMD dispatch and C++ interop must allow, so can't use forbid here"
)]
// Without SIMD intrinsics or C++ interop there is no `unsafe` code at all: make that a guarantee.
#![cfg_attr(not(any(feature = "simd", feature = "cpp")), forbid(unsafe_code))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(not(any(feature = "cpp", feature = "rust")))]
compile_error!("At least one of the features 'cpp' or 'rust' must be enabled");

// Error types are always available regardless of which codec features are enabled.
mod error;
pub use error::{FastPForError, FastPForResult};

#[cfg(feature = "cpp")]
/// Rust wrapper for the [`FastPFOR` C++ library](https://github.com/fast-pack/FastPFor)
pub mod cpp;

#[cfg(feature = "rust")]
#[deny(
    unsafe_code,
    reason = "Rust code must be safe; the only exception is calling SIMD kernels after runtime CPU feature detection"
)]
pub(crate) mod rust;

mod codec;
pub use codec::{AnyLenCodec, BlockCodec, BlockCodec64, slice_to_blocks};

pub(crate) mod helpers;

// Re-export bytemuck::Pod so that users writing generic `BlockCodec` code
// can constrain their own `Block` associated-type bounds without a separate
// `bytemuck` dependency.
pub use bytemuck::Pod;
#[cfg(feature = "rust")]
pub use rust::{
    Auto, CompositeCodec, FastPForBlock, FastPForCodec, FastPForInterleaved32x128,
    FastPForInterleaved32x256, FastPForInterleaved64x128, FastPForInterleaved64x256,
    FastPForInterleavedBlock32x128, FastPForInterleavedBlock32x256, FastPForInterleavedBlock64x128,
    FastPForInterleavedBlock64x256, FastPForSequential32x128, FastPForSequential32x256,
    FastPForSequential64x128, FastPForSequential64x256, FastPForSequentialBlock32x128,
    FastPForSequentialBlock32x256, FastPForSequentialBlock64x128, FastPForSequentialBlock64x256,
    Interleaved, JustCopy, Kernels, Layout, Portable, Sequential, VariableByte,
};

#[cfg(feature = "__testing")]
#[doc(hidden)]
/// Test-only hooks. Not part of the public API.
pub mod __testing {
    pub use crate::rust::{Implementation, implementation, with_simd_fallback};
}

// `src/test_utils.rs` uses `fastpfor::...`; alias this crate for unit tests only.
#[cfg(test)]
extern crate self as fastpfor;

#[cfg(test)]
pub(crate) mod test_utils;
