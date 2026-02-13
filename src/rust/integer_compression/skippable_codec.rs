use std::io::Cursor;

use crate::rust::{Codec, FastPForResult};

/// Headerless compression/decompression for seekable streams.
///
/// Methods operate without length headers, requiring external length tracking.
/// Useful for random access and pre-sized buffer scenarios.
pub trait Skippable {
    /// Compresses integers without writing length header.
    ///
    /// # Arguments
    /// * `input_length` - Number of integers to compress
    /// * `input_offset` - Read position cursor, advanced by `input_length`
    /// * `output_offset` - Write position cursor, advanced by bytes written
    fn headless_compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()>;

    /// Decompresses integers without reading length header.
    ///
    /// # Arguments
    /// * `input_length` - Compressed data length
    /// * `input_offset` - Read position cursor, advanced by bytes read
    /// * `output_offset` - Write position cursor, advanced by `num`
    /// * `num` - Expected number of integers to decompress
    fn headless_uncompress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
        num: u32,
    ) -> FastPForResult<()>;
}

impl Skippable for Codec {
    fn headless_compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFor(fastpfor) => {
                fastpfor.headless_compress(input, input_length, input_offset, output, output_offset)
            }
            Codec::VariableByte(vb) => {
                vb.headless_compress(input, input_length, input_offset, output, output_offset)
            }
            Codec::JustCopy(jc) => {
                jc.headless_compress(input, input_length, input_offset, output, output_offset)
            }
        }
    }

    fn headless_uncompress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
        num: u32,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFor(fastpfor) => fastpfor.headless_uncompress(
                input,
                input_length,
                input_offset,
                output,
                output_offset,
                num,
            ),
            Codec::VariableByte(vb) => vb.headless_uncompress(
                input,
                input_length,
                input_offset,
                output,
                output_offset,
                num,
            ),
            Codec::JustCopy(jc) => jc.headless_uncompress(
                input,
                input_length,
                input_offset,
                output,
                output_offset,
                num,
            ),
        }
    }
}
