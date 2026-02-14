use std::io::Cursor;

use bytes::{Buf as _, BufMut as _, BytesMut};

use crate::rust::cursor::IncrementCursor;
use crate::rust::{FastPForError, FastPForResult, Integer, Skippable};

/// Variable-byte encoding codec for integer compression.
#[derive(Debug)]
pub struct VariableByte;

// Implemented for consistency with other codecs
impl VariableByte {
    /// Creates a new instance
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
        let mut buf = BytesMut::with_capacity(input_length as usize * 8);
        for k in input_offset.position()..(input_offset.position() + u64::from(input_length)) {
            let val = input[k as usize];
            if val < (1 << 7) {
                buf.put_u8((val & 0x7F) as u8);
            } else if val < (1 << 14) {
                buf.put_u8(((val & 0x7F) | (1 << 7)) as u8);
                buf.put_u8((val >> 7) as u8);
            } else if val < (1 << 21) {
                buf.put_u8(((val & 0x7F) | (1 << 7)) as u8);
                buf.put_u8((((val >> 7) & 0x7F) | (1 << 7)) as u8);
                buf.put_u8((val >> 14) as u8);
            } else if val < (1 << 28) {
                buf.put_u8(((val & 0x7F) | (1 << 7)) as u8);
                buf.put_u8((((val >> 7) & 0x7F) | (1 << 7)) as u8);
                buf.put_u8((((val >> 14) & 0x7F) | (1 << 7)) as u8);
                buf.put_u8((val >> 21) as u8);
            } else {
                buf.put_u8(((val & 0x7F) | (1 << 7)) as u8);
                buf.put_u8((((val >> 7) & 0x7F) | (1 << 7)) as u8);
                buf.put_u8((((val >> 14) & 0x7F) | (1 << 7)) as u8);
                buf.put_u8((((val >> 21) & 0x7F) | (1 << 7)) as u8);
                buf.put_u8((val >> 28) as u8);
            }
        }
        while buf.len() % 4 != 0 {
            buf.put_u8(0xFF);
        }
        let length = buf.len();
        let output_position = output_offset.position() as usize;
        for it in output.iter_mut().skip(output_position).take(length / 4) {
            *it = buf.get_u32_le();
        }
        output_offset.add(length as u32 / 4);
        input_offset.add(input_length);

        Ok(())
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
        Err(FastPForError::Unimplemented)
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
        if input_length == 0 {
            return Ok(());
        }

        // Convert u32 array to byte view
        let byte_length = (input_length as usize) * 4;
        let input_start = input_offset.position() as usize;

        // Create a byte slice view of the input
        let input_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(input[input_start..].as_ptr().cast::<u8>(), byte_length)
        };

        let mut byte_pos = 0;
        let mut tmp_outpos = output_offset.position() as usize;

        // Fast path: process while we have at least 10 bytes remaining
        while byte_pos + 10 <= byte_length {
            let mut v: u32 = 0;
            let mut bytes_read = 0;

            // Decode up to 5 bytes for a u32 value
            for i in 0..5 {
                let c = input_bytes[byte_pos + i];

                if i < 4 {
                    // For bytes 0-3, use 7 bits each
                    v |= u32::from(c & 0x7F) << (i * 7);
                    if c < 128 {
                        bytes_read = i + 1;
                        break;
                    }
                } else {
                    // For byte 4, use only 4 bits (total: 7*4 + 4 = 32 bits)
                    v |= u32::from(c & 0x0F) << 28;
                    bytes_read = 5;
                }
            }

            byte_pos += bytes_read;
            output[tmp_outpos] = v;
            tmp_outpos += 1;
        }

        // Slow path: process remaining bytes
        while byte_pos < byte_length {
            let mut shift = 0;
            let mut v: u32 = 0;
            while byte_pos < byte_length {
                let c = input_bytes[byte_pos];
                byte_pos += 1;
                v += u32::from(c & 127) << shift;
                if c < 128 {
                    output[tmp_outpos] = v;
                    tmp_outpos += 1;
                    break;
                }
                shift += 7;
            }
        }

        output_offset.set_position(tmp_outpos as u64);
        input_offset.add(input_length);

        Ok(())
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
            let val = input[k as usize];
            if val < (1 << 7) {
                output[out_pos_tmp as usize] = (val & 0x7F) as i8;
                out_pos_tmp += 1;
            } else if val < (1 << 14) {
                output[out_pos_tmp as usize] = ((val & 0x7F) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (val >> 7) as i8;
                out_pos_tmp += 1;
            } else if val < (1 << 21) {
                output[out_pos_tmp as usize] = ((val & 0x7F) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (((val >> 7) & 0x7F) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (val >> 14) as i8;
                out_pos_tmp += 1;
            } else if val < (1 << 28) {
                output[out_pos_tmp as usize] = ((val & 0x7F) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (((val >> 7) & 0x7F) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (((val >> 14) & 0x7F) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (val >> 21) as i8;
                out_pos_tmp += 1;
            } else {
                output[out_pos_tmp as usize] = ((val & 0x7F) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (((val >> 7) & 0x7F) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (((val >> 14) & 0x7F) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (((val >> 21) & 0x7F) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (val >> 28) as i8;
                out_pos_tmp += 1;
            }
        }
        output_offset.set_position(out_pos_tmp + u64::from(input_length));
        input_offset.add(input_length);
        Ok(())
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

        while p < final_p {
            let mut v = i32::from(input[p as usize] & 0x7F);
            if input[p as usize] >= 0 {
                // High bit is NOT set, this is the last byte
                p += 1;
                output[tmp_outpos as usize] = v as u32;
                tmp_outpos += 1;
                continue;
            }

            v |= i32::from(input[p as usize + 1] & 0x7F) << 7;
            if input[p as usize + 1] >= 0 {
                // High bit is NOT set, this is the last byte
                p += 2;
                output[tmp_outpos as usize] = v as u32;
                tmp_outpos += 1;
                continue;
            }

            v |= i32::from(input[p as usize + 2] & 0x7F) << 14;
            if input[p as usize + 2] >= 0 {
                // High bit is NOT set, this is the last byte
                p += 3;
                output[tmp_outpos as usize] = v as u32;
                tmp_outpos += 1;
                continue;
            }

            v |= i32::from(input[p as usize + 3] & 0x7F) << 21;
            if input[p as usize + 3] >= 0 {
                // High bit is NOT set, this is the last byte
                p += 4;
                output[tmp_outpos as usize] = v as u32;
                tmp_outpos += 1;
                continue;
            }

            v |= i32::from(input[p as usize + 4] & 0x0F) << 28;
            p += 5;
            output[tmp_outpos as usize] = v as u32;
            tmp_outpos += 1;
        }
        output_offset.set_position(tmp_outpos);
        input_offset.add(input_length);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_u32_roundtrip(input: &[u32]) {
        let mut vb = VariableByte::new();
        let mut encoded: Vec<u32> = vec![0; input.len() * 2];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.compress(
            input,
            input.len() as u32,
            &mut input_offset,
            &mut encoded,
            &mut output_offset,
        )
        .expect("Failed to compress");

        let encoded_len = output_offset.position() as u32;
        let mut decoded: Vec<u32> = vec![0; input.len()];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.uncompress(
            &encoded,
            encoded_len,
            &mut input_offset,
            &mut decoded,
            &mut output_offset,
        )
        .expect("Failed to uncompress");

        assert_eq!(
            input.len(),
            output_offset.position() as usize,
            "Decoded length mismatch"
        );
        assert_eq!(input, &decoded[..input.len()], "Decoded data mismatch");
    }

    fn verify_i8_roundtrip(input: &[u32]) {
        let mut vb = VariableByte::new();
        let mut encoded: Vec<i8> = vec![0; input.len() * 10];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.compress(
            input,
            input.len() as u32,
            &mut input_offset,
            &mut encoded,
            &mut output_offset,
        )
        .expect("Failed to compress");

        let encoded_len = (output_offset.position() - input.len() as u64) as u32;
        let mut decoded: Vec<u32> = vec![0; input.len()];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.uncompress(
            &encoded,
            encoded_len,
            &mut input_offset,
            &mut decoded,
            &mut output_offset,
        )
        .expect("Failed to uncompress");

        assert_eq!(
            input.len(),
            output_offset.position() as usize,
            "Decoded length mismatch"
        );
        assert_eq!(input, &decoded[..input.len()], "Decoded data mismatch");
    }

    #[test]
    fn test_empty_int_array() {
        verify_u32_roundtrip(&[]);
    }

    #[test]
    fn test_empty_byte_array() {
        verify_i8_roundtrip(&[]);
    }

    #[test]
    fn test_single_small_value() {
        verify_u32_roundtrip(&[5]);
        verify_i8_roundtrip(&[5]);
    }

    #[test]
    fn test_single_large_value() {
        verify_u32_roundtrip(&[10_878_508]);
        verify_i8_roundtrip(&[10_878_508]);
    }

    #[test]
    fn test_boundary_values_7bit() {
        verify_u32_roundtrip(&[0, 127]);
        verify_i8_roundtrip(&[0, 127]);
    }

    #[test]
    fn test_boundary_values_14bit() {
        verify_u32_roundtrip(&[128, 16383]);
        verify_i8_roundtrip(&[128, 16383]);
    }

    #[test]
    fn test_boundary_values_21bit() {
        verify_u32_roundtrip(&[16384, 2_097_151]);
        verify_i8_roundtrip(&[16384, 2_097_151]);
    }

    #[test]
    fn test_boundary_values_28bit() {
        verify_u32_roundtrip(&[2_097_152, 268_435_455]);
        verify_i8_roundtrip(&[2_097_152, 268_435_455]);
    }

    #[test]
    fn test_boundary_values_32bit() {
        verify_u32_roundtrip(&[268_435_456, u32::MAX]);
        verify_i8_roundtrip(&[268_435_456, u32::MAX]);
    }

    #[test]
    fn test_increasing_sequence() {
        let input: Vec<u32> = (0..1000).collect();
        verify_u32_roundtrip(&input);
        verify_i8_roundtrip(&input);
    }

    #[test]
    fn test_max_and_min() {
        verify_u32_roundtrip(&[0, u32::MAX]);
        verify_i8_roundtrip(&[0, u32::MAX]);
    }

    #[test]
    fn test_powers_of_two() {
        let input: Vec<u32> = (0..31).map(|i| 1u32 << i).collect();
        verify_u32_roundtrip(&input);
        verify_i8_roundtrip(&input);
    }

    #[test]
    fn test_mixed_sizes() {
        let input = vec![
            5,           // 1 byte
            200,         // 2 bytes
            20_000,      // 3 bytes
            2_000_000,   // 4 bytes
            200_000_000, // 5 bytes
        ];
        verify_u32_roundtrip(&input);
        verify_i8_roundtrip(&input);
    }

    #[test]
    fn test_all_same_value() {
        let input = vec![42; 100];
        verify_u32_roundtrip(&input);
        verify_i8_roundtrip(&input);
    }

    #[test]
    fn test_alternating_small_large() {
        let mut input = Vec::new();
        for i in 0..50 {
            if i % 2 == 0 {
                input.push(1);
            } else {
                input.push(u32::MAX);
            }
        }
        verify_u32_roundtrip(&input);
        verify_i8_roundtrip(&input);
    }

    #[test]
    fn test_random_numbers_small() {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};

        let seed = RandomState::new().build_hasher().finish();
        let mut rng = seed;
        let mut input = Vec::new();

        for _ in 0..1000 {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            input.push((rng % u64::from(u32::MAX)) as u32);
        }

        verify_u32_roundtrip(&input);
        verify_i8_roundtrip(&input);
    }

    #[test]
    fn test_fuzz_case_regression() {
        // Regression test from fuzzing: input [0x00a6002c]
        let input = vec![0x00a6002c];
        verify_u32_roundtrip(&input);
        verify_i8_roundtrip(&input);
    }

    #[test]
    fn test_sequential_values() {
        let input: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        verify_u32_roundtrip(&input);
        verify_i8_roundtrip(&input);
    }

    #[test]
    fn test_sparse_values() {
        let input = vec![0, 1000000, 2000000, 3000000, 4000000];
        verify_u32_roundtrip(&input);
        verify_i8_roundtrip(&input);
    }
}
