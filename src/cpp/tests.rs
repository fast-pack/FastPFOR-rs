use crate::test_utils::roundtrip;

/// Test all codecs compile and do a basic 32-bit roundtrip
macro_rules! test_anylen {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            #[allow(non_snake_case)]
            fn $name() {
                roundtrip::<$crate::cpp::$name>(&[1u32, 2, 3, 4, 5]);
            }
        )*
    };
}

test_anylen!(
    CppBP32,
    CppCopy,
    CppFastBinaryPacking16,
    CppFastBinaryPacking32,
    CppFastBinaryPacking8,
    CppFastPFor128,
    CppFastPFor256,
    CppMaskedVByte,
    CppNewPFor,
    CppOptPFor,
    CppPFor,
    CppPFor2008,
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
    CppVarIntGb,
);

/// Simple-9/16/8b codecs require values that fit in small bit widths and a
/// block-aligned count; test them separately with 128 small values.
macro_rules! test_anylen_128 {
        ($($name:ident),* $(,)?) => {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn $name() {
                    let input: Vec<u32> = (1..=128).collect();
                    roundtrip::<$crate::cpp::$name>(&input);
                }
            )*
        };
    }

// Note: Simple9Rle crashes with heap corruption on various inputs; skip everywhere.
test_anylen_128!(CppSimple16, CppSimple8b, CppSimple9, CppSimple8bRle);

// Verify Default impl routes through new() for all generated codec types.
macro_rules! test_default {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            #[allow(non_snake_case)]
            fn $name() {
                let _codec = $crate::cpp::$name::default();
            }
        )*
    };
}

/// Use a distinct prefix to avoid name collisions with `test_anylen` tests.
mod default_impls {
    test_default!(
        CppBP32,
        CppCopy,
        CppFastBinaryPacking16,
        CppFastBinaryPacking32,
        CppFastBinaryPacking8,
        CppFastPFor128,
        CppFastPFor256,
        CppMaskedVByte,
        CppNewPFor,
        CppOptPFor,
        CppPFor,
        CppPFor2008,
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
        CppSimple9Rle,
        CppSimplePFor,
        CppStreamVByte,
        CppVByte,
        CppVarInt,
        CppVarIntGb,
    );
}
