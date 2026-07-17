//! 64-bit ([`u64`]) `FastPFOR` codec.
//!
//! This is the widened counterpart of the 32-bit [`FastPFor`](super::fastpfor::FastPFor).
//! Values, exceptions, and the exception bitmap are 64 bits wide instead of 32.
//! The output is byte-compatible with the C++ `CppFastPFor128` / `CppFastPFor256` 64-bit paths.
//!
//! [`FastPForWide`] is the 64-bit half of the public [`FastPFor128`](crate::FastPFor128) /
//! [`FastPFor256`](crate::FastPFor256) codecs and is not exported on its own.
//! It handles complete blocks, then a [`VariableByte`] tail encodes the sub-block remainder.

use std::io::Cursor;

use bytemuck::{cast_slice, cast_slice_mut};

use crate::codec::default_max_decoded_len;
use crate::helpers::AsUsize;
use crate::rust::integer_compression::fastpfor::FastPFor;
use crate::{BlockCodec64, FastPForError, FastPForResult};

/// 64-bit `FastPFOR` codec: `FastPFOR`-packed blocks plus a variable-byte tail.
///
/// `N` is the block size (128 or 256 values). This is [`FastPFor`] specialized to the
/// `u64` element type, and is the internal `u64` codec behind
/// [`FastPFor128`](crate::FastPFor128) and [`FastPFor256`](crate::FastPFor256).
pub type FastPForWide<const N: usize> = FastPFor<N, { u64::BITS as usize + 1 }, u64>;

/// Variable-byte encoding of the `u64` tail.
///
/// Each value is emitted little-endian in 7-bit groups.
/// Every byte but the last has its high bit clear.
/// The final byte sets its high bit as a terminator.
/// The stream is zero-padded to a whole number of `u32` words.
fn vbyte_encode64(input: &[u64], out: &mut Vec<u32>) {
    if input.is_empty() {
        return;
    }
    let start = out.len();
    let capacity = input.len() * 3 + 4;
    out.resize(start + capacity, 0);
    let bytes: &mut [u8] = cast_slice_mut(&mut out[start..]);
    let mut byte_pos = 0;
    for &value in input {
        let mut v = value;
        while v >= 0x80 {
            bytes[byte_pos] = (v as u8) & 0x7F;
            byte_pos += 1;
            v >>= 7;
        }
        bytes[byte_pos] = (v as u8) | 0x80;
        byte_pos += 1;
    }
    while byte_pos % 4 != 0 {
        bytes[byte_pos] = 0;
        byte_pos += 1;
    }
    out.truncate(start + byte_pos / 4);
}

/// Inverse of [`vbyte_encode64`].
/// Trailing zero padding decodes to no value, since a padding byte never sets the terminator bit.
fn vbyte_decode64(input: &[u32], out: &mut Vec<u64>) -> FastPForResult<()> {
    if input.is_empty() {
        return Ok(());
    }
    let bytes: &[u8] = cast_slice(input);
    let byte_len = bytes.len();
    let mut byte_pos = 0;
    while byte_pos < byte_len {
        let mut v: u64 = 0;
        let mut shift = 0u32;
        loop {
            if byte_pos >= byte_len {
                return Ok(());
            }
            let c = bytes[byte_pos];
            byte_pos += 1;
            if shift >= 64 {
                return Err(FastPForError::NotEnoughData);
            }
            if c >= 0x80 {
                v |= u64::from(c & 0x7F) << shift;
                out.push(v);
                break;
            }
            v |= u64::from(c) << shift;
            shift += 7;
        }
    }
    Ok(())
}

impl<const N: usize> BlockCodec64 for FastPForWide<N> {
    fn encode64(&mut self, input: &[u64], out: &mut Vec<u32>) -> FastPForResult<()> {
        let rounded = (input.len() / N) * N;
        let n_values = rounded as u32;

        let start = out.len();
        if rounded == 0 {
            out.push(0);
        } else {
            let capacity = rounded * 3 + 1024;
            out.resize(start + 1 + capacity, 0);
            out[start] = n_values;

            let mut in_off = Cursor::new(0u32);
            let mut out_off = Cursor::new(0u32);
            self.compress_blocks(
                &input[..rounded],
                n_values,
                &mut in_off,
                &mut out[start + 1..],
                &mut out_off,
            );
            let written = 1 + out_off.position() as usize;
            out.truncate(start + written);
        }

        vbyte_encode64(&input[rounded..], out);
        Ok(())
    }

    fn decode64(&mut self, input: &[u32], out: &mut Vec<u64>) -> FastPForResult<()> {
        let Some((&block_n_values, rest)) = input.split_first() else {
            return Ok(());
        };
        if block_n_values % N as u32 != 0 {
            return Err(FastPForError::NotEnoughData);
        }
        if block_n_values.as_usize() > default_max_decoded_len(input.len()) {
            return Err(FastPForError::NotEnoughData);
        }
        let n_blocks = block_n_values.as_usize() / N;

        let consumed = if n_blocks == 0 {
            1
        } else {
            let start = out.len();
            out.resize(start + n_blocks * N, 0);
            let mut in_off = Cursor::new(0u32);
            let mut out_off = Cursor::new(0u32);
            self.decode_headless_blocks(
                rest,
                block_n_values,
                &mut in_off,
                &mut out[start..],
                &mut out_off,
            )?;
            1 + in_off.position() as usize
        };

        let tail_input = input.get(consumed..).ok_or(FastPForError::NotEnoughData)?;
        vbyte_decode64(tail_input, out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip<const N: usize>(input: &[u64]) {
        let mut codec = FastPForWide::<N>::default();
        let mut encoded = Vec::new();
        codec.encode64(input, &mut encoded).unwrap();
        let mut decoded = Vec::new();
        codec.decode64(&encoded, &mut decoded).unwrap();
        assert_eq!(decoded, input, "roundtrip mismatch (N={N})");
    }

    #[test]
    fn empty() {
        roundtrip::<128>(&[]);
        roundtrip::<256>(&[]);
    }

    #[test]
    fn single_value() {
        roundtrip::<128>(&[42]);
        roundtrip::<256>(&[u64::MAX]);
    }

    #[test]
    fn sub_block_tail_only() {
        let data: Vec<u64> = (0..10).collect();
        roundtrip::<256>(&data);
    }

    #[test]
    fn exact_block() {
        let data: Vec<u64> = (0..128).collect();
        roundtrip::<128>(&data);
    }

    #[test]
    fn blocks_with_remainder() {
        let data: Vec<u64> = (0..600).collect();
        roundtrip::<256>(&data);
    }

    #[test]
    fn large_values_and_exceptions() {
        let data: Vec<u64> = (0..1024u32)
            .map(|i| if i % 7 == 0 { 1u64 << 60 } else { u64::from(i) })
            .collect();
        roundtrip::<128>(&data);
    }

    #[test]
    fn full_width_values() {
        let data: Vec<u64> = (0..256u32).map(|i| u64::MAX - u64::from(i)).collect();
        roundtrip::<256>(&data);
    }

    #[test]
    fn spans_multiple_pages() {
        let data: Vec<u64> = (0..70_000u64).map(|i| i.wrapping_mul(0x1_0001)).collect();
        roundtrip::<128>(&data);
    }

    #[cfg(feature = "cpp")]
    mod cpp_parity {
        use super::*;
        use crate::BlockCodec64;
        use crate::cpp::{CppFastPFor128, CppFastPFor256};

        fn cases() -> Vec<Vec<u64>> {
            vec![
                vec![],
                vec![42],
                vec![u64::MAX],
                (0..10).collect(),
                (0..128).collect(),
                (0..256).collect(),
                (0..600).collect(),
                (0..1024u32)
                    .map(|i| if i % 7 == 0 { 1u64 << 60 } else { u64::from(i) })
                    .collect(),
                (0..256u32).map(|i| u64::MAX - u64::from(i)).collect(),
                (0..5000u64).map(|i| i.wrapping_mul(0x1_0001)).collect(),
            ]
        }

        fn assert_parity<const N: usize>(rust: &mut FastPForWide<N>, cpp: &mut impl BlockCodec64) {
            for data in cases() {
                let mut rust_enc = Vec::new();
                rust.encode64(&data, &mut rust_enc).unwrap();
                let mut cpp_enc = Vec::new();
                cpp.encode64(&data, &mut cpp_enc).unwrap();
                assert_eq!(rust_enc, cpp_enc, "encode64 bytes differ for {data:?}");

                let mut rust_dec = Vec::new();
                rust.decode64(&cpp_enc, &mut rust_dec).unwrap();
                assert_eq!(rust_dec, data, "Rust failed to decode C++ output");

                let mut cpp_dec = Vec::new();
                cpp.decode64(&rust_enc, &mut cpp_dec).unwrap();
                assert_eq!(cpp_dec, data, "C++ failed to decode Rust output");
            }
        }

        #[test]
        fn parity_128() {
            assert_parity(
                &mut FastPForWide::<128>::default(),
                &mut CppFastPFor128::default(),
            );
        }

        #[test]
        fn parity_256() {
            assert_parity(
                &mut FastPForWide::<256>::default(),
                &mut CppFastPFor256::default(),
            );
        }
    }
}
