#![cfg(all(feature = "rust", feature = "cpp"))]

use std::io::Cursor;

use fastpfor::rust::{
    Composition, FastPFOR, FastPForResult, Integer, JustCopy, VariableByte, BLOCK_SIZE_128,
    DEFAULT_PAGE_SIZE,
};
use rand::rngs::StdRng;
use rand::{RngExt as _, SeedableRng as _};

pub enum TestCodec {
    VariableByte(VariableByte, String),
    JustCopy(JustCopy, String),
    Composition(Box<Composition>, String),
}

impl TestCodec {
    pub fn name(&self) -> &str {
        match self {
            TestCodec::Composition(_, name)
            | TestCodec::JustCopy(_, name)
            | TestCodec::VariableByte(_, name) => name,
        }
    }
    pub fn compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        match self {
            TestCodec::VariableByte(vb, _) => {
                vb.compress(input, input_length, input_offset, output, output_offset)
            }
            TestCodec::JustCopy(jc, _) => {
                jc.compress(input, input_length, input_offset, output, output_offset)
            }
            TestCodec::Composition(comp, _) => {
                comp.compress(input, input_length, input_offset, output, output_offset)
            }
        }
    }

    pub fn uncompress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        match self {
            TestCodec::VariableByte(vb, _) => {
                vb.uncompress(input, input_length, input_offset, output, output_offset)
            }
            TestCodec::JustCopy(jc, _) => {
                jc.uncompress(input, input_length, input_offset, output, output_offset)
            }
            TestCodec::Composition(comp, _) => {
                comp.uncompress(input, input_length, input_offset, output, output_offset)
            }
        }
    }
}

pub fn get_codecs() -> Vec<TestCodec> {
    vec![
        TestCodec::VariableByte(VariableByte::new(), "VariableByte".to_string()),
        TestCodec::JustCopy(JustCopy::new(), "JustCopy".to_string()),
        TestCodec::Composition(
            Box::new(Composition::new(FastPFOR::default(), VariableByte::new())),
            "FastPFOR + VariableByte".to_string(),
        ),
        TestCodec::Composition(
            Box::new(Composition::new(
                FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128),
                VariableByte::new(),
            )),
            "FastPFOR + VariableByte".to_string(),
        ),
    ]
}

pub fn test_input_sizes() -> Vec<usize> {
    (1..=8).map(|exp| (1usize << exp) * 128).collect()
}

pub fn get_test_cases(n: usize) -> Vec<Vec<u32>> {
    let mut rng = StdRng::seed_from_u64(14);

    vec![
        // Zeroes
        vec![0u32; n],
        // Same non-zero
        vec![14u32; n],
        // Ascending values
        (0..n).map(|i| i as u32).collect::<Vec<u32>>(),
        // Descending values
        (0..n).rev().map(|i| i as u32).collect::<Vec<u32>>(),
        // Bit-flipping pattern
        (0..n)
            .map(|i| ((i as u32) * 32) ^ ((i as u32) >> 1))
            .collect::<Vec<u32>>(),
        // Alternating large and small values
        (0..n)
            .map(|i| {
                let ui = i as u32;
                if ui % 2 == 0 {
                    1 << 30
                } else {
                    3
                }
            })
            .collect::<Vec<u32>>(),
        // Random u32 values
        (0..n)
            .map(|_| rng.random_range(0..(1 << 31)))
            .collect::<Vec<u32>>(),
        // Spike in the middle
        (0..n)
            .map(|i| if i == n / 2 { u32::MAX } else { 1 })
            .collect::<Vec<u32>>(),
        // An empty vector
        Vec::new(),
    ]
}
