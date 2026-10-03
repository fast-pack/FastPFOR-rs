//! Tests of the 64-bit ([`u64`]) `FastPFOR` codecs.
//!
//! The block engine is width-generic: for `u64` its exception bitmap and buffers are 64 bits wide.

#[cfg(test)]
mod tests {
    use crate::codec::BlockCodec64;
    use crate::rust::fastpfor_codec::FastPForCodec;
    use crate::rust::integer_compression::fastpfor::sealed;
    use crate::rust::kernels::Sequential;

    fn roundtrip<const N: usize>(input: &[u64])
    where
        [u64; N]: sealed::BlockSize,
    {
        let mut codec = FastPForCodec::<Sequential, u64, N>::default();
        let mut encoded = Vec::new();
        codec.encode64(input, &mut encoded).unwrap();
        let mut decoded = Vec::new();
        codec.decode64(&encoded, &mut decoded).unwrap();
        assert_eq!(decoded, input, "roundtrip mismatch (N={N})");
    }

    #[test]
    fn empty() {
        roundtrip::<128>(&[]);
        roundtrip::<256>(&[]);
    }

    #[test]
    fn single_value() {
        roundtrip::<128>(&[42]);
        roundtrip::<256>(&[u64::MAX]);
    }

    #[test]
    fn sub_block_tail_only() {
        let data: Vec<u64> = (0..10).collect();
        roundtrip::<256>(&data);
    }

    #[test]
    fn exact_block() {
        let data: Vec<u64> = (0..128).collect();
        roundtrip::<128>(&data);
    }

    #[test]
    fn blocks_with_remainder() {
        let data: Vec<u64> = (0..600).collect();
        roundtrip::<256>(&data);
    }

    #[test]
    fn large_values_and_exceptions() {
        let data: Vec<u64> = (0..1024u32)
            .map(|i| if i % 7 == 0 { 1u64 << 60 } else { u64::from(i) })
            .collect();
        roundtrip::<128>(&data);
    }

    #[test]
    fn full_width_values() {
        let data: Vec<u64> = (0..256u32).map(|i| u64::MAX - u64::from(i)).collect();
        roundtrip::<256>(&data);
    }

    #[test]
    fn spans_multiple_pages() {
        let data: Vec<u64> = (0..70_000u64).map(|i| i.wrapping_mul(0x1_0001)).collect();
        roundtrip::<128>(&data);
    }

    #[cfg(feature = "cpp")]
    mod cpp_parity {
        use super::*;
        use crate::cpp::{CppFastPFor128, CppFastPFor256};

        fn cases() -> Vec<Vec<u64>> {
            vec![
                vec![],
                vec![42],
                vec![u64::MAX],
                (0..10).collect(),
                (0..128).collect(),
                (0..256).collect(),
                (0..600).collect(),
                (0..1024u32)
                    .map(|i| if i % 7 == 0 { 1u64 << 60 } else { u64::from(i) })
                    .collect(),
                (0..256u32).map(|i| u64::MAX - u64::from(i)).collect(),
                (0..5000u64).map(|i| i.wrapping_mul(0x1_0001)).collect(),
            ]
        }

        fn assert_parity<const N: usize>(
            rust: &mut FastPForCodec<Sequential, u64, N>,
            cpp: &mut impl BlockCodec64,
        ) where
            [u64; N]: sealed::BlockSize,
        {
            for data in cases() {
                let mut rust_enc = Vec::new();
                rust.encode64(&data, &mut rust_enc).unwrap();
                let mut cpp_enc = Vec::new();
                cpp.encode64(&data, &mut cpp_enc).unwrap();
                assert_eq!(rust_enc, cpp_enc, "encode64 bytes differ for {data:?}");

                let mut rust_dec = Vec::new();
                rust.decode64(&cpp_enc, &mut rust_dec).unwrap();
                assert_eq!(rust_dec, data, "Rust failed to decode C++ output");

                let mut cpp_dec = Vec::new();
                cpp.decode64(&rust_enc, &mut cpp_dec).unwrap();
                assert_eq!(cpp_dec, data, "C++ failed to decode Rust output");
            }
        }

        #[test]
        fn parity_128() {
            assert_parity(
                &mut FastPForCodec::<Sequential, u64, 128>::default(),
                &mut CppFastPFor128::default(),
            );
        }

        #[test]
        fn parity_256() {
            assert_parity(
                &mut FastPForCodec::<Sequential, u64, 256>::default(),
                &mut CppFastPFor256::default(),
            );
        }
    }
}
