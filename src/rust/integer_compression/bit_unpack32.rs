#![expect(clippy::identity_op)]

/// Unpacks 32 integers from `input` into `output` using `bit` bits per integer.
#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
pub fn fast_unpack(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize, bit: u8) {
    match bit {
        0 => fast_unpack0(output, outpos),
        1 => fast_unpack1(input, inpos, output, outpos),
        2 => fast_unpack2(input, inpos, output, outpos),
        3 => fast_unpack3(input, inpos, output, outpos),
        4 => fast_unpack4(input, inpos, output, outpos),
        5 => fast_unpack5(input, inpos, output, outpos),
        6 => fast_unpack6(input, inpos, output, outpos),
        7 => fast_unpack7(input, inpos, output, outpos),
        8 => fast_unpack8(input, inpos, output, outpos),
        9 => fast_unpack9(input, inpos, output, outpos),
        10 => fast_unpack10(input, inpos, output, outpos),
        11 => fast_unpack11(input, inpos, output, outpos),
        12 => fast_unpack12(input, inpos, output, outpos),
        13 => fast_unpack13(input, inpos, output, outpos),
        14 => fast_unpack14(input, inpos, output, outpos),
        15 => fast_unpack15(input, inpos, output, outpos),
        16 => fast_unpack16(input, inpos, output, outpos),
        17 => fast_unpack17(input, inpos, output, outpos),
        18 => fast_unpack18(input, inpos, output, outpos),
        19 => fast_unpack19(input, inpos, output, outpos),
        20 => fast_unpack20(input, inpos, output, outpos),
        21 => fast_unpack21(input, inpos, output, outpos),
        22 => fast_unpack22(input, inpos, output, outpos),
        23 => fast_unpack23(input, inpos, output, outpos),
        24 => fast_unpack24(input, inpos, output, outpos),
        25 => fast_unpack25(input, inpos, output, outpos),
        26 => fast_unpack26(input, inpos, output, outpos),
        27 => fast_unpack27(input, inpos, output, outpos),
        28 => fast_unpack28(input, inpos, output, outpos),
        29 => fast_unpack29(input, inpos, output, outpos),
        30 => fast_unpack30(input, inpos, output, outpos),
        31 => fast_unpack31(input, inpos, output, outpos),
        32 => fast_unpack32(input, inpos, output, outpos),
        _ => panic!("Unsupported bit width"),
    }
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack0(output: &mut [u32], outpos: usize) {
    let output = &mut output[outpos..outpos + 32];
    output.fill(0);
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack1(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..=inpos];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 1;
    output[1] = (input[0] >> 1) & 1;
    output[2] = (input[0] >> 2) & 1;
    output[3] = (input[0] >> 3) & 1;
    output[4] = (input[0] >> 4) & 1;
    output[5] = (input[0] >> 5) & 1;
    output[6] = (input[0] >> 6) & 1;
    output[7] = (input[0] >> 7) & 1;
    output[8] = (input[0] >> 8) & 1;
    output[9] = (input[0] >> 9) & 1;
    output[10] = (input[0] >> 10) & 1;
    output[11] = (input[0] >> 11) & 1;
    output[12] = (input[0] >> 12) & 1;
    output[13] = (input[0] >> 13) & 1;
    output[14] = (input[0] >> 14) & 1;
    output[15] = (input[0] >> 15) & 1;
    output[16] = (input[0] >> 16) & 1;
    output[17] = (input[0] >> 17) & 1;
    output[18] = (input[0] >> 18) & 1;
    output[19] = (input[0] >> 19) & 1;
    output[20] = (input[0] >> 20) & 1;
    output[21] = (input[0] >> 21) & 1;
    output[22] = (input[0] >> 22) & 1;
    output[23] = (input[0] >> 23) & 1;
    output[24] = (input[0] >> 24) & 1;
    output[25] = (input[0] >> 25) & 1;
    output[26] = (input[0] >> 26) & 1;
    output[27] = (input[0] >> 27) & 1;
    output[28] = (input[0] >> 28) & 1;
    output[29] = (input[0] >> 29) & 1;
    output[30] = (input[0] >> 30) & 1;
    output[31] = input[0] >> 31;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack2(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 2];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 3;
    output[1] = (input[0] >> 2) & 3;
    output[2] = (input[0] >> 4) & 3;
    output[3] = (input[0] >> 6) & 3;
    output[4] = (input[0] >> 8) & 3;
    output[5] = (input[0] >> 10) & 3;
    output[6] = (input[0] >> 12) & 3;
    output[7] = (input[0] >> 14) & 3;
    output[8] = (input[0] >> 16) & 3;
    output[9] = (input[0] >> 18) & 3;
    output[10] = (input[0] >> 20) & 3;
    output[11] = (input[0] >> 22) & 3;
    output[12] = (input[0] >> 24) & 3;
    output[13] = (input[0] >> 26) & 3;
    output[14] = (input[0] >> 28) & 3;
    output[15] = input[0] >> 30;
    output[16] = (input[1] >> 0) & 3;
    output[17] = (input[1] >> 2) & 3;
    output[18] = (input[1] >> 4) & 3;
    output[19] = (input[1] >> 6) & 3;
    output[20] = (input[1] >> 8) & 3;
    output[21] = (input[1] >> 10) & 3;
    output[22] = (input[1] >> 12) & 3;
    output[23] = (input[1] >> 14) & 3;
    output[24] = (input[1] >> 16) & 3;
    output[25] = (input[1] >> 18) & 3;
    output[26] = (input[1] >> 20) & 3;
    output[27] = (input[1] >> 22) & 3;
    output[28] = (input[1] >> 24) & 3;
    output[29] = (input[1] >> 26) & 3;
    output[30] = (input[1] >> 28) & 3;
    output[31] = input[1] >> 30;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack3(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 3];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 7;
    output[1] = (input[0] >> 3) & 7;
    output[2] = (input[0] >> 6) & 7;
    output[3] = (input[0] >> 9) & 7;
    output[4] = (input[0] >> 12) & 7;
    output[5] = (input[0] >> 15) & 7;
    output[6] = (input[0] >> 18) & 7;
    output[7] = (input[0] >> 21) & 7;
    output[8] = (input[0] >> 24) & 7;
    output[9] = (input[0] >> 27) & 7;
    output[10] = (input[0] >> 30) | (input[1] & 1) << (3 - 1);
    output[11] = (input[1] >> 1) & 7;
    output[12] = (input[1] >> 4) & 7;
    output[13] = (input[1] >> 7) & 7;
    output[14] = (input[1] >> 10) & 7;
    output[15] = (input[1] >> 13) & 7;
    output[16] = (input[1] >> 16) & 7;
    output[17] = (input[1] >> 19) & 7;
    output[18] = (input[1] >> 22) & 7;
    output[19] = (input[1] >> 25) & 7;
    output[20] = (input[1] >> 28) & 7;
    output[21] = (input[1] >> 31) | (input[2] & 3) << (3 - 2);
    output[22] = (input[2] >> 2) & 7;
    output[23] = (input[2] >> 5) & 7;
    output[24] = (input[2] >> 8) & 7;
    output[25] = (input[2] >> 11) & 7;
    output[26] = (input[2] >> 14) & 7;
    output[27] = (input[2] >> 17) & 7;
    output[28] = (input[2] >> 20) & 7;
    output[29] = (input[2] >> 23) & 7;
    output[30] = (input[2] >> 26) & 7;
    output[31] = input[2] >> 29;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack4(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 4];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 15;
    output[1] = (input[0] >> 4) & 15;
    output[2] = (input[0] >> 8) & 15;
    output[3] = (input[0] >> 12) & 15;
    output[4] = (input[0] >> 16) & 15;
    output[5] = (input[0] >> 20) & 15;
    output[6] = (input[0] >> 24) & 15;
    output[7] = input[0] >> 28;
    output[8] = (input[1] >> 0) & 15;
    output[9] = (input[1] >> 4) & 15;
    output[10] = (input[1] >> 8) & 15;
    output[11] = (input[1] >> 12) & 15;
    output[12] = (input[1] >> 16) & 15;
    output[13] = (input[1] >> 20) & 15;
    output[14] = (input[1] >> 24) & 15;
    output[15] = input[1] >> 28;
    output[16] = (input[2] >> 0) & 15;
    output[17] = (input[2] >> 4) & 15;
    output[18] = (input[2] >> 8) & 15;
    output[19] = (input[2] >> 12) & 15;
    output[20] = (input[2] >> 16) & 15;
    output[21] = (input[2] >> 20) & 15;
    output[22] = (input[2] >> 24) & 15;
    output[23] = input[2] >> 28;
    output[24] = (input[3] >> 0) & 15;
    output[25] = (input[3] >> 4) & 15;
    output[26] = (input[3] >> 8) & 15;
    output[27] = (input[3] >> 12) & 15;
    output[28] = (input[3] >> 16) & 15;
    output[29] = (input[3] >> 20) & 15;
    output[30] = (input[3] >> 24) & 15;
    output[31] = input[3] >> 28;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack5(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 5];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 31;
    output[1] = (input[0] >> 5) & 31;
    output[2] = (input[0] >> 10) & 31;
    output[3] = (input[0] >> 15) & 31;
    output[4] = (input[0] >> 20) & 31;
    output[5] = (input[0] >> 25) & 31;
    output[6] = (input[0] >> 30) | ((input[1] & 7) << (5 - 3));
    output[7] = (input[1] >> 3) & 31;
    output[8] = (input[1] >> 8) & 31;
    output[9] = (input[1] >> 13) & 31;
    output[10] = (input[1] >> 18) & 31;
    output[11] = (input[1] >> 23) & 31;
    output[12] = (input[1] >> 28) | ((input[2] & 1) << (5 - 1));
    output[13] = (input[2] >> 1) & 31;
    output[14] = (input[2] >> 6) & 31;
    output[15] = (input[2] >> 11) & 31;
    output[16] = (input[2] >> 16) & 31;
    output[17] = (input[2] >> 21) & 31;
    output[18] = (input[2] >> 26) & 31;
    output[19] = (input[2] >> 31) | ((input[3] & 15) << (5 - 4));
    output[20] = (input[3] >> 4) & 31;
    output[21] = (input[3] >> 9) & 31;
    output[22] = (input[3] >> 14) & 31;
    output[23] = (input[3] >> 19) & 31;
    output[24] = (input[3] >> 24) & 31;
    output[25] = (input[3] >> 29) | ((input[4] & 3) << (5 - 2));
    output[26] = (input[4] >> 2) & 31;
    output[27] = (input[4] >> 7) & 31;
    output[28] = (input[4] >> 12) & 31;
    output[29] = (input[4] >> 17) & 31;
    output[30] = (input[4] >> 22) & 31;
    output[31] = input[4] >> 27;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack6(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 6];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 63;
    output[1] = (input[0] >> 6) & 63;
    output[2] = (input[0] >> 12) & 63;
    output[3] = (input[0] >> 18) & 63;
    output[4] = (input[0] >> 24) & 63;
    output[5] = (input[0] >> 30) | ((input[1] & 15) << (6 - 4));
    output[6] = (input[1] >> 4) & 63;
    output[7] = (input[1] >> 10) & 63;
    output[8] = (input[1] >> 16) & 63;
    output[9] = (input[1] >> 22) & 63;
    output[10] = (input[1] >> 28) | ((input[2] & 3) << (6 - 2));
    output[11] = (input[2] >> 2) & 63;
    output[12] = (input[2] >> 8) & 63;
    output[13] = (input[2] >> 14) & 63;
    output[14] = (input[2] >> 20) & 63;
    output[15] = input[2] >> 26;
    output[16] = (input[3] >> 0) & 63;
    output[17] = (input[3] >> 6) & 63;
    output[18] = (input[3] >> 12) & 63;
    output[19] = (input[3] >> 18) & 63;
    output[20] = (input[3] >> 24) & 63;
    output[21] = (input[3] >> 30) | ((input[4] & 15) << (6 - 4));
    output[22] = (input[4] >> 4) & 63;
    output[23] = (input[4] >> 10) & 63;
    output[24] = (input[4] >> 16) & 63;
    output[25] = (input[4] >> 22) & 63;
    output[26] = (input[4] >> 28) | ((input[5] & 3) << (6 - 2));
    output[27] = (input[5] >> 2) & 63;
    output[28] = (input[5] >> 8) & 63;
    output[29] = (input[5] >> 14) & 63;
    output[30] = (input[5] >> 20) & 63;
    output[31] = input[5] >> 26;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack7(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 7];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 127;
    output[1] = (input[0] >> 7) & 127;
    output[2] = (input[0] >> 14) & 127;
    output[3] = (input[0] >> 21) & 127;
    output[4] = (input[0] >> 28) | ((input[1] & 7) << (7 - 3));
    output[5] = (input[1] >> 3) & 127;
    output[6] = (input[1] >> 10) & 127;
    output[7] = (input[1] >> 17) & 127;
    output[8] = (input[1] >> 24) & 127;
    output[9] = (input[1] >> 31) | ((input[2] & 63) << (7 - 6));
    output[10] = (input[2] >> 6) & 127;
    output[11] = (input[2] >> 13) & 127;
    output[12] = (input[2] >> 20) & 127;
    output[13] = (input[2] >> 27) | ((input[3] & 3) << (7 - 2));
    output[14] = (input[3] >> 2) & 127;
    output[15] = (input[3] >> 9) & 127;
    output[16] = (input[3] >> 16) & 127;
    output[17] = (input[3] >> 23) & 127;
    output[18] = (input[3] >> 30) | ((input[4] & 31) << (7 - 5));
    output[19] = (input[4] >> 5) & 127;
    output[20] = (input[4] >> 12) & 127;
    output[21] = (input[4] >> 19) & 127;
    output[22] = (input[4] >> 26) | ((input[5] & 1) << (7 - 1));
    output[23] = (input[5] >> 1) & 127;
    output[24] = (input[5] >> 8) & 127;
    output[25] = (input[5] >> 15) & 127;
    output[26] = (input[5] >> 22) & 127;
    output[27] = (input[5] >> 29) | ((input[6] & 15) << (7 - 4));
    output[28] = (input[6] >> 4) & 127;
    output[29] = (input[6] >> 11) & 127;
    output[30] = (input[6] >> 18) & 127;
    output[31] = input[6] >> 25;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack8(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 8];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 255;
    output[1] = (input[0] >> 8) & 255;
    output[2] = (input[0] >> 16) & 255;
    output[3] = input[0] >> 24;
    output[4] = (input[1] >> 0) & 255;
    output[5] = (input[1] >> 8) & 255;
    output[6] = (input[1] >> 16) & 255;
    output[7] = input[1] >> 24;
    output[8] = (input[2] >> 0) & 255;
    output[9] = (input[2] >> 8) & 255;
    output[10] = (input[2] >> 16) & 255;
    output[11] = input[2] >> 24;
    output[12] = (input[3] >> 0) & 255;
    output[13] = (input[3] >> 8) & 255;
    output[14] = (input[3] >> 16) & 255;
    output[15] = input[3] >> 24;
    output[16] = (input[4] >> 0) & 255;
    output[17] = (input[4] >> 8) & 255;
    output[18] = (input[4] >> 16) & 255;
    output[19] = input[4] >> 24;
    output[20] = (input[5] >> 0) & 255;
    output[21] = (input[5] >> 8) & 255;
    output[22] = (input[5] >> 16) & 255;
    output[23] = input[5] >> 24;
    output[24] = (input[6] >> 0) & 255;
    output[25] = (input[6] >> 8) & 255;
    output[26] = (input[6] >> 16) & 255;
    output[27] = input[6] >> 24;
    output[28] = (input[7] >> 0) & 255;
    output[29] = (input[7] >> 8) & 255;
    output[30] = (input[7] >> 16) & 255;
    output[31] = input[7] >> 24;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack9(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 9];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 511;
    output[1] = (input[0] >> 9) & 511;
    output[2] = (input[0] >> 18) & 511;
    output[3] = (input[0] >> 27) | ((input[1] & 15) << (9 - 4));
    output[4] = (input[1] >> 4) & 511;
    output[5] = (input[1] >> 13) & 511;
    output[6] = (input[1] >> 22) & 511;
    output[7] = (input[1] >> 31) | ((input[2] & 255) << (9 - 8));
    output[8] = (input[2] >> 8) & 511;
    output[9] = (input[2] >> 17) & 511;
    output[10] = (input[2] >> 26) | ((input[3] & 7) << (9 - 3));
    output[11] = (input[3] >> 3) & 511;
    output[12] = (input[3] >> 12) & 511;
    output[13] = (input[3] >> 21) & 511;
    output[14] = (input[3] >> 30) | ((input[4] & 127) << (9 - 7));
    output[15] = (input[4] >> 7) & 511;
    output[16] = (input[4] >> 16) & 511;
    output[17] = (input[4] >> 25) | ((input[5] & 3) << (9 - 2));
    output[18] = (input[5] >> 2) & 511;
    output[19] = (input[5] >> 11) & 511;
    output[20] = (input[5] >> 20) & 511;
    output[21] = (input[5] >> 29) | ((input[6] & 63) << (9 - 6));
    output[22] = (input[6] >> 6) & 511;
    output[23] = (input[6] >> 15) & 511;
    output[24] = (input[6] >> 24) | ((input[7] & 1) << (9 - 1));
    output[25] = (input[7] >> 1) & 511;
    output[26] = (input[7] >> 10) & 511;
    output[27] = (input[7] >> 19) & 511;
    output[28] = (input[7] >> 28) | ((input[8] & 31) << (9 - 5));
    output[29] = (input[8] >> 5) & 511;
    output[30] = (input[8] >> 14) & 511;
    output[31] = input[8] >> 23;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack10(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 10];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 1023;
    output[1] = (input[0] >> 10) & 1023;
    output[2] = (input[0] >> 20) & 1023;
    output[3] = (input[0] >> 30) | ((input[1] & 255) << (10 - 8));
    output[4] = (input[1] >> 8) & 1023;
    output[5] = (input[1] >> 18) & 1023;
    output[6] = (input[1] >> 28) | ((input[2] & 63) << (10 - 6));
    output[7] = (input[2] >> 6) & 1023;
    output[8] = (input[2] >> 16) & 1023;
    output[9] = (input[2] >> 26) | ((input[3] & 15) << (10 - 4));
    output[10] = (input[3] >> 4) & 1023;
    output[11] = (input[3] >> 14) & 1023;
    output[12] = (input[3] >> 24) | ((input[4] & 3) << (10 - 2));
    output[13] = (input[4] >> 2) & 1023;
    output[14] = (input[4] >> 12) & 1023;
    output[15] = input[4] >> 22;
    output[16] = (input[5] >> 0) & 1023;
    output[17] = (input[5] >> 10) & 1023;
    output[18] = (input[5] >> 20) & 1023;
    output[19] = (input[5] >> 30) | ((input[6] & 255) << (10 - 8));
    output[20] = (input[6] >> 8) & 1023;
    output[21] = (input[6] >> 18) & 1023;
    output[22] = (input[6] >> 28) | ((input[7] & 63) << (10 - 6));
    output[23] = (input[7] >> 6) & 1023;
    output[24] = (input[7] >> 16) & 1023;
    output[25] = (input[7] >> 26) | ((input[8] & 15) << (10 - 4));
    output[26] = (input[8] >> 4) & 1023;
    output[27] = (input[8] >> 14) & 1023;
    output[28] = (input[8] >> 24) | ((input[9] & 3) << (10 - 2));
    output[29] = (input[9] >> 2) & 1023;
    output[30] = (input[9] >> 12) & 1023;
    output[31] = input[9] >> 22;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack11(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 11];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 2047;
    output[1] = (input[0] >> 11) & 2047;
    output[2] = (input[0] >> 22) | ((input[1] & 1) << (11 - 1));
    output[3] = (input[1] >> 1) & 2047;
    output[4] = (input[1] >> 12) & 2047;
    output[5] = (input[1] >> 23) | ((input[2] & 3) << (11 - 2));
    output[6] = (input[2] >> 2) & 2047;
    output[7] = (input[2] >> 13) & 2047;
    output[8] = (input[2] >> 24) | ((input[3] & 7) << (11 - 3));
    output[9] = (input[3] >> 3) & 2047;
    output[10] = (input[3] >> 14) & 2047;
    output[11] = (input[3] >> 25) | ((input[4] & 15) << (11 - 4));
    output[12] = (input[4] >> 4) & 2047;
    output[13] = (input[4] >> 15) & 2047;
    output[14] = (input[4] >> 26) | ((input[5] & 31) << (11 - 5));
    output[15] = (input[5] >> 5) & 2047;
    output[16] = (input[5] >> 16) & 2047;
    output[17] = (input[5] >> 27) | ((input[6] & 63) << (11 - 6));
    output[18] = (input[6] >> 6) & 2047;
    output[19] = (input[6] >> 17) & 2047;
    output[20] = (input[6] >> 28) | ((input[7] & 127) << (11 - 7));
    output[21] = (input[7] >> 7) & 2047;
    output[22] = (input[7] >> 18) & 2047;
    output[23] = (input[7] >> 29) | ((input[8] & 255) << (11 - 8));
    output[24] = (input[8] >> 8) & 2047;
    output[25] = (input[8] >> 19) & 2047;
    output[26] = (input[8] >> 30) | ((input[9] & 511) << (11 - 9));
    output[27] = (input[9] >> 9) & 2047;
    output[28] = (input[9] >> 20) & 2047;
    output[29] = (input[9] >> 31) | ((input[10] & 1023) << (11 - 10));
    output[30] = (input[10] >> 10) & 2047;
    output[31] = input[10] >> 21;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack12(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 12];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 4095;
    output[1] = (input[0] >> 12) & 4095;
    output[2] = (input[0] >> 24) | ((input[1] & 15) << (12 - 4));
    output[3] = (input[1] >> 4) & 4095;
    output[4] = (input[1] >> 16) & 4095;
    output[5] = (input[1] >> 28) | ((input[2] & 255) << (12 - 8));
    output[6] = (input[2] >> 8) & 4095;
    output[7] = input[2] >> 20;
    output[8] = (input[3] >> 0) & 4095;
    output[9] = (input[3] >> 12) & 4095;
    output[10] = (input[3] >> 24) | ((input[4] & 15) << (12 - 4));
    output[11] = (input[4] >> 4) & 4095;
    output[12] = (input[4] >> 16) & 4095;
    output[13] = (input[4] >> 28) | ((input[5] & 255) << (12 - 8));
    output[14] = (input[5] >> 8) & 4095;
    output[15] = input[5] >> 20;
    output[16] = (input[6] >> 0) & 4095;
    output[17] = (input[6] >> 12) & 4095;
    output[18] = (input[6] >> 24) | ((input[7] & 15) << (12 - 4));
    output[19] = (input[7] >> 4) & 4095;
    output[20] = (input[7] >> 16) & 4095;
    output[21] = (input[7] >> 28) | ((input[8] & 255) << (12 - 8));
    output[22] = (input[8] >> 8) & 4095;
    output[23] = input[8] >> 20;
    output[24] = (input[9] >> 0) & 4095;
    output[25] = (input[9] >> 12) & 4095;
    output[26] = (input[9] >> 24) | ((input[10] & 15) << (12 - 4));
    output[27] = (input[10] >> 4) & 4095;
    output[28] = (input[10] >> 16) & 4095;
    output[29] = (input[10] >> 28) | ((input[11] & 255) << (12 - 8));
    output[30] = (input[11] >> 8) & 4095;
    output[31] = input[11] >> 20;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack13(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 13];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 8191;
    output[1] = (input[0] >> 13) & 8191;
    output[2] = (input[0] >> 26) | ((input[1] & 127) << (13 - 7));
    output[3] = (input[1] >> 7) & 8191;
    output[4] = (input[1] >> 20) | ((input[2] & 1) << (13 - 1));
    output[5] = (input[2] >> 1) & 8191;
    output[6] = (input[2] >> 14) & 8191;
    output[7] = (input[2] >> 27) | ((input[3] & 255) << (13 - 8));
    output[8] = (input[3] >> 8) & 8191;
    output[9] = (input[3] >> 21) | ((input[4] & 3) << (13 - 2));
    output[10] = (input[4] >> 2) & 8191;
    output[11] = (input[4] >> 15) & 8191;
    output[12] = (input[4] >> 28) | ((input[5] & 511) << (13 - 9));
    output[13] = (input[5] >> 9) & 8191;
    output[14] = (input[5] >> 22) | ((input[6] & 7) << (13 - 3));
    output[15] = (input[6] >> 3) & 8191;
    output[16] = (input[6] >> 16) & 8191;
    output[17] = (input[6] >> 29) | ((input[7] & 1023) << (13 - 10));
    output[18] = (input[7] >> 10) & 8191;
    output[19] = (input[7] >> 23) | ((input[8] & 15) << (13 - 4));
    output[20] = (input[8] >> 4) & 8191;
    output[21] = (input[8] >> 17) & 8191;
    output[22] = (input[8] >> 30) | ((input[9] & 2047) << (13 - 11));
    output[23] = (input[9] >> 11) & 8191;
    output[24] = (input[9] >> 24) | ((input[10] & 31) << (13 - 5));
    output[25] = (input[10] >> 5) & 8191;
    output[26] = (input[10] >> 18) & 8191;
    output[27] = (input[10] >> 31) | ((input[11] & 4095) << (13 - 12));
    output[28] = (input[11] >> 12) & 8191;
    output[29] = (input[11] >> 25) | ((input[12] & 63) << (13 - 6));
    output[30] = (input[12] >> 6) & 8191;
    output[31] = input[12] >> 19;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack14(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 14];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 16383;
    output[1] = (input[0] >> 14) & 16383;
    output[2] = (input[0] >> 28) | ((input[1] & 1023) << (14 - 10));
    output[3] = (input[1] >> 10) & 16383;
    output[4] = (input[1] >> 24) | ((input[2] & 63) << (14 - 6));
    output[5] = (input[2] >> 6) & 16383;
    output[6] = (input[2] >> 20) | ((input[3] & 3) << (14 - 2));
    output[7] = (input[3] >> 2) & 16383;
    output[8] = (input[3] >> 16) & 16383;
    output[9] = (input[3] >> 30) | ((input[4] & 4095) << (14 - 12));
    output[10] = (input[4] >> 12) & 16383;
    output[11] = (input[4] >> 26) | ((input[5] & 255) << (14 - 8));
    output[12] = (input[5] >> 8) & 16383;
    output[13] = (input[5] >> 22) | ((input[6] & 15) << (14 - 4));
    output[14] = (input[6] >> 4) & 16383;
    output[15] = input[6] >> 18;
    output[16] = (input[7] >> 0) & 16383;
    output[17] = (input[7] >> 14) & 16383;
    output[18] = (input[7] >> 28) | ((input[8] & 1023) << (14 - 10));
    output[19] = (input[8] >> 10) & 16383;
    output[20] = (input[8] >> 24) | ((input[9] & 63) << (14 - 6));
    output[21] = (input[9] >> 6) & 16383;
    output[22] = (input[9] >> 20) | ((input[10] & 3) << (14 - 2));
    output[23] = (input[10] >> 2) & 16383;
    output[24] = (input[10] >> 16) & 16383;
    output[25] = (input[10] >> 30) | ((input[11] & 4095) << (14 - 12));
    output[26] = (input[11] >> 12) & 16383;
    output[27] = (input[11] >> 26) | ((input[12] & 255) << (14 - 8));
    output[28] = (input[12] >> 8) & 16383;
    output[29] = (input[12] >> 22) | ((input[13] & 15) << (14 - 4));
    output[30] = (input[13] >> 4) & 16383;
    output[31] = input[13] >> 18;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack15(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 15];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 32767;
    output[1] = (input[0] >> 15) & 32767;
    output[2] = (input[0] >> 30) | ((input[1] & 8191) << (15 - 13));
    output[3] = (input[1] >> 13) & 32767;
    output[4] = (input[1] >> 28) | ((input[2] & 2047) << (15 - 11));
    output[5] = (input[2] >> 11) & 32767;
    output[6] = (input[2] >> 26) | ((input[3] & 511) << (15 - 9));
    output[7] = (input[3] >> 9) & 32767;
    output[8] = (input[3] >> 24) | ((input[4] & 127) << (15 - 7));
    output[9] = (input[4] >> 7) & 32767;
    output[10] = (input[4] >> 22) | ((input[5] & 31) << (15 - 5));
    output[11] = (input[5] >> 5) & 32767;
    output[12] = (input[5] >> 20) | ((input[6] & 7) << (15 - 3));
    output[13] = (input[6] >> 3) & 32767;
    output[14] = (input[6] >> 18) | ((input[7] & 1) << (15 - 1));
    output[15] = (input[7] >> 1) & 32767;
    output[16] = (input[7] >> 16) & 32767;
    output[17] = (input[7] >> 31) | ((input[8] & 16383) << (15 - 14));
    output[18] = (input[8] >> 14) & 32767;
    output[19] = (input[8] >> 29) | ((input[9] & 4095) << (15 - 12));
    output[20] = (input[9] >> 12) & 32767;
    output[21] = (input[9] >> 27) | ((input[10] & 1023) << (15 - 10));
    output[22] = (input[10] >> 10) & 32767;
    output[23] = (input[10] >> 25) | ((input[11] & 255) << (15 - 8));
    output[24] = (input[11] >> 8) & 32767;
    output[25] = (input[11] >> 23) | ((input[12] & 63) << (15 - 6));
    output[26] = (input[12] >> 6) & 32767;
    output[27] = (input[12] >> 21) | ((input[13] & 15) << (15 - 4));
    output[28] = (input[13] >> 4) & 32767;
    output[29] = (input[13] >> 19) | ((input[14] & 3) << (15 - 2));
    output[30] = (input[14] >> 2) & 32767;
    output[31] = input[14] >> 17;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack16(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 16];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 65535;
    output[1] = input[0] >> 16;
    output[2] = (input[1] >> 0) & 65535;
    output[3] = input[1] >> 16;
    output[4] = (input[2] >> 0) & 65535;
    output[5] = input[2] >> 16;
    output[6] = (input[3] >> 0) & 65535;
    output[7] = input[3] >> 16;
    output[8] = (input[4] >> 0) & 65535;
    output[9] = input[4] >> 16;
    output[10] = (input[5] >> 0) & 65535;
    output[11] = input[5] >> 16;
    output[12] = (input[6] >> 0) & 65535;
    output[13] = input[6] >> 16;
    output[14] = (input[7] >> 0) & 65535;
    output[15] = input[7] >> 16;
    output[16] = (input[8] >> 0) & 65535;
    output[17] = input[8] >> 16;
    output[18] = (input[9] >> 0) & 65535;
    output[19] = input[9] >> 16;
    output[20] = (input[10] >> 0) & 65535;
    output[21] = input[10] >> 16;
    output[22] = (input[11] >> 0) & 65535;
    output[23] = input[11] >> 16;
    output[24] = (input[12] >> 0) & 65535;
    output[25] = input[12] >> 16;
    output[26] = (input[13] >> 0) & 65535;
    output[27] = input[13] >> 16;
    output[28] = (input[14] >> 0) & 65535;
    output[29] = input[14] >> 16;
    output[30] = (input[15] >> 0) & 65535;
    output[31] = input[15] >> 16;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack17(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 17];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 131071;
    output[1] = (input[0] >> 17) | ((input[1] & 3) << (17 - 2));
    output[2] = (input[1] >> 2) & 131071;
    output[3] = (input[1] >> 19) | ((input[2] & 15) << (17 - 4));
    output[4] = (input[2] >> 4) & 131071;
    output[5] = (input[2] >> 21) | ((input[3] & 63) << (17 - 6));
    output[6] = (input[3] >> 6) & 131071;
    output[7] = (input[3] >> 23) | ((input[4] & 255) << (17 - 8));
    output[8] = (input[4] >> 8) & 131071;
    output[9] = (input[4] >> 25) | ((input[5] & 1023) << (17 - 10));
    output[10] = (input[5] >> 10) & 131071;
    output[11] = (input[5] >> 27) | ((input[6] & 4095) << (17 - 12));
    output[12] = (input[6] >> 12) & 131071;
    output[13] = (input[6] >> 29) | ((input[7] & 16383) << (17 - 14));
    output[14] = (input[7] >> 14) & 131071;
    output[15] = (input[7] >> 31) | ((input[8] & 65535) << (17 - 16));
    output[16] = (input[8] >> 16) | ((input[9] & 1) << (17 - 1));
    output[17] = (input[9] >> 1) & 131071;
    output[18] = (input[9] >> 18) | ((input[10] & 7) << (17 - 3));
    output[19] = (input[10] >> 3) & 131071;
    output[20] = (input[10] >> 20) | ((input[11] & 31) << (17 - 5));
    output[21] = (input[11] >> 5) & 131071;
    output[22] = (input[11] >> 22) | ((input[12] & 127) << (17 - 7));
    output[23] = (input[12] >> 7) & 131071;
    output[24] = (input[12] >> 24) | ((input[13] & 511) << (17 - 9));
    output[25] = (input[13] >> 9) & 131071;
    output[26] = (input[13] >> 26) | ((input[14] & 2047) << (17 - 11));
    output[27] = (input[14] >> 11) & 131071;
    output[28] = (input[14] >> 28) | ((input[15] & 8191) << (17 - 13));
    output[29] = (input[15] >> 13) & 131071;
    output[30] = (input[15] >> 30) | ((input[16] & 32767) << (17 - 15));
    output[31] = input[16] >> 15;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack18(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 18];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 262143;
    output[1] = (input[0] >> 18) | ((input[1] & 15) << (18 - 4));
    output[2] = (input[1] >> 4) & 262143;
    output[3] = (input[1] >> 22) | ((input[2] & 255) << (18 - 8));
    output[4] = (input[2] >> 8) & 262143;
    output[5] = (input[2] >> 26) | ((input[3] & 4095) << (18 - 12));
    output[6] = (input[3] >> 12) & 262143;
    output[7] = (input[3] >> 30) | ((input[4] & 65535) << (18 - 16));
    output[8] = (input[4] >> 16) | ((input[5] & 3) << (18 - 2));
    output[9] = (input[5] >> 2) & 262143;
    output[10] = (input[5] >> 20) | ((input[6] & 63) << (18 - 6));
    output[11] = (input[6] >> 6) & 262143;
    output[12] = (input[6] >> 24) | ((input[7] & 1023) << (18 - 10));
    output[13] = (input[7] >> 10) & 262143;
    output[14] = (input[7] >> 28) | ((input[8] & 16383) << (18 - 14));
    output[15] = input[8] >> 14;
    output[16] = (input[9] >> 0) & 262143;
    output[17] = (input[9] >> 18) | ((input[10] & 15) << (18 - 4));
    output[18] = (input[10] >> 4) & 262143;
    output[19] = (input[10] >> 22) | ((input[11] & 255) << (18 - 8));
    output[20] = (input[11] >> 8) & 262143;
    output[21] = (input[11] >> 26) | ((input[12] & 4095) << (18 - 12));
    output[22] = (input[12] >> 12) & 262143;
    output[23] = (input[12] >> 30) | ((input[13] & 65535) << (18 - 16));
    output[24] = (input[13] >> 16) | ((input[14] & 3) << (18 - 2));
    output[25] = (input[14] >> 2) & 262143;
    output[26] = (input[14] >> 20) | ((input[15] & 63) << (18 - 6));
    output[27] = (input[15] >> 6) & 262143;
    output[28] = (input[15] >> 24) | ((input[16] & 1023) << (18 - 10));
    output[29] = (input[16] >> 10) & 262143;
    output[30] = (input[16] >> 28) | ((input[17] & 16383) << (18 - 14));
    output[31] = input[17] >> 14;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack19(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 19];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 524287;
    output[1] = (input[0] >> 19) | ((input[1] & 63) << (19 - 6));
    output[2] = (input[1] >> 6) & 524287;
    output[3] = (input[1] >> 25) | ((input[2] & 4095) << (19 - 12));
    output[4] = (input[2] >> 12) & 524287;
    output[5] = (input[2] >> 31) | ((input[3] & 262143) << (19 - 18));
    output[6] = (input[3] >> 18) | ((input[4] & 31) << (19 - 5));
    output[7] = (input[4] >> 5) & 524287;
    output[8] = (input[4] >> 24) | ((input[5] & 2047) << (19 - 11));
    output[9] = (input[5] >> 11) & 524287;
    output[10] = (input[5] >> 30) | ((input[6] & 131071) << (19 - 17));
    output[11] = (input[6] >> 17) | ((input[7] & 15) << (19 - 4));
    output[12] = (input[7] >> 4) & 524287;
    output[13] = (input[7] >> 23) | ((input[8] & 1023) << (19 - 10));
    output[14] = (input[8] >> 10) & 524287;
    output[15] = (input[8] >> 29) | ((input[9] & 65535) << (19 - 16));
    output[16] = (input[9] >> 16) | ((input[10] & 7) << (19 - 3));
    output[17] = (input[10] >> 3) & 524287;
    output[18] = (input[10] >> 22) | ((input[11] & 511) << (19 - 9));
    output[19] = (input[11] >> 9) & 524287;
    output[20] = (input[11] >> 28) | ((input[12] & 32767) << (19 - 15));
    output[21] = (input[12] >> 15) | ((input[13] & 3) << (19 - 2));
    output[22] = (input[13] >> 2) & 524287;
    output[23] = (input[13] >> 21) | ((input[14] & 255) << (19 - 8));
    output[24] = (input[14] >> 8) & 524287;
    output[25] = (input[14] >> 27) | ((input[15] & 16383) << (19 - 14));
    output[26] = (input[15] >> 14) | ((input[16] & 1) << (19 - 1));
    output[27] = (input[16] >> 1) & 524287;
    output[28] = (input[16] >> 20) | ((input[17] & 127) << (19 - 7));
    output[29] = (input[17] >> 7) & 524287;
    output[30] = (input[17] >> 26) | ((input[18] & 8191) << (19 - 13));
    output[31] = input[18] >> 13;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack20(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 20];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 1048575;
    output[1] = (input[0] >> 20) | ((input[1] & 255) << (20 - 8));
    output[2] = (input[1] >> 8) & 1048575;
    output[3] = (input[1] >> 28) | ((input[2] & 65535) << (20 - 16));
    output[4] = (input[2] >> 16) | ((input[3] & 15) << (20 - 4));
    output[5] = (input[3] >> 4) & 1048575;
    output[6] = (input[3] >> 24) | ((input[4] & 4095) << (20 - 12));
    output[7] = input[4] >> 12;
    output[8] = (input[5] >> 0) & 1048575;
    output[9] = (input[5] >> 20) | ((input[6] & 255) << (20 - 8));
    output[10] = (input[6] >> 8) & 1048575;
    output[11] = (input[6] >> 28) | ((input[7] & 65535) << (20 - 16));
    output[12] = (input[7] >> 16) | ((input[8] & 15) << (20 - 4));
    output[13] = (input[8] >> 4) & 1048575;
    output[14] = (input[8] >> 24) | ((input[9] & 4095) << (20 - 12));
    output[15] = input[9] >> 12;
    output[16] = (input[10] >> 0) & 1048575;
    output[17] = (input[10] >> 20) | ((input[11] & 255) << (20 - 8));
    output[18] = (input[11] >> 8) & 1048575;
    output[19] = (input[11] >> 28) | ((input[12] & 65535) << (20 - 16));
    output[20] = (input[12] >> 16) | ((input[13] & 15) << (20 - 4));
    output[21] = (input[13] >> 4) & 1048575;
    output[22] = (input[13] >> 24) | ((input[14] & 4095) << (20 - 12));
    output[23] = input[14] >> 12;
    output[24] = (input[15] >> 0) & 1048575;
    output[25] = (input[15] >> 20) | ((input[16] & 255) << (20 - 8));
    output[26] = (input[16] >> 8) & 1048575;
    output[27] = (input[16] >> 28) | ((input[17] & 65535) << (20 - 16));
    output[28] = (input[17] >> 16) | ((input[18] & 15) << (20 - 4));
    output[29] = (input[18] >> 4) & 1048575;
    output[30] = (input[18] >> 24) | ((input[19] & 4095) << (20 - 12));
    output[31] = input[19] >> 12;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack21(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 21];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 2097151;
    output[1] = (input[0] >> 21) | ((input[1] & 1023) << (21 - 10));
    output[2] = (input[1] >> 10) & 2097151;
    output[3] = (input[1] >> 31) | ((input[2] & 1048575) << (21 - 20));
    output[4] = (input[2] >> 20) | ((input[3] & 511) << (21 - 9));
    output[5] = (input[3] >> 9) & 2097151;
    output[6] = (input[3] >> 30) | ((input[4] & 524287) << (21 - 19));
    output[7] = (input[4] >> 19) | ((input[5] & 255) << (21 - 8));
    output[8] = (input[5] >> 8) & 2097151;
    output[9] = (input[5] >> 29) | ((input[6] & 262143) << (21 - 18));
    output[10] = (input[6] >> 18) | ((input[7] & 127) << (21 - 7));
    output[11] = (input[7] >> 7) & 2097151;
    output[12] = (input[7] >> 28) | ((input[8] & 131071) << (21 - 17));
    output[13] = (input[8] >> 17) | ((input[9] & 63) << (21 - 6));
    output[14] = (input[9] >> 6) & 2097151;
    output[15] = (input[9] >> 27) | ((input[10] & 65535) << (21 - 16));
    output[16] = (input[10] >> 16) | ((input[11] & 31) << (21 - 5));
    output[17] = (input[11] >> 5) & 2097151;
    output[18] = (input[11] >> 26) | ((input[12] & 32767) << (21 - 15));
    output[19] = (input[12] >> 15) | ((input[13] & 15) << (21 - 4));
    output[20] = (input[13] >> 4) & 2097151;
    output[21] = (input[13] >> 25) | ((input[14] & 16383) << (21 - 14));
    output[22] = (input[14] >> 14) | ((input[15] & 7) << (21 - 3));
    output[23] = (input[15] >> 3) & 2097151;
    output[24] = (input[15] >> 24) | ((input[16] & 8191) << (21 - 13));
    output[25] = (input[16] >> 13) | ((input[17] & 3) << (21 - 2));
    output[26] = (input[17] >> 2) & 2097151;
    output[27] = (input[17] >> 23) | ((input[18] & 4095) << (21 - 12));
    output[28] = (input[18] >> 12) | ((input[19] & 1) << (21 - 1));
    output[29] = (input[19] >> 1) & 2097151;
    output[30] = (input[19] >> 22) | ((input[20] & 2047) << (21 - 11));
    output[31] = input[20] >> 11;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack22(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 22];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 4194303;
    output[1] = (input[0] >> 22) | ((input[1] & 4095) << (22 - 12));
    output[2] = (input[1] >> 12) | ((input[2] & 3) << (22 - 2));
    output[3] = (input[2] >> 2) & 4194303;
    output[4] = (input[2] >> 24) | ((input[3] & 16383) << (22 - 14));
    output[5] = (input[3] >> 14) | ((input[4] & 15) << (22 - 4));
    output[6] = (input[4] >> 4) & 4194303;
    output[7] = (input[4] >> 26) | ((input[5] & 65535) << (22 - 16));
    output[8] = (input[5] >> 16) | ((input[6] & 63) << (22 - 6));
    output[9] = (input[6] >> 6) & 4194303;
    output[10] = (input[6] >> 28) | ((input[7] & 262143) << (22 - 18));
    output[11] = (input[7] >> 18) | ((input[8] & 255) << (22 - 8));
    output[12] = (input[8] >> 8) & 4194303;
    output[13] = (input[8] >> 30) | ((input[9] & 1048575) << (22 - 20));
    output[14] = (input[9] >> 20) | ((input[10] & 1023) << (22 - 10));
    output[15] = input[10] >> 10;
    output[16] = (input[11] >> 0) & 4194303;
    output[17] = (input[11] >> 22) | ((input[12] & 4095) << (22 - 12));
    output[18] = (input[12] >> 12) | ((input[13] & 3) << (22 - 2));
    output[19] = (input[13] >> 2) & 4194303;
    output[20] = (input[13] >> 24) | ((input[14] & 16383) << (22 - 14));
    output[21] = (input[14] >> 14) | ((input[15] & 15) << (22 - 4));
    output[22] = (input[15] >> 4) & 4194303;
    output[23] = (input[15] >> 26) | ((input[16] & 65535) << (22 - 16));
    output[24] = (input[16] >> 16) | ((input[17] & 63) << (22 - 6));
    output[25] = (input[17] >> 6) & 4194303;
    output[26] = (input[17] >> 28) | ((input[18] & 262143) << (22 - 18));
    output[27] = (input[18] >> 18) | ((input[19] & 255) << (22 - 8));
    output[28] = (input[19] >> 8) & 4194303;
    output[29] = (input[19] >> 30) | ((input[20] & 1048575) << (22 - 20));
    output[30] = (input[20] >> 20) | ((input[21] & 1023) << (22 - 10));
    output[31] = input[21] >> 10;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack23(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 23];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 8388607;
    output[1] = (input[0] >> 23) | ((input[1] & 16383) << (23 - 14));
    output[2] = (input[1] >> 14) | ((input[2] & 31) << (23 - 5));
    output[3] = (input[2] >> 5) & 8388607;
    output[4] = (input[2] >> 28) | ((input[3] & 524287) << (23 - 19));
    output[5] = (input[3] >> 19) | ((input[4] & 1023) << (23 - 10));
    output[6] = (input[4] >> 10) | ((input[5] & 1) << (23 - 1));
    output[7] = (input[5] >> 1) & 8388607;
    output[8] = (input[5] >> 24) | ((input[6] & 32767) << (23 - 15));
    output[9] = (input[6] >> 15) | ((input[7] & 63) << (23 - 6));
    output[10] = (input[7] >> 6) & 8388607;
    output[11] = (input[7] >> 29) | ((input[8] & 1048575) << (23 - 20));
    output[12] = (input[8] >> 20) | ((input[9] & 2047) << (23 - 11));
    output[13] = (input[9] >> 11) | ((input[10] & 3) << (23 - 2));
    output[14] = (input[10] >> 2) & 8388607;
    output[15] = (input[10] >> 25) | ((input[11] & 65535) << (23 - 16));
    output[16] = (input[11] >> 16) | ((input[12] & 127) << (23 - 7));
    output[17] = (input[12] >> 7) & 8388607;
    output[18] = (input[12] >> 30) | ((input[13] & 2097151) << (23 - 21));
    output[19] = (input[13] >> 21) | ((input[14] & 4095) << (23 - 12));
    output[20] = (input[14] >> 12) | ((input[15] & 7) << (23 - 3));
    output[21] = (input[15] >> 3) & 8388607;
    output[22] = (input[15] >> 26) | ((input[16] & 131071) << (23 - 17));
    output[23] = (input[16] >> 17) | ((input[17] & 255) << (23 - 8));
    output[24] = (input[17] >> 8) & 8388607;
    output[25] = (input[17] >> 31) | ((input[18] & 4194303) << (23 - 22));
    output[26] = (input[18] >> 22) | ((input[19] & 8191) << (23 - 13));
    output[27] = (input[19] >> 13) | ((input[20] & 15) << (23 - 4));
    output[28] = (input[20] >> 4) & 8388607;
    output[29] = (input[20] >> 27) | ((input[21] & 262143) << (23 - 18));
    output[30] = (input[21] >> 18) | ((input[22] & 511) << (23 - 9));
    output[31] = input[22] >> 9;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack24(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 24];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 16777215;
    output[1] = (input[0] >> 24) | ((input[1] & 65535) << (24 - 16));
    output[2] = (input[1] >> 16) | ((input[2] & 255) << (24 - 8));
    output[3] = input[2] >> 8;
    output[4] = (input[3] >> 0) & 16777215;
    output[5] = (input[3] >> 24) | ((input[4] & 65535) << (24 - 16));
    output[6] = (input[4] >> 16) | ((input[5] & 255) << (24 - 8));
    output[7] = input[5] >> 8;
    output[8] = (input[6] >> 0) & 16777215;
    output[9] = (input[6] >> 24) | ((input[7] & 65535) << (24 - 16));
    output[10] = (input[7] >> 16) | ((input[8] & 255) << (24 - 8));
    output[11] = input[8] >> 8;
    output[12] = (input[9] >> 0) & 16777215;
    output[13] = (input[9] >> 24) | ((input[10] & 65535) << (24 - 16));
    output[14] = (input[10] >> 16) | ((input[11] & 255) << (24 - 8));
    output[15] = input[11] >> 8;
    output[16] = (input[12] >> 0) & 16777215;
    output[17] = (input[12] >> 24) | ((input[13] & 65535) << (24 - 16));
    output[18] = (input[13] >> 16) | ((input[14] & 255) << (24 - 8));
    output[19] = input[14] >> 8;
    output[20] = (input[15] >> 0) & 16777215;
    output[21] = (input[15] >> 24) | ((input[16] & 65535) << (24 - 16));
    output[22] = (input[16] >> 16) | ((input[17] & 255) << (24 - 8));
    output[23] = input[17] >> 8;
    output[24] = (input[18] >> 0) & 16777215;
    output[25] = (input[18] >> 24) | ((input[19] & 65535) << (24 - 16));
    output[26] = (input[19] >> 16) | ((input[20] & 255) << (24 - 8));
    output[27] = input[20] >> 8;
    output[28] = (input[21] >> 0) & 16777215;
    output[29] = (input[21] >> 24) | ((input[22] & 65535) << (24 - 16));
    output[30] = (input[22] >> 16) | ((input[23] & 255) << (24 - 8));
    output[31] = input[23] >> 8;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack25(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 25];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 33554431;
    output[1] = (input[0] >> 25) | ((input[1] & 262143) << (25 - 18));
    output[2] = (input[1] >> 18) | ((input[2] & 2047) << (25 - 11));
    output[3] = (input[2] >> 11) | ((input[3] & 15) << (25 - 4));
    output[4] = (input[3] >> 4) & 33554431;
    output[5] = (input[3] >> 29) | ((input[4] & 4194303) << (25 - 22));
    output[6] = (input[4] >> 22) | ((input[5] & 32767) << (25 - 15));
    output[7] = (input[5] >> 15) | ((input[6] & 255) << (25 - 8));
    output[8] = (input[6] >> 8) | ((input[7] & 1) << (25 - 1));
    output[9] = (input[7] >> 1) & 33554431;
    output[10] = (input[7] >> 26) | ((input[8] & 524287) << (25 - 19));
    output[11] = (input[8] >> 19) | ((input[9] & 4095) << (25 - 12));
    output[12] = (input[9] >> 12) | ((input[10] & 31) << (25 - 5));
    output[13] = (input[10] >> 5) & 33554431;
    output[14] = (input[10] >> 30) | ((input[11] & 8388607) << (25 - 23));
    output[15] = (input[11] >> 23) | ((input[12] & 65535) << (25 - 16));
    output[16] = (input[12] >> 16) | ((input[13] & 511) << (25 - 9));
    output[17] = (input[13] >> 9) | ((input[14] & 3) << (25 - 2));
    output[18] = (input[14] >> 2) & 33554431;
    output[19] = (input[14] >> 27) | ((input[15] & 1048575) << (25 - 20));
    output[20] = (input[15] >> 20) | ((input[16] & 8191) << (25 - 13));
    output[21] = (input[16] >> 13) | ((input[17] & 63) << (25 - 6));
    output[22] = (input[17] >> 6) & 33554431;
    output[23] = (input[17] >> 31) | ((input[18] & 16777215) << (25 - 24));
    output[24] = (input[18] >> 24) | ((input[19] & 131071) << (25 - 17));
    output[25] = (input[19] >> 17) | ((input[20] & 1023) << (25 - 10));
    output[26] = (input[20] >> 10) | ((input[21] & 7) << (25 - 3));
    output[27] = (input[21] >> 3) & 33554431;
    output[28] = (input[21] >> 28) | ((input[22] & 2097151) << (25 - 21));
    output[29] = (input[22] >> 21) | ((input[23] & 16383) << (25 - 14));
    output[30] = (input[23] >> 14) | ((input[24] & 127) << (25 - 7));
    output[31] = input[24] >> 7;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack26(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 26];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 67108863;
    output[1] = (input[0] >> 26) | ((input[1] & 1048575) << (26 - 20));
    output[2] = (input[1] >> 20) | ((input[2] & 16383) << (26 - 14));
    output[3] = (input[2] >> 14) | ((input[3] & 255) << (26 - 8));
    output[4] = (input[3] >> 8) | ((input[4] & 3) << (26 - 2));
    output[5] = (input[4] >> 2) & 67108863;
    output[6] = (input[4] >> 28) | ((input[5] & 4194303) << (26 - 22));
    output[7] = (input[5] >> 22) | ((input[6] & 65535) << (26 - 16));
    output[8] = (input[6] >> 16) | ((input[7] & 1023) << (26 - 10));
    output[9] = (input[7] >> 10) | ((input[8] & 15) << (26 - 4));
    output[10] = (input[8] >> 4) & 67108863;
    output[11] = (input[8] >> 30) | ((input[9] & 16777215) << (26 - 24));
    output[12] = (input[9] >> 24) | ((input[10] & 262143) << (26 - 18));
    output[13] = (input[10] >> 18) | ((input[11] & 4095) << (26 - 12));
    output[14] = (input[11] >> 12) | ((input[12] & 63) << (26 - 6));
    output[15] = input[12] >> 6;
    output[16] = (input[13] >> 0) & 67108863;
    output[17] = (input[13] >> 26) | ((input[14] & 1048575) << (26 - 20));
    output[18] = (input[14] >> 20) | ((input[15] & 16383) << (26 - 14));
    output[19] = (input[15] >> 14) | ((input[16] & 255) << (26 - 8));
    output[20] = (input[16] >> 8) | ((input[17] & 3) << (26 - 2));
    output[21] = (input[17] >> 2) & 67108863;
    output[22] = (input[17] >> 28) | ((input[18] & 4194303) << (26 - 22));
    output[23] = (input[18] >> 22) | ((input[19] & 65535) << (26 - 16));
    output[24] = (input[19] >> 16) | ((input[20] & 1023) << (26 - 10));
    output[25] = (input[20] >> 10) | ((input[21] & 15) << (26 - 4));
    output[26] = (input[21] >> 4) & 67108863;
    output[27] = (input[21] >> 30) | ((input[22] & 16777215) << (26 - 24));
    output[28] = (input[22] >> 24) | ((input[23] & 262143) << (26 - 18));
    output[29] = (input[23] >> 18) | ((input[24] & 4095) << (26 - 12));
    output[30] = (input[24] >> 12) | ((input[25] & 63) << (26 - 6));
    output[31] = input[25] >> 6;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack27(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 27];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 134217727;
    output[1] = (input[0] >> 27) | ((input[1] & 4194303) << (27 - 22));
    output[2] = (input[1] >> 22) | ((input[2] & 131071) << (27 - 17));
    output[3] = (input[2] >> 17) | ((input[3] & 4095) << (27 - 12));
    output[4] = (input[3] >> 12) | ((input[4] & 127) << (27 - 7));
    output[5] = (input[4] >> 7) | ((input[5] & 3) << (27 - 2));
    output[6] = (input[5] >> 2) & 134217727;
    output[7] = (input[5] >> 29) | ((input[6] & 16777215) << (27 - 24));
    output[8] = (input[6] >> 24) | ((input[7] & 524287) << (27 - 19));
    output[9] = (input[7] >> 19) | ((input[8] & 16383) << (27 - 14));
    output[10] = (input[8] >> 14) | ((input[9] & 511) << (27 - 9));
    output[11] = (input[9] >> 9) | ((input[10] & 15) << (27 - 4));
    output[12] = (input[10] >> 4) & 134217727;
    output[13] = (input[10] >> 31) | ((input[11] & 67108863) << (27 - 26));
    output[14] = (input[11] >> 26) | ((input[12] & 2097151) << (27 - 21));
    output[15] = (input[12] >> 21) | ((input[13] & 65535) << (27 - 16));
    output[16] = (input[13] >> 16) | ((input[14] & 2047) << (27 - 11));
    output[17] = (input[14] >> 11) | ((input[15] & 63) << (27 - 6));
    output[18] = (input[15] >> 6) | ((input[16] & 1) << (27 - 1));
    output[19] = (input[16] >> 1) & 134217727;
    output[20] = (input[16] >> 28) | ((input[17] & 8388607) << (27 - 23));
    output[21] = (input[17] >> 23) | ((input[18] & 262143) << (27 - 18));
    output[22] = (input[18] >> 18) | ((input[19] & 8191) << (27 - 13));
    output[23] = (input[19] >> 13) | ((input[20] & 255) << (27 - 8));
    output[24] = (input[20] >> 8) | ((input[21] & 7) << (27 - 3));
    output[25] = (input[21] >> 3) & 134217727;
    output[26] = (input[21] >> 30) | ((input[22] & 33554431) << (27 - 25));
    output[27] = (input[22] >> 25) | ((input[23] & 1048575) << (27 - 20));
    output[28] = (input[23] >> 20) | ((input[24] & 32767) << (27 - 15));
    output[29] = (input[24] >> 15) | ((input[25] & 1023) << (27 - 10));
    output[30] = (input[25] >> 10) | ((input[26] & 31) << (27 - 5));
    output[31] = input[26] >> 5;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack28(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 28];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 268435455;
    output[1] = (input[0] >> 28) | ((input[1] & 16777215) << (28 - 24));
    output[2] = (input[1] >> 24) | ((input[2] & 1048575) << (28 - 20));
    output[3] = (input[2] >> 20) | ((input[3] & 65535) << (28 - 16));
    output[4] = (input[3] >> 16) | ((input[4] & 4095) << (28 - 12));
    output[5] = (input[4] >> 12) | ((input[5] & 255) << (28 - 8));
    output[6] = (input[5] >> 8) | ((input[6] & 15) << (28 - 4));
    output[7] = input[6] >> 4;
    output[8] = (input[7] >> 0) & 268435455;
    output[9] = (input[7] >> 28) | ((input[8] & 16777215) << (28 - 24));
    output[10] = (input[8] >> 24) | ((input[9] & 1048575) << (28 - 20));
    output[11] = (input[9] >> 20) | ((input[10] & 65535) << (28 - 16));
    output[12] = (input[10] >> 16) | ((input[11] & 4095) << (28 - 12));
    output[13] = (input[11] >> 12) | ((input[12] & 255) << (28 - 8));
    output[14] = (input[12] >> 8) | ((input[13] & 15) << (28 - 4));
    output[15] = input[13] >> 4;
    output[16] = (input[14] >> 0) & 268435455;
    output[17] = (input[14] >> 28) | ((input[15] & 16777215) << (28 - 24));
    output[18] = (input[15] >> 24) | ((input[16] & 1048575) << (28 - 20));
    output[19] = (input[16] >> 20) | ((input[17] & 65535) << (28 - 16));
    output[20] = (input[17] >> 16) | ((input[18] & 4095) << (28 - 12));
    output[21] = (input[18] >> 12) | ((input[19] & 255) << (28 - 8));
    output[22] = (input[19] >> 8) | ((input[20] & 15) << (28 - 4));
    output[23] = input[20] >> 4;
    output[24] = (input[21] >> 0) & 268435455;
    output[25] = (input[21] >> 28) | ((input[22] & 16777215) << (28 - 24));
    output[26] = (input[22] >> 24) | ((input[23] & 1048575) << (28 - 20));
    output[27] = (input[23] >> 20) | ((input[24] & 65535) << (28 - 16));
    output[28] = (input[24] >> 16) | ((input[25] & 4095) << (28 - 12));
    output[29] = (input[25] >> 12) | ((input[26] & 255) << (28 - 8));
    output[30] = (input[26] >> 8) | ((input[27] & 15) << (28 - 4));
    output[31] = input[27] >> 4;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack29(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 29];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 536870911;
    output[1] = (input[0] >> 29) | ((input[1] & 67108863) << (29 - 26));
    output[2] = (input[1] >> 26) | ((input[2] & 8388607) << (29 - 23));
    output[3] = (input[2] >> 23) | ((input[3] & 1048575) << (29 - 20));
    output[4] = (input[3] >> 20) | ((input[4] & 131071) << (29 - 17));
    output[5] = (input[4] >> 17) | ((input[5] & 16383) << (29 - 14));
    output[6] = (input[5] >> 14) | ((input[6] & 2047) << (29 - 11));
    output[7] = (input[6] >> 11) | ((input[7] & 255) << (29 - 8));
    output[8] = (input[7] >> 8) | ((input[8] & 31) << (29 - 5));
    output[9] = (input[8] >> 5) | ((input[9] & 3) << (29 - 2));
    output[10] = (input[9] >> 2) & 536870911;
    output[11] = (input[9] >> 31) | ((input[10] & 268435455) << (29 - 28));
    output[12] = (input[10] >> 28) | ((input[11] & 33554431) << (29 - 25));
    output[13] = (input[11] >> 25) | ((input[12] & 4194303) << (29 - 22));
    output[14] = (input[12] >> 22) | ((input[13] & 524287) << (29 - 19));
    output[15] = (input[13] >> 19) | ((input[14] & 65535) << (29 - 16));
    output[16] = (input[14] >> 16) | ((input[15] & 8191) << (29 - 13));
    output[17] = (input[15] >> 13) | ((input[16] & 1023) << (29 - 10));
    output[18] = (input[16] >> 10) | ((input[17] & 127) << (29 - 7));
    output[19] = (input[17] >> 7) | ((input[18] & 15) << (29 - 4));
    output[20] = (input[18] >> 4) | ((input[19] & 1) << (29 - 1));
    output[21] = (input[19] >> 1) & 536870911;
    output[22] = (input[19] >> 30) | ((input[20] & 134217727) << (29 - 27));
    output[23] = (input[20] >> 27) | ((input[21] & 16777215) << (29 - 24));
    output[24] = (input[21] >> 24) | ((input[22] & 2097151) << (29 - 21));
    output[25] = (input[22] >> 21) | ((input[23] & 262143) << (29 - 18));
    output[26] = (input[23] >> 18) | ((input[24] & 32767) << (29 - 15));
    output[27] = (input[24] >> 15) | ((input[25] & 4095) << (29 - 12));
    output[28] = (input[25] >> 12) | ((input[26] & 511) << (29 - 9));
    output[29] = (input[26] >> 9) | ((input[27] & 63) << (29 - 6));
    output[30] = (input[27] >> 6) | ((input[28] & 7) << (29 - 3));
    output[31] = input[28] >> 3;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack30(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 30];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 1073741823;
    output[1] = (input[0] >> 30) | ((input[1] & 268435455) << (30 - 28));
    output[2] = (input[1] >> 28) | ((input[2] & 67108863) << (30 - 26));
    output[3] = (input[2] >> 26) | ((input[3] & 16777215) << (30 - 24));
    output[4] = (input[3] >> 24) | ((input[4] & 4194303) << (30 - 22));
    output[5] = (input[4] >> 22) | ((input[5] & 1048575) << (30 - 20));
    output[6] = (input[5] >> 20) | ((input[6] & 262143) << (30 - 18));
    output[7] = (input[6] >> 18) | ((input[7] & 65535) << (30 - 16));
    output[8] = (input[7] >> 16) | ((input[8] & 16383) << (30 - 14));
    output[9] = (input[8] >> 14) | ((input[9] & 4095) << (30 - 12));
    output[10] = (input[9] >> 12) | ((input[10] & 1023) << (30 - 10));
    output[11] = (input[10] >> 10) | ((input[11] & 255) << (30 - 8));
    output[12] = (input[11] >> 8) | ((input[12] & 63) << (30 - 6));
    output[13] = (input[12] >> 6) | ((input[13] & 15) << (30 - 4));
    output[14] = (input[13] >> 4) | ((input[14] & 3) << (30 - 2));
    output[15] = input[14] >> 2;
    output[16] = (input[15] >> 0) & 1073741823;
    output[17] = (input[15] >> 30) | ((input[16] & 268435455) << (30 - 28));
    output[18] = (input[16] >> 28) | ((input[17] & 67108863) << (30 - 26));
    output[19] = (input[17] >> 26) | ((input[18] & 16777215) << (30 - 24));
    output[20] = (input[18] >> 24) | ((input[19] & 4194303) << (30 - 22));
    output[21] = (input[19] >> 22) | ((input[20] & 1048575) << (30 - 20));
    output[22] = (input[20] >> 20) | ((input[21] & 262143) << (30 - 18));
    output[23] = (input[21] >> 18) | ((input[22] & 65535) << (30 - 16));
    output[24] = (input[22] >> 16) | ((input[23] & 16383) << (30 - 14));
    output[25] = (input[23] >> 14) | ((input[24] & 4095) << (30 - 12));
    output[26] = (input[24] >> 12) | ((input[25] & 1023) << (30 - 10));
    output[27] = (input[25] >> 10) | ((input[26] & 255) << (30 - 8));
    output[28] = (input[26] >> 8) | ((input[27] & 63) << (30 - 6));
    output[29] = (input[27] >> 6) | ((input[28] & 15) << (30 - 4));
    output[30] = (input[28] >> 4) | ((input[29] & 3) << (30 - 2));
    output[31] = input[29] >> 2;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack31(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 31];
    let output = &mut output[outpos..outpos + 32];
    output[0] = (input[0] >> 0) & 2147483647;
    output[1] = (input[0] >> 31) | ((input[1] & 1073741823) << (31 - 30));
    output[2] = (input[1] >> 30) | ((input[2] & 536870911) << (31 - 29));
    output[3] = (input[2] >> 29) | ((input[3] & 268435455) << (31 - 28));
    output[4] = (input[3] >> 28) | ((input[4] & 134217727) << (31 - 27));
    output[5] = (input[4] >> 27) | ((input[5] & 67108863) << (31 - 26));
    output[6] = (input[5] >> 26) | ((input[6] & 33554431) << (31 - 25));
    output[7] = (input[6] >> 25) | ((input[7] & 16777215) << (31 - 24));
    output[8] = (input[7] >> 24) | ((input[8] & 8388607) << (31 - 23));
    output[9] = (input[8] >> 23) | ((input[9] & 4194303) << (31 - 22));
    output[10] = (input[9] >> 22) | ((input[10] & 2097151) << (31 - 21));
    output[11] = (input[10] >> 21) | ((input[11] & 1048575) << (31 - 20));
    output[12] = (input[11] >> 20) | ((input[12] & 524287) << (31 - 19));
    output[13] = (input[12] >> 19) | ((input[13] & 262143) << (31 - 18));
    output[14] = (input[13] >> 18) | ((input[14] & 131071) << (31 - 17));
    output[15] = (input[14] >> 17) | ((input[15] & 65535) << (31 - 16));
    output[16] = (input[15] >> 16) | ((input[16] & 32767) << (31 - 15));
    output[17] = (input[16] >> 15) | ((input[17] & 16383) << (31 - 14));
    output[18] = (input[17] >> 14) | ((input[18] & 8191) << (31 - 13));
    output[19] = (input[18] >> 13) | ((input[19] & 4095) << (31 - 12));
    output[20] = (input[19] >> 12) | ((input[20] & 2047) << (31 - 11));
    output[21] = (input[20] >> 11) | ((input[21] & 1023) << (31 - 10));
    output[22] = (input[21] >> 10) | ((input[22] & 511) << (31 - 9));
    output[23] = (input[22] >> 9) | ((input[23] & 255) << (31 - 8));
    output[24] = (input[23] >> 8) | ((input[24] & 127) << (31 - 7));
    output[25] = (input[24] >> 7) | ((input[25] & 63) << (31 - 6));
    output[26] = (input[25] >> 6) | ((input[26] & 31) << (31 - 5));
    output[27] = (input[26] >> 5) | ((input[27] & 15) << (31 - 4));
    output[28] = (input[27] >> 4) | ((input[28] & 7) << (31 - 3));
    output[29] = (input[28] >> 3) | ((input[29] & 3) << (31 - 2));
    output[30] = (input[29] >> 2) | ((input[30] & 1) << (31 - 1));
    output[31] = input[30] >> 1;
}

#[expect(
    clippy::inline_always,
    reason = "critical hot path: inlining these 33 dispatch arms eliminates indirect calls in the decode loop"
)]
#[inline(always)]
fn fast_unpack32(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    let input = &input[inpos..inpos + 32];
    let output = &mut output[outpos..outpos + 32];
    output.copy_from_slice(input);
}
