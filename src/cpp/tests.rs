use crate::cpp::codecs::tests::roundtrip_32;

// Test all codecs compile and do a basic 32-bit roundtrip
macro_rules! test_anylen {
        ($($name:ident),*) => {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn $name() {
                    roundtrip_32(&mut $crate::cpp::$name::new(), &[1u32, 2, 3, 4, 5]);
                }
            )*
        };
    }

test_anylen!(
    BP32Codec,
    CopyCodec,
    FastBinaryPacking8Codec,
    FastBinaryPacking16Codec,
    FastBinaryPacking32Codec,
    FastPFor128Codec,
    FastPFor256Codec,
    MaskedVByteCodec,
    NewPForCodec,
    OptPForCodec,
    PFor2008Codec,
    PForCodec,
    SimdBinaryPackingCodec,
    SimdFastPFor128Codec,
    SimdFastPFor256Codec,
    SimdGroupSimpleCodec,
    SimdGroupSimpleRingBufCodec,
    SimdNewPForCodec,
    SimdOptPForCodec,
    SimdPForCodec,
    SimdSimplePForCodec,
    SimplePForCodec,
    StreamVByteCodec,
    VByteCodec,
    VarIntCodec,
    VarIntGbCodec
);

// Simple-9/16/8b codecs require values that fit in small bit widths and a
// block-aligned count; test them separately with 128 small values.
macro_rules! test_anylen_128 {
        ($($name:ident),*) => {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn $name() {
                    let input: Vec<u32> = (1..=128).collect();
                    roundtrip_32(&mut $crate::cpp::$name::new(), &input);
                }
            )*
        };
    }

// Note: Simple9Rle crashes with heap corruption on various inputs; skip everywhere.
test_anylen_128!(Simple16Codec, Simple8bCodec, Simple9Codec);

// Simple8bRle reinterpret-casts uint32_t* → uint64_t* inside the C++ header,
// which is UB on strict-alignment architectures (ARM64 requires 8-byte alignment
// for 64-bit loads/stores and will SIGSEGV on unaligned access). The codec is
// otherwise correct on x86/x86_64 where unaligned access is handled in hardware.
// Tracked upstream; skip on aarch64 until fixed in the submodule.
// #[cfg(not(target_arch = "aarch64"))]
test_anylen_128!(Simple8bRleCodec);

// Verify Default impl routes through new() for all generated codec types.
macro_rules! test_default {
        ($($name:ident),*) => {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn $name() {
                    let _codec = $crate::cpp::$name::default();
                }
            )*
        };
    }

// Use a distinct prefix to avoid name collisions with test_anylen tests.
mod default_impls {
    test_default!(
        BP32Codec,
        CopyCodec,
        FastBinaryPacking8Codec,
        FastBinaryPacking16Codec,
        FastBinaryPacking32Codec,
        FastPFor128Codec,
        FastPFor256Codec,
        MaskedVByteCodec,
        NewPForCodec,
        OptPForCodec,
        PFor2008Codec,
        PForCodec,
        SimdBinaryPackingCodec,
        SimdFastPFor128Codec,
        SimdFastPFor256Codec,
        SimdGroupSimpleCodec,
        SimdGroupSimpleRingBufCodec,
        SimdNewPForCodec,
        SimdOptPForCodec,
        SimdPForCodec,
        SimdSimplePForCodec,
        Simple16Codec,
        Simple8bCodec,
        Simple8bRleCodec,
        Simple9Codec,
        SimplePForCodec,
        StreamVByteCodec,
        VByteCodec,
        VarIntCodec,
        VarIntGbCodec
    );
}

mod default_impls2 {
    // #[cfg(not(target_arch = "aarch64"))]
    test_default!(Simple9RleCodec);
}
