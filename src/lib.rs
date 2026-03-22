#![cfg_attr(not(feature = "cpp"), forbid(unsafe_code))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(not(any(feature = "cpp", feature = "rust")))]
compile_error!("At least one of the features 'cpp' or 'rust' must be enabled");

// Error types are always available regardless of which codec features are enabled.
mod error;
pub use error::FastPForError;

#[cfg(feature = "cpp")]
/// Rust wrapper for the [`FastPFOR` C++ library](https://github.com/fast-pack/FastPFor)
pub mod cpp;

#[cfg(feature = "rust")]
#[forbid(unsafe_code, reason = "Rust code must always be safe")]
pub(crate) mod rust;

mod codec;
#[cfg(feature = "cpp")]
pub use codec::BlockCodec64;
pub use codec::{AnyLenCodec, BlockCodec, slice_to_blocks};

pub(crate) mod helpers;

// Re-export bytemuck::Pod so that users writing generic `BlockCodec` code
// can constrain their own `Block` associated-type bounds without a separate
// `bytemuck` dependency.
pub use bytemuck::Pod;
#[cfg(feature = "rust")]
pub use rust::{
    CompositeCodec, FastPFor, FastPFor128, FastPFor256, FastPForBlock128, FastPForBlock256,
    JustCopy, VariableByte,
};
