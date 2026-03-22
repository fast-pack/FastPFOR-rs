use cxx::UniquePtr;

use crate::FastPForError;
use crate::codec::{AnyLenCodec, BlockCodec64};
use crate::cpp::ffi;
use crate::cpp::wrappers::{
    decode32_anylen_ffi, decode64_to_vec_ffi, encode32_to_vec_ffi, encode64_to_vec_ffi,
};

// ── Codec struct definitions ─────────────────────────────────────────────────
//
// Single macro: all C++ codecs implement AnyLenCodec. Codecs marked with `@ 64`
// also implement BlockCodec64 for 64-bit integer support.

/// Macro for C++ codec wrappers: struct + Default + `AnyLenCodec`.
macro_rules! implement_cpp_codecs {
    ($(
        $(#[$($attrs:tt)*])*
        $name:ident => $ffi:ident ,
    )*) => {
        $(
            $(#[$($attrs)*])*
            pub struct $name(UniquePtr<ffi::IntegerCODEC>);

            impl $name {
                /// Creates a new instance of this codec.
                #[must_use]
                pub fn new() -> Self {
                    Self(ffi::$ffi())
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl AnyLenCodec for $name {
                fn encode(&mut self, input: &[u32], out: &mut Vec<u32>) -> Result<(), FastPForError> {
                    encode32_to_vec_ffi(&self.0, input, out)
                }

                fn decode(
                    &mut self,
                    input: &[u32],
                    out: &mut Vec<u32>,
                    expected_len: Option<u32>,
                ) -> Result<(), FastPForError> {
                    decode32_anylen_ffi(&self.0, input, out, expected_len)
                }
            }
        )*
    };
}

// ── All C++ codecs (composite / any-length) ─
implement_cpp_codecs! {
    /// Binary Packing codec optimized for 32-bit blocks.
    CppBP32 => BP32_codec,

    /// Copy codec that performs no compression.
    CppCopy => copy_codec,

    /// Fast binary packing with 8-bit processing blocks.
    CppFastBinaryPacking8 => fastbinarypacking8_codec,

    /// Fast binary packing with 16-bit processing blocks.
    CppFastBinaryPacking16 => fastbinarypacking16_codec,

    /// Fast binary packing with 32-bit processing blocks.
    CppFastBinaryPacking32 => fastbinarypacking32_codec,

    /// Fast [`CppPFor`] with 128-value processing blocks.
    CppFastPFor128 => fastpfor128_codec,

    /// Fast [`CppPFor`] with 256-value processing blocks.
    CppFastPFor256 => fastpfor256_codec,

    /// Masked [`CppVByte`] with SIMD optimizations.
    CppMaskedVByte => maskedvbyte_codec,

    /// New [`CppPFor`].
    CppNewPFor => newpfor_codec,

    /// Optimized [`CppPFor`].
    CppOptPFor => optpfor_codec,

    /// [`CppPFor`] based on the 2008 research paper.
    CppPFor2008 => pfor2008_codec,

    /// Standard Patched Frame of Reference codec.
    CppPFor => pfor_codec,

    /// SIMD-accelerated binary packing codec.
    CppSimdBinaryPacking => simdbinarypacking_codec,

    /// SIMD-optimized [`CppFastPFor128`] with 128-value blocks.
    CppSimdFastPFor128 => simdfastpfor128_codec,

    /// SIMD-optimized [`CppFastPFor256`] with 256-value blocks.
    CppSimdFastPFor256 => simdfastpfor256_codec,

    /// SIMD group simple codec.
    CppSimdGroupSimple => simdgroupsimple_codec,

    /// SIMD group simple codec with ring buffer optimization.
    CppSimdGroupSimpleRingBuf => simdgroupsimple_ringbuf_codec,

    /// SIMD-accelerated [`CppNewPFor`].
    CppSimdNewPFor => simdnewpfor_codec,

    /// SIMD-accelerated [`CppOptPFor`].
    CppSimdOptPFor => simdoptpfor_codec,

    /// SIMD-accelerated [`CppPFor`].
    CppSimdPFor => simdpfor_codec,

    /// SIMD-accelerated [`CppSimplePFor`].
    CppSimdSimplePFor => simdsimplepfor_codec,

    /// Simple-16 encoding scheme.
    CppSimple16 => simple16_codec,

    /// Simple-8b encoding scheme.
    CppSimple8b => simple8b_codec,

    /// Simple-8b encoding with run-length encoding.
    CppSimple8bRle => simple8b_rle_codec,

    /// Simple-9 encoding scheme.
    CppSimple9 => simple9_codec,

    /// Simple-9 encoding with run-length encoding.
    CppSimple9Rle => simple9_rle_codec,

    /// Simple Patched Frame of Reference ([`CppPFor`]) codec.
    CppSimplePFor => simplepfor_codec,

    // CppSnappy => snappy_codec,  // Conditional with #ifdef

    /// [`StreamVByte`](https://github.com/lemire/streamvbyte) encoding for fast variable-byte compression.
    CppStreamVByte => streamvbyte_codec,

    /// Standard variable-byte encoding.
    CppVByte => vbyte_codec,

    /// Variable-length integer encoding.
    CppVarInt => varint_codec,

    // CppVarIntG8iu => varintg8iu_codec,  // Conditional with #ifdef

    /// Group Variable-length integer encoding with optimizations.
    CppVarIntGb => varintgb_codec,

    // CppVsEncoding => vsencoding_codec,  // This is leaking memory
}

/// Adds `BlockCodec64` impl for codecs that support 64-bit integers.
macro_rules! implement_cpp_codecs_64 {
    ($($name:ident => $ffi:ident ,)*) => {
        $(
            impl BlockCodec64 for $name {
                fn encode64(&mut self, input: &[u64], out: &mut Vec<u32>) -> Result<(), FastPForError> {
                    encode64_to_vec_ffi(&self.0, input, out)
                }
                fn decode64(&mut self, input: &[u32], out: &mut Vec<u64>) -> Result<(), FastPForError> {
                    decode64_to_vec_ffi(&self.0, input, out)
                }
            }
        )*
    };
}

implement_cpp_codecs_64! {
    CppFastPFor128 => fastpfor128_codec,
    CppFastPFor256 => fastpfor256_codec,
    CppVarInt => varint_codec,
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::codec::{AnyLenCodec, BlockCodec64};
    use crate::cpp::codecs::{CppFastPFor128, CppFastPFor256, CppVByte, CppVarInt};

    pub fn roundtrip_32(codec: &mut (impl AnyLenCodec + ?Sized), input: &[u32]) {
        let mut compressed = Vec::new();
        codec.encode(input, &mut compressed).unwrap();
        let mut decoded = Vec::new();
        codec.decode(&compressed, &mut decoded, None).unwrap();
        assert_eq!(decoded, input);
    }

    /// C++ `fastpfor256_codec` returns `CompositeCodec<FastPFor<8>, VariableByte>` — already
    /// any-length. Use it directly; do not wrap in Rust `CompositeCodec`.
    #[test]
    fn test_cpp_fastpfor256_composite_anylen() {
        let mut codec = CppFastPFor256::new();
        roundtrip_32(&mut codec, &[1, 2, 3, 4, 5]);
        let data: Vec<u32> = (0..600).collect();
        roundtrip_32(&mut codec, &data);
    }

    #[test]
    fn test_fastpfor128_anylen() {
        let data: Vec<u32> = (0..128).collect();
        roundtrip_32(&mut CppFastPFor128::new(), &data);
    }

    #[test]
    fn test_fastpfor256_anylen() {
        let data: Vec<u32> = (0..256).collect();
        roundtrip_32(&mut CppFastPFor256::new(), &data);
    }

    #[test]
    fn test_fastpfor256_u64() {
        let input: Vec<u64> = (0..256).collect();
        let mut codec = CppFastPFor256::new();
        let mut compressed = Vec::new();
        codec.encode64(&input, &mut compressed).unwrap();
        let mut decoded = Vec::new();
        codec.decode64(&compressed, &mut decoded).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_varint_u64() {
        let input = vec![1u64, 2, 3, 4, 5];
        let mut codec = CppVarInt::new();
        let mut compressed = Vec::new();
        codec.encode64(&input, &mut compressed).unwrap();
        let mut decoded = Vec::new();
        codec.decode64(&compressed, &mut decoded).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_decode32_empty_input() {
        let mut codec = CppVByte::new();
        let mut out = Vec::new();
        codec.decode(&[], &mut out, None).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn test_decode32_cpp_empty_format() {
        let mut codec = CppFastPFor128::new();
        let mut out = Vec::new();
        codec.decode(&[0u32], &mut out, Some(0)).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn test_decode64_empty_input() {
        let mut codec = CppFastPFor256::new();
        let mut out: Vec<u64> = Vec::new();
        codec.decode64(&[], &mut out).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn test_decode64_empty_format() {
        let mut codec = CppVarInt::new();
        let mut out: Vec<u64> = Vec::new();
        codec.decode64(&[], &mut out).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn test_decode_empty_input() {
        let mut codec = CppFastPFor128::new();
        let mut out = Vec::new();
        codec.decode(&[], &mut out, None).unwrap();
        assert!(out.is_empty());
    }
}
