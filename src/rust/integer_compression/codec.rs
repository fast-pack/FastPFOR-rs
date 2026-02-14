use std::io::Cursor;

use crate::rust::{FastPFOR, FastPForResult, Integer, JustCopy, VariableByte};
use crate::CodecToSlice;

/// Type-erased wrapper for compression codecs.
///
/// Allows different codec types to be used interchangeably through a unified interface.
pub enum Codec {
    /// [`FastPFOR`] compression codec
    FastPFor(Box<FastPFOR>),
    /// [`VariableByte`] compression codec
    VariableByte(VariableByte),
    /// Pass-through codec (no compression)
    JustCopy(JustCopy),
}

impl Integer<u32> for Codec {
    fn compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFor(fastpfor) => {
                fastpfor.compress(input, input_length, input_offset, output, output_offset)
            }
            Codec::VariableByte(vb) => {
                vb.compress(input, input_length, input_offset, output, output_offset)
            }
            Codec::JustCopy(jc) => {
                jc.compress(input, input_length, input_offset, output, output_offset)
            }
        }
    }

    fn uncompress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        match self {
            Codec::FastPFor(fastpfor) => {
                fastpfor.uncompress(input, input_length, input_offset, output, output_offset)
            }
            Codec::VariableByte(vb) => {
                vb.uncompress(input, input_length, input_offset, output, output_offset)
            }
            Codec::JustCopy(jc) => {
                jc.uncompress(input, input_length, input_offset, output, output_offset)
            }
        }
    }
}

impl CodecToSlice<u32> for Codec {
    type Error = crate::rust::FastPForError;

    fn compress_to_slice<'out>(
        &mut self,
        input: &[u32],
        output: &'out mut [u32],
    ) -> Result<&'out [u32], Self::Error> {
        let mut output_offset = Cursor::new(0);
        let input_length = input
            .len()
            .try_into()
            .map_err(|_| Self::Error::InvalidInputLength(input.len()))?;

        self.compress(
            input,
            input_length,
            &mut Cursor::new(0),
            output,
            &mut output_offset,
        )?;

        let written = output_offset.position() as usize;
        Ok(&output[..written])
    }

    fn decompress_to_slice<'out>(
        &mut self,
        input: &[u32],
        output: &'out mut [u32],
    ) -> Result<&'out [u32], Self::Error> {
        let mut output_offset = Cursor::new(0);
        let input_length: u32 = input
            .len()
            .try_into()
            .map_err(|_| Self::Error::InvalidInputLength(input.len()))?;

        self.uncompress(
            input,
            input_length,
            &mut Cursor::new(0),
            output,
            &mut output_offset,
        )?;

        let written = output_offset.position() as usize;
        Ok(&output[..written])
    }
}

impl From<FastPFOR> for Codec {
    fn from(fastpfor: FastPFOR) -> Self {
        Codec::FastPFor(Box::new(fastpfor))
    }
}

impl From<VariableByte> for Codec {
    fn from(vb: VariableByte) -> Self {
        Codec::VariableByte(vb)
    }
}

impl From<JustCopy> for Codec {
    fn from(jc: JustCopy) -> Self {
        Codec::JustCopy(jc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_compress_to_slice() {
        let data = vec![1, 2, 3, 4, 5];
        let mut rust_codec = Codec::from(VariableByte::new());
        let mut compressed = vec![0u32; data.len() * 4];

        let compressed_len = {
            let result = rust_codec
                .compress_to_slice(&data, &mut compressed)
                .unwrap();
            result.len()
        };

        let mut decompressed = vec![0u32; data.len()];
        let result = rust_codec
            .decompress_to_slice(&compressed[..compressed_len], &mut decompressed)
            .unwrap();
        assert_eq!(result, &data[..]);
    }
}
