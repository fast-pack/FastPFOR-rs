use std::io::Cursor;

use crate::bytebuffer::ByteBuffer;
use crate::cursor::IncrementCursor;
use crate::integer_compression::helpers::{extract7bits, extract_7bits_maskless};
use crate::{FastPForError, FastPForResult, Integer, Skippable};

#[derive(Debug)]
pub struct VariableByte;

// Implemented for consistency with other codecs
impl VariableByte {
    pub fn new() -> VariableByte {
        VariableByte
    }
}

// Implemented for consistency with other codecs
impl Default for VariableByte {
    fn default() -> Self {
        VariableByte::new()
    }
}

impl Skippable for VariableByte {
    fn headless_compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        if input_length == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        let mut buf = ByteBuffer::new(input_length * 8);
        for k in input_offset.position()..(input_offset.position() + input_length as u64) {
            let val = input[k as usize] as i64;
            if val < (1 << 7) {
                buf.put((val | (1 << 7)) as u8);
            } else if val < (1 << 14) {
                buf.put(extract7bits(0, val));
                buf.put(extract_7bits_maskless(1, val) | (1 << 7));
            } else if val < (1 << 21) {
                buf.put(extract7bits(0, val));
                buf.put(extract7bits(1, val));
                buf.put(extract_7bits_maskless(2, val) | (1 << 7));
            } else if val < (1 << 28) {
                buf.put(extract7bits(0, val));
                buf.put(extract7bits(1, val));
                buf.put(extract7bits(2, val));
                buf.put(extract_7bits_maskless(3, val) | (1 << 7));
            } else {
                buf.put(extract7bits(0, val));
                buf.put(extract7bits(1, val));
                buf.put(extract7bits(2, val));
                buf.put(extract7bits(3, val));
                buf.put(extract_7bits_maskless(4, val) | (1 << 7));
            }
        }
        while buf.position() % 4 != 0 {
            buf.put(0);
        }
        let length = buf.position();
        buf.flip();
        let ibuf = buf.as_int_buffer();
        ibuf.get(
            output,
            output_offset.position() as usize,
            (length / 4) as usize,
        );
        output_offset.add(length / 4);
        input_offset.add(input_length);

        FastPForResult::Ok(())
    }

    #[expect(unused_variables)]
    fn headless_uncompress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
        num: u32,
    ) -> FastPForResult<()> {
        FastPForResult::Err(FastPForError::UnsupportedOperationError(
            "Unimplemented".to_string(),
        ))
    }
}

impl Integer<u32> for VariableByte {
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
        let mut s = 0;
        let mut val = 0;
        let mut p = input_offset.position() as u32;
        let final_p = input_offset.position() as u32 + input_length;
        let mut tmp_outpos = output_offset.position();
        let mut shift = 0;
        let mut v = 0;

        while p < final_p {
            val = input[p as usize];
            let c = val >> s;
            s += 8;
            p += s >> 5;
            s &= 31;
            shift = shift.min(31); // for safety
            v += (c & 127) << shift;
            if (c & 128) == 128 {
                output[tmp_outpos as usize] = v;
                tmp_outpos += 1;
                v = 0;
                shift = 0;
            } else {
                shift += 7;
            }
        }
        output_offset.set_position(tmp_outpos);
        input_offset.add(input_length);

        FastPForResult::Ok(())
    }
}

impl Integer<i8> for VariableByte {
    fn compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [i8],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        if input_length == 0 {
            // Return early if there is no data to compress
            return Ok(());
        }
        let mut out_pos_tmp = output_offset.position();
        for k in input_offset.position() as u32..(input_offset.position() as u32 + input_length) {
            let val = input[k as usize] as i64;
            if val < (1 << 7) {
                output[out_pos_tmp as usize] = (val | (1 << 7)) as i8;
                out_pos_tmp += 1;
            } else if val < (1 << 14) {
                output[out_pos_tmp as usize] = extract7bits(0, val) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (extract_7bits_maskless(1, val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
            } else if val < (1 << 21) {
                output[out_pos_tmp as usize] = extract7bits(0, val) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = extract7bits(1, val) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (extract_7bits_maskless(2, val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
            } else if val < (1 << 28) {
                output[out_pos_tmp as usize] = extract7bits(0, val) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = extract7bits(1, val) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = extract7bits(2, val) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (extract_7bits_maskless(3, val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
            } else {
                output[out_pos_tmp as usize] = extract7bits(0, val) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = extract7bits(1, val) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = extract7bits(2, val) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = extract7bits(3, val) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (extract_7bits_maskless(4, val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
            }
        }
        output_offset.set_position(out_pos_tmp + input_length as u64);
        input_offset.add(input_length);
        FastPForResult::Ok(())
    }
    fn uncompress(
        &mut self,
        input: &[i8],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        let mut p = input_offset.position() as u32;
        let final_p = input_offset.position() as u32 + input_length;
        let mut tmp_outpos = output_offset.position();
        let mut v = 0;
        while p < final_p {
            v = input[p as usize] as i32;
            if input[p as usize] < 0 {
                p += 1;
                continue;
            }
            v |= (input[p as usize + 1] as i32) << 7;
            if input[p as usize + 1] < 0 {
                p += 2;
                continue;
            }
            v |= (input[p as usize + 2] as i32) << 14;
            if input[p as usize + 2] < 0 {
                p += 3;
                continue;
            }
            v |= (input[p as usize + 3] as i32) << 21;
            if input[p as usize + 3] < 0 {
                p += 4;
                continue;
            }
            v |= (input[p as usize + 4] as i32) << 28;
            p += 5;
            output[tmp_outpos as usize] = v as u32;
            tmp_outpos += 1;
        }
        output_offset.set_position(tmp_outpos);
        input_offset.add(input_length);
        FastPForResult::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_int_array() {
        let input: Vec<u32> = vec![];
        let mut output: Vec<u32> = vec![];
        let mut vb = VariableByte::new();
        vb.compress(
            &input,
            0,
            &mut Cursor::new(0),
            &mut output,
            &mut Cursor::new(0),
        )
        .expect("Failed to compress");
        let mut answer: Vec<u32> = vec![];
        vb.uncompress(
            &output,
            output.len() as u32,
            &mut Cursor::new(0),
            &mut answer,
            &mut Cursor::new(0),
        )
        .expect("Failed to uncompress");
        assert_eq!(input, answer);
    }

    #[test]
    fn test_empty_byte_array() {
        let input: Vec<u32> = vec![];
        let mut output: Vec<i8> = vec![];
        let mut vb = VariableByte::new();
        vb.compress(
            &input,
            0,
            &mut Cursor::new(0),
            &mut output,
            &mut Cursor::new(0),
        )
        .expect("Failed to compress");
        let mut answer: Vec<u32> = vec![];
        vb.uncompress(
            &output,
            output.len() as u32,
            &mut Cursor::new(0),
            &mut answer,
            &mut Cursor::new(0),
        )
        .expect("Failed to uncompress");
        assert_eq!(input, answer);
    }
}
