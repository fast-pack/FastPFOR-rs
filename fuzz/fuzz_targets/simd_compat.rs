#![no_main]

use std::fmt::Debug;

use arbitrary::Arbitrary;
use fastpfor::{AnyLenCodec, CompositeCodec, FastPFor, Scalar, Simd, VariableByte};
use libfuzzer_sys::fuzz_target;

const MAX_DECODE_WORDS: usize = 4096;

#[derive(Arbitrary, Debug)]
enum Op<T> {
    Encode(Vec<T>),
    Decode(Vec<u32>),
}

#[derive(Arbitrary, Debug)]
enum Codec {
    Narrow128,
    Narrow256,
    Wide128,
    Wide256,
}

#[derive(Arbitrary, Debug)]
struct Input {
    codec: Codec,
    page_blocks: u8,
    narrow: Vec<Op<u32>>,
    wide: Vec<Op<u64>>,
}

fn decode<C: AnyLenCodec>(codec: &mut C, input: &[u32]) -> Result<Vec<C::Elem>, String> {
    let mut out = Vec::new();
    codec
        .decode(input, &mut out, None)
        .map(|()| out)
        .map_err(|e| format!("{e:?}"))
}

macro_rules! codec {
    ($n:literal, $t:ty, $k:ty, $page_blocks:expr) => {{
        let block = if $page_blocks == 0 {
            FastPFor::<$n, $t, $k>::default()
        } else {
            FastPFor::<$n, $t, $k>::new(u32::from($page_blocks) * $n)
                .expect("page size is a multiple of the block size")
        };
        CompositeCodec::new(block, VariableByte::<$t>::new())
    }};
}

fn check<T, S, V>(mut scalar: S, mut simd: V, ops: &[Op<T>])
where
    T: Copy + Debug + PartialEq,
    S: AnyLenCodec<Elem = T>,
    V: AnyLenCodec<Elem = T>,
{
    for op in ops {
        match op {
            Op::Encode(data) => {
                let mut scalar_enc = Vec::new();
                scalar
                    .encode(data, &mut scalar_enc)
                    .expect("scalar encode failed");
                let mut simd_enc = Vec::new();
                simd.encode(data, &mut simd_enc)
                    .expect("SIMD encode failed");
                assert_eq!(simd_enc, scalar_enc, "SIMD and scalar encodings differ");

                let from_scalar =
                    decode(&mut simd, &scalar_enc).expect("SIMD decode of scalar output failed");
                assert_eq!(&from_scalar, data, "SIMD decode of scalar output mismatch");
                let from_simd =
                    decode(&mut scalar, &simd_enc).expect("scalar decode of SIMD output failed");
                assert_eq!(&from_simd, data, "scalar decode of SIMD output mismatch");
                let own = decode(&mut simd, &simd_enc).expect("SIMD roundtrip failed");
                assert_eq!(&own, data, "SIMD roundtrip mismatch");
            }
            Op::Decode(words) => {
                let words = &words[..words.len().min(MAX_DECODE_WORDS)];
                assert_eq!(
                    decode(&mut simd, words),
                    decode(&mut scalar, words),
                    "SIMD and scalar disagree on arbitrary input"
                );
            }
        }
    }
}

fuzz_target!(|input: Input| {
    let p = input.page_blocks;
    match input.codec {
        Codec::Narrow128 => check(
            codec!(128, u32, Scalar, p),
            codec!(128, u32, Simd, p),
            &input.narrow,
        ),
        Codec::Narrow256 => check(
            codec!(256, u32, Scalar, p),
            codec!(256, u32, Simd, p),
            &input.narrow,
        ),
        Codec::Wide128 => check(
            codec!(128, u64, Scalar, p),
            codec!(128, u64, Simd, p),
            &input.wide,
        ),
        Codec::Wide256 => check(
            codec!(256, u64, Scalar, p),
            codec!(256, u64, Simd, p),
            &input.wide,
        ),
    }
});
