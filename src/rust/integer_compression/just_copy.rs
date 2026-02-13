use std::io::Cursor;

use crate::rust::{FastPForError, FastPForResult, Integer, Skippable};

#[derive(Debug)]
pub struct JustCopy;

impl JustCopy {
    pub fn new() -> Self {
        JustCopy
    }
}

impl Default for JustCopy {
    fn default() -> Self {
        JustCopy::new()
    }
}

impl Skippable for JustCopy {
    fn headless_compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        let start_input = input_offset.position() as usize;
        let end_input = start_input + input_length as usize;
        let start_output = output_offset.position() as usize;
        let end_output = start_output + input_length as usize;

        if end_input > input.len() {
            return Err(FastPForError::NotEnoughData);
        }
        if end_output > output.len() {
            return Err(FastPForError::OutputBufferTooSmall);
        }

        output[start_output..end_output].copy_from_slice(&input[start_input..end_input]);

        input_offset.set_position(end_input as u64);
        output_offset.set_position(end_output as u64);

        Ok(())
    }

    fn headless_uncompress(
        &mut self,
        input: &[u32],
        #[expect(unused)] input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
        num: u32,
    ) -> FastPForResult<()> {
        let start_input = input_offset.position() as usize;
        let end_input = start_input + num as usize;
        let start_output = output_offset.position() as usize;
        let end_output = start_output + num as usize;

        if end_input > input.len() {
            return Err(FastPForError::NotEnoughData);
        }
        if end_output > output.len() {
            return Err(FastPForError::OutputBufferTooSmall);
        }

        output[start_output..end_output].copy_from_slice(&input[start_input..end_input]);

        input_offset.set_position(end_input as u64);
        output_offset.set_position(end_output as u64);
        Ok(())
    }
}

impl Integer<u32> for JustCopy {
    fn compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        self.headless_compress(input, input_length, input_offset, output, output_offset)
    }

    fn uncompress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        let start_input = input_offset.position() as usize;
        let end_input = start_input + input_length as usize;
        let start_output = output_offset.position() as usize;
        let end_output = start_output + input_length as usize;

        // Ensure we don't exceed the slice bounds
        if end_input > input.len() {
            return Err(FastPForError::NotEnoughData);
        }
        if end_output > output.len() {
            return Err(FastPForError::OutputBufferTooSmall);
        }

        output[start_output..end_output].copy_from_slice(&input[start_input..end_input]);

        // Update the cursor positions
        input_offset.set_position(end_input as u64);
        output_offset.set_position(end_output as u64);

        Ok(())
    }
}
