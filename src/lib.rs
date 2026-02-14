#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(not(any(feature = "cpp", feature = "rust",)))]
compile_error!("At least one of the features 'cpp' or 'rust' must be enabled");

// FIXME: need decide on the external API. Some ideas:
//  - offer two sets of similar APIs - rust and cpp ffi
//  - it will be possible to enable/disable each with a feature flag
//  - introduce a new feature-agnostic API that will forward to either
//  - if both are enabled, forward to the more stable (ffi probably)
#[cfg(feature = "cpp")]
/// Rust wrapper for the [`FastPFOR` C++ library](https://github.com/fast-pack/FastPFor)
pub mod cpp;

#[cfg(feature = "rust")]
/// Rust re-implementation of `FastPFor` (work in progress)
pub mod rust;
