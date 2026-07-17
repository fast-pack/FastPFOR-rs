//! Generic scalar bit-packing for 64-bit values.
//!
//! Packs and unpacks groups of 32 values at any bit width `0..=64` using the same
//! little-endian bitstream layout as the hand-unrolled 32-bit kernels in
//! [`bitpacking`](super::bitpacking): value `j` occupies bits `[j*bit, (j+1)*bit)`
//! of the concatenated stream. Each call moves exactly `bit` `u32` words.

const fn low_mask(bit: u8) -> u64 {
    if bit >= 64 {
        u64::MAX
    } else {
        (1u64 << bit) - 1
    }
}

/// Packs 32 values from `input[inpos..]` into `output[outpos..]` at `bit` bits each.
pub fn pack_wide(input: &[u64], inpos: usize, output: &mut [u32], outpos: usize, bit: u8) {
    if bit == 0 {
        return;
    }
    let mask = u128::from(low_mask(bit));
    let mut acc: u128 = 0;
    let mut filled: u32 = 0;
    let mut out = outpos;
    for j in 0..32 {
        acc |= (u128::from(input[inpos + j]) & mask) << filled;
        filled += u32::from(bit);
        while filled >= 32 {
            output[out] = acc as u32;
            out += 1;
            acc >>= 32;
            filled -= 32;
        }
    }
}

/// Unpacks 32 values from `input[inpos..]` into `output[outpos..]` at `bit` bits each.
pub fn unpack_wide(input: &[u32], inpos: usize, output: &mut [u64], outpos: usize, bit: u8) {
    if bit == 0 {
        output[outpos..outpos + 32].fill(0);
        return;
    }
    let mask = u128::from(low_mask(bit));
    let mut acc: u128 = 0;
    let mut avail: u32 = 0;
    let mut inp = inpos;
    for j in 0..32 {
        while avail < u32::from(bit) {
            acc |= u128::from(input[inp]) << avail;
            inp += 1;
            avail += 32;
        }
        output[outpos + j] = (acc & mask) as u64;
        acc >>= u32::from(bit);
        avail -= u32::from(bit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::integer_compression::{bitpacking, bitunpacking};

    /// The wide packer must produce byte-identical output to the proven u32 kernels
    /// for every width `1..=32`, validating its bit ordering without needing C++.
    #[test]
    fn wide_matches_u32_kernels() {
        let values32: [u32; 32] = std::array::from_fn(|i| (i as u32).wrapping_mul(2_654_435_761));

        for bit in 1..=32u8 {
            let mask = if bit == 32 {
                u32::MAX
            } else {
                (1u32 << bit) - 1
            };
            let masked32: [u32; 32] = std::array::from_fn(|i| values32[i] & mask);
            let masked64: [u64; 32] = std::array::from_fn(|i| u64::from(masked32[i]));

            let mut out_ref = vec![0u32; bit as usize];
            bitpacking::fast_pack(&masked32, 0, &mut out_ref, 0, bit);

            let mut out_wide = vec![0u32; bit as usize];
            pack_wide(&masked64, 0, &mut out_wide, 0, bit);

            assert_eq!(out_ref, out_wide, "pack mismatch at bit={bit}");

            let mut back_ref = vec![0u32; 32];
            bitunpacking::fast_unpack(&out_ref, 0, &mut back_ref, 0, bit);
            let mut back_wide = vec![0u64; 32];
            unpack_wide(&out_wide, 0, &mut back_wide, 0, bit);

            for i in 0..32 {
                assert_eq!(
                    u64::from(back_ref[i]),
                    back_wide[i],
                    "unpack mismatch at bit={bit}"
                );
            }
        }
    }

    #[test]
    fn wide_roundtrip_all_widths() {
        for bit in 0..=64u8 {
            let mask = low_mask(bit);
            let values: [u64; 32] =
                std::array::from_fn(|i| (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) & mask);

            let mut packed = vec![0u32; bit as usize];
            pack_wide(&values, 0, &mut packed, 0, bit);

            let mut back = vec![0u64; 32];
            unpack_wide(&packed, 0, &mut back, 0, bit);

            assert_eq!(values.to_vec(), back, "roundtrip mismatch at bit={bit}");
        }
    }
}
