#![no_main]

use std::fmt::Debug;

use arbitrary::Arbitrary;
use fastpfor::{
    AnyLenCodec, BlockCodec, CompositeCodec, FastPFor, Scalar, Simd, VariableByte, slice_to_blocks,
};
use libfuzzer_sys::fuzz_target;

const MAX_DECODE_WORDS: usize = 4096;

trait Elem: Copy + Debug + PartialEq {
    const BITS: u8;
    fn from_u64(v: u64) -> Self;
}

impl Elem for u32 {
    const BITS: u8 = 32;
    #[expect(clippy::cast_possible_truncation, reason = "keeps the low 32 bits")]
    fn from_u64(v: u64) -> Self {
        v as Self
    }
}

impl Elem for u64 {
    const BITS: u8 = 64;
    fn from_u64(v: u64) -> Self {
        v
    }
}

#[derive(Arbitrary, Debug)]
struct Block {
    base_bits: u8,
    exception_bits: u8,
    exception_positions: Vec<u8>,
    seed: u64,
}

#[derive(Arbitrary, Debug)]
enum Data<T> {
    Raw(Vec<T>),
    Shaped { blocks: Vec<Block>, tail: Vec<T> },
}

#[derive(Arbitrary, Debug)]
enum Mutation {
    FlipBit { at: u16, bit: u8 },
    Set { at: u16, value: u32 },
    Truncate { at: u16 },
}

#[derive(Arbitrary, Debug)]
enum Op<T> {
    Encode(Data<T>),
    EncodeBlocks(Data<T>),
    DecodeCorrupted {
        data: Data<T>,
        mutations: Vec<Mutation>,
    },
    Decode(Vec<u32>),
    DecodeBlocks {
        words: Vec<u32>,
        expected_len: Option<u32>,
    },
}

#[derive(Arbitrary, Debug)]
enum Case {
    Narrow128(Vec<Op<u32>>),
    Narrow256(Vec<Op<u32>>),
    Wide128(Vec<Op<u64>>),
    Wide256(Vec<Op<u64>>),
}

#[derive(Arbitrary, Debug)]
struct Input {
    page_blocks: u8,
    case: Case,
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn random_bits(state: &mut u64, bits: u8) -> u64 {
    match bits {
        0 => 0,
        64.. => xorshift(state),
        b => xorshift(state) & ((1 << b) - 1),
    }
}

fn materialize<T: Elem>(data: &Data<T>, block_size: usize) -> Vec<T> {
    match data {
        Data::Raw(values) => values.clone(),
        Data::Shaped { blocks, tail } => {
            let mut values = Vec::with_capacity(blocks.len() * block_size + tail.len());
            for block in blocks {
                let mut state = block.seed | 1;
                let base_bits = block.base_bits % (T::BITS + 1);
                let exception_bits = base_bits.max(block.exception_bits % (T::BITS + 1));
                let start = values.len();
                values.extend(
                    (0..block_size).map(|_| T::from_u64(random_bits(&mut state, base_bits))),
                );
                for &pos in &block.exception_positions {
                    values[start + usize::from(pos) % block_size] =
                        T::from_u64(random_bits(&mut state, exception_bits));
                }
            }
            values.extend_from_slice(tail);
            values
        }
    }
}

fn mutate(words: &mut Vec<u32>, mutations: &[Mutation]) {
    for mutation in mutations {
        if words.is_empty() {
            return;
        }
        match *mutation {
            Mutation::FlipBit { at, bit } => {
                let i = usize::from(at) % words.len();
                words[i] ^= 1 << (bit % 32);
            }
            Mutation::Set { at, value } => {
                let i = usize::from(at) % words.len();
                words[i] = value;
            }
            Mutation::Truncate { at } => words.truncate(usize::from(at) % words.len()),
        }
    }
}

fn encode<C: AnyLenCodec>(codec: &mut C, data: &[C::Elem]) -> Vec<u32> {
    let mut out = Vec::new();
    codec.encode(data, &mut out).expect("encode failed");
    out
}

fn decode<C: AnyLenCodec>(codec: &mut C, input: &[u32]) -> Result<Vec<C::Elem>, String> {
    let mut out = Vec::new();
    codec
        .decode(input, &mut out, None)
        .map(|()| out)
        .map_err(|e| format!("{e:?}"))
}

fn decode_blocks<C: BlockCodec>(
    codec: &mut C,
    input: &[u32],
    expected_len: Option<u32>,
) -> Result<(usize, Vec<C::Elem>), String> {
    let mut out = Vec::new();
    codec
        .decode_blocks(input, expected_len, &mut out)
        .map(|consumed| (consumed, out))
        .map_err(|e| format!("{e:?}"))
}

struct Pair<A, B> {
    scalar: A,
    simd: B,
}

fn check<T, S, V, BS, BV>(
    mut any_len: Pair<S, V>,
    mut blocks: Pair<BS, BV>,
    block_size: usize,
    ops: &[Op<T>],
) where
    T: Elem,
    S: AnyLenCodec<Elem = T>,
    V: AnyLenCodec<Elem = T>,
    BS: BlockCodec<Elem = T>,
    BV: BlockCodec<Elem = T, Block = BS::Block>,
{
    for op in ops {
        match op {
            Op::Encode(data) => {
                let data = materialize(data, block_size);
                let scalar_enc = encode(&mut any_len.scalar, &data);
                let simd_enc = encode(&mut any_len.simd, &data);
                assert_eq!(simd_enc, scalar_enc, "SIMD and scalar encodings differ");
                assert_eq!(decode(&mut any_len.simd, &scalar_enc).as_ref(), Ok(&data));
                assert_eq!(decode(&mut any_len.scalar, &simd_enc).as_ref(), Ok(&data));
                assert_eq!(decode(&mut any_len.simd, &simd_enc).as_ref(), Ok(&data));
            }
            Op::EncodeBlocks(data) => {
                let data = materialize(data, block_size);
                let (aligned, _) = slice_to_blocks::<BS>(&data);
                let mut scalar_enc = Vec::new();
                blocks
                    .scalar
                    .encode_blocks(aligned, &mut scalar_enc)
                    .expect("scalar encode_blocks failed");
                let mut simd_enc = Vec::new();
                blocks
                    .simd
                    .encode_blocks(aligned, &mut simd_enc)
                    .expect("SIMD encode_blocks failed");
                assert_eq!(
                    simd_enc, scalar_enc,
                    "SIMD and scalar block encodings differ"
                );
                let expected_len = Some(u32::try_from(aligned.len() * block_size).unwrap());
                let expected = Ok((
                    scalar_enc.len(),
                    data[..aligned.len() * block_size].to_vec(),
                ));
                assert_eq!(
                    decode_blocks(&mut blocks.simd, &scalar_enc, expected_len),
                    expected
                );
                assert_eq!(
                    decode_blocks(&mut blocks.scalar, &simd_enc, expected_len),
                    expected
                );
            }
            Op::DecodeCorrupted { data, mutations } => {
                let data = materialize(data, block_size);
                let mut words = encode(&mut any_len.scalar, &data);
                mutate(&mut words, mutations);
                assert_eq!(
                    decode(&mut any_len.simd, &words),
                    decode(&mut any_len.scalar, &words),
                    "SIMD and scalar disagree on corrupted input"
                );
            }
            Op::Decode(words) => {
                let words = &words[..words.len().min(MAX_DECODE_WORDS)];
                assert_eq!(
                    decode(&mut any_len.simd, words),
                    decode(&mut any_len.scalar, words),
                    "SIMD and scalar disagree on arbitrary input"
                );
            }
            Op::DecodeBlocks {
                words,
                expected_len,
            } => {
                let words = &words[..words.len().min(MAX_DECODE_WORDS)];
                let max_len = BS::max_decompressed_len(words.len());
                let expected_len = expected_len.filter(|&n| n as usize <= max_len);
                assert_eq!(
                    decode_blocks(&mut blocks.simd, words, expected_len),
                    decode_blocks(&mut blocks.scalar, words, expected_len),
                    "SIMD and scalar disagree on arbitrary block input"
                );
            }
        }
    }
}

macro_rules! block_codec {
    ($n:literal, $t:ty, $k:ty, $page_blocks:expr) => {
        if $page_blocks == 0 {
            FastPFor::<$n, $t, $k>::default()
        } else {
            FastPFor::<$n, $t, $k>::new(u32::from($page_blocks) * $n)
                .expect("page size is a multiple of the block size")
        }
    };
}

macro_rules! run {
    ($n:literal, $t:ty, $page_blocks:expr, $ops:expr) => {
        check(
            Pair {
                scalar: CompositeCodec::new(
                    block_codec!($n, $t, Scalar, $page_blocks),
                    VariableByte::<$t>::new(),
                ),
                simd: CompositeCodec::new(
                    block_codec!($n, $t, Simd, $page_blocks),
                    VariableByte::<$t>::new(),
                ),
            },
            Pair {
                scalar: block_codec!($n, $t, Scalar, $page_blocks),
                simd: block_codec!($n, $t, Simd, $page_blocks),
            },
            $n,
            $ops,
        )
    };
}

fuzz_target!(|input: Input| {
    let page_blocks = input.page_blocks;
    match &input.case {
        Case::Narrow128(ops) => run!(128, u32, page_blocks, ops),
        Case::Narrow256(ops) => run!(256, u32, page_blocks, ops),
        Case::Wide128(ops) => run!(128, u64, page_blocks, ops),
        Case::Wide256(ops) => run!(256, u64, page_blocks, ops),
    }
});
