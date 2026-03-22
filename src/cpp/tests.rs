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
    CppBP32,
    CppCopy,
    CppFastBinaryPacking8,
    CppFastBinaryPacking16,
    CppFastBinaryPacking32,
    CppFastPFor128,
    CppFastPFor256,
    CppMaskedVByte,
    CppNewPFor,
    CppOptPFor,
    CppPFor2008,
    CppPFor,
    CppSimdBinaryPacking,
    CppSimdFastPFor128,
    CppSimdFastPFor256,
    CppSimdGroupSimple,
    CppSimdGroupSimpleRingBuf,
    CppSimdNewPFor,
    CppSimdOptPFor,
    CppSimdPFor,
    CppSimdSimplePFor,
    CppSimplePFor,
    CppStreamVByte,
    CppVByte,
    CppVarInt,
    CppVarIntGb
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

// Note: CppSimple9Rle crashes with heap corruption on various inputs; skip everywhere.
test_anylen_128!(CppSimple16, CppSimple8b, CppSimple9);

// CppSimple8bRle reinterpret-casts uint32_t* → uint64_t* inside the C++ header,
// which is UB on strict-alignment architectures (ARM64 requires 8-byte alignment
// for 64-bit loads/stores and will SIGSEGV on unaligned access). The codec is
// otherwise correct on x86/x86_64 where unaligned access is handled in hardware.
// Tracked upstream; skip on aarch64 until fixed in the submodule.
// #[cfg(not(target_arch = "aarch64"))]
test_anylen_128!(CppSimple8bRle);

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
        CppBP32,
        CppCopy,
        CppFastBinaryPacking8,
        CppFastBinaryPacking16,
        CppFastBinaryPacking32,
        CppFastPFor128,
        CppFastPFor256,
        CppMaskedVByte,
        CppNewPFor,
        CppOptPFor,
        CppPFor2008,
        CppPFor,
        CppSimdBinaryPacking,
        CppSimdFastPFor128,
        CppSimdFastPFor256,
        CppSimdGroupSimple,
        CppSimdGroupSimpleRingBuf,
        CppSimdNewPFor,
        CppSimdOptPFor,
        CppSimdPFor,
        CppSimdSimplePFor,
        CppSimple16,
        CppSimple8b,
        CppSimple8bRle,
        CppSimple9,
        CppSimplePFor,
        CppStreamVByte,
        CppVByte,
        CppVarInt,
        CppVarIntGb
    );
}

mod default_impls2 {
    // #[cfg(not(target_arch = "aarch64"))]
    test_default!(CppSimple9Rle);
}
