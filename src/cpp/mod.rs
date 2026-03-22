//! C++ codec wrappers — see the [crate-level documentation](crate) for usage and codec selection.
//!
//! All C++ codecs are composite (any-length) and implement [`AnyLenCodec`] only.
//! Codecs marked with `@ 64` also implement [`BlockCodec64`] for 64-bit integers.
//!
//! **Thread safety:** instances have internal state and are not thread-safe. Use one per thread.

mod codecs;
#[cfg(test)]
mod tests;
mod wrappers;

pub use codecs::*;

/// FFI bridge to the C++ FastPFOR library.
///
/// This module contains the raw FFI declarations for interfacing with the C++ code.
/// It is not directly accessible from outside this module.
#[cxx::bridge(namespace = "FastPForLib")]
mod ffi {
    unsafe extern "C++" {
        include!("fastpfor_bridge.h");

        /// Opaque handle to a C++ IntegerCODEC instance.
        type IntegerCODEC;

        // Codec factory functions - each creates a new codec instance wrapped in a UniquePtr
        fn fastbinarypacking8_codec() -> UniquePtr<IntegerCODEC>;
        fn fastbinarypacking16_codec() -> UniquePtr<IntegerCODEC>;
        fn fastbinarypacking32_codec() -> UniquePtr<IntegerCODEC>;
        fn BP32_codec() -> UniquePtr<IntegerCODEC>;
        // fn vsencoding_codec() -> UniquePtr<IntegerCODEC>;
        fn fastpfor128_codec() -> UniquePtr<IntegerCODEC>;
        fn fastpfor256_codec() -> UniquePtr<IntegerCODEC>;
        fn simdfastpfor128_codec() -> UniquePtr<IntegerCODEC>;
        fn simdfastpfor256_codec() -> UniquePtr<IntegerCODEC>;
        fn simplepfor_codec() -> UniquePtr<IntegerCODEC>;
        fn simdsimplepfor_codec() -> UniquePtr<IntegerCODEC>;
        fn pfor_codec() -> UniquePtr<IntegerCODEC>;
        fn simdpfor_codec() -> UniquePtr<IntegerCODEC>;
        fn pfor2008_codec() -> UniquePtr<IntegerCODEC>;
        fn simdnewpfor_codec() -> UniquePtr<IntegerCODEC>;
        fn newpfor_codec() -> UniquePtr<IntegerCODEC>;
        fn optpfor_codec() -> UniquePtr<IntegerCODEC>;
        fn simdoptpfor_codec() -> UniquePtr<IntegerCODEC>;
        fn varint_codec() -> UniquePtr<IntegerCODEC>;
        fn vbyte_codec() -> UniquePtr<IntegerCODEC>;
        fn maskedvbyte_codec() -> UniquePtr<IntegerCODEC>;
        fn streamvbyte_codec() -> UniquePtr<IntegerCODEC>;
        fn varintgb_codec() -> UniquePtr<IntegerCODEC>;
        fn simple16_codec() -> UniquePtr<IntegerCODEC>;
        fn simple9_codec() -> UniquePtr<IntegerCODEC>;
        fn simple9_rle_codec() -> UniquePtr<IntegerCODEC>;
        fn simple8b_codec() -> UniquePtr<IntegerCODEC>;
        fn simple8b_rle_codec() -> UniquePtr<IntegerCODEC>;
        // TODO: conditional with #ifdef, might support later
        // fn varintg8iu_codec() -> UniquePtr<IntegerCODEC>;
        // fn snappy_codec() -> UniquePtr<IntegerCODEC>;
        fn simdbinarypacking_codec() -> UniquePtr<IntegerCODEC>;
        fn simdgroupsimple_codec() -> UniquePtr<IntegerCODEC>;
        fn simdgroupsimple_ringbuf_codec() -> UniquePtr<IntegerCODEC>;
        fn copy_codec() -> UniquePtr<IntegerCODEC>;

        // Core encoding/decoding functions bridging to C++ IntegerCODEC methods

        /// Encodes 32-bit integers.
        ///
        /// Returns the number of 32-bit words written to output.
        fn codec_encode32(
            codec: &UniquePtr<IntegerCODEC>,
            input: &[u32],
            output: &mut [u32],
        ) -> Result<usize>;

        /// Encodes 64-bit integers into 32-bit word representation.
        ///
        /// Returns the number of 32-bit words written to output.
        fn codec_encode64(
            codec: &UniquePtr<IntegerCODEC>,
            input: &[u64],
            output: &mut [u32],
        ) -> Result<usize>;

        /// Decodes 32-bit integers.
        ///
        /// Returns the number of integers written to output.
        fn codec_decode32(
            codec: &UniquePtr<IntegerCODEC>,
            input: &[u32],
            output: &mut [u32],
        ) -> Result<usize>;

        /// Decodes 64-bit integers from 32-bit word representation.
        ///
        /// Returns the number of 64-bit integers written to output.
        fn codec_decode64(
            codec: &UniquePtr<IntegerCODEC>,
            input: &[u32],
            output: &mut [u64],
        ) -> Result<usize>;
    }
}
