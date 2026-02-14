#![doc = include_str!("README.md")]

// Re-export CXX Exception type to simplify usage
pub use cxx::Exception;
use cxx::UniquePtr;

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
        fn vsencoding_codec() -> UniquePtr<IntegerCODEC>;
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

/// Internal trait for providing access to the underlying C++ codec.
trait CodecWrapper {
    fn codec(&self) -> &UniquePtr<ffi::IntegerCODEC>;
}

/// Trait for codecs that support 32-bit integer compression.
///
/// # Example
///
/// ```no_run
/// # use fastpfor::cpp::{BP32Codec, Codec32};
/// let codec = BP32Codec::new();
/// let data = vec![10, 20, 30, 40];
/// let mut compressed = vec![0u32; data.len() + 1024];
///
/// let encoded = codec.encode32(&data, &mut compressed).unwrap();
/// println!("Compressed {} integers into {} words", data.len(), encoded.len());
///
/// let mut decompressed = vec![0u32; data.len()];
/// let result = codec.decode32(encoded, &mut decompressed).unwrap();
/// assert_eq!(result, &data[..]);
/// ```
#[expect(private_bounds)]
pub trait Codec32: CodecWrapper {
    /// Encodes 32-bit integers into compressed form.
    ///
    /// Returns a mutable slice containing only the compressed data (a sub-slice of `output`).
    fn encode32<'out>(
        &self,
        input: &[u32],
        output: &'out mut [u32],
    ) -> Result<&'out mut [u32], Exception> {
        let n = ffi::codec_encode32(self.codec(), input, output)?;
        Ok(&mut output[..n])
    }

    /// Decodes compressed 32-bit integers.
    ///
    /// Returns a mutable slice containing only the decompressed data (a sub-slice of `output`).
    fn decode32<'out>(
        &self,
        input: &[u32],
        output: &'out mut [u32],
    ) -> Result<&'out mut [u32], Exception> {
        let n = ffi::codec_decode32(self.codec(), input, output)?;
        Ok(&mut output[..n])
    }
}

/// Trait for codecs that support 64-bit integer compression.
///
/// Only certain codecs support 64-bit integers. These are marked with the `@ 64`
/// annotation in the `implement_codecs!` macro invocation.
///
/// # Compressed Format
///
/// 64-bit integers are compressed into 32-bit word arrays for compatibility with
/// the underlying C++ library.
///
/// # Example
///
/// ```no_run
/// # use fastpfor::cpp::{FastPFor128Codec, Codec64};
/// let codec = FastPFor128Codec::new();
/// let data = vec![100u64, 200, 300, 400];
/// let mut compressed = vec![0u32; data.len() * 2 + 1024];
///
/// let encoded = codec.encode64(&data, &mut compressed).unwrap();
/// println!("Compressed {} 64-bit integers into {} words", data.len(), encoded.len());
///
/// let mut decompressed = vec![0u64; data.len()];
/// let result = codec.decode64(encoded, &mut decompressed).unwrap();
/// assert_eq!(result, &data[..]);
/// ```
#[expect(private_bounds)]
pub trait Codec64: CodecWrapper {
    /// Encodes 64-bit integers into compressed 32-bit word form.
    ///
    /// Returns a mutable slice containing only the compressed data (a sub-slice of `output`).
    fn encode64<'out>(
        &self,
        input: &[u64],
        output: &'out mut [u32],
    ) -> Result<&'out mut [u32], Exception> {
        let n = ffi::codec_encode64(self.codec(), input, output)?;
        Ok(&mut output[..n])
    }

    /// Decodes 64-bit integers from compressed 32-bit word form.
    ///
    /// Returns a mutable slice containing only the decompressed data (a sub-slice of `output`).
    fn decode64<'out>(
        &self,
        input: &[u32],
        output: &'out mut [u64],
    ) -> Result<&'out mut [u64], Exception> {
        let n = ffi::codec_decode64(self.codec(), input, output)?;
        Ok(&mut output[..n])
    }
}

/// Macro to generate codec wrapper types and their implementations.
///
/// The `@ 64` marker indicates that a codec supports 64-bit integers.
macro_rules! implement_codecs {
    ($(
        $(#[$($attrs:tt)*])*
        $name:ident $(@ $is_64:literal)? => $ffi:ident ,
    )*) => {
        $(
            $(#[$($attrs)*])*
            pub struct $name(UniquePtr<ffi::IntegerCODEC>);

            impl $name {
                /// Creates a new instance of this codec.
                pub fn new() -> Self {
                    Self(ffi::$ffi())
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl CodecWrapper for $name {
                fn codec(&self) -> &UniquePtr<ffi::IntegerCODEC> {
                    &self.0
                }
            }

            impl Codec32 for $name {}
            $(
                // hack to only expand this block if $is_64 is set
                const _ : () = { let _ = $is_64; };
                impl Codec64 for $name {}
            )*
        )*

        #[cfg(test)]
        mod codec_tests {
            use super::*;

            $(
                #[test]
                #[expect(non_snake_case)]
                fn $name() {
                    $(
                        // hack to only expand this block if $is_64 is set
                        const _ : () = { let _ = $is_64; };
                        roundtrip_64($name::new());
                    )*
                    roundtrip_32($name::new());
                }
            )*

            fn roundtrip_32(codec: impl Codec32) {
                let input = vec![1, 2, 3, 4, 5];
                let mut output = vec![0; 10];
                let encoded = codec.encode32(&input, &mut output).unwrap();
                let mut decoded = vec![0; 10];
                let decoded = codec.decode32(encoded, &mut decoded).unwrap();
                assert_eq!(decoded, input);
            }

            fn roundtrip_64(codec: impl Codec64) {
                let input = vec![1, 2, 3, 4, 5];
                let mut output = vec![0; 10];
                let encoded = codec.encode64(&input, &mut output).unwrap();

                let mut decoded = vec![0; 10];
                let decoded = codec.decode64(encoded, &mut decoded).unwrap();
                assert_eq!(decoded, input);
            }
        }
    };
}

// Codec implementations generated by the macro above.
//
// Codecs marked with `@ 64` support both 32-bit and 64-bit integers.
implement_codecs! {
    /// Binary Packing codec optimized for 32-bit blocks.
    BP32Codec => BP32_codec,

    /// Copy codec that performs no compression.
    CopyCodec => copy_codec,

    /// Fast binary packing with 8-bit processing blocks.
    FastBinaryPacking8Codec => fastbinarypacking8_codec,

    /// Fast binary packing with 16-bit processing blocks.
    FastBinaryPacking16Codec => fastbinarypacking16_codec,

    /// Fast binary packing with 32-bit processing blocks.
    FastBinaryPacking32Codec => fastbinarypacking32_codec,

    /// Fast [`PForCodec`] with 128-value processing blocks.
    FastPFor128Codec @ 64 => fastpfor128_codec,

    /// Fast [`PForCodec`] with 256-value processing blocks.
    FastPFor256Codec @ 64 => fastpfor256_codec,

    /// Masked [`VByteCodec`] with SIMD optimizations.
    MaskedVByteCodec => maskedvbyte_codec,

    /// New [`PForCodec`].
    NewPForCodec => newpfor_codec,

    /// Optimized [`PForCodec`].
    OptPForCodec => optpfor_codec,

    /// [`PForCodec`] based on the 2008 research paper.
    PFor2008Codec => pfor2008_codec,

    /// Standard Patched Frame of Reference codec.
    PForCodec => pfor_codec,

    /// SIMD-accelerated binary packing codec.
    SimdBinaryPackingCodec => simdbinarypacking_codec,

    /// SIMD-optimized [`FastPFor128Codec`] with 128-value blocks.
    SimdFastPFor128Codec => simdfastpfor128_codec,

    /// SIMD-optimized [`FastPFor256Codec`] with 256-value blocks.
    SimdFastPFor256Codec => simdfastpfor256_codec,

    /// SIMD group simple codec.
    SimdGroupSimpleCodec => simdgroupsimple_codec,

    /// SIMD group simple codec with ring buffer optimization.
    SimdGroupSimpleRingBufCodec => simdgroupsimple_ringbuf_codec,

    /// SIMD-accelerated [`NewPForCodec`].
    SimdNewPForCodec => simdnewpfor_codec,

    /// SIMD-accelerated [`OptPForCodec`].
    SimdOptPForCodec => simdoptpfor_codec,

    /// SIMD-accelerated [`PForCodec`].
    SimdPForCodec => simdpfor_codec,

    /// SIMD-accelerated [`SimplePForCodec`].
    SimdSimplePForCodec => simdsimplepfor_codec,

    /// Simple-16 encoding scheme.
    Simple16Codec => simple16_codec,

    /// Simple-8b encoding scheme.
    Simple8bCodec => simple8b_codec,

    /// Simple-8b encoding with run-length encoding.
    Simple8bRleCodec => simple8b_rle_codec,

    /// Simple-9 encoding scheme.
    Simple9Codec => simple9_codec,

    /// Simple-9 encoding with run-length encoding.
    Simple9RleCodec => simple9_rle_codec,

    /// Simple Patched Frame of Reference ([`PForCodec`]) codec.
    SimplePForCodec => simplepfor_codec,

    // SnappyCodec => snappy_codec,  // Conditional with #ifdef

    /// [`StreamVByte`](https://github.com/lemire/streamvbyte) encoding for fast variable-byte compression.
    StreamVByteCodec => streamvbyte_codec,

    /// Standard variable-byte encoding.
    VByteCodec => vbyte_codec,

    /// Variable-length integer encoding.
    VarIntCodec @ 64 => varint_codec,

    // VarIntG8iuCodec => varintg8iu_codec,  // Conditional with #ifdef

    /// Group Variable-length integer encoding with optimizations.
    VarIntGbCodec => varintgb_codec,

    // VsEncodingCodec => vsencoding_codec,  // This is leaking memory
}

#[cfg(test)]
mod tests {
    use super::*;

    // These duplicate the macro-generated tests, but we want to test that
    // the macro expansion itself works correctly

    #[test]
    fn test_32() {
        let codec = FastPFor128Codec::new();
        let input = vec![1, 2, 3, 4, 5];
        let mut output = vec![0; 10];
        let mut output2 = vec![0; 10];
        let encoded = codec.encode32(&input, &mut output).unwrap();
        let encoded2 = codec.encode32(&input, &mut output2).unwrap();
        assert_eq!(encoded, encoded2);

        let mut decoded = vec![0; 10];
        let mut decoded2 = vec![0; 10];
        let decoded = codec.decode32(encoded, &mut decoded).unwrap();
        let decoded2 = codec.decode32(encoded, &mut decoded2).unwrap();
        assert_eq!(decoded, decoded2);

        assert_eq!(decoded, input);
    }

    #[test]
    fn test_64() {
        let codec = FastPFor128Codec::new();
        let input = vec![1, 2, 3, 4, 5];
        let mut output = vec![0; 10];
        let mut output2 = vec![0; 10];
        let encoded = codec.encode64(&input, &mut output).unwrap();
        let encoded2 = codec.encode64(&input, &mut output2).unwrap();
        assert_eq!(encoded, encoded2);

        let mut decoded = vec![0; 10];
        let mut decoded2 = vec![0; 10];
        let decoded = codec.decode64(encoded, &mut decoded).unwrap();
        let decoded2 = codec.decode64(encoded, &mut decoded2).unwrap();
        assert_eq!(decoded, decoded2);

        assert_eq!(decoded, input);
    }
}
