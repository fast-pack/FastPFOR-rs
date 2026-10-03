#![deny(
    unsafe_code,
    reason = "x86 SIMD dispatch and C++ interop must allow, so can't use forbid here"
)]
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
#[allow(deprecated, reason = "re-exports deprecated names")]
pub use rust::{
    Auto, CompositeCodec, FastPFor, FastPFor128, FastPFor256, FastPForBlock, FastPForBlock128,
    FastPForBlock256, FastPForBlockWide128, FastPForBlockWide256, FastPForSequential32x128,
    FastPForSequential32x256, FastPForSequential64x128, FastPForSequential64x256,
    FastPForSequentialBlock32x128, FastPForSequentialBlock32x256, FastPForSequentialBlock64x128,
    FastPForSequentialBlock64x256, FastPForSimd128, FastPForSimd256, FastPForSimdBlock128,
    FastPForSimdBlock256, FastPForSimdBlockWide128, FastPForSimdBlockWide256, FastPForSimdWide128,
    FastPForSimdWide256, FastPForWide128, FastPForWide256, JustCopy, Kernels, Layout, Portable,
    Scalar, Sequential, Simd, VariableByte,
};

#[cfg(feature = "__testing")]
#[doc(hidden)]
/// Test-only hooks. Not part of the public API.
pub mod __testing {
    pub use crate::rust::with_simd_fallback;
}

// `src/test_utils.rs` uses `fastpfor::...`; alias this crate for unit tests only.
#[cfg(test)]
extern crate self as fastpfor;

#[cfg(test)]
pub(crate) mod test_utils;
