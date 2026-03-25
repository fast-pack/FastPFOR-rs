use std::io::Cursor;

use bytemuck::{cast_slice, cast_slice_mut};

use crate::codec::AnyLenCodec;
use crate::helpers::AsUsize;
use crate::rust::cursor::IncrementCursor;
use crate::{FastPForError, FastPForResult};

/// Variable-byte encoding codec for integer compression.
#[derive(Debug, Default)]
pub struct VariableByte;

// Helper functions with const generics for extracting 7-bit chunks
impl VariableByte {
    /// Extract 7 bits from position i (with masking)
    const fn extract_7bits<const I: u32>(val: u32) -> u8 {
        ((val >> (7 * I)) & ((1 << 7) - 1)) as u8
    }

    /// Extract 7 bits from position i (without masking, for last byte)
    const fn extract_7bits_maskless<const I: u32>(val: u32) -> u8 {
        (val >> (7 * I)) as u8
    }
}

// Implemented for consistency with other codecs
impl VariableByte {
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Compress `input_length` u32 values from `input[input_offset..]` into
    /// `output[output_offset..]` as packed variable-byte u8 values (stored in
    /// u32 words, padded to 4-byte alignment with `0xFF`).
    #[allow(clippy::unnecessary_wraps)]
    fn compress_into_slice(
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        if input_length == 0 {
            return Ok(());
        }

        let output_start = output_offset.position() as usize;
        let output_bytes: &mut [u8] = &mut cast_slice_mut::<u32, u8>(output)[output_start * 4..];

        // Lemire format: last byte has high bit set (c >= 128 means end of value).
        let mut byte_pos = 0;
        for k in input_offset.position()..(input_offset.position() + u64::from(input_length)) {
            let val = input[k as usize];
            if val < (1 << 7) {
                output_bytes[byte_pos] = Self::extract_7bits::<0>(val) | (1 << 7);
                byte_pos += 1;
            } else if val < (1 << 14) {
                output_bytes[byte_pos] = Self::extract_7bits::<0>(val);
                output_bytes[byte_pos + 1] = Self::extract_7bits_maskless::<1>(val) | (1 << 7);
                byte_pos += 2;
            } else if val < (1 << 21) {
                output_bytes[byte_pos] = Self::extract_7bits::<0>(val);
                output_bytes[byte_pos + 1] = Self::extract_7bits::<1>(val);
                output_bytes[byte_pos + 2] = Self::extract_7bits_maskless::<2>(val) | (1 << 7);
                byte_pos += 3;
            } else if val < (1 << 28) {
                output_bytes[byte_pos] = Self::extract_7bits::<0>(val);
                output_bytes[byte_pos + 1] = Self::extract_7bits::<1>(val);
                output_bytes[byte_pos + 2] = Self::extract_7bits::<2>(val);
                output_bytes[byte_pos + 3] = Self::extract_7bits_maskless::<3>(val) | (1 << 7);
                byte_pos += 4;
            } else {
                output_bytes[byte_pos] = Self::extract_7bits::<0>(val);
                output_bytes[byte_pos + 1] = Self::extract_7bits::<1>(val);
                output_bytes[byte_pos + 2] = Self::extract_7bits::<2>(val);
                output_bytes[byte_pos + 3] = Self::extract_7bits::<3>(val);
                output_bytes[byte_pos + 4] = Self::extract_7bits_maskless::<4>(val) | (1 << 7);
                byte_pos += 5;
            }
        }

        // Pad to 4-byte alignment with 0 (lemire uses 0, not 0xFF)
        while byte_pos % 4 != 0 {
            output_bytes[byte_pos] = 0;
            byte_pos += 1;
        }

        output_offset.add(byte_pos as u32 / 4);
        input_offset.add(input_length);

        Ok(())
    }

    /// Decompress `input_length` u32 words of variable-byte data from
    /// `input[input_offset..]` into `output[output_offset..]`.
    fn decompress_from_u32_slice(
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        if input_length == 0 {
            return Ok(());
        }

        let byte_length = input_length.as_usize() * 4;
        let input_start = input_offset.position() as usize;

        let input_bytes: &[u8] =
            &cast_slice::<u32, u8>(input)[input_start * 4..input_start * 4 + byte_length];

        let mut byte_pos = 0;
        let mut tmp_outpos = output_offset.position() as usize;

        // Lemire format: high bit set (c >= 128) means last byte of value.
        // Fast path: process while we have at least 10 bytes remaining
        while byte_pos + 10 <= byte_length {
            let mut v: u32 = 0;
            let mut bytes_read = 0;

            for i in 0..5 {
                let c = input_bytes[byte_pos + i];

                if i < 4 {
                    v |= u32::from(c & 0x7F) << (i * 7);
                    if c >= 128 {
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
            if tmp_outpos >= output.len() {
                return Err(FastPForError::OutputBufferTooSmall);
            }
            output[tmp_outpos] = v;
            tmp_outpos += 1;
        }

        // Slow path: process remaining bytes (lemire: c >= 128 = last byte)
        while byte_pos < byte_length {
            let mut v: u32 = 0;
            let mut decoded = false;
            for i in 0..5usize {
                if byte_pos >= byte_length {
                    break;
                }
                let c = input_bytes[byte_pos];
                byte_pos += 1;
                if i < 4 {
                    v |= u32::from(c & 0x7F) << (i * 7);
                    if c >= 128 {
                        decoded = true;
                        break;
                    }
                } else {
                    // 5th byte: only 4 bits contribute (7*4 bits already used)
                    v |= u32::from(c & 0x0F) << 28;
                    decoded = true;
                }
            }
            if decoded {
                if tmp_outpos >= output.len() {
                    return Err(FastPForError::OutputBufferTooSmall);
                }
                output[tmp_outpos] = v;
                tmp_outpos += 1;
            }
        }

        output_offset.set_position(tmp_outpos as u64);
        input_offset.add(input_length);

        Ok(())
    }

    /// Compress `input_length` u32 values into an `i8` slice using sign-bit
    /// continuation encoding (negative i8 = more bytes follow).
    #[cfg(test)]
    #[allow(clippy::unnecessary_wraps)]
    fn compress_to_i8_slice(
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [i8],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()> {
        if input_length == 0 {
            return Ok(());
        }
        let mut out_pos_tmp = output_offset.position();
        for k in input_offset.position() as u32..(input_offset.position() as u32 + input_length) {
            let val = input[k.as_usize()];
            if val < (1 << 7) {
                output[out_pos_tmp as usize] = Self::extract_7bits::<0>(val) as i8;
                out_pos_tmp += 1;
            } else if val < (1 << 14) {
                output[out_pos_tmp as usize] = (Self::extract_7bits::<0>(val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = Self::extract_7bits_maskless::<1>(val) as i8;
                out_pos_tmp += 1;
            } else if val < (1 << 21) {
                output[out_pos_tmp as usize] = (Self::extract_7bits::<0>(val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (Self::extract_7bits::<1>(val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = Self::extract_7bits_maskless::<2>(val) as i8;
                out_pos_tmp += 1;
            } else if val < (1 << 28) {
                output[out_pos_tmp as usize] = (Self::extract_7bits::<0>(val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (Self::extract_7bits::<1>(val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (Self::extract_7bits::<2>(val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = Self::extract_7bits_maskless::<3>(val) as i8;
                out_pos_tmp += 1;
            } else {
                output[out_pos_tmp as usize] = (Self::extract_7bits::<0>(val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (Self::extract_7bits::<1>(val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (Self::extract_7bits::<2>(val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = (Self::extract_7bits::<3>(val) | (1 << 7)) as i8;
                out_pos_tmp += 1;
                output[out_pos_tmp as usize] = Self::extract_7bits_maskless::<4>(val) as i8;
                out_pos_tmp += 1;
            }
        }
        output_offset.set_position(out_pos_tmp + u64::from(input_length));
        input_offset.add(input_length);
        Ok(())
    }

    /// Decompress `input_length` i8 values (sign-bit continuation encoding)
    /// into u32 output.
    #[cfg(test)]
    #[allow(clippy::unnecessary_wraps)]
    fn decompress_from_i8_slice(
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
            let mut v = i32::from(input[p.as_usize()] & 0x7F);
            if input[p.as_usize()] >= 0 {
                p += 1;
                output[tmp_outpos as usize] = v as u32;
                tmp_outpos += 1;
                continue;
            }

            v |= i32::from(input[p.as_usize() + 1] & 0x7F) << 7;
            if input[p.as_usize() + 1] >= 0 {
                p += 2;
                output[tmp_outpos as usize] = v as u32;
                tmp_outpos += 1;
                continue;
            }

            v |= i32::from(input[p.as_usize() + 2] & 0x7F) << 14;
            if input[p.as_usize() + 2] >= 0 {
                p += 3;
                output[tmp_outpos as usize] = v as u32;
                tmp_outpos += 1;
                continue;
            }

            v |= i32::from(input[p.as_usize() + 3] & 0x7F) << 21;
            if input[p.as_usize() + 3] >= 0 {
                p += 4;
                output[tmp_outpos as usize] = v as u32;
                tmp_outpos += 1;
                continue;
            }

            v |= i32::from(input[p.as_usize() + 4] & 0x0F) << 28;
            p += 5;
            output[tmp_outpos as usize] = v as u32;
            tmp_outpos += 1;
        }
        output_offset.set_position(tmp_outpos);
        input_offset.add(input_length);
        Ok(())
    }
}

impl AnyLenCodec for VariableByte {
    fn encode(&mut self, input: &[u32], out: &mut Vec<u32>) -> FastPForResult<()> {
        let capacity = input.len() * 2 + 4;
        let start = out.len();
        out.resize(start + capacity, 0);
        let mut in_off = Cursor::new(0u32);
        let mut out_off = Cursor::new(0u32);
        Self::compress_into_slice(
            input,
            input.len() as u32,
            &mut in_off,
            &mut out[start..],
            &mut out_off,
        )?;
        let written = out_off.position() as usize;
        out.truncate(start + written);
        Ok(())
    }

    fn decode(
        &mut self,
        input: &[u32],
        out: &mut Vec<u32>,
        expected_len: Option<u32>,
    ) -> FastPForResult<()> {
        let capacity = if let Some(expected) = expected_len {
            expected.is_valid_expected(Self::max_decompressed_len(input.len()))?
        } else {
            input.len() * 4
        };
        let start = out.len();
        out.reserve(capacity);
        out.resize(start + capacity, 0);
        let mut in_off = Cursor::new(0u32);
        let mut out_off = Cursor::new(0u32);
        Self::decompress_from_u32_slice(
            input,
            input.len() as u32,
            &mut in_off,
            &mut out[start..],
            &mut out_off,
        )?;
        let written = out_off.position() as usize;
        out.truncate(start + written);
        if let Some(n) = expected_len {
            written.is_decoded_mismatch(n)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    use super::*;
    use crate::test_utils::{compress, decompress, roundtrip};

    fn verify_u32_roundtrip(input: &[u32]) {
        let mut encoded: Vec<u32> = vec![0; input.len() * 2 + 1];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        VariableByte::compress_into_slice(
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

        VariableByte::decompress_from_u32_slice(
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
        let mut encoded: Vec<i8> = vec![0; input.len() * 10];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        VariableByte::compress_to_i8_slice(
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

        VariableByte::decompress_from_i8_slice(
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

    #[test]
    fn test_variable_byte_default() {
        let data = vec![1u32, 2, 3];
        roundtrip::<VariableByte>(&data);
    }

    /// `decompress_from_u32_slice` returns `OutputBufferTooSmall` when the
    /// output buffer is exhausted mid-stream (fast path, ≥10 bytes remaining).
    #[test]
    fn test_decompress_output_too_small_fast_path() {
        // Encode 16 values so the fast path (≥10 bytes) is exercised.
        let input: Vec<u32> = (0..16).collect();
        let mut encoded: Vec<u32> = vec![0; input.len() * 2 + 1];
        let mut in_off = Cursor::new(0u32);
        let mut out_off = Cursor::new(0u32);
        VariableByte::compress_into_slice(
            &input,
            input.len() as u32,
            &mut in_off,
            &mut encoded,
            &mut out_off,
        )
        .unwrap();
        let encoded_len = out_off.position() as u32;

        // Output buffer with room for only 4 values — must error.
        let mut tiny_out = vec![0u32; 4];
        let result = VariableByte::decompress_from_u32_slice(
            &encoded,
            encoded_len,
            &mut Cursor::new(0u32),
            &mut tiny_out,
            &mut Cursor::new(0u32),
        );
        assert!(
            matches!(result, Err(FastPForError::OutputBufferTooSmall)),
            "expected OutputBufferTooSmall, got {result:?}"
        );
    }

    /// `decompress_from_u32_slice` returns `OutputBufferTooSmall` when the
    /// output buffer is exhausted in the slow path (<10 bytes remaining).
    #[test]
    fn test_decompress_output_too_small_slow_path() {
        // Encode 2 values so only the slow path is exercised (< 10 bytes).
        let input = vec![1u32, 2];
        let mut encoded: Vec<u32> = vec![0; input.len() * 2 + 1];
        let mut in_off = Cursor::new(0u32);
        let mut out_off = Cursor::new(0u32);
        VariableByte::compress_into_slice(
            &input,
            input.len() as u32,
            &mut in_off,
            &mut encoded,
            &mut out_off,
        )
        .unwrap();
        let encoded_len = out_off.position() as u32;

        // Zero-capacity output — must error.
        let result = VariableByte::decompress_from_u32_slice(
            &encoded,
            encoded_len,
            &mut Cursor::new(0u32),
            &mut [],
            &mut Cursor::new(0u32),
        );
        assert!(
            matches!(result, Err(FastPForError::OutputBufferTooSmall)),
            "expected OutputBufferTooSmall, got {result:?}"
        );
    }

    #[test]
    fn test_anylen_decode_with_expected_len_ok() {
        let data = vec![1u32, 2, 3];
        let encoded = compress::<VariableByte>(&data).unwrap();
        let decoded = decompress::<VariableByte>(&encoded, Some(3)).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_anylen_decode_expected_len_mismatch_errors() {
        // expected_len must be >= actual to avoid OutputBufferTooSmall; use a larger
        // value to exercise the is_decoded_mismatch path.
        let encoded = compress::<VariableByte>(&[1u32, 2, 3]).unwrap();
        decompress::<VariableByte>(&encoded, Some(10)).unwrap_err();
    }
}
