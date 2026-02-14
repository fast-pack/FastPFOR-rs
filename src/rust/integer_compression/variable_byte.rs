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
            std::slice::from_raw_parts(input[input_start..].as_ptr() as *const u8, byte_length)
        };

        let mut byte_pos = 0;
        let mut tmp_outpos = output_offset.position() as usize;

        // Fast path: process while we have at least 10 bytes remaining
        while byte_pos + 10 <= byte_length {
            let mut v: u32 = 0;
            let mut c: u8;

            c = input_bytes[byte_pos];
            v = (c & 0x7F) as u32;
            if c < 128 {
                byte_pos += 1;
                output[tmp_outpos] = v;
                tmp_outpos += 1;
                continue;
            }

            c = input_bytes[byte_pos + 1];
            v |= ((c & 0x7F) as u32) << 7;
            if c < 128 {
                byte_pos += 2;
                output[tmp_outpos] = v;
                tmp_outpos += 1;
                continue;
            }

            c = input_bytes[byte_pos + 2];
            v |= ((c & 0x7F) as u32) << 14;
            if c < 128 {
                byte_pos += 3;
                output[tmp_outpos] = v;
                tmp_outpos += 1;
                continue;
            }

            c = input_bytes[byte_pos + 3];
            v |= ((c & 0x7F) as u32) << 21;
            if c < 128 {
                byte_pos += 4;
                output[tmp_outpos] = v;
                tmp_outpos += 1;
                continue;
            }

            c = input_bytes[byte_pos + 4];
            byte_pos += 5;
            v |= ((c & 0x0F) as u32) << 28;
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
                v += ((c & 127) as u32) << shift;
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

        let encoded_len = (output_offset.position() as u64 - input.len() as u64) as u32;
        let mut decoded: Vec<u32> = vec![0; input.len()];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.uncompress(
            &encoded,
            encoded_len as u32,
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
            input.push((rng % (u32::MAX as u64)) as u32);
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

    #[test]
    fn test_exact_encoding_single_byte() {
        // Test that encoding matches C++ byte-for-byte
        let mut vb = VariableByte::new();
        let input = vec![5u32];
        let mut encoded: Vec<i8> = vec![0; 10];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.compress(
            &input,
            input.len() as u32,
            &mut input_offset,
            &mut encoded,
            &mut output_offset,
        )
        .expect("Failed to compress");

        // Value 5 should encode as 0x05 (5 & 0x7F, no high bit)
        assert_eq!(encoded[0], 5i8);
    }

    #[test]
    fn test_exact_encoding_two_bytes() {
        // Test that encoding matches C++ byte-for-byte
        let mut vb = VariableByte::new();
        let input = vec![200u32];
        let mut encoded: Vec<i8> = vec![0; 10];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.compress(
            &input,
            input.len() as u32,
            &mut input_offset,
            &mut encoded,
            &mut output_offset,
        )
        .expect("Failed to compress");

        // Value 200 = 0b11001000
        // Should encode as: [0xC8, 0x01]
        // 0xC8 = -56 = ((200 & 0x7F) | 0x80) as signed
        // 0x01 = 1 = (200 >> 7)
        assert_eq!(encoded[0], -56i8); // 0xC8
        assert_eq!(encoded[1], 1i8); // 0x01
    }

    #[test]
    fn test_exact_encoding_fuzz_case() {
        // Regression: the fuzz case that was failing
        // Input: 0x00a6002c = 10,878,508
        let mut vb = VariableByte::new();
        let input = vec![0x00a6002c];
        let mut encoded: Vec<i8> = vec![0; 10];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.compress(
            &input,
            input.len() as u32,
            &mut input_offset,
            &mut encoded,
            &mut output_offset,
        )
        .expect("Failed to compress");

        // Now decode and verify we get back the original
        let encoded_len = (output_offset.position() as u64 - input.len() as u64) as u32;
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

        assert_eq!(input, &decoded[..input.len()]);
    }

    #[test]
    fn test_debug_compression_output() {
        // Debug test to see what we actually produce
        let mut vb = VariableByte::new();
        let input = vec![0x00a6002cu32];
        let mut compressed: Vec<u32> = vec![0; 10];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.compress(
            &input,
            input.len() as u32,
            &mut input_offset,
            &mut compressed,
            &mut output_offset,
        )
        .expect("Failed to compress");

        let compressed_len = output_offset.position() as usize;
        println!("Input: 0x{:08x}", input[0]);
        println!("Compressed length: {} u32 words", compressed_len);
        for i in 0..compressed_len {
            let bytes = compressed[i].to_le_bytes();
            println!(
                "  Word {}: 0x{:08x} = bytes [{:02x} {:02x} {:02x} {:02x}]",
                i, compressed[i], bytes[0], bytes[1], bytes[2], bytes[3]
            );
        }

        // Now decompress
        let mut decoded: Vec<u32> = vec![0; 10];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.uncompress(
            &compressed,
            compressed_len as u32,
            &mut input_offset,
            &mut decoded,
            &mut output_offset,
        )
        .expect("Failed to uncompress");

        let decoded_len = output_offset.position() as usize;
        println!("Decoded {} values", decoded_len);
        for i in 0..decoded_len {
            println!("  Value {}: 0x{:08x}", i, decoded[i]);
        }

        assert_eq!(input, &decoded[..decoded_len]);
    }

    #[test]
    fn test_u32_decompress_fuzz_case() {
        // Exact fuzzer failure case:
        // Input: [0x00a6002c] = [10,878,508]
        // Compressed (from C++): [0x059880ac]
        let mut vb = VariableByte::new();
        let expected_output = vec![0x00a6002cu32];

        // First compress it ourselves
        let mut compressed: Vec<u32> = vec![0; expected_output.len() * 2];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.compress(
            &expected_output,
            expected_output.len() as u32,
            &mut input_offset,
            &mut compressed,
            &mut output_offset,
        )
        .expect("Failed to compress");

        let compressed_len = output_offset.position() as u32;

        // Now decompress and verify
        let mut decoded: Vec<u32> = vec![0; expected_output.len()];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.uncompress(
            &compressed,
            compressed_len,
            &mut input_offset,
            &mut decoded,
            &mut output_offset,
        )
        .expect("Failed to uncompress");

        assert_eq!(expected_output.len(), output_offset.position() as usize);
        assert_eq!(expected_output, &decoded[..expected_output.len()]);

        // Also test decompressing the exact C++ output
        let cpp_compressed = vec![0x059880acu32];
        let mut decoded2: Vec<u32> = vec![0; expected_output.len()];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        vb.uncompress(
            &cpp_compressed,
            1,
            &mut input_offset,
            &mut decoded2,
            &mut output_offset,
        )
        .expect("Failed to uncompress C++ data");

        assert_eq!(
            expected_output,
            &decoded2[..output_offset.position() as usize]
        );
    }
}
