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

/// Low-level compression interface using caller-provided buffers.
///
/// Codecs write into pre-allocated slices and return a sub-slice showing exactly
/// what was written. Works across FFI boundaries and allows buffer reuse.
///
/// # Type Parameters
///
/// - `In`: Input data type (e.g., `u32` or `u64` for integer codecs)
/// - `Out`: Compressed output type (defaults to `In`, but may differ - e.g.,
///   64-bit integers compress to 32-bit words: `CodecToSlice<u64, u32>`)
///
/// # Buffer Sizing
///
/// Caller must ensure output buffers are large enough. For compression, estimate
/// `input.len() * 2 + 1024`. For decompression, size depends on the codec.
pub trait CodecToSlice<In, Out = In> {
    /// Error type returned by compression/decompression operations.
    type Error;

    /// Compresses input into output buffer, returning slice of data written.
    fn compress_to_slice<'out>(
        &mut self,
        input: &[In],
        output: &'out mut [Out],
    ) -> Result<&'out [Out], Self::Error>;

    /// Decompresses input into output buffer, returning slice of data written.
    ///
    /// Output size cannot be known in advance for some codecs (e.g., RLE).
    fn decompress_to_slice<'out>(
        &mut self,
        input: &[Out],
        output: &'out mut [In],
    ) -> Result<&'out [In], Self::Error>;
}
