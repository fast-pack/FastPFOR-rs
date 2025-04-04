#![cfg(feature = "rust")]

use std::io::Cursor;

use fastpfor::rust::{
    Composition, FastPFOR, FastPForResult, Integer, JustCopy, VariableByte, BLOCK_SIZE_128,
    DEFAULT_PAGE_SIZE,
};

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
