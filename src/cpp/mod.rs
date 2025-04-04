use cxx::UniquePtr;

#[cxx::bridge(namespace = "FastPForLib")]
mod ffi {
    unsafe extern "C++" {
        include!("fastpfor_bridge.h");

        type IntegerCODEC;

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

        // Encode methods: returns number of output values

        fn codec_encode32(
            codec: &UniquePtr<IntegerCODEC>,
            input: &[u32],
            output: &mut [u32],
        ) -> Result<usize>;

        fn codec_encode64(
            codec: &UniquePtr<IntegerCODEC>,
            input: &[u64],
            output: &mut [u32],
        ) -> Result<usize>;

        // Decode methods: returns consumed input values

        fn codec_decode32(
            codec: &UniquePtr<IntegerCODEC>,
            input: &[u32],
            output: &mut [u32],
        ) -> Result<usize>;

        fn codec_decode64(
            codec: &UniquePtr<IntegerCODEC>,
            input: &[u32],
            output: &mut [u64],
        ) -> Result<usize>;
    }
}

trait CodecWrapper {
    fn codec(&self) -> &UniquePtr<ffi::IntegerCODEC>;
}

#[expect(private_bounds)]
pub trait Codec32: CodecWrapper {
    /// Encode a slice of 32-bit integers.
    fn encode32<'out>(
        &self,
        input: &[u32],
        output: &'out mut [u32],
    ) -> Result<&'out mut [u32], cxx::Exception> {
        let n = ffi::codec_encode32(self.codec(), input, output)?;
        Ok(&mut output[..n])
    }

    /// Decode a slice of 32-bit integers.
    fn decode32<'out>(
        &self,
        input: &[u32],
        output: &'out mut [u32],
    ) -> Result<&'out mut [u32], cxx::Exception> {
        let n = ffi::codec_decode32(self.codec(), input, output)?;
        Ok(&mut output[..n])
    }
}

#[expect(private_bounds)]
pub trait Codec64: CodecWrapper {
    /// Encode a slice of 64-bit integers.
    fn encode64<'out>(
        &self,
        input: &[u64],
        output: &'out mut [u32],
    ) -> Result<&'out mut [u32], cxx::Exception> {
        let n = ffi::codec_encode64(self.codec(), input, output)?;
        Ok(&mut output[..n])
    }

    /// Decode a slice of 64-bit integers.
    fn decode64<'out>(
        &self,
        input: &[u32],
        output: &'out mut [u64],
    ) -> Result<&'out mut [u64], cxx::Exception> {
        let n = ffi::codec_decode64(self.codec(), input, output)?;
        Ok(&mut output[..n])
    }
}

macro_rules! implement_codecs {
    ($($name:ident $(@ $is_64:literal)? => $ffi:ident , )*) => {
        $(
            pub struct $name(UniquePtr<ffi::IntegerCODEC>);

            impl $name {
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
                #[allow(non_snake_case)]
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

implement_codecs! {
    BP32Codec => BP32_codec,
    CopyCodec => copy_codec,
    FastBinaryPacking8Codec => fastbinarypacking8_codec,
    FastPFor128Codec @ 64 => fastpfor128_codec,
    FastPFor256Codec @ 64 => fastpfor256_codec,
    FastBinaryPacking16Codec => fastbinarypacking16_codec,
    FastBinaryPacking32Codec => fastbinarypacking32_codec,
    MaskedVByteCodec => maskedvbyte_codec,
    NewPForCodec => newpfor_codec,
    OptPForCodec => optpfor_codec,
    PFor2008Codec => pfor2008_codec,
    PForCodec => pfor_codec,
    SimdBinaryPackingCodec => simdbinarypacking_codec,
    SimdFastPFor128Codec => simdfastpfor128_codec,
    SimdFastPFor256Codec => simdfastpfor256_codec,
    SimdGroupSimpleCodec => simdgroupsimple_codec,
    SimdGroupSimpleRingBufCodec => simdgroupsimple_ringbuf_codec,
    SimdNewPForCodec => simdnewpfor_codec,
    SimdOptPForCodec => simdoptpfor_codec,
    SimdPForCodec => simdpfor_codec,
    SimdSimplePForCodec => simdsimplepfor_codec,
    Simple16Codec => simple16_codec,
    Simple8bCodec => simple8b_codec,
    Simple8bRleCodec => simple8b_rle_codec,
    Simple9Codec => simple9_codec,
    Simple9RleCodec => simple9_rle_codec,
    SimplePForCodec => simplepfor_codec,
    // SnappyCodec => snappy_codec,  // Conditional with #ifdef
    StreamVByteCodec => streamvbyte_codec,
    VByteCodec => vbyte_codec,
    VarIntCodec @ 64 => varint_codec,
    // VarIntG8iuCodec => varintg8iu_codec,  // Conditional with #ifdef
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
