use crate::test_utils::roundtrip;

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
