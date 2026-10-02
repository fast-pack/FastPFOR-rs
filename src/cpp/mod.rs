//! C++ codec wrappers — see the [crate-level documentation](crate) for usage and codec selection.
//!
//! All C++ codecs are composite (any-length) and implement [`AnyLenCodec`] only.
//! Codecs marked with `@ 64` also implement [`BlockCodec64`] for 64-bit integers.
//!
//! **Thread safety:** instances have internal state and are not thread-safe. Use one per thread.

mod codecs;
mod ffi;
mod wrappers;

pub use codecs::*;
