//! Scalar bit-packing for 64-bit values.
//!
//! Packs and unpacks groups of 32 values at any bit width `0..=64`.
//! The layout is a little-endian bitstream, matching the hand-unrolled 32-bit kernels in [`bitpacking`](super::bit_pack32).
//! Value `j` occupies bits `[j*bit, (j+1)*bit)` of the concatenated stream.
//! Each call moves exactly `bit` `u32` words.
//!
//! Each width has its own fully unrolled kernel, so word offsets and shifts are compile-time constants.

const fn low_mask(bit: u8) -> u64 {
    if bit >= 64 {
        u64::MAX
    } else {
        (1u64 << bit) - 1
    }
}

/// Expands `$body` with a `const $b: usize` set to `$bit`, for widths `1..=64`.
macro_rules! dispatch_wide {
    ($bit:expr, |$b:ident| $body:expr) => {
        dispatch_wide!(@arms $bit, $b, $body;
            1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
            17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
            33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48
            49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64)
    };
    (@arms $bit:expr, $b:ident, $body:expr; $($n:literal)*) => {
        match $bit {
            $($n => {
                const $b: usize = $n;
                $body
            })*
            _ => panic!("Unsupported bit width"),
        }
    };
}

/// Expands `$body` 32 times with `const $j: usize` set to `0..32`, so indices fold to constants.
macro_rules! for_each_value {
    (|$j:ident| $body:block) => {
        for_each_value!(@values $j $body;
            0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
            16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)
    };
    (@values $j:ident $body:block; $($n:literal)*) => {
        $({
            const $j: usize = $n;
            $body
        })*
    };
}

/// Packs 32 values from `input[inpos..]` into `output[outpos..]` at `bit` bits each.
pub fn pack_wide(input: &[u64], inpos: usize, output: &mut [u32], outpos: usize, bit: u8) {
    if bit == 0 {
        return;
    }
    let input: &[u64; 32] = input[inpos..inpos + 32]
        .try_into()
        .expect("32-value subslice");
    dispatch_wide!(bit, |B| pack_bits::<B>(
        input,
        &mut output[outpos..outpos + B]
    ));
}

/// Unpacks 32 values from `input[inpos..]` into `output[outpos..]` at `bit` bits each.
pub fn unpack_wide(input: &[u32], inpos: usize, output: &mut [u64], outpos: usize, bit: u8) {
    let output: &mut [u64; 32] = (&mut output[outpos..outpos + 32])
        .try_into()
        .expect("32-value subslice");
    if bit == 0 {
        output.fill(0);
    } else {
        dispatch_wide!(bit, |B| unpack_bits::<B>(&input[inpos..inpos + B], output));
    }
}

/// Packs at `B` bits, `0 < B <= 64`: each value spans one to three output words.
#[inline]
fn pack_bits<const B: usize>(input: &[u64; 32], output: &mut [u32]) {
    let mask = low_mask(B as u8);
    let mut words = [0u32; B];
    for_each_value!(|J| {
        let v = input[J] & mask;
        let (w, s) = (J * B / 32, J * B % 32);
        words[w] |= (v << s) as u32;
        if s + B > 32 {
            words[w + 1] |= (v >> (32 - s)) as u32;
        }
        if s + B > 64 {
            words[w + 2] |= (v >> (64 - s)) as u32;
        }
    });
    output.copy_from_slice(&words);
}

/// Unpacks at `B` bits, `0 < B <= 64`: each value spans one to three input words.
#[inline]
fn unpack_bits<const B: usize>(input: &[u32], output: &mut [u64; 32]) {
    let input: &[u32; B] = input.try_into().expect("B-word subslice");
    let mask = low_mask(B as u8);
    for_each_value!(|J| {
        let (w, s) = (J * B / 32, J * B % 32);
        let mut v = u64::from(input[w]) >> s;
        if s + B > 32 {
            v |= u64::from(input[w + 1]) << (32 - s);
        }
        if s + B > 64 {
            v |= u64::from(input[w + 2]) << (64 - s);
        }
        output[J] = v & mask;
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::integer_compression::{bit_pack32, bit_unpack32};

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
            bit_pack32::fast_pack(&masked32, 0, &mut out_ref, 0, bit);

            let mut out_wide = vec![0u32; bit as usize];
            pack_wide(&masked64, 0, &mut out_wide, 0, bit);

            assert_eq!(out_ref, out_wide, "pack mismatch at bit={bit}");

            let mut back_ref = vec![0u32; 32];
            bit_unpack32::fast_unpack(&out_ref, 0, &mut back_ref, 0, bit);
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
