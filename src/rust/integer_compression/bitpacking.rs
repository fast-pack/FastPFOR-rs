#![expect(clippy::identity_op)]

/// Packs 32 integers from `input` into `output` using `bit` bits per integer.
pub fn fast_pack(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize, bit: u8) {
    match bit {
        0 => (),
        1 => fast_pack1(input, inpos, output, outpos),
        2 => fast_pack2(input, inpos, output, outpos),
        3 => fast_pack3(input, inpos, output, outpos),
        4 => fast_pack4(input, inpos, output, outpos),
        5 => fast_pack5(input, inpos, output, outpos),
        6 => fast_pack6(input, inpos, output, outpos),
        7 => fast_pack7(input, inpos, output, outpos),
        8 => fast_pack8(input, inpos, output, outpos),
        9 => fast_pack9(input, inpos, output, outpos),
        10 => fast_pack10(input, inpos, output, outpos),
        11 => fast_pack11(input, inpos, output, outpos),
        12 => fast_pack12(input, inpos, output, outpos),
        13 => fast_pack13(input, inpos, output, outpos),
        14 => fast_pack14(input, inpos, output, outpos),
        15 => fast_pack15(input, inpos, output, outpos),
        16 => fast_pack16(input, inpos, output, outpos),
        17 => fast_pack17(input, inpos, output, outpos),
        18 => fast_pack18(input, inpos, output, outpos),
        19 => fast_pack19(input, inpos, output, outpos),
        20 => fast_pack20(input, inpos, output, outpos),
        21 => fast_pack21(input, inpos, output, outpos),
        22 => fast_pack22(input, inpos, output, outpos),
        23 => fast_pack23(input, inpos, output, outpos),
        24 => fast_pack24(input, inpos, output, outpos),
        25 => fast_pack25(input, inpos, output, outpos),
        26 => fast_pack26(input, inpos, output, outpos),
        27 => fast_pack27(input, inpos, output, outpos),
        28 => fast_pack28(input, inpos, output, outpos),
        29 => fast_pack29(input, inpos, output, outpos),
        30 => fast_pack30(input, inpos, output, outpos),
        31 => fast_pack31(input, inpos, output, outpos),
        32 => fast_pack32(input, inpos, output, outpos),
        _ => panic!("Unsupported bit width"),
    }
}

fn fast_pack1(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[outpos] = (input[inpos + 0] & 1)
        | ((input[inpos + 1] & 1) << 1)
        | ((input[inpos + 2] & 1) << 2)
        | ((input[inpos + 3] & 1) << 3)
        | ((input[inpos + 4] & 1) << 4)
        | ((input[inpos + 5] & 1) << 5)
        | ((input[inpos + 6] & 1) << 6)
        | ((input[inpos + 7] & 1) << 7)
        | ((input[inpos + 8] & 1) << 8)
        | ((input[inpos + 9] & 1) << 9)
        | ((input[inpos + 10] & 1) << 10)
        | ((input[inpos + 11] & 1) << 11)
        | ((input[inpos + 12] & 1) << 12)
        | ((input[inpos + 13] & 1) << 13)
        | ((input[inpos + 14] & 1) << 14)
        | ((input[inpos + 15] & 1) << 15)
        | ((input[inpos + 16] & 1) << 16)
        | ((input[inpos + 17] & 1) << 17)
        | ((input[inpos + 18] & 1) << 18)
        | ((input[inpos + 19] & 1) << 19)
        | ((input[inpos + 20] & 1) << 20)
        | ((input[inpos + 21] & 1) << 21)
        | ((input[inpos + 22] & 1) << 22)
        | ((input[inpos + 23] & 1) << 23)
        | ((input[inpos + 24] & 1) << 24)
        | ((input[inpos + 25] & 1) << 25)
        | ((input[inpos + 26] & 1) << 26)
        | ((input[inpos + 27] & 1) << 27)
        | ((input[inpos + 28] & 1) << 28)
        | ((input[inpos + 29] & 1) << 29)
        | ((input[inpos + 30] & 1) << 30)
        | ((input[inpos + 31] & 1) << 31);
}

fn fast_pack2(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 3)
        | ((input[1 + inpos] & 3) << 2)
        | ((input[2 + inpos] & 3) << 4)
        | ((input[3 + inpos] & 3) << 6)
        | ((input[4 + inpos] & 3) << 8)
        | ((input[5 + inpos] & 3) << 10)
        | ((input[6 + inpos] & 3) << 12)
        | ((input[7 + inpos] & 3) << 14)
        | ((input[8 + inpos] & 3) << 16)
        | ((input[9 + inpos] & 3) << 18)
        | ((input[10 + inpos] & 3) << 20)
        | ((input[11 + inpos] & 3) << 22)
        | ((input[12 + inpos] & 3) << 24)
        | ((input[13 + inpos] & 3) << 26)
        | ((input[14 + inpos] & 3) << 28)
        | (input[15 + inpos] << 30);
    output[1 + outpos] = (input[16 + inpos] & 3)
        | ((input[17 + inpos] & 3) << 2)
        | ((input[18 + inpos] & 3) << 4)
        | ((input[19 + inpos] & 3) << 6)
        | ((input[20 + inpos] & 3) << 8)
        | ((input[21 + inpos] & 3) << 10)
        | ((input[22 + inpos] & 3) << 12)
        | ((input[23 + inpos] & 3) << 14)
        | ((input[24 + inpos] & 3) << 16)
        | ((input[25 + inpos] & 3) << 18)
        | ((input[26 + inpos] & 3) << 20)
        | ((input[27 + inpos] & 3) << 22)
        | ((input[28 + inpos] & 3) << 24)
        | ((input[29 + inpos] & 3) << 26)
        | ((input[30 + inpos] & 3) << 28)
        | (input[31 + inpos] << 30);
}

fn fast_pack3(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 7)
        | ((input[1 + inpos] & 7) << 3)
        | ((input[2 + inpos] & 7) << 6)
        | ((input[3 + inpos] & 7) << 9)
        | ((input[4 + inpos] & 7) << 12)
        | ((input[5 + inpos] & 7) << 15)
        | ((input[6 + inpos] & 7) << 18)
        | ((input[7 + inpos] & 7) << 21)
        | ((input[8 + inpos] & 7) << 24)
        | ((input[9 + inpos] & 7) << 27)
        | ((input[10 + inpos]) << 30);
    output[1 + outpos] = ((input[10 + inpos] & 7) >> (3 - 1))
        | ((input[11 + inpos] & 7) << 1)
        | ((input[12 + inpos] & 7) << 4)
        | ((input[13 + inpos] & 7) << 7)
        | ((input[14 + inpos] & 7) << 10)
        | ((input[15 + inpos] & 7) << 13)
        | ((input[16 + inpos] & 7) << 16)
        | ((input[17 + inpos] & 7) << 19)
        | ((input[18 + inpos] & 7) << 22)
        | ((input[19 + inpos] & 7) << 25)
        | ((input[20 + inpos] & 7) << 28)
        | ((input[21 + inpos]) << 31);
    output[2 + outpos] = ((input[21 + inpos] & 7) >> (3 - 2))
        | ((input[22 + inpos] & 7) << 2)
        | ((input[23 + inpos] & 7) << 5)
        | ((input[24 + inpos] & 7) << 8)
        | ((input[25 + inpos] & 7) << 11)
        | ((input[26 + inpos] & 7) << 14)
        | ((input[27 + inpos] & 7) << 17)
        | ((input[28 + inpos] & 7) << 20)
        | ((input[29 + inpos] & 7) << 23)
        | ((input[30 + inpos] & 7) << 26)
        | ((input[31 + inpos]) << 29);
}

fn fast_pack4(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 15)
        | ((input[1 + inpos] & 15) << 4)
        | ((input[2 + inpos] & 15) << 8)
        | ((input[3 + inpos] & 15) << 12)
        | ((input[4 + inpos] & 15) << 16)
        | ((input[5 + inpos] & 15) << 20)
        | ((input[6 + inpos] & 15) << 24)
        | ((input[7 + inpos]) << 28);
    output[1 + outpos] = (input[8 + inpos] & 15)
        | ((input[9 + inpos] & 15) << 4)
        | ((input[10 + inpos] & 15) << 8)
        | ((input[11 + inpos] & 15) << 12)
        | ((input[12 + inpos] & 15) << 16)
        | ((input[13 + inpos] & 15) << 20)
        | ((input[14 + inpos] & 15) << 24)
        | ((input[15 + inpos]) << 28);
    output[2 + outpos] = (input[16 + inpos] & 15)
        | ((input[17 + inpos] & 15) << 4)
        | ((input[18 + inpos] & 15) << 8)
        | ((input[19 + inpos] & 15) << 12)
        | ((input[20 + inpos] & 15) << 16)
        | ((input[21 + inpos] & 15) << 20)
        | ((input[22 + inpos] & 15) << 24)
        | ((input[23 + inpos]) << 28);
    output[3 + outpos] = (input[24 + inpos] & 15)
        | ((input[25 + inpos] & 15) << 4)
        | ((input[26 + inpos] & 15) << 8)
        | ((input[27 + inpos] & 15) << 12)
        | ((input[28 + inpos] & 15) << 16)
        | ((input[29 + inpos] & 15) << 20)
        | ((input[30 + inpos] & 15) << 24)
        | ((input[31 + inpos]) << 28);
}

fn fast_pack5(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 31)
        | ((input[1 + inpos] & 31) << 5)
        | ((input[2 + inpos] & 31) << 10)
        | ((input[3 + inpos] & 31) << 15)
        | ((input[4 + inpos] & 31) << 20)
        | ((input[5 + inpos] & 31) << 25)
        | ((input[6 + inpos]) << 30);
    output[1 + outpos] = ((input[6 + inpos] & 31) >> (5 - 3))
        | ((input[7 + inpos] & 31) << 3)
        | ((input[8 + inpos] & 31) << 8)
        | ((input[9 + inpos] & 31) << 13)
        | ((input[10 + inpos] & 31) << 18)
        | ((input[11 + inpos] & 31) << 23)
        | ((input[12 + inpos]) << 28);
    output[2 + outpos] = ((input[12 + inpos] & 31) >> (5 - 1))
        | ((input[13 + inpos] & 31) << 1)
        | ((input[14 + inpos] & 31) << 6)
        | ((input[15 + inpos] & 31) << 11)
        | ((input[16 + inpos] & 31) << 16)
        | ((input[17 + inpos] & 31) << 21)
        | ((input[18 + inpos] & 31) << 26)
        | ((input[19 + inpos]) << 31);
    output[3 + outpos] = ((input[19 + inpos] & 31) >> (5 - 4))
        | ((input[20 + inpos] & 31) << 4)
        | ((input[21 + inpos] & 31) << 9)
        | ((input[22 + inpos] & 31) << 14)
        | ((input[23 + inpos] & 31) << 19)
        | ((input[24 + inpos] & 31) << 24)
        | ((input[25 + inpos]) << 29);
    output[4 + outpos] = ((input[25 + inpos] & 31) >> (5 - 2))
        | ((input[26 + inpos] & 31) << 2)
        | ((input[27 + inpos] & 31) << 7)
        | ((input[28 + inpos] & 31) << 12)
        | ((input[29 + inpos] & 31) << 17)
        | ((input[30 + inpos] & 31) << 22)
        | ((input[31 + inpos]) << 27);
}

fn fast_pack6(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 63)
        | ((input[1 + inpos] & 63) << 6)
        | ((input[2 + inpos] & 63) << 12)
        | ((input[3 + inpos] & 63) << 18)
        | ((input[4 + inpos] & 63) << 24)
        | ((input[5 + inpos]) << 30);
    output[1 + outpos] = ((input[5 + inpos] & 63) >> (6 - 4))
        | ((input[6 + inpos] & 63) << 4)
        | ((input[7 + inpos] & 63) << 10)
        | ((input[8 + inpos] & 63) << 16)
        | ((input[9 + inpos] & 63) << 22)
        | ((input[10 + inpos]) << 28);
    output[2 + outpos] = ((input[10 + inpos] & 63) >> (6 - 2))
        | ((input[11 + inpos] & 63) << 2)
        | ((input[12 + inpos] & 63) << 8)
        | ((input[13 + inpos] & 63) << 14)
        | ((input[14 + inpos] & 63) << 20)
        | ((input[15 + inpos]) << 26);
    output[3 + outpos] = (input[16 + inpos] & 63)
        | ((input[17 + inpos] & 63) << 6)
        | ((input[18 + inpos] & 63) << 12)
        | ((input[19 + inpos] & 63) << 18)
        | ((input[20 + inpos] & 63) << 24)
        | ((input[21 + inpos]) << 30);
    output[4 + outpos] = ((input[21 + inpos] & 63) >> (6 - 4))
        | ((input[22 + inpos] & 63) << 4)
        | ((input[23 + inpos] & 63) << 10)
        | ((input[24 + inpos] & 63) << 16)
        | ((input[25 + inpos] & 63) << 22)
        | ((input[26 + inpos]) << 28);
    output[5 + outpos] = ((input[26 + inpos] & 63) >> (6 - 2))
        | ((input[27 + inpos] & 63) << 2)
        | ((input[28 + inpos] & 63) << 8)
        | ((input[29 + inpos] & 63) << 14)
        | ((input[30 + inpos] & 63) << 20)
        | ((input[31 + inpos]) << 26);
}

fn fast_pack7(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 127)
        | ((input[1 + inpos] & 127) << 7)
        | ((input[2 + inpos] & 127) << 14)
        | ((input[3 + inpos] & 127) << 21)
        | ((input[4 + inpos]) << 28);
    output[1 + outpos] = ((input[4 + inpos] & 127) >> (7 - 3))
        | ((input[5 + inpos] & 127) << 3)
        | ((input[6 + inpos] & 127) << 10)
        | ((input[7 + inpos] & 127) << 17)
        | ((input[8 + inpos] & 127) << 24)
        | ((input[9 + inpos]) << 31);
    output[2 + outpos] = ((input[9 + inpos] & 127) >> (7 - 6))
        | ((input[10 + inpos] & 127) << 6)
        | ((input[11 + inpos] & 127) << 13)
        | ((input[12 + inpos] & 127) << 20)
        | ((input[13 + inpos]) << 27);
    output[3 + outpos] = ((input[13 + inpos] & 127) >> (7 - 2))
        | ((input[14 + inpos] & 127) << 2)
        | ((input[15 + inpos] & 127) << 9)
        | ((input[16 + inpos] & 127) << 16)
        | ((input[17 + inpos] & 127) << 23)
        | ((input[18 + inpos]) << 30);
    output[4 + outpos] = ((input[18 + inpos] & 127) >> (7 - 5))
        | ((input[19 + inpos] & 127) << 5)
        | ((input[20 + inpos] & 127) << 12)
        | ((input[21 + inpos] & 127) << 19)
        | ((input[22 + inpos]) << 26);
    output[5 + outpos] = ((input[22 + inpos] & 127) >> (7 - 1))
        | ((input[23 + inpos] & 127) << 1)
        | ((input[24 + inpos] & 127) << 8)
        | ((input[25 + inpos] & 127) << 15)
        | ((input[26 + inpos] & 127) << 22)
        | ((input[27 + inpos]) << 29);
    output[6 + outpos] = ((input[27 + inpos] & 127) >> (7 - 4))
        | ((input[28 + inpos] & 127) << 4)
        | ((input[29 + inpos] & 127) << 11)
        | ((input[30 + inpos] & 127) << 18)
        | ((input[31 + inpos]) << 25);
}

fn fast_pack8(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 255)
        | ((input[1 + inpos] & 255) << 8)
        | ((input[2 + inpos] & 255) << 16)
        | ((input[3 + inpos]) << 24);
    output[1 + outpos] = (input[4 + inpos] & 255)
        | ((input[5 + inpos] & 255) << 8)
        | ((input[6 + inpos] & 255) << 16)
        | ((input[7 + inpos]) << 24);
    output[2 + outpos] = (input[8 + inpos] & 255)
        | ((input[9 + inpos] & 255) << 8)
        | ((input[10 + inpos] & 255) << 16)
        | ((input[11 + inpos]) << 24);
    output[3 + outpos] = (input[12 + inpos] & 255)
        | ((input[13 + inpos] & 255) << 8)
        | ((input[14 + inpos] & 255) << 16)
        | ((input[15 + inpos]) << 24);
    output[4 + outpos] = (input[16 + inpos] & 255)
        | ((input[17 + inpos] & 255) << 8)
        | ((input[18 + inpos] & 255) << 16)
        | ((input[19 + inpos]) << 24);
    output[5 + outpos] = (input[20 + inpos] & 255)
        | ((input[21 + inpos] & 255) << 8)
        | ((input[22 + inpos] & 255) << 16)
        | ((input[23 + inpos]) << 24);
    output[6 + outpos] = (input[24 + inpos] & 255)
        | ((input[25 + inpos] & 255) << 8)
        | ((input[26 + inpos] & 255) << 16)
        | ((input[27 + inpos]) << 24);
    output[7 + outpos] = (input[28 + inpos] & 255)
        | ((input[29 + inpos] & 255) << 8)
        | ((input[30 + inpos] & 255) << 16)
        | ((input[31 + inpos]) << 24);
}

fn fast_pack9(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 511)
        | ((input[1 + inpos] & 511) << 9)
        | ((input[2 + inpos] & 511) << 18)
        | ((input[3 + inpos]) << 27);
    output[1 + outpos] = ((input[3 + inpos] & 511) >> (9 - 4))
        | ((input[4 + inpos] & 511) << 4)
        | ((input[5 + inpos] & 511) << 13)
        | ((input[6 + inpos] & 511) << 22)
        | ((input[7 + inpos]) << 31);
    output[2 + outpos] = ((input[7 + inpos] & 511) >> (9 - 8))
        | ((input[8 + inpos] & 511) << 8)
        | ((input[9 + inpos] & 511) << 17)
        | ((input[10 + inpos]) << 26);
    output[3 + outpos] = ((input[10 + inpos] & 511) >> (9 - 3))
        | ((input[11 + inpos] & 511) << 3)
        | ((input[12 + inpos] & 511) << 12)
        | ((input[13 + inpos] & 511) << 21)
        | ((input[14 + inpos]) << 30);
    output[4 + outpos] = ((input[14 + inpos] & 511) >> (9 - 7))
        | ((input[15 + inpos] & 511) << 7)
        | ((input[16 + inpos] & 511) << 16)
        | ((input[17 + inpos]) << 25);
    output[5 + outpos] = ((input[17 + inpos] & 511) >> (9 - 2))
        | ((input[18 + inpos] & 511) << 2)
        | ((input[19 + inpos] & 511) << 11)
        | ((input[20 + inpos] & 511) << 20)
        | ((input[21 + inpos]) << 29);
    output[6 + outpos] = ((input[21 + inpos] & 511) >> (9 - 6))
        | ((input[22 + inpos] & 511) << 6)
        | ((input[23 + inpos] & 511) << 15)
        | ((input[24 + inpos]) << 24);
    output[7 + outpos] = ((input[24 + inpos] & 511) >> (9 - 1))
        | ((input[25 + inpos] & 511) << 1)
        | ((input[26 + inpos] & 511) << 10)
        | ((input[27 + inpos] & 511) << 19)
        | ((input[28 + inpos]) << 28);
    output[8 + outpos] = ((input[28 + inpos] & 511) >> (9 - 5))
        | ((input[29 + inpos] & 511) << 5)
        | ((input[30 + inpos] & 511) << 14)
        | ((input[31 + inpos]) << 23);
}

fn fast_pack10(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 1023)
        | ((input[1 + inpos] & 1023) << 10)
        | ((input[2 + inpos] & 1023) << 20)
        | ((input[3 + inpos]) << 30);
    output[1 + outpos] = ((input[3 + inpos] & 1023) >> (10 - 8))
        | ((input[4 + inpos] & 1023) << 8)
        | ((input[5 + inpos] & 1023) << 18)
        | ((input[6 + inpos]) << 28);
    output[2 + outpos] = ((input[6 + inpos] & 1023) >> (10 - 6))
        | ((input[7 + inpos] & 1023) << 6)
        | ((input[8 + inpos] & 1023) << 16)
        | ((input[9 + inpos]) << 26);
    output[3 + outpos] = ((input[9 + inpos] & 1023) >> (10 - 4))
        | ((input[10 + inpos] & 1023) << 4)
        | ((input[11 + inpos] & 1023) << 14)
        | ((input[12 + inpos]) << 24);
    output[4 + outpos] = ((input[12 + inpos] & 1023) >> (10 - 2))
        | ((input[13 + inpos] & 1023) << 2)
        | ((input[14 + inpos] & 1023) << 12)
        | ((input[15 + inpos]) << 22);
    output[5 + outpos] = (input[16 + inpos] & 1023)
        | ((input[17 + inpos] & 1023) << 10)
        | ((input[18 + inpos] & 1023) << 20)
        | ((input[19 + inpos]) << 30);
    output[6 + outpos] = ((input[19 + inpos] & 1023) >> (10 - 8))
        | ((input[20 + inpos] & 1023) << 8)
        | ((input[21 + inpos] & 1023) << 18)
        | ((input[22 + inpos]) << 28);
    output[7 + outpos] = ((input[22 + inpos] & 1023) >> (10 - 6))
        | ((input[23 + inpos] & 1023) << 6)
        | ((input[24 + inpos] & 1023) << 16)
        | ((input[25 + inpos]) << 26);
    output[8 + outpos] = ((input[25 + inpos] & 1023) >> (10 - 4))
        | ((input[26 + inpos] & 1023) << 4)
        | ((input[27 + inpos] & 1023) << 14)
        | ((input[28 + inpos]) << 24);
    output[9 + outpos] = ((input[28 + inpos] & 1023) >> (10 - 2))
        | ((input[29 + inpos] & 1023) << 2)
        | ((input[30 + inpos] & 1023) << 12)
        | ((input[31 + inpos]) << 22);
}

fn fast_pack11(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] =
        (input[0 + inpos] & 2047) | ((input[1 + inpos] & 2047) << 11) | ((input[2 + inpos]) << 22);
    output[1 + outpos] = ((input[2 + inpos] & 2047) >> (11 - 1))
        | ((input[3 + inpos] & 2047) << 1)
        | ((input[4 + inpos] & 2047) << 12)
        | ((input[5 + inpos]) << 23);
    output[2 + outpos] = ((input[5 + inpos] & 2047) >> (11 - 2))
        | ((input[6 + inpos] & 2047) << 2)
        | ((input[7 + inpos] & 2047) << 13)
        | ((input[8 + inpos]) << 24);
    output[3 + outpos] = ((input[8 + inpos] & 2047) >> (11 - 3))
        | ((input[9 + inpos] & 2047) << 3)
        | ((input[10 + inpos] & 2047) << 14)
        | ((input[11 + inpos]) << 25);
    output[4 + outpos] = ((input[11 + inpos] & 2047) >> (11 - 4))
        | ((input[12 + inpos] & 2047) << 4)
        | ((input[13 + inpos] & 2047) << 15)
        | ((input[14 + inpos]) << 26);
    output[5 + outpos] = ((input[14 + inpos] & 2047) >> (11 - 5))
        | ((input[15 + inpos] & 2047) << 5)
        | ((input[16 + inpos] & 2047) << 16)
        | ((input[17 + inpos]) << 27);
    output[6 + outpos] = ((input[17 + inpos] & 2047) >> (11 - 6))
        | ((input[18 + inpos] & 2047) << 6)
        | ((input[19 + inpos] & 2047) << 17)
        | ((input[20 + inpos]) << 28);
    output[7 + outpos] = ((input[20 + inpos] & 2047) >> (11 - 7))
        | ((input[21 + inpos] & 2047) << 7)
        | ((input[22 + inpos] & 2047) << 18)
        | ((input[23 + inpos]) << 29);
    output[8 + outpos] = ((input[23 + inpos] & 2047) >> (11 - 8))
        | ((input[24 + inpos] & 2047) << 8)
        | ((input[25 + inpos] & 2047) << 19)
        | ((input[26 + inpos]) << 30);
    output[9 + outpos] = ((input[26 + inpos] & 2047) >> (11 - 9))
        | ((input[27 + inpos] & 2047) << 9)
        | ((input[28 + inpos] & 2047) << 20)
        | ((input[29 + inpos]) << 31);
    output[10 + outpos] = ((input[29 + inpos] & 2047) >> (11 - 10))
        | ((input[30 + inpos] & 2047) << 10)
        | ((input[31 + inpos]) << 21);
}

fn fast_pack12(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] =
        (input[0 + inpos] & 4095) | ((input[1 + inpos] & 4095) << 12) | ((input[2 + inpos]) << 24);
    output[1 + outpos] = ((input[2 + inpos] & 4095) >> (12 - 4))
        | ((input[3 + inpos] & 4095) << 4)
        | ((input[4 + inpos] & 4095) << 16)
        | ((input[5 + inpos]) << 28);
    output[2 + outpos] = ((input[5 + inpos] & 4095) >> (12 - 8))
        | ((input[6 + inpos] & 4095) << 8)
        | ((input[7 + inpos]) << 20);
    output[3 + outpos] =
        (input[8 + inpos] & 4095) | ((input[9 + inpos] & 4095) << 12) | ((input[10 + inpos]) << 24);
    output[4 + outpos] = ((input[10 + inpos] & 4095) >> (12 - 4))
        | ((input[11 + inpos] & 4095) << 4)
        | ((input[12 + inpos] & 4095) << 16)
        | ((input[13 + inpos]) << 28);
    output[5 + outpos] = ((input[13 + inpos] & 4095) >> (12 - 8))
        | ((input[14 + inpos] & 4095) << 8)
        | ((input[15 + inpos]) << 20);
    output[6 + outpos] = (input[16 + inpos] & 4095)
        | ((input[17 + inpos] & 4095) << 12)
        | ((input[18 + inpos]) << 24);
    output[7 + outpos] = ((input[18 + inpos] & 4095) >> (12 - 4))
        | ((input[19 + inpos] & 4095) << 4)
        | ((input[20 + inpos] & 4095) << 16)
        | ((input[21 + inpos]) << 28);
    output[8 + outpos] = ((input[21 + inpos] & 4095) >> (12 - 8))
        | ((input[22 + inpos] & 4095) << 8)
        | ((input[23 + inpos]) << 20);
    output[9 + outpos] = (input[24 + inpos] & 4095)
        | ((input[25 + inpos] & 4095) << 12)
        | ((input[26 + inpos]) << 24);
    output[10 + outpos] = ((input[26 + inpos] & 4095) >> (12 - 4))
        | ((input[27 + inpos] & 4095) << 4)
        | ((input[28 + inpos] & 4095) << 16)
        | ((input[29 + inpos]) << 28);
    output[11 + outpos] = ((input[29 + inpos] & 4095) >> (12 - 8))
        | ((input[30 + inpos] & 4095) << 8)
        | ((input[31 + inpos]) << 20);
}

fn fast_pack13(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] =
        (input[0 + inpos] & 8191) | ((input[1 + inpos] & 8191) << 13) | ((input[2 + inpos]) << 26);
    output[1 + outpos] = ((input[2 + inpos] & 8191) >> (13 - 7))
        | ((input[3 + inpos] & 8191) << 7)
        | ((input[4 + inpos]) << 20);
    output[2 + outpos] = ((input[4 + inpos] & 8191) >> (13 - 1))
        | ((input[5 + inpos] & 8191) << 1)
        | ((input[6 + inpos] & 8191) << 14)
        | ((input[7 + inpos]) << 27);
    output[3 + outpos] = ((input[7 + inpos] & 8191) >> (13 - 8))
        | ((input[8 + inpos] & 8191) << 8)
        | ((input[9 + inpos]) << 21);
    output[4 + outpos] = ((input[9 + inpos] & 8191) >> (13 - 2))
        | ((input[10 + inpos] & 8191) << 2)
        | ((input[11 + inpos] & 8191) << 15)
        | ((input[12 + inpos]) << 28);
    output[5 + outpos] = ((input[12 + inpos] & 8191) >> (13 - 9))
        | ((input[13 + inpos] & 8191) << 9)
        | ((input[14 + inpos]) << 22);
    output[6 + outpos] = ((input[14 + inpos] & 8191) >> (13 - 3))
        | ((input[15 + inpos] & 8191) << 3)
        | ((input[16 + inpos] & 8191) << 16)
        | ((input[17 + inpos]) << 29);
    output[7 + outpos] = ((input[17 + inpos] & 8191) >> (13 - 10))
        | ((input[18 + inpos] & 8191) << 10)
        | ((input[19 + inpos]) << 23);
    output[8 + outpos] = ((input[19 + inpos] & 8191) >> (13 - 4))
        | ((input[20 + inpos] & 8191) << 4)
        | ((input[21 + inpos] & 8191) << 17)
        | ((input[22 + inpos]) << 30);
    output[9 + outpos] = ((input[22 + inpos] & 8191) >> (13 - 11))
        | ((input[23 + inpos] & 8191) << 11)
        | ((input[24 + inpos]) << 24);
    output[10 + outpos] = ((input[24 + inpos] & 8191) >> (13 - 5))
        | ((input[25 + inpos] & 8191) << 5)
        | ((input[26 + inpos] & 8191) << 18)
        | ((input[27 + inpos]) << 31);
    output[11 + outpos] = ((input[27 + inpos] & 8191) >> (13 - 12))
        | ((input[28 + inpos] & 8191) << 12)
        | ((input[29 + inpos]) << 25);
    output[12 + outpos] = ((input[29 + inpos] & 8191) >> (13 - 6))
        | ((input[30 + inpos] & 8191) << 6)
        | ((input[31 + inpos]) << 19);
}

fn fast_pack14(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 16383)
        | ((input[1 + inpos] & 16383) << 14)
        | ((input[2 + inpos]) << 28);
    output[1 + outpos] = ((input[2 + inpos] & 16383) >> (14 - 10))
        | ((input[3 + inpos] & 16383) << 10)
        | ((input[4 + inpos]) << 24);
    output[2 + outpos] = ((input[4 + inpos] & 16383) >> (14 - 6))
        | ((input[5 + inpos] & 16383) << 6)
        | ((input[6 + inpos]) << 20);
    output[3 + outpos] = ((input[6 + inpos] & 16383) >> (14 - 2))
        | ((input[7 + inpos] & 16383) << 2)
        | ((input[8 + inpos] & 16383) << 16)
        | ((input[9 + inpos]) << 30);
    output[4 + outpos] = ((input[9 + inpos] & 16383) >> (14 - 12))
        | ((input[10 + inpos] & 16383) << 12)
        | ((input[11 + inpos]) << 26);
    output[5 + outpos] = ((input[11 + inpos] & 16383) >> (14 - 8))
        | ((input[12 + inpos] & 16383) << 8)
        | ((input[13 + inpos]) << 22);
    output[6 + outpos] = ((input[13 + inpos] & 16383) >> (14 - 4))
        | ((input[14 + inpos] & 16383) << 4)
        | ((input[15 + inpos]) << 18);
    output[7 + outpos] = (input[16 + inpos] & 16383)
        | ((input[17 + inpos] & 16383) << 14)
        | ((input[18 + inpos]) << 28);
    output[8 + outpos] = ((input[18 + inpos] & 16383) >> (14 - 10))
        | ((input[19 + inpos] & 16383) << 10)
        | ((input[20 + inpos]) << 24);
    output[9 + outpos] = ((input[20 + inpos] & 16383) >> (14 - 6))
        | ((input[21 + inpos] & 16383) << 6)
        | ((input[22 + inpos]) << 20);
    output[10 + outpos] = ((input[22 + inpos] & 16383) >> (14 - 2))
        | ((input[23 + inpos] & 16383) << 2)
        | ((input[24 + inpos] & 16383) << 16)
        | ((input[25 + inpos]) << 30);
    output[11 + outpos] = ((input[25 + inpos] & 16383) >> (14 - 12))
        | ((input[26 + inpos] & 16383) << 12)
        | ((input[27 + inpos]) << 26);
    output[12 + outpos] = ((input[27 + inpos] & 16383) >> (14 - 8))
        | ((input[28 + inpos] & 16383) << 8)
        | ((input[29 + inpos]) << 22);
    output[13 + outpos] = ((input[29 + inpos] & 16383) >> (14 - 4))
        | ((input[30 + inpos] & 16383) << 4)
        | ((input[31 + inpos]) << 18);
}

fn fast_pack15(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 32767)
        | ((input[1 + inpos] & 32767) << 15)
        | ((input[2 + inpos]) << 30);
    output[1 + outpos] = ((input[2 + inpos] & 32767) >> (15 - 13))
        | ((input[3 + inpos] & 32767) << 13)
        | ((input[4 + inpos]) << 28);
    output[2 + outpos] = ((input[4 + inpos] & 32767) >> (15 - 11))
        | ((input[5 + inpos] & 32767) << 11)
        | ((input[6 + inpos]) << 26);
    output[3 + outpos] = ((input[6 + inpos] & 32767) >> (15 - 9))
        | ((input[7 + inpos] & 32767) << 9)
        | ((input[8 + inpos]) << 24);
    output[4 + outpos] = ((input[8 + inpos] & 32767) >> (15 - 7))
        | ((input[9 + inpos] & 32767) << 7)
        | ((input[10 + inpos]) << 22);
    output[5 + outpos] = ((input[10 + inpos] & 32767) >> (15 - 5))
        | ((input[11 + inpos] & 32767) << 5)
        | ((input[12 + inpos]) << 20);
    output[6 + outpos] = ((input[12 + inpos] & 32767) >> (15 - 3))
        | ((input[13 + inpos] & 32767) << 3)
        | ((input[14 + inpos]) << 18);
    output[7 + outpos] = ((input[14 + inpos] & 32767) >> (15 - 1))
        | ((input[15 + inpos] & 32767) << 1)
        | ((input[16 + inpos] & 32767) << 16)
        | ((input[17 + inpos]) << 31);
    output[8 + outpos] = ((input[17 + inpos] & 32767) >> (15 - 14))
        | ((input[18 + inpos] & 32767) << 14)
        | ((input[19 + inpos]) << 29);
    output[9 + outpos] = ((input[19 + inpos] & 32767) >> (15 - 12))
        | ((input[20 + inpos] & 32767) << 12)
        | ((input[21 + inpos]) << 27);
    output[10 + outpos] = ((input[21 + inpos] & 32767) >> (15 - 10))
        | ((input[22 + inpos] & 32767) << 10)
        | ((input[23 + inpos]) << 25);
    output[11 + outpos] = ((input[23 + inpos] & 32767) >> (15 - 8))
        | ((input[24 + inpos] & 32767) << 8)
        | ((input[25 + inpos]) << 23);
    output[12 + outpos] = ((input[25 + inpos] & 32767) >> (15 - 6))
        | ((input[26 + inpos] & 32767) << 6)
        | ((input[27 + inpos]) << 21);
    output[13 + outpos] = ((input[27 + inpos] & 32767) >> (15 - 4))
        | ((input[28 + inpos] & 32767) << 4)
        | ((input[29 + inpos]) << 19);
    output[14 + outpos] = ((input[29 + inpos] & 32767) >> (15 - 2))
        | ((input[30 + inpos] & 32767) << 2)
        | ((input[31 + inpos]) << 17);
}

fn fast_pack16(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 65535) | ((input[1 + inpos]) << 16);
    output[1 + outpos] = (input[2 + inpos] & 65535) | ((input[3 + inpos]) << 16);
    output[2 + outpos] = (input[4 + inpos] & 65535) | ((input[5 + inpos]) << 16);
    output[3 + outpos] = (input[6 + inpos] & 65535) | ((input[7 + inpos]) << 16);
    output[4 + outpos] = (input[8 + inpos] & 65535) | ((input[9 + inpos]) << 16);
    output[5 + outpos] = (input[10 + inpos] & 65535) | ((input[11 + inpos]) << 16);
    output[6 + outpos] = (input[12 + inpos] & 65535) | ((input[13 + inpos]) << 16);
    output[7 + outpos] = (input[14 + inpos] & 65535) | ((input[15 + inpos]) << 16);
    output[8 + outpos] = (input[16 + inpos] & 65535) | ((input[17 + inpos]) << 16);
    output[9 + outpos] = (input[18 + inpos] & 65535) | ((input[19 + inpos]) << 16);
    output[10 + outpos] = (input[20 + inpos] & 65535) | ((input[21 + inpos]) << 16);
    output[11 + outpos] = (input[22 + inpos] & 65535) | ((input[23 + inpos]) << 16);
    output[12 + outpos] = (input[24 + inpos] & 65535) | ((input[25 + inpos]) << 16);
    output[13 + outpos] = (input[26 + inpos] & 65535) | ((input[27 + inpos]) << 16);
    output[14 + outpos] = (input[28 + inpos] & 65535) | ((input[29 + inpos]) << 16);
    output[15 + outpos] = (input[30 + inpos] & 65535) | ((input[31 + inpos]) << 16);
}

fn fast_pack17(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 131071) | ((input[1 + inpos]) << 17);
    output[1 + outpos] = (input[1 + inpos] & 131071) >> (17 - 2)
        | (input[2 + inpos] & 131071) << 2
        | (input[3 + inpos]) << 19;
    output[2 + outpos] = (input[3 + inpos] & 131071) >> (17 - 4)
        | (input[4 + inpos] & 131071) << 4
        | (input[5 + inpos]) << 21;
    output[3 + outpos] = (input[5 + inpos] & 131071) >> (17 - 6)
        | (input[6 + inpos] & 131071) << 6
        | (input[7 + inpos]) << 23;
    output[4 + outpos] = (input[7 + inpos] & 131071) >> (17 - 8)
        | (input[8 + inpos] & 131071) << 8
        | (input[9 + inpos]) << 25;
    output[5 + outpos] = (input[9 + inpos] & 131071) >> (17 - 10)
        | (input[10 + inpos] & 131071) << 10
        | (input[11 + inpos]) << 27;
    output[6 + outpos] = (input[11 + inpos] & 131071) >> (17 - 12)
        | (input[12 + inpos] & 131071) << 12
        | (input[13 + inpos]) << 29;
    output[7 + outpos] = (input[13 + inpos] & 131071) >> (17 - 14)
        | (input[14 + inpos] & 131071) << 14
        | (input[15 + inpos]) << 31;
    output[8 + outpos] = (input[15 + inpos] & 131071) >> (17 - 16) | (input[16 + inpos]) << 16;
    output[9 + outpos] = (input[16 + inpos] & 131071) >> (17 - 1)
        | (input[17 + inpos] & 131071) << 1
        | (input[18 + inpos]) << 18;
    output[10 + outpos] = (input[18 + inpos] & 131071) >> (17 - 3)
        | (input[19 + inpos] & 131071) << 3
        | (input[20 + inpos]) << 20;
    output[11 + outpos] = (input[20 + inpos] & 131071) >> (17 - 5)
        | (input[21 + inpos] & 131071) << 5
        | (input[22 + inpos]) << 22;
    output[12 + outpos] = (input[22 + inpos] & 131071) >> (17 - 7)
        | (input[23 + inpos] & 131071) << 7
        | (input[24 + inpos]) << 24;
    output[13 + outpos] = (input[24 + inpos] & 131071) >> (17 - 9)
        | (input[25 + inpos] & 131071) << 9
        | (input[26 + inpos]) << 26;
    output[14 + outpos] = (input[26 + inpos] & 131071) >> (17 - 11)
        | (input[27 + inpos] & 131071) << 11
        | (input[28 + inpos]) << 28;
    output[15 + outpos] = (input[28 + inpos] & 131071) >> (17 - 13)
        | (input[29 + inpos] & 131071) << 13
        | (input[30 + inpos]) << 30;
    output[16 + outpos] = (input[30 + inpos] & 131071) >> (17 - 15) | (input[31 + inpos]) << 15;
}

fn fast_pack18(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 262143) | ((input[1 + inpos]) << 18);
    output[1 + outpos] = (input[1 + inpos] & 262143) >> (18 - 4)
        | (input[2 + inpos] & 262143) << 4
        | (input[3 + inpos]) << 22;
    output[2 + outpos] = (input[3 + inpos] & 262143) >> (18 - 8)
        | (input[4 + inpos] & 262143) << 8
        | (input[5 + inpos]) << 26;
    output[3 + outpos] = (input[5 + inpos] & 262143) >> (18 - 12)
        | (input[6 + inpos] & 262143) << 12
        | (input[7 + inpos]) << 30;
    output[4 + outpos] = (input[7 + inpos] & 262143) >> (18 - 16) | (input[8 + inpos]) << 16;
    output[5 + outpos] = (input[8 + inpos] & 262143) >> (18 - 2)
        | (input[9 + inpos] & 262143) << 2
        | (input[10 + inpos]) << 20;
    output[6 + outpos] = (input[10 + inpos] & 262143) >> (18 - 6)
        | (input[11 + inpos] & 262143) << 6
        | (input[12 + inpos]) << 24;
    output[7 + outpos] = (input[12 + inpos] & 262143) >> (18 - 10)
        | (input[13 + inpos] & 262143) << 10
        | (input[14 + inpos]) << 28;
    output[8 + outpos] = (input[14 + inpos] & 262143) >> (18 - 14) | (input[15 + inpos]) << 14;
    output[9 + outpos] = (input[16 + inpos] & 262143) | ((input[17 + inpos]) << 18);
    output[10 + outpos] = (input[17 + inpos] & 262143) >> (18 - 4)
        | (input[18 + inpos] & 262143) << 4
        | (input[19 + inpos]) << 22;
    output[11 + outpos] = (input[19 + inpos] & 262143) >> (18 - 8)
        | (input[20 + inpos] & 262143) << 8
        | (input[21 + inpos]) << 26;
    output[12 + outpos] = (input[21 + inpos] & 262143) >> (18 - 12)
        | (input[22 + inpos] & 262143) << 12
        | (input[23 + inpos]) << 30;
    output[13 + outpos] = (input[23 + inpos] & 262143) >> (18 - 16) | (input[24 + inpos]) << 16;
    output[14 + outpos] = (input[24 + inpos] & 262143) >> (18 - 2)
        | (input[25 + inpos] & 262143) << 2
        | (input[26 + inpos]) << 20;
    output[15 + outpos] = (input[26 + inpos] & 262143) >> (18 - 6)
        | (input[27 + inpos] & 262143) << 6
        | (input[28 + inpos]) << 24;
    output[16 + outpos] = (input[28 + inpos] & 262143) >> (18 - 10)
        | (input[29 + inpos] & 262143) << 10
        | (input[30 + inpos]) << 28;
    output[17 + outpos] = (input[30 + inpos] & 262143) >> (18 - 14) | (input[31 + inpos]) << 14;
}

fn fast_pack19(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 524287) | ((input[1 + inpos]) << 19);
    output[1 + outpos] = (input[1 + inpos] & 524287) >> (19 - 6)
        | (input[2 + inpos] & 524287) << 6
        | (input[3 + inpos]) << 25;
    output[2 + outpos] = (input[3 + inpos] & 524287) >> (19 - 12)
        | (input[4 + inpos] & 524287) << 12
        | (input[5 + inpos]) << 31;
    output[3 + outpos] = (input[5 + inpos] & 524287) >> (19 - 18) | (input[6 + inpos]) << 18;
    output[4 + outpos] = (input[6 + inpos] & 524287) >> (19 - 5)
        | (input[7 + inpos] & 524287) << 5
        | (input[8 + inpos]) << 24;
    output[5 + outpos] = (input[8 + inpos] & 524287) >> (19 - 11)
        | (input[9 + inpos] & 524287) << 11
        | (input[10 + inpos]) << 30;
    output[6 + outpos] = (input[10 + inpos] & 524287) >> (19 - 17) | (input[11 + inpos]) << 17;
    output[7 + outpos] = (input[11 + inpos] & 524287) >> (19 - 4)
        | (input[12 + inpos] & 524287) << 4
        | (input[13 + inpos]) << 23;
    output[8 + outpos] = (input[13 + inpos] & 524287) >> (19 - 10)
        | (input[14 + inpos] & 524287) << 10
        | (input[15 + inpos]) << 29;
    output[9 + outpos] = (input[15 + inpos] & 524287) >> (19 - 16) | (input[16 + inpos]) << 16;
    output[10 + outpos] = (input[16 + inpos] & 524287) >> (19 - 3)
        | (input[17 + inpos] & 524287) << 3
        | (input[18 + inpos]) << 22;
    output[11 + outpos] = (input[18 + inpos] & 524287) >> (19 - 9)
        | (input[19 + inpos] & 524287) << 9
        | (input[20 + inpos]) << 28;
    output[12 + outpos] = (input[20 + inpos] & 524287) >> (19 - 15) | (input[21 + inpos]) << 15;
    output[13 + outpos] = (input[21 + inpos] & 524287) >> (19 - 2)
        | (input[22 + inpos] & 524287) << 2
        | (input[23 + inpos]) << 21;
    output[14 + outpos] = (input[23 + inpos] & 524287) >> (19 - 8)
        | (input[24 + inpos] & 524287) << 8
        | (input[25 + inpos]) << 27;
    output[15 + outpos] = (input[25 + inpos] & 524287) >> (19 - 14) | (input[26 + inpos]) << 14;
    output[16 + outpos] = (input[26 + inpos] & 524287) >> (19 - 1)
        | (input[27 + inpos] & 524287) << 1
        | (input[28 + inpos]) << 20;
    output[17 + outpos] = (input[28 + inpos] & 524287) >> (19 - 7)
        | (input[29 + inpos] & 524287) << 7
        | (input[30 + inpos]) << 26;
    output[18 + outpos] = (input[30 + inpos] & 524287) >> (19 - 13) | (input[31 + inpos]) << 13;
}

fn fast_pack20(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    // out[0 + outpos] = (in[0 + inpos] & 1048575)
    //         | ((in[1 + inpos]) << 20);
    // out[1 + outpos] = ((in[1 + inpos] & 1048575) >>> (20 - 8))
    //         | ((in[2 + inpos] & 1048575) << 8)
    //         | ((in[3 + inpos]) << 28);
    // out[2 + outpos] = ((in[3 + inpos] & 1048575) >>> (20 - 16))
    //         | ((in[4 + inpos]) << 16);
    // out[3 + outpos] = ((in[4 + inpos] & 1048575) >>> (20 - 4))
    //         | ((in[5 + inpos] & 1048575) << 4)
    //         | ((in[6 + inpos]) << 24);
    // out[4 + outpos] = ((in[6 + inpos] & 1048575) >>> (20 - 12))
    //         | ((in[7 + inpos]) << 12);
    // out[5 + outpos] = (in[8 + inpos] & 1048575)
    //         | ((in[9 + inpos]) << 20);
    // out[6 + outpos] = ((in[9 + inpos] & 1048575) >>> (20 - 8))
    //         | ((in[10 + inpos] & 1048575) << 8)
    //         | ((in[11 + inpos]) << 28);
    // out[7 + outpos] = ((in[11 + inpos] & 1048575) >>> (20 - 16))
    //         | ((in[12 + inpos]) << 16);
    // out[8 + outpos] = ((in[12 + inpos] & 1048575) >>> (20 - 4))
    //         | ((in[13 + inpos] & 1048575) << 4)
    //         | ((in[14 + inpos]) << 24);
    // out[9 + outpos] = ((in[14 + inpos] & 1048575) >>> (20 - 12))
    //         | ((in[15 + inpos]) << 12);
    // out[10 + outpos] = (in[16 + inpos] & 1048575)
    //         | ((in[17 + inpos]) << 20);
    // out[11 + outpos] = ((in[17 + inpos] & 1048575) >>> (20 - 8))
    //         | ((in[18 + inpos] & 1048575) << 8)
    //         | ((in[19 + inpos]) << 28);
    // out[12 + outpos] = ((in[19 + inpos] & 1048575) >>> (20 - 16))
    //         | ((in[20 + inpos]) << 16);
    // out[13 + outpos] = ((in[20 + inpos] & 1048575) >>> (20 - 4))
    //         | ((in[21 + inpos] & 1048575) << 4)
    //         | ((in[22 + inpos]) << 24);
    // out[14 + outpos] = ((in[22 + inpos] & 1048575) >>> (20 - 12))
    //         | ((in[23 + inpos]) << 12);
    // out[15 + outpos] = (in[24 + inpos] & 1048575)
    //         | ((in[25 + inpos]) << 20);
    // out[16 + outpos] = ((in[25 + inpos] & 1048575) >>> (20 - 8))
    //         | ((in[26 + inpos] & 1048575) << 8)
    //         | ((in[27 + inpos]) << 28);
    // out[17 + outpos] = ((in[27 + inpos] & 1048575) >>> (20 - 16))
    //         | ((in[28 + inpos]) << 16);
    // out[18 + outpos] = ((in[28 + inpos] & 1048575) >>> (20 - 4))
    //         | ((in[29 + inpos] & 1048575) << 4)
    //         | ((in[30 + inpos]) << 24);
    // out[19 + outpos] = ((in[30 + inpos] & 1048575) >>> (20 - 12))
    //         | ((in[31 + inpos]) << 12);
    output[0 + outpos] = (input[0 + inpos] & 1048575) | ((input[1 + inpos]) << 20);
    output[1 + outpos] = (input[1 + inpos] & 1048575) >> (20 - 8)
        | (input[2 + inpos] & 1048575) << 8
        | (input[3 + inpos]) << 28;
    output[2 + outpos] = (input[3 + inpos] & 1048575) >> (20 - 16) | (input[4 + inpos]) << 16;
    output[3 + outpos] = (input[4 + inpos] & 1048575) >> (20 - 4)
        | (input[5 + inpos] & 1048575) << 4
        | (input[6 + inpos]) << 24;
    output[4 + outpos] = (input[6 + inpos] & 1048575) >> (20 - 12) | (input[7 + inpos]) << 12;
    output[5 + outpos] = (input[8 + inpos] & 1048575) | ((input[9 + inpos]) << 20);
    output[6 + outpos] = (input[9 + inpos] & 1048575) >> (20 - 8)
        | (input[10 + inpos] & 1048575) << 8
        | (input[11 + inpos]) << 28;
    output[7 + outpos] = (input[11 + inpos] & 1048575) >> (20 - 16) | (input[12 + inpos]) << 16;
    output[8 + outpos] = (input[12 + inpos] & 1048575) >> (20 - 4)
        | (input[13 + inpos] & 1048575) << 4
        | (input[14 + inpos]) << 24;
    output[9 + outpos] = (input[14 + inpos] & 1048575) >> (20 - 12) | (input[15 + inpos]) << 12;
    output[10 + outpos] = (input[16 + inpos] & 1048575) | ((input[17 + inpos]) << 20);
    output[11 + outpos] = (input[17 + inpos] & 1048575) >> (20 - 8)
        | (input[18 + inpos] & 1048575) << 8
        | (input[19 + inpos]) << 28;
    output[12 + outpos] = (input[19 + inpos] & 1048575) >> (20 - 16) | (input[20 + inpos]) << 16;
    output[13 + outpos] = (input[20 + inpos] & 1048575) >> (20 - 4)
        | (input[21 + inpos] & 1048575) << 4
        | (input[22 + inpos]) << 24;
    output[14 + outpos] = (input[22 + inpos] & 1048575) >> (20 - 12) | (input[23 + inpos]) << 12;
    output[15 + outpos] = (input[24 + inpos] & 1048575) | ((input[25 + inpos]) << 20);
    output[16 + outpos] = (input[25 + inpos] & 1048575) >> (20 - 8)
        | (input[26 + inpos] & 1048575) << 8
        | (input[27 + inpos]) << 28;
    output[17 + outpos] = (input[27 + inpos] & 1048575) >> (20 - 16) | (input[28 + inpos]) << 16;
    output[18 + outpos] = (input[28 + inpos] & 1048575) >> (20 - 4)
        | (input[29 + inpos] & 1048575) << 4
        | (input[30 + inpos]) << 24;
    output[19 + outpos] = (input[30 + inpos] & 1048575) >> (20 - 12) | (input[31 + inpos]) << 12;
}

fn fast_pack21(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 2097151) | ((input[1 + inpos]) << 21);
    output[1 + outpos] = (input[1 + inpos] & 2097151) >> (21 - 10)
        | (input[2 + inpos] & 2097151) << 10
        | (input[3 + inpos]) << 31;
    output[2 + outpos] = (input[3 + inpos] & 2097151) >> (21 - 20) | (input[4 + inpos]) << 20;
    output[3 + outpos] = (input[4 + inpos] & 2097151) >> (21 - 9)
        | (input[5 + inpos] & 2097151) << 9
        | (input[6 + inpos]) << 30;
    output[4 + outpos] = (input[6 + inpos] & 2097151) >> (21 - 19) | (input[7 + inpos]) << 19;
    output[5 + outpos] = (input[7 + inpos] & 2097151) >> (21 - 8)
        | (input[8 + inpos] & 2097151) << 8
        | (input[9 + inpos]) << 29;
    output[6 + outpos] = (input[9 + inpos] & 2097151) >> (21 - 18) | (input[10 + inpos]) << 18;
    output[7 + outpos] = (input[10 + inpos] & 2097151) >> (21 - 7)
        | (input[11 + inpos] & 2097151) << 7
        | (input[12 + inpos]) << 28;
    output[8 + outpos] = (input[12 + inpos] & 2097151) >> (21 - 17) | (input[13 + inpos]) << 17;
    output[9 + outpos] = (input[13 + inpos] & 2097151) >> (21 - 6)
        | (input[14 + inpos] & 2097151) << 6
        | (input[15 + inpos]) << 27;
    output[10 + outpos] = (input[15 + inpos] & 2097151) >> (21 - 16) | (input[16 + inpos]) << 16;
    output[11 + outpos] = (input[16 + inpos] & 2097151) >> (21 - 5)
        | (input[17 + inpos] & 2097151) << 5
        | (input[18 + inpos]) << 26;
    output[12 + outpos] = (input[18 + inpos] & 2097151) >> (21 - 15) | (input[19 + inpos]) << 15;
    output[13 + outpos] = (input[19 + inpos] & 2097151) >> (21 - 4)
        | (input[20 + inpos] & 2097151) << 4
        | (input[21 + inpos]) << 25;
    output[14 + outpos] = (input[21 + inpos] & 2097151) >> (21 - 14) | (input[22 + inpos]) << 14;
    output[15 + outpos] = (input[22 + inpos] & 2097151) >> (21 - 3)
        | (input[23 + inpos] & 2097151) << 3
        | (input[24 + inpos]) << 24;
    output[16 + outpos] = (input[24 + inpos] & 2097151) >> (21 - 13) | (input[25 + inpos]) << 13;
    output[17 + outpos] = (input[25 + inpos] & 2097151) >> (21 - 2)
        | (input[26 + inpos] & 2097151) << 2
        | (input[27 + inpos]) << 23;
    output[18 + outpos] = (input[27 + inpos] & 2097151) >> (21 - 12) | (input[28 + inpos]) << 12;
    output[19 + outpos] = (input[28 + inpos] & 2097151) >> (21 - 1)
        | (input[29 + inpos] & 2097151) << 1
        | (input[30 + inpos]) << 22;
    output[20 + outpos] = (input[30 + inpos] & 2097151) >> (21 - 11) | (input[31 + inpos]) << 11;
}

fn fast_pack22(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 4194303) | ((input[1 + inpos]) << 22);
    output[1 + outpos] = (input[1 + inpos] & 4194303) >> (22 - 12) | (input[2 + inpos]) << 12;
    output[2 + outpos] = (input[2 + inpos] & 4194303) >> (22 - 2)
        | (input[3 + inpos] & 4194303) << 2
        | (input[4 + inpos]) << 24;
    output[3 + outpos] = (input[4 + inpos] & 4194303) >> (22 - 14) | (input[5 + inpos]) << 14;
    output[4 + outpos] = (input[5 + inpos] & 4194303) >> (22 - 4)
        | (input[6 + inpos] & 4194303) << 4
        | (input[7 + inpos]) << 26;
    output[5 + outpos] = (input[7 + inpos] & 4194303) >> (22 - 16) | (input[8 + inpos]) << 16;
    output[6 + outpos] = (input[8 + inpos] & 4194303) >> (22 - 6)
        | (input[9 + inpos] & 4194303) << 6
        | (input[10 + inpos]) << 28;
    output[7 + outpos] = (input[10 + inpos] & 4194303) >> (22 - 18) | (input[11 + inpos]) << 18;
    output[8 + outpos] = (input[11 + inpos] & 4194303) >> (22 - 8)
        | (input[12 + inpos] & 4194303) << 8
        | (input[13 + inpos]) << 30;
    output[9 + outpos] = (input[13 + inpos] & 4194303) >> (22 - 20) | (input[14 + inpos]) << 20;
    output[10 + outpos] = (input[14 + inpos] & 4194303) >> (22 - 10) | (input[15 + inpos]) << 10;
    output[11 + outpos] = (input[16 + inpos] & 4194303) | ((input[17 + inpos]) << 22);
    output[12 + outpos] = (input[17 + inpos] & 4194303) >> (22 - 12) | (input[18 + inpos]) << 12;
    output[13 + outpos] = (input[18 + inpos] & 4194303) >> (22 - 2)
        | (input[19 + inpos] & 4194303) << 2
        | (input[20 + inpos]) << 24;
    output[14 + outpos] = (input[20 + inpos] & 4194303) >> (22 - 14) | (input[21 + inpos]) << 14;
    output[15 + outpos] = (input[21 + inpos] & 4194303) >> (22 - 4)
        | (input[22 + inpos] & 4194303) << 4
        | (input[23 + inpos]) << 26;
    output[16 + outpos] = (input[23 + inpos] & 4194303) >> (22 - 16) | (input[24 + inpos]) << 16;
    output[17 + outpos] = (input[24 + inpos] & 4194303) >> (22 - 6)
        | (input[25 + inpos] & 4194303) << 6
        | (input[26 + inpos]) << 28;
    output[18 + outpos] = (input[26 + inpos] & 4194303) >> (22 - 18) | (input[27 + inpos]) << 18;
    output[19 + outpos] = (input[27 + inpos] & 4194303) >> (22 - 8)
        | (input[28 + inpos] & 4194303) << 8
        | (input[29 + inpos]) << 30;
    output[20 + outpos] = (input[29 + inpos] & 4194303) >> (22 - 20) | (input[30 + inpos]) << 20;
    output[21 + outpos] = (input[30 + inpos] & 4194303) >> (22 - 10) | (input[31 + inpos]) << 10;
}
fn fast_pack23(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 8388607) | ((input[1 + inpos]) << 23);
    output[1 + outpos] = (input[1 + inpos] & 8388607) >> (23 - 14) | (input[2 + inpos]) << 14;
    output[2 + outpos] = (input[2 + inpos] & 8388607) >> (23 - 5)
        | (input[3 + inpos] & 8388607) << 5
        | (input[4 + inpos]) << 28;
    output[3 + outpos] = (input[4 + inpos] & 8388607) >> (23 - 19) | (input[5 + inpos]) << 19;
    output[4 + outpos] = (input[5 + inpos] & 8388607) >> (23 - 10) | (input[6 + inpos]) << 10;
    output[5 + outpos] = (input[6 + inpos] & 8388607) >> (23 - 1)
        | (input[7 + inpos] & 8388607) << 1
        | (input[8 + inpos]) << 24;
    output[6 + outpos] = (input[8 + inpos] & 8388607) >> (23 - 15) | (input[9 + inpos]) << 15;
    output[7 + outpos] = (input[9 + inpos] & 8388607) >> (23 - 6)
        | (input[10 + inpos] & 8388607) << 6
        | (input[11 + inpos]) << 29;
    output[8 + outpos] = (input[11 + inpos] & 8388607) >> (23 - 20) | (input[12 + inpos]) << 20;
    output[9 + outpos] = (input[12 + inpos] & 8388607) >> (23 - 11) | (input[13 + inpos]) << 11;
    output[10 + outpos] = (input[13 + inpos] & 8388607) >> (23 - 2)
        | (input[14 + inpos] & 8388607) << 2
        | (input[15 + inpos]) << 25;
    output[11 + outpos] = (input[15 + inpos] & 8388607) >> (23 - 16) | (input[16 + inpos]) << 16;
    output[12 + outpos] = (input[16 + inpos] & 8388607) >> (23 - 7)
        | (input[17 + inpos] & 8388607) << 7
        | (input[18 + inpos]) << 30;
    output[13 + outpos] = (input[18 + inpos] & 8388607) >> (23 - 21) | (input[19 + inpos]) << 21;
    output[14 + outpos] = (input[19 + inpos] & 8388607) >> (23 - 12) | (input[20 + inpos]) << 12;
    output[15 + outpos] = (input[20 + inpos] & 8388607) >> (23 - 3)
        | (input[21 + inpos] & 8388607) << 3
        | (input[22 + inpos]) << 26;
    output[16 + outpos] = (input[22 + inpos] & 8388607) >> (23 - 17) | (input[23 + inpos]) << 17;
    output[17 + outpos] = (input[23 + inpos] & 8388607) >> (23 - 8)
        | (input[24 + inpos] & 8388607) << 8
        | (input[25 + inpos]) << 31;
    output[18 + outpos] = (input[25 + inpos] & 8388607) >> (23 - 22) | (input[26 + inpos]) << 22;
    output[19 + outpos] = (input[26 + inpos] & 8388607) >> (23 - 13) | (input[27 + inpos]) << 13;
    output[20 + outpos] = (input[27 + inpos] & 8388607) >> (23 - 4)
        | (input[28 + inpos] & 8388607) << 4
        | (input[29 + inpos]) << 27;
    output[21 + outpos] = (input[29 + inpos] & 8388607) >> (23 - 18) | (input[30 + inpos]) << 18;
    output[22 + outpos] = (input[30 + inpos] & 8388607) >> (23 - 9) | (input[31 + inpos]) << 9;
}

fn fast_pack24(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 16777215) | ((input[1 + inpos]) << 24);
    output[1 + outpos] = (input[1 + inpos] & 16777215) >> (24 - 16) | (input[2 + inpos]) << 16;
    output[2 + outpos] = (input[2 + inpos] & 16777215) >> (24 - 8) | (input[3 + inpos]) << 8;
    output[3 + outpos] = (input[4 + inpos] & 16777215) | ((input[5 + inpos]) << 24);
    output[4 + outpos] = (input[5 + inpos] & 16777215) >> (24 - 16) | (input[6 + inpos]) << 16;
    output[5 + outpos] = (input[6 + inpos] & 16777215) >> (24 - 8) | (input[7 + inpos]) << 8;
    output[6 + outpos] = (input[8 + inpos] & 16777215) | ((input[9 + inpos]) << 24);
    output[7 + outpos] = (input[9 + inpos] & 16777215) >> (24 - 16) | (input[10 + inpos]) << 16;
    output[8 + outpos] = (input[10 + inpos] & 16777215) >> (24 - 8) | (input[11 + inpos]) << 8;
    output[9 + outpos] = (input[12 + inpos] & 16777215) | ((input[13 + inpos]) << 24);
    output[10 + outpos] = (input[13 + inpos] & 16777215) >> (24 - 16) | (input[14 + inpos]) << 16;
    output[11 + outpos] = (input[14 + inpos] & 16777215) >> (24 - 8) | (input[15 + inpos]) << 8;
    output[12 + outpos] = (input[16 + inpos] & 16777215) | ((input[17 + inpos]) << 24);
    output[13 + outpos] = (input[17 + inpos] & 16777215) >> (24 - 16) | (input[18 + inpos]) << 16;
    output[14 + outpos] = (input[18 + inpos] & 16777215) >> (24 - 8) | (input[19 + inpos]) << 8;
    output[15 + outpos] = (input[20 + inpos] & 16777215) | ((input[21 + inpos]) << 24);
    output[16 + outpos] = (input[21 + inpos] & 16777215) >> (24 - 16) | (input[22 + inpos]) << 16;
    output[17 + outpos] = (input[22 + inpos] & 16777215) >> (24 - 8) | (input[23 + inpos]) << 8;
    output[18 + outpos] = (input[24 + inpos] & 16777215) | ((input[25 + inpos]) << 24);
    output[19 + outpos] = (input[25 + inpos] & 16777215) >> (24 - 16) | (input[26 + inpos]) << 16;
    output[20 + outpos] = (input[26 + inpos] & 16777215) >> (24 - 8) | (input[27 + inpos]) << 8;
    output[21 + outpos] = (input[28 + inpos] & 16777215) | ((input[29 + inpos]) << 24);
    output[22 + outpos] = (input[29 + inpos] & 16777215) >> (24 - 16) | (input[30 + inpos]) << 16;
    output[23 + outpos] = (input[30 + inpos] & 16777215) >> (24 - 8) | (input[31 + inpos]) << 8;
}

fn fast_pack25(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 33554431) | ((input[1 + inpos]) << 25);
    output[1 + outpos] = (input[1 + inpos] & 33554431) >> (25 - 18) | (input[2 + inpos]) << 18;
    output[2 + outpos] = (input[2 + inpos] & 33554431) >> (25 - 11) | (input[3 + inpos]) << 11;
    output[3 + outpos] = (input[3 + inpos] & 33554431) >> (25 - 4)
        | (input[4 + inpos] & 33554431) << 4
        | (input[5 + inpos]) << 29;
    output[4 + outpos] = (input[5 + inpos] & 33554431) >> (25 - 22) | (input[6 + inpos]) << 22;
    output[5 + outpos] = (input[6 + inpos] & 33554431) >> (25 - 15) | (input[7 + inpos]) << 15;
    output[6 + outpos] = (input[7 + inpos] & 33554431) >> (25 - 8) | (input[8 + inpos]) << 8;
    output[7 + outpos] = (input[8 + inpos] & 33554431) >> (25 - 1)
        | (input[9 + inpos] & 33554431) << 1
        | (input[10 + inpos]) << 26;
    output[8 + outpos] = (input[10 + inpos] & 33554431) >> (25 - 19) | (input[11 + inpos]) << 19;
    output[9 + outpos] = (input[11 + inpos] & 33554431) >> (25 - 12) | (input[12 + inpos]) << 12;
    output[10 + outpos] = (input[12 + inpos] & 33554431) >> (25 - 5)
        | (input[13 + inpos] & 33554431) << 5
        | (input[14 + inpos]) << 30;
    output[11 + outpos] = (input[14 + inpos] & 33554431) >> (25 - 23) | (input[15 + inpos]) << 23;
    output[12 + outpos] = (input[15 + inpos] & 33554431) >> (25 - 16) | (input[16 + inpos]) << 16;
    output[13 + outpos] = (input[16 + inpos] & 33554431) >> (25 - 9) | (input[17 + inpos]) << 9;
    output[14 + outpos] = (input[17 + inpos] & 33554431) >> (25 - 2)
        | (input[18 + inpos] & 33554431) << 2
        | (input[19 + inpos]) << 27;
    output[15 + outpos] = (input[19 + inpos] & 33554431) >> (25 - 20) | (input[20 + inpos]) << 20;
    output[16 + outpos] = (input[20 + inpos] & 33554431) >> (25 - 13) | (input[21 + inpos]) << 13;
    output[17 + outpos] = (input[21 + inpos] & 33554431) >> (25 - 6)
        | (input[22 + inpos] & 33554431) << 6
        | (input[23 + inpos]) << 31;
    output[18 + outpos] = (input[23 + inpos] & 33554431) >> (25 - 24) | (input[24 + inpos]) << 24;
    output[19 + outpos] = (input[24 + inpos] & 33554431) >> (25 - 17) | (input[25 + inpos]) << 17;
    output[20 + outpos] = (input[25 + inpos] & 33554431) >> (25 - 10) | (input[26 + inpos]) << 10;
    output[21 + outpos] = (input[26 + inpos] & 33554431) >> (25 - 3)
        | (input[27 + inpos] & 33554431) << 3
        | (input[28 + inpos]) << 28;
    output[22 + outpos] = (input[28 + inpos] & 33554431) >> (25 - 21) | (input[29 + inpos]) << 21;
    output[23 + outpos] = (input[29 + inpos] & 33554431) >> (25 - 14) | (input[30 + inpos]) << 14;
    output[24 + outpos] = (input[30 + inpos] & 33554431) >> (25 - 7) | (input[31 + inpos]) << 7;
}

fn fast_pack26(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 67108863) | ((input[1 + inpos]) << 26);
    output[1 + outpos] = (input[1 + inpos] & 67108863) >> (26 - 20) | (input[2 + inpos]) << 20;
    output[2 + outpos] = (input[2 + inpos] & 67108863) >> (26 - 14) | (input[3 + inpos]) << 14;
    output[3 + outpos] = (input[3 + inpos] & 67108863) >> (26 - 8) | (input[4 + inpos]) << 8;
    output[4 + outpos] = (input[4 + inpos] & 67108863) >> (26 - 2)
        | (input[5 + inpos] & 67108863) << 2
        | (input[6 + inpos]) << 28;
    output[5 + outpos] = (input[6 + inpos] & 67108863) >> (26 - 22) | (input[7 + inpos]) << 22;
    output[6 + outpos] = (input[7 + inpos] & 67108863) >> (26 - 16) | (input[8 + inpos]) << 16;
    output[7 + outpos] = (input[8 + inpos] & 67108863) >> (26 - 10) | (input[9 + inpos]) << 10;
    output[8 + outpos] = (input[9 + inpos] & 67108863) >> (26 - 4)
        | (input[10 + inpos] & 67108863) << 4
        | (input[11 + inpos]) << 30;
    output[9 + outpos] = (input[11 + inpos] & 67108863) >> (26 - 24) | (input[12 + inpos]) << 24;
    output[10 + outpos] = (input[12 + inpos] & 67108863) >> (26 - 18) | (input[13 + inpos]) << 18;
    output[11 + outpos] = (input[13 + inpos] & 67108863) >> (26 - 12) | (input[14 + inpos]) << 12;
    output[12 + outpos] = (input[14 + inpos] & 67108863) >> (26 - 6) | (input[15 + inpos]) << 6;
    output[13 + outpos] = input[16 + inpos] & 67108863 | (input[17 + inpos]) << 26;
    output[14 + outpos] = (input[17 + inpos] & 67108863) >> (26 - 20) | (input[18 + inpos]) << 20;
    output[15 + outpos] = (input[18 + inpos] & 67108863) >> (26 - 14) | (input[19 + inpos]) << 14;
    output[16 + outpos] = (input[19 + inpos] & 67108863) >> (26 - 8) | (input[20 + inpos]) << 8;
    output[17 + outpos] = (input[20 + inpos] & 67108863) >> (26 - 2)
        | (input[21 + inpos] & 67108863) << 2
        | (input[22 + inpos]) << 28;
    output[18 + outpos] = (input[22 + inpos] & 67108863) >> (26 - 22) | (input[23 + inpos]) << 22;
    output[19 + outpos] = (input[23 + inpos] & 67108863) >> (26 - 16) | (input[24 + inpos]) << 16;
    output[20 + outpos] = (input[24 + inpos] & 67108863) >> (26 - 10) | (input[25 + inpos]) << 10;
    output[21 + outpos] = (input[25 + inpos] & 67108863) >> (26 - 4)
        | (input[26 + inpos] & 67108863) << 4
        | (input[27 + inpos]) << 30;
    output[22 + outpos] = (input[27 + inpos] & 67108863) >> (26 - 24) | (input[28 + inpos]) << 24;
    output[23 + outpos] = (input[28 + inpos] & 67108863) >> (26 - 18) | (input[29 + inpos]) << 18;
    output[24 + outpos] = (input[29 + inpos] & 67108863) >> (26 - 12) | (input[30 + inpos]) << 12;
    output[25 + outpos] = (input[30 + inpos] & 67108863) >> (26 - 6) | (input[31 + inpos]) << 6;
}

fn fast_pack27(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 134217727) | ((input[1 + inpos]) << 27);
    output[1 + outpos] = (input[1 + inpos] & 134217727) >> (27 - 22) | (input[2 + inpos]) << 22;
    output[2 + outpos] = (input[2 + inpos] & 134217727) >> (27 - 17) | (input[3 + inpos]) << 17;
    output[3 + outpos] = (input[3 + inpos] & 134217727) >> (27 - 12) | (input[4 + inpos]) << 12;
    output[4 + outpos] = (input[4 + inpos] & 134217727) >> (27 - 7) | (input[5 + inpos]) << 7;
    output[5 + outpos] = (input[5 + inpos] & 134217727) >> (27 - 2)
        | (input[6 + inpos] & 134217727) << 2
        | (input[7 + inpos]) << 29;
    output[6 + outpos] = (input[7 + inpos] & 134217727) >> (27 - 24) | (input[8 + inpos]) << 24;
    output[7 + outpos] = (input[8 + inpos] & 134217727) >> (27 - 19) | (input[9 + inpos]) << 19;
    output[8 + outpos] = (input[9 + inpos] & 134217727) >> (27 - 14) | (input[10 + inpos]) << 14;
    output[9 + outpos] = (input[10 + inpos] & 134217727) >> (27 - 9) | (input[11 + inpos]) << 9;
    output[10 + outpos] = (input[11 + inpos] & 134217727) >> (27 - 4)
        | (input[12 + inpos] & 134217727) << 4
        | (input[13 + inpos]) << 31;
    output[11 + outpos] = (input[13 + inpos] & 134217727) >> (27 - 26) | (input[14 + inpos]) << 26;
    output[12 + outpos] = (input[14 + inpos] & 134217727) >> (27 - 21) | (input[15 + inpos]) << 21;
    output[13 + outpos] = (input[15 + inpos] & 134217727) >> (27 - 16) | (input[16 + inpos]) << 16;
    output[14 + outpos] = (input[16 + inpos] & 134217727) >> (27 - 11) | (input[17 + inpos]) << 11;
    output[15 + outpos] = (input[17 + inpos] & 134217727) >> (27 - 6) | (input[18 + inpos]) << 6;
    output[16 + outpos] = (input[18 + inpos] & 134217727) >> (27 - 1)
        | (input[19 + inpos] & 134217727) << 1
        | (input[20 + inpos]) << 28;
    output[17 + outpos] = (input[20 + inpos] & 134217727) >> (27 - 23) | (input[21 + inpos]) << 23;
    output[18 + outpos] = (input[21 + inpos] & 134217727) >> (27 - 18) | (input[22 + inpos]) << 18;
    output[19 + outpos] = (input[22 + inpos] & 134217727) >> (27 - 13) | (input[23 + inpos]) << 13;
    output[20 + outpos] = (input[23 + inpos] & 134217727) >> (27 - 8) | (input[24 + inpos]) << 8;
    output[21 + outpos] = (input[24 + inpos] & 134217727) >> (27 - 3)
        | (input[25 + inpos] & 134217727) << 3
        | (input[26 + inpos]) << 30;
    output[22 + outpos] = (input[26 + inpos] & 134217727) >> (27 - 25) | (input[27 + inpos]) << 25;
    output[23 + outpos] = (input[27 + inpos] & 134217727) >> (27 - 20) | (input[28 + inpos]) << 20;
    output[24 + outpos] = (input[28 + inpos] & 134217727) >> (27 - 15) | (input[29 + inpos]) << 15;
    output[25 + outpos] = (input[29 + inpos] & 134217727) >> (27 - 10) | (input[30 + inpos]) << 10;
    output[26 + outpos] = (input[30 + inpos] & 134217727) >> (27 - 5) | (input[31 + inpos]) << 5;
}

fn fast_pack28(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 268435455) | ((input[1 + inpos]) << 28);
    output[1 + outpos] = (input[1 + inpos] & 268435455) >> (28 - 24) | (input[2 + inpos]) << 24;
    output[2 + outpos] = (input[2 + inpos] & 268435455) >> (28 - 20) | (input[3 + inpos]) << 20;
    output[3 + outpos] = (input[3 + inpos] & 268435455) >> (28 - 16) | (input[4 + inpos]) << 16;
    output[4 + outpos] = (input[4 + inpos] & 268435455) >> (28 - 12) | (input[5 + inpos]) << 12;
    output[5 + outpos] = (input[5 + inpos] & 268435455) >> (28 - 8) | (input[6 + inpos]) << 8;
    output[6 + outpos] = (input[6 + inpos] & 268435455) >> (28 - 4) | (input[7 + inpos]) << 4;
    output[7 + outpos] = (input[8 + inpos] & 268435455) | ((input[9 + inpos]) << 28);
    output[8 + outpos] = (input[9 + inpos] & 268435455) >> (28 - 24) | (input[10 + inpos]) << 24;
    output[9 + outpos] = (input[10 + inpos] & 268435455) >> (28 - 20) | (input[11 + inpos]) << 20;
    output[10 + outpos] = (input[11 + inpos] & 268435455) >> (28 - 16) | (input[12 + inpos]) << 16;
    output[11 + outpos] = (input[12 + inpos] & 268435455) >> (28 - 12) | (input[13 + inpos]) << 12;
    output[12 + outpos] = (input[13 + inpos] & 268435455) >> (28 - 8) | (input[14 + inpos]) << 8;
    output[13 + outpos] = (input[14 + inpos] & 268435455) >> (28 - 4) | (input[15 + inpos]) << 4;
    output[14 + outpos] = (input[16 + inpos] & 268435455) | ((input[17 + inpos]) << 28);
    output[15 + outpos] = (input[17 + inpos] & 268435455) >> (28 - 24) | (input[18 + inpos]) << 24;
    output[16 + outpos] = (input[18 + inpos] & 268435455) >> (28 - 20) | (input[19 + inpos]) << 20;
    output[17 + outpos] = (input[19 + inpos] & 268435455) >> (28 - 16) | (input[20 + inpos]) << 16;
    output[18 + outpos] = (input[20 + inpos] & 268435455) >> (28 - 12) | (input[21 + inpos]) << 12;
    output[19 + outpos] = (input[21 + inpos] & 268435455) >> (28 - 8) | (input[22 + inpos]) << 8;
    output[20 + outpos] = (input[22 + inpos] & 268435455) >> (28 - 4) | (input[23 + inpos]) << 4;
    output[21 + outpos] = (input[24 + inpos] & 268435455) | ((input[25 + inpos]) << 28);
    output[22 + outpos] = (input[25 + inpos] & 268435455) >> (28 - 24) | (input[26 + inpos]) << 24;
    output[23 + outpos] = (input[26 + inpos] & 268435455) >> (28 - 20) | (input[27 + inpos]) << 20;
    output[24 + outpos] = (input[27 + inpos] & 268435455) >> (28 - 16) | (input[28 + inpos]) << 16;
    output[25 + outpos] = (input[28 + inpos] & 268435455) >> (28 - 12) | (input[29 + inpos]) << 12;
    output[26 + outpos] = (input[29 + inpos] & 268435455) >> (28 - 8) | (input[30 + inpos]) << 8;
    output[27 + outpos] = (input[30 + inpos] & 268435455) >> (28 - 4) | (input[31 + inpos]) << 4;
}

fn fast_pack29(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 536870911) | ((input[1 + inpos]) << 29);
    output[1 + outpos] = (input[1 + inpos] & 536870911) >> (29 - 26) | (input[2 + inpos]) << 26;
    output[2 + outpos] = (input[2 + inpos] & 536870911) >> (29 - 23) | (input[3 + inpos]) << 23;
    output[3 + outpos] = (input[3 + inpos] & 536870911) >> (29 - 20) | (input[4 + inpos]) << 20;
    output[4 + outpos] = (input[4 + inpos] & 536870911) >> (29 - 17) | (input[5 + inpos]) << 17;
    output[5 + outpos] = (input[5 + inpos] & 536870911) >> (29 - 14) | (input[6 + inpos]) << 14;
    output[6 + outpos] = (input[6 + inpos] & 536870911) >> (29 - 11) | (input[7 + inpos]) << 11;
    output[7 + outpos] = (input[7 + inpos] & 536870911) >> (29 - 8) | (input[8 + inpos]) << 8;
    output[8 + outpos] = (input[8 + inpos] & 536870911) >> (29 - 5) | (input[9 + inpos]) << 5;
    output[9 + outpos] = (input[9 + inpos] & 536870911) >> (29 - 2)
        | (input[10 + inpos] & 536870911) << 2
        | (input[11 + inpos]) << 31;
    output[10 + outpos] = (input[11 + inpos] & 536870911) >> (29 - 28) | (input[12 + inpos]) << 28;
    output[11 + outpos] = (input[12 + inpos] & 536870911) >> (29 - 25) | (input[13 + inpos]) << 25;
    output[12 + outpos] = (input[13 + inpos] & 536870911) >> (29 - 22) | (input[14 + inpos]) << 22;
    output[13 + outpos] = (input[14 + inpos] & 536870911) >> (29 - 19) | (input[15 + inpos]) << 19;
    output[14 + outpos] = (input[15 + inpos] & 536870911) >> (29 - 16) | (input[16 + inpos]) << 16;
    output[15 + outpos] = (input[16 + inpos] & 536870911) >> (29 - 13) | (input[17 + inpos]) << 13;
    output[16 + outpos] = (input[17 + inpos] & 536870911) >> (29 - 10) | (input[18 + inpos]) << 10;
    output[17 + outpos] = (input[18 + inpos] & 536870911) >> (29 - 7) | (input[19 + inpos]) << 7;
    output[18 + outpos] = (input[19 + inpos] & 536870911) >> (29 - 4) | (input[20 + inpos]) << 4;
    output[19 + outpos] = (input[20 + inpos] & 536870911) >> (29 - 1)
        | (input[21 + inpos] & 536870911) << 1
        | (input[22 + inpos]) << 30;
    output[20 + outpos] = (input[22 + inpos] & 536870911) >> (29 - 27) | (input[23 + inpos]) << 27;
    output[21 + outpos] = (input[23 + inpos] & 536870911) >> (29 - 24) | (input[24 + inpos]) << 24;
    output[22 + outpos] = (input[24 + inpos] & 536870911) >> (29 - 21) | (input[25 + inpos]) << 21;
    output[23 + outpos] = (input[25 + inpos] & 536870911) >> (29 - 18) | (input[26 + inpos]) << 18;
    output[24 + outpos] = (input[26 + inpos] & 536870911) >> (29 - 15) | (input[27 + inpos]) << 15;
    output[25 + outpos] = (input[27 + inpos] & 536870911) >> (29 - 12) | (input[28 + inpos]) << 12;
    output[26 + outpos] = (input[28 + inpos] & 536870911) >> (29 - 9) | (input[29 + inpos]) << 9;
    output[27 + outpos] = (input[29 + inpos] & 536870911) >> (29 - 6) | (input[30 + inpos]) << 6;
    output[28 + outpos] = (input[30 + inpos] & 536870911) >> (29 - 3) | (input[31 + inpos]) << 3;
}

fn fast_pack30(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 1073741823) | ((input[1 + inpos]) << 30);
    output[1 + outpos] = (input[1 + inpos] & 1073741823) >> (30 - 28) | (input[2 + inpos]) << 28;
    output[2 + outpos] = (input[2 + inpos] & 1073741823) >> (30 - 26) | (input[3 + inpos]) << 26;
    output[3 + outpos] = (input[3 + inpos] & 1073741823) >> (30 - 24) | (input[4 + inpos]) << 24;
    output[4 + outpos] = (input[4 + inpos] & 1073741823) >> (30 - 22) | (input[5 + inpos]) << 22;
    output[5 + outpos] = (input[5 + inpos] & 1073741823) >> (30 - 20) | (input[6 + inpos]) << 20;
    output[6 + outpos] = (input[6 + inpos] & 1073741823) >> (30 - 18) | (input[7 + inpos]) << 18;
    output[7 + outpos] = (input[7 + inpos] & 1073741823) >> (30 - 16) | (input[8 + inpos]) << 16;
    output[8 + outpos] = (input[8 + inpos] & 1073741823) >> (30 - 14) | (input[9 + inpos]) << 14;
    output[9 + outpos] = (input[9 + inpos] & 1073741823) >> (30 - 12) | (input[10 + inpos]) << 12;
    output[10 + outpos] = (input[10 + inpos] & 1073741823) >> (30 - 10) | (input[11 + inpos]) << 10;
    output[11 + outpos] = (input[11 + inpos] & 1073741823) >> (30 - 8) | (input[12 + inpos]) << 8;
    output[12 + outpos] = (input[12 + inpos] & 1073741823) >> (30 - 6) | (input[13 + inpos]) << 6;
    output[13 + outpos] = (input[13 + inpos] & 1073741823) >> (30 - 4) | (input[14 + inpos]) << 4;
    output[14 + outpos] = (input[14 + inpos] & 1073741823) >> (30 - 2) | (input[15 + inpos]) << 2;
    output[15 + outpos] = (input[16 + inpos] & 1073741823) | ((input[17 + inpos]) << 30);
    output[16 + outpos] = (input[17 + inpos] & 1073741823) >> (30 - 28) | (input[18 + inpos]) << 28;
    output[17 + outpos] = (input[18 + inpos] & 1073741823) >> (30 - 26) | (input[19 + inpos]) << 26;
    output[18 + outpos] = (input[19 + inpos] & 1073741823) >> (30 - 24) | (input[20 + inpos]) << 24;
    output[19 + outpos] = (input[20 + inpos] & 1073741823) >> (30 - 22) | (input[21 + inpos]) << 22;
    output[20 + outpos] = (input[21 + inpos] & 1073741823) >> (30 - 20) | (input[22 + inpos]) << 20;
    output[21 + outpos] = (input[22 + inpos] & 1073741823) >> (30 - 18) | (input[23 + inpos]) << 18;
    output[22 + outpos] = (input[23 + inpos] & 1073741823) >> (30 - 16) | (input[24 + inpos]) << 16;
    output[23 + outpos] = (input[24 + inpos] & 1073741823) >> (30 - 14) | (input[25 + inpos]) << 14;
    output[24 + outpos] = (input[25 + inpos] & 1073741823) >> (30 - 12) | (input[26 + inpos]) << 12;
    output[25 + outpos] = (input[26 + inpos] & 1073741823) >> (30 - 10) | (input[27 + inpos]) << 10;
    output[26 + outpos] = (input[27 + inpos] & 1073741823) >> (30 - 8) | (input[28 + inpos]) << 8;
    output[27 + outpos] = (input[28 + inpos] & 1073741823) >> (30 - 6) | (input[29 + inpos]) << 6;
    output[28 + outpos] = (input[29 + inpos] & 1073741823) >> (30 - 4) | (input[30 + inpos]) << 4;
    output[29 + outpos] = (input[30 + inpos] & 1073741823) >> (30 - 2) | (input[31 + inpos]) << 2;
}

fn fast_pack31(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] & 2147483647) | ((input[1 + inpos]) << 31);
    output[1 + outpos] = (input[1 + inpos] & 2147483647) >> (31 - 30) | (input[2 + inpos]) << 30;
    output[2 + outpos] = (input[2 + inpos] & 2147483647) >> (31 - 29) | (input[3 + inpos]) << 29;
    output[3 + outpos] = (input[3 + inpos] & 2147483647) >> (31 - 28) | (input[4 + inpos]) << 28;
    output[4 + outpos] = (input[4 + inpos] & 2147483647) >> (31 - 27) | (input[5 + inpos]) << 27;
    output[5 + outpos] = (input[5 + inpos] & 2147483647) >> (31 - 26) | (input[6 + inpos]) << 26;
    output[6 + outpos] = (input[6 + inpos] & 2147483647) >> (31 - 25) | (input[7 + inpos]) << 25;
    output[7 + outpos] = (input[7 + inpos] & 2147483647) >> (31 - 24) | (input[8 + inpos]) << 24;
    output[8 + outpos] = (input[8 + inpos] & 2147483647) >> (31 - 23) | (input[9 + inpos]) << 23;
    output[9 + outpos] = (input[9 + inpos] & 2147483647) >> (31 - 22) | (input[10 + inpos]) << 22;
    output[10 + outpos] = (input[10 + inpos] & 2147483647) >> (31 - 21) | (input[11 + inpos]) << 21;
    output[11 + outpos] = (input[11 + inpos] & 2147483647) >> (31 - 20) | (input[12 + inpos]) << 20;
    output[12 + outpos] = (input[12 + inpos] & 2147483647) >> (31 - 19) | (input[13 + inpos]) << 19;
    output[13 + outpos] = (input[13 + inpos] & 2147483647) >> (31 - 18) | (input[14 + inpos]) << 18;
    output[14 + outpos] = (input[14 + inpos] & 2147483647) >> (31 - 17) | (input[15 + inpos]) << 17;
    output[15 + outpos] = (input[15 + inpos] & 2147483647) >> (31 - 16) | (input[16 + inpos]) << 16;
    output[16 + outpos] = (input[16 + inpos] & 2147483647) >> (31 - 15) | (input[17 + inpos]) << 15;
    output[17 + outpos] = (input[17 + inpos] & 2147483647) >> (31 - 14) | (input[18 + inpos]) << 14;
    output[18 + outpos] = (input[18 + inpos] & 2147483647) >> (31 - 13) | (input[19 + inpos]) << 13;
    output[19 + outpos] = (input[19 + inpos] & 2147483647) >> (31 - 12) | (input[20 + inpos]) << 12;
    output[20 + outpos] = (input[20 + inpos] & 2147483647) >> (31 - 11) | (input[21 + inpos]) << 11;
    output[21 + outpos] = (input[21 + inpos] & 2147483647) >> (31 - 10) | (input[22 + inpos]) << 10;
    output[22 + outpos] = (input[22 + inpos] & 2147483647) >> (31 - 9) | (input[23 + inpos]) << 9;
    output[23 + outpos] = (input[23 + inpos] & 2147483647) >> (31 - 8) | (input[24 + inpos]) << 8;
    output[24 + outpos] = (input[24 + inpos] & 2147483647) >> (31 - 7) | (input[25 + inpos]) << 7;
    output[25 + outpos] = (input[25 + inpos] & 2147483647) >> (31 - 6) | (input[26 + inpos]) << 6;
    output[26 + outpos] = (input[26 + inpos] & 2147483647) >> (31 - 5) | (input[27 + inpos]) << 5;
    output[27 + outpos] = (input[27 + inpos] & 2147483647) >> (31 - 4) | (input[28 + inpos]) << 4;
    output[28 + outpos] = (input[28 + inpos] & 2147483647) >> (31 - 3) | (input[29 + inpos]) << 3;
    output[29 + outpos] = (input[29 + inpos] & 2147483647) >> (31 - 2) | (input[30 + inpos]) << 2;
    output[30 + outpos] = (input[30 + inpos] & 2147483647) >> (31 - 1) | (input[31 + inpos]) << 1;
}

fn fast_pack32(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[outpos..outpos + 32].copy_from_slice(&input[inpos..inpos + 32]);
}

/// Unpacks 32 integers from `input` into `output` using `bit` bits per integer.
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

fn fast_unpack0(output: &mut [u32], outpos: usize) {
    output[outpos..outpos + 32].fill(0);
}

fn fast_unpack1(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 1;
    output[1 + outpos] = (input[0 + inpos] >> 1) & 1;
    output[2 + outpos] = (input[0 + inpos] >> 2) & 1;
    output[3 + outpos] = (input[0 + inpos] >> 3) & 1;
    output[4 + outpos] = (input[0 + inpos] >> 4) & 1;
    output[5 + outpos] = (input[0 + inpos] >> 5) & 1;
    output[6 + outpos] = (input[0 + inpos] >> 6) & 1;
    output[7 + outpos] = (input[0 + inpos] >> 7) & 1;
    output[8 + outpos] = (input[0 + inpos] >> 8) & 1;
    output[9 + outpos] = (input[0 + inpos] >> 9) & 1;
    output[10 + outpos] = (input[0 + inpos] >> 10) & 1;
    output[11 + outpos] = (input[0 + inpos] >> 11) & 1;
    output[12 + outpos] = (input[0 + inpos] >> 12) & 1;
    output[13 + outpos] = (input[0 + inpos] >> 13) & 1;
    output[14 + outpos] = (input[0 + inpos] >> 14) & 1;
    output[15 + outpos] = (input[0 + inpos] >> 15) & 1;
    output[16 + outpos] = (input[0 + inpos] >> 16) & 1;
    output[17 + outpos] = (input[0 + inpos] >> 17) & 1;
    output[18 + outpos] = (input[0 + inpos] >> 18) & 1;
    output[19 + outpos] = (input[0 + inpos] >> 19) & 1;
    output[20 + outpos] = (input[0 + inpos] >> 20) & 1;
    output[21 + outpos] = (input[0 + inpos] >> 21) & 1;
    output[22 + outpos] = (input[0 + inpos] >> 22) & 1;
    output[23 + outpos] = (input[0 + inpos] >> 23) & 1;
    output[24 + outpos] = (input[0 + inpos] >> 24) & 1;
    output[25 + outpos] = (input[0 + inpos] >> 25) & 1;
    output[26 + outpos] = (input[0 + inpos] >> 26) & 1;
    output[27 + outpos] = (input[0 + inpos] >> 27) & 1;
    output[28 + outpos] = (input[0 + inpos] >> 28) & 1;
    output[29 + outpos] = (input[0 + inpos] >> 29) & 1;
    output[30 + outpos] = (input[0 + inpos] >> 30) & 1;
    output[31 + outpos] = input[0 + inpos] >> 31;
}

fn fast_unpack2(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 3;
    output[1 + outpos] = (input[0 + inpos] >> 2) & 3;
    output[2 + outpos] = (input[0 + inpos] >> 4) & 3;
    output[3 + outpos] = (input[0 + inpos] >> 6) & 3;
    output[4 + outpos] = (input[0 + inpos] >> 8) & 3;
    output[5 + outpos] = (input[0 + inpos] >> 10) & 3;
    output[6 + outpos] = (input[0 + inpos] >> 12) & 3;
    output[7 + outpos] = (input[0 + inpos] >> 14) & 3;
    output[8 + outpos] = (input[0 + inpos] >> 16) & 3;
    output[9 + outpos] = (input[0 + inpos] >> 18) & 3;
    output[10 + outpos] = (input[0 + inpos] >> 20) & 3;
    output[11 + outpos] = (input[0 + inpos] >> 22) & 3;
    output[12 + outpos] = (input[0 + inpos] >> 24) & 3;
    output[13 + outpos] = (input[0 + inpos] >> 26) & 3;
    output[14 + outpos] = (input[0 + inpos] >> 28) & 3;
    output[15 + outpos] = input[0 + inpos] >> 30;
    output[16 + outpos] = (input[1 + inpos] >> 0) & 3;
    output[17 + outpos] = (input[1 + inpos] >> 2) & 3;
    output[18 + outpos] = (input[1 + inpos] >> 4) & 3;
    output[19 + outpos] = (input[1 + inpos] >> 6) & 3;
    output[20 + outpos] = (input[1 + inpos] >> 8) & 3;
    output[21 + outpos] = (input[1 + inpos] >> 10) & 3;
    output[22 + outpos] = (input[1 + inpos] >> 12) & 3;
    output[23 + outpos] = (input[1 + inpos] >> 14) & 3;
    output[24 + outpos] = (input[1 + inpos] >> 16) & 3;
    output[25 + outpos] = (input[1 + inpos] >> 18) & 3;
    output[26 + outpos] = (input[1 + inpos] >> 20) & 3;
    output[27 + outpos] = (input[1 + inpos] >> 22) & 3;
    output[28 + outpos] = (input[1 + inpos] >> 24) & 3;
    output[29 + outpos] = (input[1 + inpos] >> 26) & 3;
    output[30 + outpos] = (input[1 + inpos] >> 28) & 3;
    output[31 + outpos] = input[1 + inpos] >> 30;
}

fn fast_unpack3(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 7;
    output[1 + outpos] = (input[0 + inpos] >> 3) & 7;
    output[2 + outpos] = (input[0 + inpos] >> 6) & 7;
    output[3 + outpos] = (input[0 + inpos] >> 9) & 7;
    output[4 + outpos] = (input[0 + inpos] >> 12) & 7;
    output[5 + outpos] = (input[0 + inpos] >> 15) & 7;
    output[6 + outpos] = (input[0 + inpos] >> 18) & 7;
    output[7 + outpos] = (input[0 + inpos] >> 21) & 7;
    output[8 + outpos] = (input[0 + inpos] >> 24) & 7;
    output[9 + outpos] = (input[0 + inpos] >> 27) & 7;
    output[10 + outpos] = (input[0 + inpos] >> 30) | (input[1 + inpos] & 1) << (3 - 1);
    output[11 + outpos] = (input[1 + inpos] >> 1) & 7;
    output[12 + outpos] = (input[1 + inpos] >> 4) & 7;
    output[13 + outpos] = (input[1 + inpos] >> 7) & 7;
    output[14 + outpos] = (input[1 + inpos] >> 10) & 7;
    output[15 + outpos] = (input[1 + inpos] >> 13) & 7;
    output[16 + outpos] = (input[1 + inpos] >> 16) & 7;
    output[17 + outpos] = (input[1 + inpos] >> 19) & 7;
    output[18 + outpos] = (input[1 + inpos] >> 22) & 7;
    output[19 + outpos] = (input[1 + inpos] >> 25) & 7;
    output[20 + outpos] = (input[1 + inpos] >> 28) & 7;
    output[21 + outpos] = (input[1 + inpos] >> 31) | (input[2 + inpos] & 3) << (3 - 2);
    output[22 + outpos] = (input[2 + inpos] >> 2) & 7;
    output[23 + outpos] = (input[2 + inpos] >> 5) & 7;
    output[24 + outpos] = (input[2 + inpos] >> 8) & 7;
    output[25 + outpos] = (input[2 + inpos] >> 11) & 7;
    output[26 + outpos] = (input[2 + inpos] >> 14) & 7;
    output[27 + outpos] = (input[2 + inpos] >> 17) & 7;
    output[28 + outpos] = (input[2 + inpos] >> 20) & 7;
    output[29 + outpos] = (input[2 + inpos] >> 23) & 7;
    output[30 + outpos] = (input[2 + inpos] >> 26) & 7;
    output[31 + outpos] = input[2 + inpos] >> 29;
}

fn fast_unpack4(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 15;
    output[1 + outpos] = (input[0 + inpos] >> 4) & 15;
    output[2 + outpos] = (input[0 + inpos] >> 8) & 15;
    output[3 + outpos] = (input[0 + inpos] >> 12) & 15;
    output[4 + outpos] = (input[0 + inpos] >> 16) & 15;
    output[5 + outpos] = (input[0 + inpos] >> 20) & 15;
    output[6 + outpos] = (input[0 + inpos] >> 24) & 15;
    output[7 + outpos] = input[0 + inpos] >> 28;
    output[8 + outpos] = (input[1 + inpos] >> 0) & 15;
    output[9 + outpos] = (input[1 + inpos] >> 4) & 15;
    output[10 + outpos] = (input[1 + inpos] >> 8) & 15;
    output[11 + outpos] = (input[1 + inpos] >> 12) & 15;
    output[12 + outpos] = (input[1 + inpos] >> 16) & 15;
    output[13 + outpos] = (input[1 + inpos] >> 20) & 15;
    output[14 + outpos] = (input[1 + inpos] >> 24) & 15;
    output[15 + outpos] = input[1 + inpos] >> 28;
    output[16 + outpos] = (input[2 + inpos] >> 0) & 15;
    output[17 + outpos] = (input[2 + inpos] >> 4) & 15;
    output[18 + outpos] = (input[2 + inpos] >> 8) & 15;
    output[19 + outpos] = (input[2 + inpos] >> 12) & 15;
    output[20 + outpos] = (input[2 + inpos] >> 16) & 15;
    output[21 + outpos] = (input[2 + inpos] >> 20) & 15;
    output[22 + outpos] = (input[2 + inpos] >> 24) & 15;
    output[23 + outpos] = input[2 + inpos] >> 28;
    output[24 + outpos] = (input[3 + inpos] >> 0) & 15;
    output[25 + outpos] = (input[3 + inpos] >> 4) & 15;
    output[26 + outpos] = (input[3 + inpos] >> 8) & 15;
    output[27 + outpos] = (input[3 + inpos] >> 12) & 15;
    output[28 + outpos] = (input[3 + inpos] >> 16) & 15;
    output[29 + outpos] = (input[3 + inpos] >> 20) & 15;
    output[30 + outpos] = (input[3 + inpos] >> 24) & 15;
    output[31 + outpos] = input[3 + inpos] >> 28;
}

fn fast_unpack5(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 31;
    output[1 + outpos] = (input[0 + inpos] >> 5) & 31;
    output[2 + outpos] = (input[0 + inpos] >> 10) & 31;
    output[3 + outpos] = (input[0 + inpos] >> 15) & 31;
    output[4 + outpos] = (input[0 + inpos] >> 20) & 31;
    output[5 + outpos] = (input[0 + inpos] >> 25) & 31;
    output[6 + outpos] = (input[0 + inpos] >> 30) | ((input[1 + inpos] & 7) << (5 - 3));
    output[7 + outpos] = (input[1 + inpos] >> 3) & 31;
    output[8 + outpos] = (input[1 + inpos] >> 8) & 31;
    output[9 + outpos] = (input[1 + inpos] >> 13) & 31;
    output[10 + outpos] = (input[1 + inpos] >> 18) & 31;
    output[11 + outpos] = (input[1 + inpos] >> 23) & 31;
    output[12 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 1) << (5 - 1));
    output[13 + outpos] = (input[2 + inpos] >> 1) & 31;
    output[14 + outpos] = (input[2 + inpos] >> 6) & 31;
    output[15 + outpos] = (input[2 + inpos] >> 11) & 31;
    output[16 + outpos] = (input[2 + inpos] >> 16) & 31;
    output[17 + outpos] = (input[2 + inpos] >> 21) & 31;
    output[18 + outpos] = (input[2 + inpos] >> 26) & 31;
    output[19 + outpos] = (input[2 + inpos] >> 31) | ((input[3 + inpos] & 15) << (5 - 4));
    output[20 + outpos] = (input[3 + inpos] >> 4) & 31;
    output[21 + outpos] = (input[3 + inpos] >> 9) & 31;
    output[22 + outpos] = (input[3 + inpos] >> 14) & 31;
    output[23 + outpos] = (input[3 + inpos] >> 19) & 31;
    output[24 + outpos] = (input[3 + inpos] >> 24) & 31;
    output[25 + outpos] = (input[3 + inpos] >> 29) | ((input[4 + inpos] & 3) << (5 - 2));
    output[26 + outpos] = (input[4 + inpos] >> 2) & 31;
    output[27 + outpos] = (input[4 + inpos] >> 7) & 31;
    output[28 + outpos] = (input[4 + inpos] >> 12) & 31;
    output[29 + outpos] = (input[4 + inpos] >> 17) & 31;
    output[30 + outpos] = (input[4 + inpos] >> 22) & 31;
    output[31 + outpos] = input[4 + inpos] >> 27;
}

fn fast_unpack6(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 63;
    output[1 + outpos] = (input[0 + inpos] >> 6) & 63;
    output[2 + outpos] = (input[0 + inpos] >> 12) & 63;
    output[3 + outpos] = (input[0 + inpos] >> 18) & 63;
    output[4 + outpos] = (input[0 + inpos] >> 24) & 63;
    output[5 + outpos] = (input[0 + inpos] >> 30) | ((input[1 + inpos] & 15) << (6 - 4));
    output[6 + outpos] = (input[1 + inpos] >> 4) & 63;
    output[7 + outpos] = (input[1 + inpos] >> 10) & 63;
    output[8 + outpos] = (input[1 + inpos] >> 16) & 63;
    output[9 + outpos] = (input[1 + inpos] >> 22) & 63;
    output[10 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 3) << (6 - 2));
    output[11 + outpos] = (input[2 + inpos] >> 2) & 63;
    output[12 + outpos] = (input[2 + inpos] >> 8) & 63;
    output[13 + outpos] = (input[2 + inpos] >> 14) & 63;
    output[14 + outpos] = (input[2 + inpos] >> 20) & 63;
    output[15 + outpos] = input[2 + inpos] >> 26;
    output[16 + outpos] = (input[3 + inpos] >> 0) & 63;
    output[17 + outpos] = (input[3 + inpos] >> 6) & 63;
    output[18 + outpos] = (input[3 + inpos] >> 12) & 63;
    output[19 + outpos] = (input[3 + inpos] >> 18) & 63;
    output[20 + outpos] = (input[3 + inpos] >> 24) & 63;
    output[21 + outpos] = (input[3 + inpos] >> 30) | ((input[4 + inpos] & 15) << (6 - 4));
    output[22 + outpos] = (input[4 + inpos] >> 4) & 63;
    output[23 + outpos] = (input[4 + inpos] >> 10) & 63;
    output[24 + outpos] = (input[4 + inpos] >> 16) & 63;
    output[25 + outpos] = (input[4 + inpos] >> 22) & 63;
    output[26 + outpos] = (input[4 + inpos] >> 28) | ((input[5 + inpos] & 3) << (6 - 2));
    output[27 + outpos] = (input[5 + inpos] >> 2) & 63;
    output[28 + outpos] = (input[5 + inpos] >> 8) & 63;
    output[29 + outpos] = (input[5 + inpos] >> 14) & 63;
    output[30 + outpos] = (input[5 + inpos] >> 20) & 63;
    output[31 + outpos] = input[5 + inpos] >> 26;
}

fn fast_unpack7(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 127;
    output[1 + outpos] = (input[0 + inpos] >> 7) & 127;
    output[2 + outpos] = (input[0 + inpos] >> 14) & 127;
    output[3 + outpos] = (input[0 + inpos] >> 21) & 127;
    output[4 + outpos] = (input[0 + inpos] >> 28) | ((input[1 + inpos] & 7) << (7 - 3));
    output[5 + outpos] = (input[1 + inpos] >> 3) & 127;
    output[6 + outpos] = (input[1 + inpos] >> 10) & 127;
    output[7 + outpos] = (input[1 + inpos] >> 17) & 127;
    output[8 + outpos] = (input[1 + inpos] >> 24) & 127;
    output[9 + outpos] = (input[1 + inpos] >> 31) | ((input[2 + inpos] & 63) << (7 - 6));
    output[10 + outpos] = (input[2 + inpos] >> 6) & 127;
    output[11 + outpos] = (input[2 + inpos] >> 13) & 127;
    output[12 + outpos] = (input[2 + inpos] >> 20) & 127;
    output[13 + outpos] = (input[2 + inpos] >> 27) | ((input[3 + inpos] & 3) << (7 - 2));
    output[14 + outpos] = (input[3 + inpos] >> 2) & 127;
    output[15 + outpos] = (input[3 + inpos] >> 9) & 127;
    output[16 + outpos] = (input[3 + inpos] >> 16) & 127;
    output[17 + outpos] = (input[3 + inpos] >> 23) & 127;
    output[18 + outpos] = (input[3 + inpos] >> 30) | ((input[4 + inpos] & 31) << (7 - 5));
    output[19 + outpos] = (input[4 + inpos] >> 5) & 127;
    output[20 + outpos] = (input[4 + inpos] >> 12) & 127;
    output[21 + outpos] = (input[4 + inpos] >> 19) & 127;
    output[22 + outpos] = (input[4 + inpos] >> 26) | ((input[5 + inpos] & 1) << (7 - 1));
    output[23 + outpos] = (input[5 + inpos] >> 1) & 127;
    output[24 + outpos] = (input[5 + inpos] >> 8) & 127;
    output[25 + outpos] = (input[5 + inpos] >> 15) & 127;
    output[26 + outpos] = (input[5 + inpos] >> 22) & 127;
    output[27 + outpos] = (input[5 + inpos] >> 29) | ((input[6 + inpos] & 15) << (7 - 4));
    output[28 + outpos] = (input[6 + inpos] >> 4) & 127;
    output[29 + outpos] = (input[6 + inpos] >> 11) & 127;
    output[30 + outpos] = (input[6 + inpos] >> 18) & 127;
    output[31 + outpos] = input[6 + inpos] >> 25;
}

fn fast_unpack8(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 255;
    output[1 + outpos] = (input[0 + inpos] >> 8) & 255;
    output[2 + outpos] = (input[0 + inpos] >> 16) & 255;
    output[3 + outpos] = input[0 + inpos] >> 24;
    output[4 + outpos] = (input[1 + inpos] >> 0) & 255;
    output[5 + outpos] = (input[1 + inpos] >> 8) & 255;
    output[6 + outpos] = (input[1 + inpos] >> 16) & 255;
    output[7 + outpos] = input[1 + inpos] >> 24;
    output[8 + outpos] = (input[2 + inpos] >> 0) & 255;
    output[9 + outpos] = (input[2 + inpos] >> 8) & 255;
    output[10 + outpos] = (input[2 + inpos] >> 16) & 255;
    output[11 + outpos] = input[2 + inpos] >> 24;
    output[12 + outpos] = (input[3 + inpos] >> 0) & 255;
    output[13 + outpos] = (input[3 + inpos] >> 8) & 255;
    output[14 + outpos] = (input[3 + inpos] >> 16) & 255;
    output[15 + outpos] = input[3 + inpos] >> 24;
    output[16 + outpos] = (input[4 + inpos] >> 0) & 255;
    output[17 + outpos] = (input[4 + inpos] >> 8) & 255;
    output[18 + outpos] = (input[4 + inpos] >> 16) & 255;
    output[19 + outpos] = input[4 + inpos] >> 24;
    output[20 + outpos] = (input[5 + inpos] >> 0) & 255;
    output[21 + outpos] = (input[5 + inpos] >> 8) & 255;
    output[22 + outpos] = (input[5 + inpos] >> 16) & 255;
    output[23 + outpos] = input[5 + inpos] >> 24;
    output[24 + outpos] = (input[6 + inpos] >> 0) & 255;
    output[25 + outpos] = (input[6 + inpos] >> 8) & 255;
    output[26 + outpos] = (input[6 + inpos] >> 16) & 255;
    output[27 + outpos] = input[6 + inpos] >> 24;
    output[28 + outpos] = (input[7 + inpos] >> 0) & 255;
    output[29 + outpos] = (input[7 + inpos] >> 8) & 255;
    output[30 + outpos] = (input[7 + inpos] >> 16) & 255;
    output[31 + outpos] = input[7 + inpos] >> 24;
}

fn fast_unpack9(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 511;
    output[1 + outpos] = (input[0 + inpos] >> 9) & 511;
    output[2 + outpos] = (input[0 + inpos] >> 18) & 511;
    output[3 + outpos] = (input[0 + inpos] >> 27) | ((input[1 + inpos] & 15) << (9 - 4));
    output[4 + outpos] = (input[1 + inpos] >> 4) & 511;
    output[5 + outpos] = (input[1 + inpos] >> 13) & 511;
    output[6 + outpos] = (input[1 + inpos] >> 22) & 511;
    output[7 + outpos] = (input[1 + inpos] >> 31) | ((input[2 + inpos] & 255) << (9 - 8));
    output[8 + outpos] = (input[2 + inpos] >> 8) & 511;
    output[9 + outpos] = (input[2 + inpos] >> 17) & 511;
    output[10 + outpos] = (input[2 + inpos] >> 26) | ((input[3 + inpos] & 7) << (9 - 3));
    output[11 + outpos] = (input[3 + inpos] >> 3) & 511;
    output[12 + outpos] = (input[3 + inpos] >> 12) & 511;
    output[13 + outpos] = (input[3 + inpos] >> 21) & 511;
    output[14 + outpos] = (input[3 + inpos] >> 30) | ((input[4 + inpos] & 127) << (9 - 7));
    output[15 + outpos] = (input[4 + inpos] >> 7) & 511;
    output[16 + outpos] = (input[4 + inpos] >> 16) & 511;
    output[17 + outpos] = (input[4 + inpos] >> 25) | ((input[5 + inpos] & 3) << (9 - 2));
    output[18 + outpos] = (input[5 + inpos] >> 2) & 511;
    output[19 + outpos] = (input[5 + inpos] >> 11) & 511;
    output[20 + outpos] = (input[5 + inpos] >> 20) & 511;
    output[21 + outpos] = (input[5 + inpos] >> 29) | ((input[6 + inpos] & 63) << (9 - 6));
    output[22 + outpos] = (input[6 + inpos] >> 6) & 511;
    output[23 + outpos] = (input[6 + inpos] >> 15) & 511;
    output[24 + outpos] = (input[6 + inpos] >> 24) | ((input[7 + inpos] & 1) << (9 - 1));
    output[25 + outpos] = (input[7 + inpos] >> 1) & 511;
    output[26 + outpos] = (input[7 + inpos] >> 10) & 511;
    output[27 + outpos] = (input[7 + inpos] >> 19) & 511;
    output[28 + outpos] = (input[7 + inpos] >> 28) | ((input[8 + inpos] & 31) << (9 - 5));
    output[29 + outpos] = (input[8 + inpos] >> 5) & 511;
    output[30 + outpos] = (input[8 + inpos] >> 14) & 511;
    output[31 + outpos] = input[8 + inpos] >> 23;
}

fn fast_unpack10(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 1023;
    output[1 + outpos] = (input[0 + inpos] >> 10) & 1023;
    output[2 + outpos] = (input[0 + inpos] >> 20) & 1023;
    output[3 + outpos] = (input[0 + inpos] >> 30) | ((input[1 + inpos] & 255) << (10 - 8));
    output[4 + outpos] = (input[1 + inpos] >> 8) & 1023;
    output[5 + outpos] = (input[1 + inpos] >> 18) & 1023;
    output[6 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 63) << (10 - 6));
    output[7 + outpos] = (input[2 + inpos] >> 6) & 1023;
    output[8 + outpos] = (input[2 + inpos] >> 16) & 1023;
    output[9 + outpos] = (input[2 + inpos] >> 26) | ((input[3 + inpos] & 15) << (10 - 4));
    output[10 + outpos] = (input[3 + inpos] >> 4) & 1023;
    output[11 + outpos] = (input[3 + inpos] >> 14) & 1023;
    output[12 + outpos] = (input[3 + inpos] >> 24) | ((input[4 + inpos] & 3) << (10 - 2));
    output[13 + outpos] = (input[4 + inpos] >> 2) & 1023;
    output[14 + outpos] = (input[4 + inpos] >> 12) & 1023;
    output[15 + outpos] = input[4 + inpos] >> 22;
    output[16 + outpos] = (input[5 + inpos] >> 0) & 1023;
    output[17 + outpos] = (input[5 + inpos] >> 10) & 1023;
    output[18 + outpos] = (input[5 + inpos] >> 20) & 1023;
    output[19 + outpos] = (input[5 + inpos] >> 30) | ((input[6 + inpos] & 255) << (10 - 8));
    output[20 + outpos] = (input[6 + inpos] >> 8) & 1023;
    output[21 + outpos] = (input[6 + inpos] >> 18) & 1023;
    output[22 + outpos] = (input[6 + inpos] >> 28) | ((input[7 + inpos] & 63) << (10 - 6));
    output[23 + outpos] = (input[7 + inpos] >> 6) & 1023;
    output[24 + outpos] = (input[7 + inpos] >> 16) & 1023;
    output[25 + outpos] = (input[7 + inpos] >> 26) | ((input[8 + inpos] & 15) << (10 - 4));
    output[26 + outpos] = (input[8 + inpos] >> 4) & 1023;
    output[27 + outpos] = (input[8 + inpos] >> 14) & 1023;
    output[28 + outpos] = (input[8 + inpos] >> 24) | ((input[9 + inpos] & 3) << (10 - 2));
    output[29 + outpos] = (input[9 + inpos] >> 2) & 1023;
    output[30 + outpos] = (input[9 + inpos] >> 12) & 1023;
    output[31 + outpos] = input[9 + inpos] >> 22;
}

fn fast_unpack11(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 2047;
    output[1 + outpos] = (input[0 + inpos] >> 11) & 2047;
    output[2 + outpos] = (input[0 + inpos] >> 22) | ((input[1 + inpos] & 1) << (11 - 1));
    output[3 + outpos] = (input[1 + inpos] >> 1) & 2047;
    output[4 + outpos] = (input[1 + inpos] >> 12) & 2047;
    output[5 + outpos] = (input[1 + inpos] >> 23) | ((input[2 + inpos] & 3) << (11 - 2));
    output[6 + outpos] = (input[2 + inpos] >> 2) & 2047;
    output[7 + outpos] = (input[2 + inpos] >> 13) & 2047;
    output[8 + outpos] = (input[2 + inpos] >> 24) | ((input[3 + inpos] & 7) << (11 - 3));
    output[9 + outpos] = (input[3 + inpos] >> 3) & 2047;
    output[10 + outpos] = (input[3 + inpos] >> 14) & 2047;
    output[11 + outpos] = (input[3 + inpos] >> 25) | ((input[4 + inpos] & 15) << (11 - 4));
    output[12 + outpos] = (input[4 + inpos] >> 4) & 2047;
    output[13 + outpos] = (input[4 + inpos] >> 15) & 2047;
    output[14 + outpos] = (input[4 + inpos] >> 26) | ((input[5 + inpos] & 31) << (11 - 5));
    output[15 + outpos] = (input[5 + inpos] >> 5) & 2047;
    output[16 + outpos] = (input[5 + inpos] >> 16) & 2047;
    output[17 + outpos] = (input[5 + inpos] >> 27) | ((input[6 + inpos] & 63) << (11 - 6));
    output[18 + outpos] = (input[6 + inpos] >> 6) & 2047;
    output[19 + outpos] = (input[6 + inpos] >> 17) & 2047;
    output[20 + outpos] = (input[6 + inpos] >> 28) | ((input[7 + inpos] & 127) << (11 - 7));
    output[21 + outpos] = (input[7 + inpos] >> 7) & 2047;
    output[22 + outpos] = (input[7 + inpos] >> 18) & 2047;
    output[23 + outpos] = (input[7 + inpos] >> 29) | ((input[8 + inpos] & 255) << (11 - 8));
    output[24 + outpos] = (input[8 + inpos] >> 8) & 2047;
    output[25 + outpos] = (input[8 + inpos] >> 19) & 2047;
    output[26 + outpos] = (input[8 + inpos] >> 30) | ((input[9 + inpos] & 511) << (11 - 9));
    output[27 + outpos] = (input[9 + inpos] >> 9) & 2047;
    output[28 + outpos] = (input[9 + inpos] >> 20) & 2047;
    output[29 + outpos] = (input[9 + inpos] >> 31) | ((input[10 + inpos] & 1023) << (11 - 10));
    output[30 + outpos] = (input[10 + inpos] >> 10) & 2047;
    output[31 + outpos] = input[10 + inpos] >> 21;
}

fn fast_unpack12(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 4095;
    output[1 + outpos] = (input[0 + inpos] >> 12) & 4095;
    output[2 + outpos] = (input[0 + inpos] >> 24) | ((input[1 + inpos] & 15) << (12 - 4));
    output[3 + outpos] = (input[1 + inpos] >> 4) & 4095;
    output[4 + outpos] = (input[1 + inpos] >> 16) & 4095;
    output[5 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 255) << (12 - 8));
    output[6 + outpos] = (input[2 + inpos] >> 8) & 4095;
    output[7 + outpos] = input[2 + inpos] >> 20;
    output[8 + outpos] = (input[3 + inpos] >> 0) & 4095;
    output[9 + outpos] = (input[3 + inpos] >> 12) & 4095;
    output[10 + outpos] = (input[3 + inpos] >> 24) | ((input[4 + inpos] & 15) << (12 - 4));
    output[11 + outpos] = (input[4 + inpos] >> 4) & 4095;
    output[12 + outpos] = (input[4 + inpos] >> 16) & 4095;
    output[13 + outpos] = (input[4 + inpos] >> 28) | ((input[5 + inpos] & 255) << (12 - 8));
    output[14 + outpos] = (input[5 + inpos] >> 8) & 4095;
    output[15 + outpos] = input[5 + inpos] >> 20;
    output[16 + outpos] = (input[6 + inpos] >> 0) & 4095;
    output[17 + outpos] = (input[6 + inpos] >> 12) & 4095;
    output[18 + outpos] = (input[6 + inpos] >> 24) | ((input[7 + inpos] & 15) << (12 - 4));
    output[19 + outpos] = (input[7 + inpos] >> 4) & 4095;
    output[20 + outpos] = (input[7 + inpos] >> 16) & 4095;
    output[21 + outpos] = (input[7 + inpos] >> 28) | ((input[8 + inpos] & 255) << (12 - 8));
    output[22 + outpos] = (input[8 + inpos] >> 8) & 4095;
    output[23 + outpos] = input[8 + inpos] >> 20;
    output[24 + outpos] = (input[9 + inpos] >> 0) & 4095;
    output[25 + outpos] = (input[9 + inpos] >> 12) & 4095;
    output[26 + outpos] = (input[9 + inpos] >> 24) | ((input[10 + inpos] & 15) << (12 - 4));
    output[27 + outpos] = (input[10 + inpos] >> 4) & 4095;
    output[28 + outpos] = (input[10 + inpos] >> 16) & 4095;
    output[29 + outpos] = (input[10 + inpos] >> 28) | ((input[11 + inpos] & 255) << (12 - 8));
    output[30 + outpos] = (input[11 + inpos] >> 8) & 4095;
    output[31 + outpos] = input[11 + inpos] >> 20;
}

fn fast_unpack13(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 8191;
    output[1 + outpos] = (input[0 + inpos] >> 13) & 8191;
    output[2 + outpos] = (input[0 + inpos] >> 26) | ((input[1 + inpos] & 127) << (13 - 7));
    output[3 + outpos] = (input[1 + inpos] >> 7) & 8191;
    output[4 + outpos] = (input[1 + inpos] >> 20) | ((input[2 + inpos] & 1) << (13 - 1));
    output[5 + outpos] = (input[2 + inpos] >> 1) & 8191;
    output[6 + outpos] = (input[2 + inpos] >> 14) & 8191;
    output[7 + outpos] = (input[2 + inpos] >> 27) | ((input[3 + inpos] & 255) << (13 - 8));
    output[8 + outpos] = (input[3 + inpos] >> 8) & 8191;
    output[9 + outpos] = (input[3 + inpos] >> 21) | ((input[4 + inpos] & 3) << (13 - 2));
    output[10 + outpos] = (input[4 + inpos] >> 2) & 8191;
    output[11 + outpos] = (input[4 + inpos] >> 15) & 8191;
    output[12 + outpos] = (input[4 + inpos] >> 28) | ((input[5 + inpos] & 511) << (13 - 9));
    output[13 + outpos] = (input[5 + inpos] >> 9) & 8191;
    output[14 + outpos] = (input[5 + inpos] >> 22) | ((input[6 + inpos] & 7) << (13 - 3));
    output[15 + outpos] = (input[6 + inpos] >> 3) & 8191;
    output[16 + outpos] = (input[6 + inpos] >> 16) & 8191;
    output[17 + outpos] = (input[6 + inpos] >> 29) | ((input[7 + inpos] & 1023) << (13 - 10));
    output[18 + outpos] = (input[7 + inpos] >> 10) & 8191;
    output[19 + outpos] = (input[7 + inpos] >> 23) | ((input[8 + inpos] & 15) << (13 - 4));
    output[20 + outpos] = (input[8 + inpos] >> 4) & 8191;
    output[21 + outpos] = (input[8 + inpos] >> 17) & 8191;
    output[22 + outpos] = (input[8 + inpos] >> 30) | ((input[9 + inpos] & 2047) << (13 - 11));
    output[23 + outpos] = (input[9 + inpos] >> 11) & 8191;
    output[24 + outpos] = (input[9 + inpos] >> 24) | ((input[10 + inpos] & 31) << (13 - 5));
    output[25 + outpos] = (input[10 + inpos] >> 5) & 8191;
    output[26 + outpos] = (input[10 + inpos] >> 18) & 8191;
    output[27 + outpos] = (input[10 + inpos] >> 31) | ((input[11 + inpos] & 4095) << (13 - 12));
    output[28 + outpos] = (input[11 + inpos] >> 12) & 8191;
    output[29 + outpos] = (input[11 + inpos] >> 25) | ((input[12 + inpos] & 63) << (13 - 6));
    output[30 + outpos] = (input[12 + inpos] >> 6) & 8191;
    output[31 + outpos] = input[12 + inpos] >> 19;
}

fn fast_unpack14(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 16383;
    output[1 + outpos] = (input[0 + inpos] >> 14) & 16383;
    output[2 + outpos] = (input[0 + inpos] >> 28) | ((input[1 + inpos] & 1023) << (14 - 10));
    output[3 + outpos] = (input[1 + inpos] >> 10) & 16383;
    output[4 + outpos] = (input[1 + inpos] >> 24) | ((input[2 + inpos] & 63) << (14 - 6));
    output[5 + outpos] = (input[2 + inpos] >> 6) & 16383;
    output[6 + outpos] = (input[2 + inpos] >> 20) | ((input[3 + inpos] & 3) << (14 - 2));
    output[7 + outpos] = (input[3 + inpos] >> 2) & 16383;
    output[8 + outpos] = (input[3 + inpos] >> 16) & 16383;
    output[9 + outpos] = (input[3 + inpos] >> 30) | ((input[4 + inpos] & 4095) << (14 - 12));
    output[10 + outpos] = (input[4 + inpos] >> 12) & 16383;
    output[11 + outpos] = (input[4 + inpos] >> 26) | ((input[5 + inpos] & 255) << (14 - 8));
    output[12 + outpos] = (input[5 + inpos] >> 8) & 16383;
    output[13 + outpos] = (input[5 + inpos] >> 22) | ((input[6 + inpos] & 15) << (14 - 4));
    output[14 + outpos] = (input[6 + inpos] >> 4) & 16383;
    output[15 + outpos] = input[6 + inpos] >> 18;
    output[16 + outpos] = (input[7 + inpos] >> 0) & 16383;
    output[17 + outpos] = (input[7 + inpos] >> 14) & 16383;
    output[18 + outpos] = (input[7 + inpos] >> 28) | ((input[8 + inpos] & 1023) << (14 - 10));
    output[19 + outpos] = (input[8 + inpos] >> 10) & 16383;
    output[20 + outpos] = (input[8 + inpos] >> 24) | ((input[9 + inpos] & 63) << (14 - 6));
    output[21 + outpos] = (input[9 + inpos] >> 6) & 16383;
    output[22 + outpos] = (input[9 + inpos] >> 20) | ((input[10 + inpos] & 3) << (14 - 2));
    output[23 + outpos] = (input[10 + inpos] >> 2) & 16383;
    output[24 + outpos] = (input[10 + inpos] >> 16) & 16383;
    output[25 + outpos] = (input[10 + inpos] >> 30) | ((input[11 + inpos] & 4095) << (14 - 12));
    output[26 + outpos] = (input[11 + inpos] >> 12) & 16383;
    output[27 + outpos] = (input[11 + inpos] >> 26) | ((input[12 + inpos] & 255) << (14 - 8));
    output[28 + outpos] = (input[12 + inpos] >> 8) & 16383;
    output[29 + outpos] = (input[12 + inpos] >> 22) | ((input[13 + inpos] & 15) << (14 - 4));
    output[30 + outpos] = (input[13 + inpos] >> 4) & 16383;
    output[31 + outpos] = input[13 + inpos] >> 18;
}

fn fast_unpack15(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 32767;
    output[1 + outpos] = (input[0 + inpos] >> 15) & 32767;
    output[2 + outpos] = (input[0 + inpos] >> 30) | ((input[1 + inpos] & 8191) << (15 - 13));
    output[3 + outpos] = (input[1 + inpos] >> 13) & 32767;
    output[4 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 2047) << (15 - 11));
    output[5 + outpos] = (input[2 + inpos] >> 11) & 32767;
    output[6 + outpos] = (input[2 + inpos] >> 26) | ((input[3 + inpos] & 511) << (15 - 9));
    output[7 + outpos] = (input[3 + inpos] >> 9) & 32767;
    output[8 + outpos] = (input[3 + inpos] >> 24) | ((input[4 + inpos] & 127) << (15 - 7));
    output[9 + outpos] = (input[4 + inpos] >> 7) & 32767;
    output[10 + outpos] = (input[4 + inpos] >> 22) | ((input[5 + inpos] & 31) << (15 - 5));
    output[11 + outpos] = (input[5 + inpos] >> 5) & 32767;
    output[12 + outpos] = (input[5 + inpos] >> 20) | ((input[6 + inpos] & 7) << (15 - 3));
    output[13 + outpos] = (input[6 + inpos] >> 3) & 32767;
    output[14 + outpos] = (input[6 + inpos] >> 18) | ((input[7 + inpos] & 1) << (15 - 1));
    output[15 + outpos] = (input[7 + inpos] >> 1) & 32767;
    output[16 + outpos] = (input[7 + inpos] >> 16) & 32767;
    output[17 + outpos] = (input[7 + inpos] >> 31) | ((input[8 + inpos] & 16383) << (15 - 14));
    output[18 + outpos] = (input[8 + inpos] >> 14) & 32767;
    output[19 + outpos] = (input[8 + inpos] >> 29) | ((input[9 + inpos] & 4095) << (15 - 12));
    output[20 + outpos] = (input[9 + inpos] >> 12) & 32767;
    output[21 + outpos] = (input[9 + inpos] >> 27) | ((input[10 + inpos] & 1023) << (15 - 10));
    output[22 + outpos] = (input[10 + inpos] >> 10) & 32767;
    output[23 + outpos] = (input[10 + inpos] >> 25) | ((input[11 + inpos] & 255) << (15 - 8));
    output[24 + outpos] = (input[11 + inpos] >> 8) & 32767;
    output[25 + outpos] = (input[11 + inpos] >> 23) | ((input[12 + inpos] & 63) << (15 - 6));
    output[26 + outpos] = (input[12 + inpos] >> 6) & 32767;
    output[27 + outpos] = (input[12 + inpos] >> 21) | ((input[13 + inpos] & 15) << (15 - 4));
    output[28 + outpos] = (input[13 + inpos] >> 4) & 32767;
    output[29 + outpos] = (input[13 + inpos] >> 19) | ((input[14 + inpos] & 3) << (15 - 2));
    output[30 + outpos] = (input[14 + inpos] >> 2) & 32767;
    output[31 + outpos] = input[14 + inpos] >> 17;
}

fn fast_unpack16(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 65535;
    output[1 + outpos] = input[0 + inpos] >> 16;
    output[2 + outpos] = (input[1 + inpos] >> 0) & 65535;
    output[3 + outpos] = input[1 + inpos] >> 16;
    output[4 + outpos] = (input[2 + inpos] >> 0) & 65535;
    output[5 + outpos] = input[2 + inpos] >> 16;
    output[6 + outpos] = (input[3 + inpos] >> 0) & 65535;
    output[7 + outpos] = input[3 + inpos] >> 16;
    output[8 + outpos] = (input[4 + inpos] >> 0) & 65535;
    output[9 + outpos] = input[4 + inpos] >> 16;
    output[10 + outpos] = (input[5 + inpos] >> 0) & 65535;
    output[11 + outpos] = input[5 + inpos] >> 16;
    output[12 + outpos] = (input[6 + inpos] >> 0) & 65535;
    output[13 + outpos] = input[6 + inpos] >> 16;
    output[14 + outpos] = (input[7 + inpos] >> 0) & 65535;
    output[15 + outpos] = input[7 + inpos] >> 16;
    output[16 + outpos] = (input[8 + inpos] >> 0) & 65535;
    output[17 + outpos] = input[8 + inpos] >> 16;
    output[18 + outpos] = (input[9 + inpos] >> 0) & 65535;
    output[19 + outpos] = input[9 + inpos] >> 16;
    output[20 + outpos] = (input[10 + inpos] >> 0) & 65535;
    output[21 + outpos] = input[10 + inpos] >> 16;
    output[22 + outpos] = (input[11 + inpos] >> 0) & 65535;
    output[23 + outpos] = input[11 + inpos] >> 16;
    output[24 + outpos] = (input[12 + inpos] >> 0) & 65535;
    output[25 + outpos] = input[12 + inpos] >> 16;
    output[26 + outpos] = (input[13 + inpos] >> 0) & 65535;
    output[27 + outpos] = input[13 + inpos] >> 16;
    output[28 + outpos] = (input[14 + inpos] >> 0) & 65535;
    output[29 + outpos] = input[14 + inpos] >> 16;
    output[30 + outpos] = (input[15 + inpos] >> 0) & 65535;
    output[31 + outpos] = input[15 + inpos] >> 16;
}

fn fast_unpack17(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 131071;
    output[1 + outpos] = (input[0 + inpos] >> 17) | ((input[1 + inpos] & 3) << (17 - 2));
    output[2 + outpos] = (input[1 + inpos] >> 2) & 131071;
    output[3 + outpos] = (input[1 + inpos] >> 19) | ((input[2 + inpos] & 15) << (17 - 4));
    output[4 + outpos] = (input[2 + inpos] >> 4) & 131071;
    output[5 + outpos] = (input[2 + inpos] >> 21) | ((input[3 + inpos] & 63) << (17 - 6));
    output[6 + outpos] = (input[3 + inpos] >> 6) & 131071;
    output[7 + outpos] = (input[3 + inpos] >> 23) | ((input[4 + inpos] & 255) << (17 - 8));
    output[8 + outpos] = (input[4 + inpos] >> 8) & 131071;
    output[9 + outpos] = (input[4 + inpos] >> 25) | ((input[5 + inpos] & 1023) << (17 - 10));
    output[10 + outpos] = (input[5 + inpos] >> 10) & 131071;
    output[11 + outpos] = (input[5 + inpos] >> 27) | ((input[6 + inpos] & 4095) << (17 - 12));
    output[12 + outpos] = (input[6 + inpos] >> 12) & 131071;
    output[13 + outpos] = (input[6 + inpos] >> 29) | ((input[7 + inpos] & 16383) << (17 - 14));
    output[14 + outpos] = (input[7 + inpos] >> 14) & 131071;
    output[15 + outpos] = (input[7 + inpos] >> 31) | ((input[8 + inpos] & 65535) << (17 - 16));
    output[16 + outpos] = (input[8 + inpos] >> 16) | ((input[9 + inpos] & 1) << (17 - 1));
    output[17 + outpos] = (input[9 + inpos] >> 1) & 131071;
    output[18 + outpos] = (input[9 + inpos] >> 18) | ((input[10 + inpos] & 7) << (17 - 3));
    output[19 + outpos] = (input[10 + inpos] >> 3) & 131071;
    output[20 + outpos] = (input[10 + inpos] >> 20) | ((input[11 + inpos] & 31) << (17 - 5));
    output[21 + outpos] = (input[11 + inpos] >> 5) & 131071;
    output[22 + outpos] = (input[11 + inpos] >> 22) | ((input[12 + inpos] & 127) << (17 - 7));
    output[23 + outpos] = (input[12 + inpos] >> 7) & 131071;
    output[24 + outpos] = (input[12 + inpos] >> 24) | ((input[13 + inpos] & 511) << (17 - 9));
    output[25 + outpos] = (input[13 + inpos] >> 9) & 131071;
    output[26 + outpos] = (input[13 + inpos] >> 26) | ((input[14 + inpos] & 2047) << (17 - 11));
    output[27 + outpos] = (input[14 + inpos] >> 11) & 131071;
    output[28 + outpos] = (input[14 + inpos] >> 28) | ((input[15 + inpos] & 8191) << (17 - 13));
    output[29 + outpos] = (input[15 + inpos] >> 13) & 131071;
    output[30 + outpos] = (input[15 + inpos] >> 30) | ((input[16 + inpos] & 32767) << (17 - 15));
    output[31 + outpos] = input[16 + inpos] >> 15;
}

fn fast_unpack18(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 262143;
    output[1 + outpos] = (input[0 + inpos] >> 18) | ((input[1 + inpos] & 15) << (18 - 4));
    output[2 + outpos] = (input[1 + inpos] >> 4) & 262143;
    output[3 + outpos] = (input[1 + inpos] >> 22) | ((input[2 + inpos] & 255) << (18 - 8));
    output[4 + outpos] = (input[2 + inpos] >> 8) & 262143;
    output[5 + outpos] = (input[2 + inpos] >> 26) | ((input[3 + inpos] & 4095) << (18 - 12));
    output[6 + outpos] = (input[3 + inpos] >> 12) & 262143;
    output[7 + outpos] = (input[3 + inpos] >> 30) | ((input[4 + inpos] & 65535) << (18 - 16));
    output[8 + outpos] = (input[4 + inpos] >> 16) | ((input[5 + inpos] & 3) << (18 - 2));
    output[9 + outpos] = (input[5 + inpos] >> 2) & 262143;
    output[10 + outpos] = (input[5 + inpos] >> 20) | ((input[6 + inpos] & 63) << (18 - 6));
    output[11 + outpos] = (input[6 + inpos] >> 6) & 262143;
    output[12 + outpos] = (input[6 + inpos] >> 24) | ((input[7 + inpos] & 1023) << (18 - 10));
    output[13 + outpos] = (input[7 + inpos] >> 10) & 262143;
    output[14 + outpos] = (input[7 + inpos] >> 28) | ((input[8 + inpos] & 16383) << (18 - 14));
    output[15 + outpos] = input[8 + inpos] >> 14;
    output[16 + outpos] = (input[9 + inpos] >> 0) & 262143;
    output[17 + outpos] = (input[9 + inpos] >> 18) | ((input[10 + inpos] & 15) << (18 - 4));
    output[18 + outpos] = (input[10 + inpos] >> 4) & 262143;
    output[19 + outpos] = (input[10 + inpos] >> 22) | ((input[11 + inpos] & 255) << (18 - 8));
    output[20 + outpos] = (input[11 + inpos] >> 8) & 262143;
    output[21 + outpos] = (input[11 + inpos] >> 26) | ((input[12 + inpos] & 4095) << (18 - 12));
    output[22 + outpos] = (input[12 + inpos] >> 12) & 262143;
    output[23 + outpos] = (input[12 + inpos] >> 30) | ((input[13 + inpos] & 65535) << (18 - 16));
    output[24 + outpos] = (input[13 + inpos] >> 16) | ((input[14 + inpos] & 3) << (18 - 2));
    output[25 + outpos] = (input[14 + inpos] >> 2) & 262143;
    output[26 + outpos] = (input[14 + inpos] >> 20) | ((input[15 + inpos] & 63) << (18 - 6));
    output[27 + outpos] = (input[15 + inpos] >> 6) & 262143;
    output[28 + outpos] = (input[15 + inpos] >> 24) | ((input[16 + inpos] & 1023) << (18 - 10));
    output[29 + outpos] = (input[16 + inpos] >> 10) & 262143;
    output[30 + outpos] = (input[16 + inpos] >> 28) | ((input[17 + inpos] & 16383) << (18 - 14));
    output[31 + outpos] = input[17 + inpos] >> 14;
}

fn fast_unpack19(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 524287;
    output[1 + outpos] = (input[0 + inpos] >> 19) | ((input[1 + inpos] & 63) << (19 - 6));
    output[2 + outpos] = (input[1 + inpos] >> 6) & 524287;
    output[3 + outpos] = (input[1 + inpos] >> 25) | ((input[2 + inpos] & 4095) << (19 - 12));
    output[4 + outpos] = (input[2 + inpos] >> 12) & 524287;
    output[5 + outpos] = (input[2 + inpos] >> 31) | ((input[3 + inpos] & 262143) << (19 - 18));
    output[6 + outpos] = (input[3 + inpos] >> 18) | ((input[4 + inpos] & 31) << (19 - 5));
    output[7 + outpos] = (input[4 + inpos] >> 5) & 524287;
    output[8 + outpos] = (input[4 + inpos] >> 24) | ((input[5 + inpos] & 2047) << (19 - 11));
    output[9 + outpos] = (input[5 + inpos] >> 11) & 524287;
    output[10 + outpos] = (input[5 + inpos] >> 30) | ((input[6 + inpos] & 131071) << (19 - 17));
    output[11 + outpos] = (input[6 + inpos] >> 17) | ((input[7 + inpos] & 15) << (19 - 4));
    output[12 + outpos] = (input[7 + inpos] >> 4) & 524287;
    output[13 + outpos] = (input[7 + inpos] >> 23) | ((input[8 + inpos] & 1023) << (19 - 10));
    output[14 + outpos] = (input[8 + inpos] >> 10) & 524287;
    output[15 + outpos] = (input[8 + inpos] >> 29) | ((input[9 + inpos] & 65535) << (19 - 16));
    output[16 + outpos] = (input[9 + inpos] >> 16) | ((input[10 + inpos] & 7) << (19 - 3));
    output[17 + outpos] = (input[10 + inpos] >> 3) & 524287;
    output[18 + outpos] = (input[10 + inpos] >> 22) | ((input[11 + inpos] & 511) << (19 - 9));
    output[19 + outpos] = (input[11 + inpos] >> 9) & 524287;
    output[20 + outpos] = (input[11 + inpos] >> 28) | ((input[12 + inpos] & 32767) << (19 - 15));
    output[21 + outpos] = (input[12 + inpos] >> 15) | ((input[13 + inpos] & 3) << (19 - 2));
    output[22 + outpos] = (input[13 + inpos] >> 2) & 524287;
    output[23 + outpos] = (input[13 + inpos] >> 21) | ((input[14 + inpos] & 255) << (19 - 8));
    output[24 + outpos] = (input[14 + inpos] >> 8) & 524287;
    output[25 + outpos] = (input[14 + inpos] >> 27) | ((input[15 + inpos] & 16383) << (19 - 14));
    output[26 + outpos] = (input[15 + inpos] >> 14) | ((input[16 + inpos] & 1) << (19 - 1));
    output[27 + outpos] = (input[16 + inpos] >> 1) & 524287;
    output[28 + outpos] = (input[16 + inpos] >> 20) | ((input[17 + inpos] & 127) << (19 - 7));
    output[29 + outpos] = (input[17 + inpos] >> 7) & 524287;
    output[30 + outpos] = (input[17 + inpos] >> 26) | ((input[18 + inpos] & 8191) << (19 - 13));
    output[31 + outpos] = input[18 + inpos] >> 13;
}

fn fast_unpack20(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 1048575;
    output[1 + outpos] = (input[0 + inpos] >> 20) | ((input[1 + inpos] & 255) << (20 - 8));
    output[2 + outpos] = (input[1 + inpos] >> 8) & 1048575;
    output[3 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 65535) << (20 - 16));
    output[4 + outpos] = (input[2 + inpos] >> 16) | ((input[3 + inpos] & 15) << (20 - 4));
    output[5 + outpos] = (input[3 + inpos] >> 4) & 1048575;
    output[6 + outpos] = (input[3 + inpos] >> 24) | ((input[4 + inpos] & 4095) << (20 - 12));
    output[7 + outpos] = input[4 + inpos] >> 12;
    output[8 + outpos] = (input[5 + inpos] >> 0) & 1048575;
    output[9 + outpos] = (input[5 + inpos] >> 20) | ((input[6 + inpos] & 255) << (20 - 8));
    output[10 + outpos] = (input[6 + inpos] >> 8) & 1048575;
    output[11 + outpos] = (input[6 + inpos] >> 28) | ((input[7 + inpos] & 65535) << (20 - 16));
    output[12 + outpos] = (input[7 + inpos] >> 16) | ((input[8 + inpos] & 15) << (20 - 4));
    output[13 + outpos] = (input[8 + inpos] >> 4) & 1048575;
    output[14 + outpos] = (input[8 + inpos] >> 24) | ((input[9 + inpos] & 4095) << (20 - 12));
    output[15 + outpos] = input[9 + inpos] >> 12;
    output[16 + outpos] = (input[10 + inpos] >> 0) & 1048575;
    output[17 + outpos] = (input[10 + inpos] >> 20) | ((input[11 + inpos] & 255) << (20 - 8));
    output[18 + outpos] = (input[11 + inpos] >> 8) & 1048575;
    output[19 + outpos] = (input[11 + inpos] >> 28) | ((input[12 + inpos] & 65535) << (20 - 16));
    output[20 + outpos] = (input[12 + inpos] >> 16) | ((input[13 + inpos] & 15) << (20 - 4));
    output[21 + outpos] = (input[13 + inpos] >> 4) & 1048575;
    output[22 + outpos] = (input[13 + inpos] >> 24) | ((input[14 + inpos] & 4095) << (20 - 12));
    output[23 + outpos] = input[14 + inpos] >> 12;
    output[24 + outpos] = (input[15 + inpos] >> 0) & 1048575;
    output[25 + outpos] = (input[15 + inpos] >> 20) | ((input[16 + inpos] & 255) << (20 - 8));
    output[26 + outpos] = (input[16 + inpos] >> 8) & 1048575;
    output[27 + outpos] = (input[16 + inpos] >> 28) | ((input[17 + inpos] & 65535) << (20 - 16));
    output[28 + outpos] = (input[17 + inpos] >> 16) | ((input[18 + inpos] & 15) << (20 - 4));
    output[29 + outpos] = (input[18 + inpos] >> 4) & 1048575;
    output[30 + outpos] = (input[18 + inpos] >> 24) | ((input[19 + inpos] & 4095) << (20 - 12));
    output[31 + outpos] = input[19 + inpos] >> 12;
}

fn fast_unpack21(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 2097151;
    output[1 + outpos] = (input[0 + inpos] >> 21) | ((input[1 + inpos] & 1023) << (21 - 10));
    output[2 + outpos] = (input[1 + inpos] >> 10) & 2097151;
    output[3 + outpos] = (input[1 + inpos] >> 31) | ((input[2 + inpos] & 1048575) << (21 - 20));
    output[4 + outpos] = (input[2 + inpos] >> 20) | ((input[3 + inpos] & 511) << (21 - 9));
    output[5 + outpos] = (input[3 + inpos] >> 9) & 2097151;
    output[6 + outpos] = (input[3 + inpos] >> 30) | ((input[4 + inpos] & 524287) << (21 - 19));
    output[7 + outpos] = (input[4 + inpos] >> 19) | ((input[5 + inpos] & 255) << (21 - 8));
    output[8 + outpos] = (input[5 + inpos] >> 8) & 2097151;
    output[9 + outpos] = (input[5 + inpos] >> 29) | ((input[6 + inpos] & 262143) << (21 - 18));
    output[10 + outpos] = (input[6 + inpos] >> 18) | ((input[7 + inpos] & 127) << (21 - 7));
    output[11 + outpos] = (input[7 + inpos] >> 7) & 2097151;
    output[12 + outpos] = (input[7 + inpos] >> 28) | ((input[8 + inpos] & 131071) << (21 - 17));
    output[13 + outpos] = (input[8 + inpos] >> 17) | ((input[9 + inpos] & 63) << (21 - 6));
    output[14 + outpos] = (input[9 + inpos] >> 6) & 2097151;
    output[15 + outpos] = (input[9 + inpos] >> 27) | ((input[10 + inpos] & 65535) << (21 - 16));
    output[16 + outpos] = (input[10 + inpos] >> 16) | ((input[11 + inpos] & 31) << (21 - 5));
    output[17 + outpos] = (input[11 + inpos] >> 5) & 2097151;
    output[18 + outpos] = (input[11 + inpos] >> 26) | ((input[12 + inpos] & 32767) << (21 - 15));
    output[19 + outpos] = (input[12 + inpos] >> 15) | ((input[13 + inpos] & 15) << (21 - 4));
    output[20 + outpos] = (input[13 + inpos] >> 4) & 2097151;
    output[21 + outpos] = (input[13 + inpos] >> 25) | ((input[14 + inpos] & 16383) << (21 - 14));
    output[22 + outpos] = (input[14 + inpos] >> 14) | ((input[15 + inpos] & 7) << (21 - 3));
    output[23 + outpos] = (input[15 + inpos] >> 3) & 2097151;
    output[24 + outpos] = (input[15 + inpos] >> 24) | ((input[16 + inpos] & 8191) << (21 - 13));
    output[25 + outpos] = (input[16 + inpos] >> 13) | ((input[17 + inpos] & 3) << (21 - 2));
    output[26 + outpos] = (input[17 + inpos] >> 2) & 2097151;
    output[27 + outpos] = (input[17 + inpos] >> 23) | ((input[18 + inpos] & 4095) << (21 - 12));
    output[28 + outpos] = (input[18 + inpos] >> 12) | ((input[19 + inpos] & 1) << (21 - 1));
    output[29 + outpos] = (input[19 + inpos] >> 1) & 2097151;
    output[30 + outpos] = (input[19 + inpos] >> 22) | ((input[20 + inpos] & 2047) << (21 - 11));
    output[31 + outpos] = input[20 + inpos] >> 11;
}

fn fast_unpack22(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 4194303;
    output[1 + outpos] = (input[0 + inpos] >> 22) | ((input[1 + inpos] & 4095) << (22 - 12));
    output[2 + outpos] = (input[1 + inpos] >> 12) | ((input[2 + inpos] & 3) << (22 - 2));
    output[3 + outpos] = (input[2 + inpos] >> 2) & 4194303;
    output[4 + outpos] = (input[2 + inpos] >> 24) | ((input[3 + inpos] & 16383) << (22 - 14));
    output[5 + outpos] = (input[3 + inpos] >> 14) | ((input[4 + inpos] & 15) << (22 - 4));
    output[6 + outpos] = (input[4 + inpos] >> 4) & 4194303;
    output[7 + outpos] = (input[4 + inpos] >> 26) | ((input[5 + inpos] & 65535) << (22 - 16));
    output[8 + outpos] = (input[5 + inpos] >> 16) | ((input[6 + inpos] & 63) << (22 - 6));
    output[9 + outpos] = (input[6 + inpos] >> 6) & 4194303;
    output[10 + outpos] = (input[6 + inpos] >> 28) | ((input[7 + inpos] & 262143) << (22 - 18));
    output[11 + outpos] = (input[7 + inpos] >> 18) | ((input[8 + inpos] & 255) << (22 - 8));
    output[12 + outpos] = (input[8 + inpos] >> 8) & 4194303;
    output[13 + outpos] = (input[8 + inpos] >> 30) | ((input[9 + inpos] & 1048575) << (22 - 20));
    output[14 + outpos] = (input[9 + inpos] >> 20) | ((input[10 + inpos] & 1023) << (22 - 10));
    output[15 + outpos] = input[10 + inpos] >> 10;
    output[16 + outpos] = (input[11 + inpos] >> 0) & 4194303;
    output[17 + outpos] = (input[11 + inpos] >> 22) | ((input[12 + inpos] & 4095) << (22 - 12));
    output[18 + outpos] = (input[12 + inpos] >> 12) | ((input[13 + inpos] & 3) << (22 - 2));
    output[19 + outpos] = (input[13 + inpos] >> 2) & 4194303;
    output[20 + outpos] = (input[13 + inpos] >> 24) | ((input[14 + inpos] & 16383) << (22 - 14));
    output[21 + outpos] = (input[14 + inpos] >> 14) | ((input[15 + inpos] & 15) << (22 - 4));
    output[22 + outpos] = (input[15 + inpos] >> 4) & 4194303;
    output[23 + outpos] = (input[15 + inpos] >> 26) | ((input[16 + inpos] & 65535) << (22 - 16));
    output[24 + outpos] = (input[16 + inpos] >> 16) | ((input[17 + inpos] & 63) << (22 - 6));
    output[25 + outpos] = (input[17 + inpos] >> 6) & 4194303;
    output[26 + outpos] = (input[17 + inpos] >> 28) | ((input[18 + inpos] & 262143) << (22 - 18));
    output[27 + outpos] = (input[18 + inpos] >> 18) | ((input[19 + inpos] & 255) << (22 - 8));
    output[28 + outpos] = (input[19 + inpos] >> 8) & 4194303;
    output[29 + outpos] = (input[19 + inpos] >> 30) | ((input[20 + inpos] & 1048575) << (22 - 20));
    output[30 + outpos] = (input[20 + inpos] >> 20) | ((input[21 + inpos] & 1023) << (22 - 10));
    output[31 + outpos] = input[21 + inpos] >> 10;
}

fn fast_unpack23(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 8388607;
    output[1 + outpos] = (input[0 + inpos] >> 23) | ((input[1 + inpos] & 16383) << (23 - 14));
    output[2 + outpos] = (input[1 + inpos] >> 14) | ((input[2 + inpos] & 31) << (23 - 5));
    output[3 + outpos] = (input[2 + inpos] >> 5) & 8388607;
    output[4 + outpos] = (input[2 + inpos] >> 28) | ((input[3 + inpos] & 524287) << (23 - 19));
    output[5 + outpos] = (input[3 + inpos] >> 19) | ((input[4 + inpos] & 1023) << (23 - 10));
    output[6 + outpos] = (input[4 + inpos] >> 10) | ((input[5 + inpos] & 1) << (23 - 1));
    output[7 + outpos] = (input[5 + inpos] >> 1) & 8388607;
    output[8 + outpos] = (input[5 + inpos] >> 24) | ((input[6 + inpos] & 32767) << (23 - 15));
    output[9 + outpos] = (input[6 + inpos] >> 15) | ((input[7 + inpos] & 63) << (23 - 6));
    output[10 + outpos] = (input[7 + inpos] >> 6) & 8388607;
    output[11 + outpos] = (input[7 + inpos] >> 29) | ((input[8 + inpos] & 1048575) << (23 - 20));
    output[12 + outpos] = (input[8 + inpos] >> 20) | ((input[9 + inpos] & 2047) << (23 - 11));
    output[13 + outpos] = (input[9 + inpos] >> 11) | ((input[10 + inpos] & 3) << (23 - 2));
    output[14 + outpos] = (input[10 + inpos] >> 2) & 8388607;
    output[15 + outpos] = (input[10 + inpos] >> 25) | ((input[11 + inpos] & 65535) << (23 - 16));
    output[16 + outpos] = (input[11 + inpos] >> 16) | ((input[12 + inpos] & 127) << (23 - 7));
    output[17 + outpos] = (input[12 + inpos] >> 7) & 8388607;
    output[18 + outpos] = (input[12 + inpos] >> 30) | ((input[13 + inpos] & 2097151) << (23 - 21));
    output[19 + outpos] = (input[13 + inpos] >> 21) | ((input[14 + inpos] & 4095) << (23 - 12));
    output[20 + outpos] = (input[14 + inpos] >> 12) | ((input[15 + inpos] & 7) << (23 - 3));
    output[21 + outpos] = (input[15 + inpos] >> 3) & 8388607;
    output[22 + outpos] = (input[15 + inpos] >> 26) | ((input[16 + inpos] & 131071) << (23 - 17));
    output[23 + outpos] = (input[16 + inpos] >> 17) | ((input[17 + inpos] & 255) << (23 - 8));
    output[24 + outpos] = (input[17 + inpos] >> 8) & 8388607;
    output[25 + outpos] = (input[17 + inpos] >> 31) | ((input[18 + inpos] & 4194303) << (23 - 22));
    output[26 + outpos] = (input[18 + inpos] >> 22) | ((input[19 + inpos] & 8191) << (23 - 13));
    output[27 + outpos] = (input[19 + inpos] >> 13) | ((input[20 + inpos] & 15) << (23 - 4));
    output[28 + outpos] = (input[20 + inpos] >> 4) & 8388607;
    output[29 + outpos] = (input[20 + inpos] >> 27) | ((input[21 + inpos] & 262143) << (23 - 18));
    output[30 + outpos] = (input[21 + inpos] >> 18) | ((input[22 + inpos] & 511) << (23 - 9));
    output[31 + outpos] = input[22 + inpos] >> 9;
}

fn fast_unpack24(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 16777215;
    output[1 + outpos] = (input[0 + inpos] >> 24) | ((input[1 + inpos] & 65535) << (24 - 16));
    output[2 + outpos] = (input[1 + inpos] >> 16) | ((input[2 + inpos] & 255) << (24 - 8));
    output[3 + outpos] = input[2 + inpos] >> 8;
    output[4 + outpos] = (input[3 + inpos] >> 0) & 16777215;
    output[5 + outpos] = (input[3 + inpos] >> 24) | ((input[4 + inpos] & 65535) << (24 - 16));
    output[6 + outpos] = (input[4 + inpos] >> 16) | ((input[5 + inpos] & 255) << (24 - 8));
    output[7 + outpos] = input[5 + inpos] >> 8;
    output[8 + outpos] = (input[6 + inpos] >> 0) & 16777215;
    output[9 + outpos] = (input[6 + inpos] >> 24) | ((input[7 + inpos] & 65535) << (24 - 16));
    output[10 + outpos] = (input[7 + inpos] >> 16) | ((input[8 + inpos] & 255) << (24 - 8));
    output[11 + outpos] = input[8 + inpos] >> 8;
    output[12 + outpos] = (input[9 + inpos] >> 0) & 16777215;
    output[13 + outpos] = (input[9 + inpos] >> 24) | ((input[10 + inpos] & 65535) << (24 - 16));
    output[14 + outpos] = (input[10 + inpos] >> 16) | ((input[11 + inpos] & 255) << (24 - 8));
    output[15 + outpos] = input[11 + inpos] >> 8;
    output[16 + outpos] = (input[12 + inpos] >> 0) & 16777215;
    output[17 + outpos] = (input[12 + inpos] >> 24) | ((input[13 + inpos] & 65535) << (24 - 16));
    output[18 + outpos] = (input[13 + inpos] >> 16) | ((input[14 + inpos] & 255) << (24 - 8));
    output[19 + outpos] = input[14 + inpos] >> 8;
    output[20 + outpos] = (input[15 + inpos] >> 0) & 16777215;
    output[21 + outpos] = (input[15 + inpos] >> 24) | ((input[16 + inpos] & 65535) << (24 - 16));
    output[22 + outpos] = (input[16 + inpos] >> 16) | ((input[17 + inpos] & 255) << (24 - 8));
    output[23 + outpos] = input[17 + inpos] >> 8;
    output[24 + outpos] = (input[18 + inpos] >> 0) & 16777215;
    output[25 + outpos] = (input[18 + inpos] >> 24) | ((input[19 + inpos] & 65535) << (24 - 16));
    output[26 + outpos] = (input[19 + inpos] >> 16) | ((input[20 + inpos] & 255) << (24 - 8));
    output[27 + outpos] = input[20 + inpos] >> 8;
    output[28 + outpos] = (input[21 + inpos] >> 0) & 16777215;
    output[29 + outpos] = (input[21 + inpos] >> 24) | ((input[22 + inpos] & 65535) << (24 - 16));
    output[30 + outpos] = (input[22 + inpos] >> 16) | ((input[23 + inpos] & 255) << (24 - 8));
    output[31 + outpos] = input[23 + inpos] >> 8;
}

fn fast_unpack25(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 33554431;
    output[1 + outpos] = (input[0 + inpos] >> 25) | ((input[1 + inpos] & 262143) << (25 - 18));
    output[2 + outpos] = (input[1 + inpos] >> 18) | ((input[2 + inpos] & 2047) << (25 - 11));
    output[3 + outpos] = (input[2 + inpos] >> 11) | ((input[3 + inpos] & 15) << (25 - 4));
    output[4 + outpos] = (input[3 + inpos] >> 4) & 33554431;
    output[5 + outpos] = (input[3 + inpos] >> 29) | ((input[4 + inpos] & 4194303) << (25 - 22));
    output[6 + outpos] = (input[4 + inpos] >> 22) | ((input[5 + inpos] & 32767) << (25 - 15));
    output[7 + outpos] = (input[5 + inpos] >> 15) | ((input[6 + inpos] & 255) << (25 - 8));
    output[8 + outpos] = (input[6 + inpos] >> 8) | ((input[7 + inpos] & 1) << (25 - 1));
    output[9 + outpos] = (input[7 + inpos] >> 1) & 33554431;
    output[10 + outpos] = (input[7 + inpos] >> 26) | ((input[8 + inpos] & 524287) << (25 - 19));
    output[11 + outpos] = (input[8 + inpos] >> 19) | ((input[9 + inpos] & 4095) << (25 - 12));
    output[12 + outpos] = (input[9 + inpos] >> 12) | ((input[10 + inpos] & 31) << (25 - 5));
    output[13 + outpos] = (input[10 + inpos] >> 5) & 33554431;
    output[14 + outpos] = (input[10 + inpos] >> 30) | ((input[11 + inpos] & 8388607) << (25 - 23));
    output[15 + outpos] = (input[11 + inpos] >> 23) | ((input[12 + inpos] & 65535) << (25 - 16));
    output[16 + outpos] = (input[12 + inpos] >> 16) | ((input[13 + inpos] & 511) << (25 - 9));
    output[17 + outpos] = (input[13 + inpos] >> 9) | ((input[14 + inpos] & 3) << (25 - 2));
    output[18 + outpos] = (input[14 + inpos] >> 2) & 33554431;
    output[19 + outpos] = (input[14 + inpos] >> 27) | ((input[15 + inpos] & 1048575) << (25 - 20));
    output[20 + outpos] = (input[15 + inpos] >> 20) | ((input[16 + inpos] & 8191) << (25 - 13));
    output[21 + outpos] = (input[16 + inpos] >> 13) | ((input[17 + inpos] & 63) << (25 - 6));
    output[22 + outpos] = (input[17 + inpos] >> 6) & 33554431;
    output[23 + outpos] = (input[17 + inpos] >> 31) | ((input[18 + inpos] & 16777215) << (25 - 24));
    output[24 + outpos] = (input[18 + inpos] >> 24) | ((input[19 + inpos] & 131071) << (25 - 17));
    output[25 + outpos] = (input[19 + inpos] >> 17) | ((input[20 + inpos] & 1023) << (25 - 10));
    output[26 + outpos] = (input[20 + inpos] >> 10) | ((input[21 + inpos] & 7) << (25 - 3));
    output[27 + outpos] = (input[21 + inpos] >> 3) & 33554431;
    output[28 + outpos] = (input[21 + inpos] >> 28) | ((input[22 + inpos] & 2097151) << (25 - 21));
    output[29 + outpos] = (input[22 + inpos] >> 21) | ((input[23 + inpos] & 16383) << (25 - 14));
    output[30 + outpos] = (input[23 + inpos] >> 14) | ((input[24 + inpos] & 127) << (25 - 7));
    output[31 + outpos] = input[24 + inpos] >> 7;
}

fn fast_unpack26(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 67108863;
    output[1 + outpos] = (input[0 + inpos] >> 26) | ((input[1 + inpos] & 1048575) << (26 - 20));
    output[2 + outpos] = (input[1 + inpos] >> 20) | ((input[2 + inpos] & 16383) << (26 - 14));
    output[3 + outpos] = (input[2 + inpos] >> 14) | ((input[3 + inpos] & 255) << (26 - 8));
    output[4 + outpos] = (input[3 + inpos] >> 8) | ((input[4 + inpos] & 3) << (26 - 2));
    output[5 + outpos] = (input[4 + inpos] >> 2) & 67108863;
    output[6 + outpos] = (input[4 + inpos] >> 28) | ((input[5 + inpos] & 4194303) << (26 - 22));
    output[7 + outpos] = (input[5 + inpos] >> 22) | ((input[6 + inpos] & 65535) << (26 - 16));
    output[8 + outpos] = (input[6 + inpos] >> 16) | ((input[7 + inpos] & 1023) << (26 - 10));
    output[9 + outpos] = (input[7 + inpos] >> 10) | ((input[8 + inpos] & 15) << (26 - 4));
    output[10 + outpos] = (input[8 + inpos] >> 4) & 67108863;
    output[11 + outpos] = (input[8 + inpos] >> 30) | ((input[9 + inpos] & 16777215) << (26 - 24));
    output[12 + outpos] = (input[9 + inpos] >> 24) | ((input[10 + inpos] & 262143) << (26 - 18));
    output[13 + outpos] = (input[10 + inpos] >> 18) | ((input[11 + inpos] & 4095) << (26 - 12));
    output[14 + outpos] = (input[11 + inpos] >> 12) | ((input[12 + inpos] & 63) << (26 - 6));
    output[15 + outpos] = input[12 + inpos] >> 6;
    output[16 + outpos] = (input[13 + inpos] >> 0) & 67108863;
    output[17 + outpos] = (input[13 + inpos] >> 26) | ((input[14 + inpos] & 1048575) << (26 - 20));
    output[18 + outpos] = (input[14 + inpos] >> 20) | ((input[15 + inpos] & 16383) << (26 - 14));
    output[19 + outpos] = (input[15 + inpos] >> 14) | ((input[16 + inpos] & 255) << (26 - 8));
    output[20 + outpos] = (input[16 + inpos] >> 8) | ((input[17 + inpos] & 3) << (26 - 2));
    output[21 + outpos] = (input[17 + inpos] >> 2) & 67108863;
    output[22 + outpos] = (input[17 + inpos] >> 28) | ((input[18 + inpos] & 4194303) << (26 - 22));
    output[23 + outpos] = (input[18 + inpos] >> 22) | ((input[19 + inpos] & 65535) << (26 - 16));
    output[24 + outpos] = (input[19 + inpos] >> 16) | ((input[20 + inpos] & 1023) << (26 - 10));
    output[25 + outpos] = (input[20 + inpos] >> 10) | ((input[21 + inpos] & 15) << (26 - 4));
    output[26 + outpos] = (input[21 + inpos] >> 4) & 67108863;
    output[27 + outpos] = (input[21 + inpos] >> 30) | ((input[22 + inpos] & 16777215) << (26 - 24));
    output[28 + outpos] = (input[22 + inpos] >> 24) | ((input[23 + inpos] & 262143) << (26 - 18));
    output[29 + outpos] = (input[23 + inpos] >> 18) | ((input[24 + inpos] & 4095) << (26 - 12));
    output[30 + outpos] = (input[24 + inpos] >> 12) | ((input[25 + inpos] & 63) << (26 - 6));
    output[31 + outpos] = input[25 + inpos] >> 6;
}

fn fast_unpack27(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 134217727;
    output[1 + outpos] = (input[0 + inpos] >> 27) | ((input[1 + inpos] & 4194303) << (27 - 22));
    output[2 + outpos] = (input[1 + inpos] >> 22) | ((input[2 + inpos] & 131071) << (27 - 17));
    output[3 + outpos] = (input[2 + inpos] >> 17) | ((input[3 + inpos] & 4095) << (27 - 12));
    output[4 + outpos] = (input[3 + inpos] >> 12) | ((input[4 + inpos] & 127) << (27 - 7));
    output[5 + outpos] = (input[4 + inpos] >> 7) | ((input[5 + inpos] & 3) << (27 - 2));
    output[6 + outpos] = (input[5 + inpos] >> 2) & 134217727;
    output[7 + outpos] = (input[5 + inpos] >> 29) | ((input[6 + inpos] & 16777215) << (27 - 24));
    output[8 + outpos] = (input[6 + inpos] >> 24) | ((input[7 + inpos] & 524287) << (27 - 19));
    output[9 + outpos] = (input[7 + inpos] >> 19) | ((input[8 + inpos] & 16383) << (27 - 14));
    output[10 + outpos] = (input[8 + inpos] >> 14) | ((input[9 + inpos] & 511) << (27 - 9));
    output[11 + outpos] = (input[9 + inpos] >> 9) | ((input[10 + inpos] & 15) << (27 - 4));
    output[12 + outpos] = (input[10 + inpos] >> 4) & 134217727;
    output[13 + outpos] = (input[10 + inpos] >> 31) | ((input[11 + inpos] & 67108863) << (27 - 26));
    output[14 + outpos] = (input[11 + inpos] >> 26) | ((input[12 + inpos] & 2097151) << (27 - 21));
    output[15 + outpos] = (input[12 + inpos] >> 21) | ((input[13 + inpos] & 65535) << (27 - 16));
    output[16 + outpos] = (input[13 + inpos] >> 16) | ((input[14 + inpos] & 2047) << (27 - 11));
    output[17 + outpos] = (input[14 + inpos] >> 11) | ((input[15 + inpos] & 63) << (27 - 6));
    output[18 + outpos] = (input[15 + inpos] >> 6) | ((input[16 + inpos] & 1) << (27 - 1));
    output[19 + outpos] = (input[16 + inpos] >> 1) & 134217727;
    output[20 + outpos] = (input[16 + inpos] >> 28) | ((input[17 + inpos] & 8388607) << (27 - 23));
    output[21 + outpos] = (input[17 + inpos] >> 23) | ((input[18 + inpos] & 262143) << (27 - 18));
    output[22 + outpos] = (input[18 + inpos] >> 18) | ((input[19 + inpos] & 8191) << (27 - 13));
    output[23 + outpos] = (input[19 + inpos] >> 13) | ((input[20 + inpos] & 255) << (27 - 8));
    output[24 + outpos] = (input[20 + inpos] >> 8) | ((input[21 + inpos] & 7) << (27 - 3));
    output[25 + outpos] = (input[21 + inpos] >> 3) & 134217727;
    output[26 + outpos] = (input[21 + inpos] >> 30) | ((input[22 + inpos] & 33554431) << (27 - 25));
    output[27 + outpos] = (input[22 + inpos] >> 25) | ((input[23 + inpos] & 1048575) << (27 - 20));
    output[28 + outpos] = (input[23 + inpos] >> 20) | ((input[24 + inpos] & 32767) << (27 - 15));
    output[29 + outpos] = (input[24 + inpos] >> 15) | ((input[25 + inpos] & 1023) << (27 - 10));
    output[30 + outpos] = (input[25 + inpos] >> 10) | ((input[26 + inpos] & 31) << (27 - 5));
    output[31 + outpos] = input[26 + inpos] >> 5;
}

fn fast_unpack28(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 268435455;
    output[1 + outpos] = (input[0 + inpos] >> 28) | ((input[1 + inpos] & 16777215) << (28 - 24));
    output[2 + outpos] = (input[1 + inpos] >> 24) | ((input[2 + inpos] & 1048575) << (28 - 20));
    output[3 + outpos] = (input[2 + inpos] >> 20) | ((input[3 + inpos] & 65535) << (28 - 16));
    output[4 + outpos] = (input[3 + inpos] >> 16) | ((input[4 + inpos] & 4095) << (28 - 12));
    output[5 + outpos] = (input[4 + inpos] >> 12) | ((input[5 + inpos] & 255) << (28 - 8));
    output[6 + outpos] = (input[5 + inpos] >> 8) | ((input[6 + inpos] & 15) << (28 - 4));
    output[7 + outpos] = input[6 + inpos] >> 4;
    output[8 + outpos] = (input[7 + inpos] >> 0) & 268435455;
    output[9 + outpos] = (input[7 + inpos] >> 28) | ((input[8 + inpos] & 16777215) << (28 - 24));
    output[10 + outpos] = (input[8 + inpos] >> 24) | ((input[9 + inpos] & 1048575) << (28 - 20));
    output[11 + outpos] = (input[9 + inpos] >> 20) | ((input[10 + inpos] & 65535) << (28 - 16));
    output[12 + outpos] = (input[10 + inpos] >> 16) | ((input[11 + inpos] & 4095) << (28 - 12));
    output[13 + outpos] = (input[11 + inpos] >> 12) | ((input[12 + inpos] & 255) << (28 - 8));
    output[14 + outpos] = (input[12 + inpos] >> 8) | ((input[13 + inpos] & 15) << (28 - 4));
    output[15 + outpos] = input[13 + inpos] >> 4;
    output[16 + outpos] = (input[14 + inpos] >> 0) & 268435455;
    output[17 + outpos] = (input[14 + inpos] >> 28) | ((input[15 + inpos] & 16777215) << (28 - 24));
    output[18 + outpos] = (input[15 + inpos] >> 24) | ((input[16 + inpos] & 1048575) << (28 - 20));
    output[19 + outpos] = (input[16 + inpos] >> 20) | ((input[17 + inpos] & 65535) << (28 - 16));
    output[20 + outpos] = (input[17 + inpos] >> 16) | ((input[18 + inpos] & 4095) << (28 - 12));
    output[21 + outpos] = (input[18 + inpos] >> 12) | ((input[19 + inpos] & 255) << (28 - 8));
    output[22 + outpos] = (input[19 + inpos] >> 8) | ((input[20 + inpos] & 15) << (28 - 4));
    output[23 + outpos] = input[20 + inpos] >> 4;
    output[24 + outpos] = (input[21 + inpos] >> 0) & 268435455;
    output[25 + outpos] = (input[21 + inpos] >> 28) | ((input[22 + inpos] & 16777215) << (28 - 24));
    output[26 + outpos] = (input[22 + inpos] >> 24) | ((input[23 + inpos] & 1048575) << (28 - 20));
    output[27 + outpos] = (input[23 + inpos] >> 20) | ((input[24 + inpos] & 65535) << (28 - 16));
    output[28 + outpos] = (input[24 + inpos] >> 16) | ((input[25 + inpos] & 4095) << (28 - 12));
    output[29 + outpos] = (input[25 + inpos] >> 12) | ((input[26 + inpos] & 255) << (28 - 8));
    output[30 + outpos] = (input[26 + inpos] >> 8) | ((input[27 + inpos] & 15) << (28 - 4));
    output[31 + outpos] = input[27 + inpos] >> 4;
}

fn fast_unpack29(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 536870911;
    output[1 + outpos] = (input[0 + inpos] >> 29) | ((input[1 + inpos] & 67108863) << (29 - 26));
    output[2 + outpos] = (input[1 + inpos] >> 26) | ((input[2 + inpos] & 8388607) << (29 - 23));
    output[3 + outpos] = (input[2 + inpos] >> 23) | ((input[3 + inpos] & 1048575) << (29 - 20));
    output[4 + outpos] = (input[3 + inpos] >> 20) | ((input[4 + inpos] & 131071) << (29 - 17));
    output[5 + outpos] = (input[4 + inpos] >> 17) | ((input[5 + inpos] & 16383) << (29 - 14));
    output[6 + outpos] = (input[5 + inpos] >> 14) | ((input[6 + inpos] & 2047) << (29 - 11));
    output[7 + outpos] = (input[6 + inpos] >> 11) | ((input[7 + inpos] & 255) << (29 - 8));
    output[8 + outpos] = (input[7 + inpos] >> 8) | ((input[8 + inpos] & 31) << (29 - 5));
    output[9 + outpos] = (input[8 + inpos] >> 5) | ((input[9 + inpos] & 3) << (29 - 2));
    output[10 + outpos] = (input[9 + inpos] >> 2) & 536870911;
    output[11 + outpos] = (input[9 + inpos] >> 31) | ((input[10 + inpos] & 268435455) << (29 - 28));
    output[12 + outpos] = (input[10 + inpos] >> 28) | ((input[11 + inpos] & 33554431) << (29 - 25));
    output[13 + outpos] = (input[11 + inpos] >> 25) | ((input[12 + inpos] & 4194303) << (29 - 22));
    output[14 + outpos] = (input[12 + inpos] >> 22) | ((input[13 + inpos] & 524287) << (29 - 19));
    output[15 + outpos] = (input[13 + inpos] >> 19) | ((input[14 + inpos] & 65535) << (29 - 16));
    output[16 + outpos] = (input[14 + inpos] >> 16) | ((input[15 + inpos] & 8191) << (29 - 13));
    output[17 + outpos] = (input[15 + inpos] >> 13) | ((input[16 + inpos] & 1023) << (29 - 10));
    output[18 + outpos] = (input[16 + inpos] >> 10) | ((input[17 + inpos] & 127) << (29 - 7));
    output[19 + outpos] = (input[17 + inpos] >> 7) | ((input[18 + inpos] & 15) << (29 - 4));
    output[20 + outpos] = (input[18 + inpos] >> 4) | ((input[19 + inpos] & 1) << (29 - 1));
    output[21 + outpos] = (input[19 + inpos] >> 1) & 536870911;
    output[22 + outpos] =
        (input[19 + inpos] >> 30) | ((input[20 + inpos] & 134217727) << (29 - 27));
    output[23 + outpos] = (input[20 + inpos] >> 27) | ((input[21 + inpos] & 16777215) << (29 - 24));
    output[24 + outpos] = (input[21 + inpos] >> 24) | ((input[22 + inpos] & 2097151) << (29 - 21));
    output[25 + outpos] = (input[22 + inpos] >> 21) | ((input[23 + inpos] & 262143) << (29 - 18));
    output[26 + outpos] = (input[23 + inpos] >> 18) | ((input[24 + inpos] & 32767) << (29 - 15));
    output[27 + outpos] = (input[24 + inpos] >> 15) | ((input[25 + inpos] & 4095) << (29 - 12));
    output[28 + outpos] = (input[25 + inpos] >> 12) | ((input[26 + inpos] & 511) << (29 - 9));
    output[29 + outpos] = (input[26 + inpos] >> 9) | ((input[27 + inpos] & 63) << (29 - 6));
    output[30 + outpos] = (input[27 + inpos] >> 6) | ((input[28 + inpos] & 7) << (29 - 3));
    output[31 + outpos] = input[28 + inpos] >> 3;
}

fn fast_unpack30(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 1073741823;
    output[1 + outpos] = (input[0 + inpos] >> 30) | ((input[1 + inpos] & 268435455) << (30 - 28));
    output[2 + outpos] = (input[1 + inpos] >> 28) | ((input[2 + inpos] & 67108863) << (30 - 26));
    output[3 + outpos] = (input[2 + inpos] >> 26) | ((input[3 + inpos] & 16777215) << (30 - 24));
    output[4 + outpos] = (input[3 + inpos] >> 24) | ((input[4 + inpos] & 4194303) << (30 - 22));
    output[5 + outpos] = (input[4 + inpos] >> 22) | ((input[5 + inpos] & 1048575) << (30 - 20));
    output[6 + outpos] = (input[5 + inpos] >> 20) | ((input[6 + inpos] & 262143) << (30 - 18));
    output[7 + outpos] = (input[6 + inpos] >> 18) | ((input[7 + inpos] & 65535) << (30 - 16));
    output[8 + outpos] = (input[7 + inpos] >> 16) | ((input[8 + inpos] & 16383) << (30 - 14));
    output[9 + outpos] = (input[8 + inpos] >> 14) | ((input[9 + inpos] & 4095) << (30 - 12));
    output[10 + outpos] = (input[9 + inpos] >> 12) | ((input[10 + inpos] & 1023) << (30 - 10));
    output[11 + outpos] = (input[10 + inpos] >> 10) | ((input[11 + inpos] & 255) << (30 - 8));
    output[12 + outpos] = (input[11 + inpos] >> 8) | ((input[12 + inpos] & 63) << (30 - 6));
    output[13 + outpos] = (input[12 + inpos] >> 6) | ((input[13 + inpos] & 15) << (30 - 4));
    output[14 + outpos] = (input[13 + inpos] >> 4) | ((input[14 + inpos] & 3) << (30 - 2));
    output[15 + outpos] = input[14 + inpos] >> 2;
    output[16 + outpos] = (input[15 + inpos] >> 0) & 1073741823;
    output[17 + outpos] =
        (input[15 + inpos] >> 30) | ((input[16 + inpos] & 268435455) << (30 - 28));
    output[18 + outpos] = (input[16 + inpos] >> 28) | ((input[17 + inpos] & 67108863) << (30 - 26));
    output[19 + outpos] = (input[17 + inpos] >> 26) | ((input[18 + inpos] & 16777215) << (30 - 24));
    output[20 + outpos] = (input[18 + inpos] >> 24) | ((input[19 + inpos] & 4194303) << (30 - 22));
    output[21 + outpos] = (input[19 + inpos] >> 22) | ((input[20 + inpos] & 1048575) << (30 - 20));
    output[22 + outpos] = (input[20 + inpos] >> 20) | ((input[21 + inpos] & 262143) << (30 - 18));
    output[23 + outpos] = (input[21 + inpos] >> 18) | ((input[22 + inpos] & 65535) << (30 - 16));
    output[24 + outpos] = (input[22 + inpos] >> 16) | ((input[23 + inpos] & 16383) << (30 - 14));
    output[25 + outpos] = (input[23 + inpos] >> 14) | ((input[24 + inpos] & 4095) << (30 - 12));
    output[26 + outpos] = (input[24 + inpos] >> 12) | ((input[25 + inpos] & 1023) << (30 - 10));
    output[27 + outpos] = (input[25 + inpos] >> 10) | ((input[26 + inpos] & 255) << (30 - 8));
    output[28 + outpos] = (input[26 + inpos] >> 8) | ((input[27 + inpos] & 63) << (30 - 6));
    output[29 + outpos] = (input[27 + inpos] >> 6) | ((input[28 + inpos] & 15) << (30 - 4));
    output[30 + outpos] = (input[28 + inpos] >> 4) | ((input[29 + inpos] & 3) << (30 - 2));
    output[31 + outpos] = input[29 + inpos] >> 2;
}

fn fast_unpack31(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[0 + outpos] = (input[0 + inpos] >> 0) & 2147483647;
    output[1 + outpos] = (input[0 + inpos] >> 31) | ((input[1 + inpos] & 1073741823) << (31 - 30));
    output[2 + outpos] = (input[1 + inpos] >> 30) | ((input[2 + inpos] & 536870911) << (31 - 29));
    output[3 + outpos] = (input[2 + inpos] >> 29) | ((input[3 + inpos] & 268435455) << (31 - 28));
    output[4 + outpos] = (input[3 + inpos] >> 28) | ((input[4 + inpos] & 134217727) << (31 - 27));
    output[5 + outpos] = (input[4 + inpos] >> 27) | ((input[5 + inpos] & 67108863) << (31 - 26));
    output[6 + outpos] = (input[5 + inpos] >> 26) | ((input[6 + inpos] & 33554431) << (31 - 25));
    output[7 + outpos] = (input[6 + inpos] >> 25) | ((input[7 + inpos] & 16777215) << (31 - 24));
    output[8 + outpos] = (input[7 + inpos] >> 24) | ((input[8 + inpos] & 8388607) << (31 - 23));
    output[9 + outpos] = (input[8 + inpos] >> 23) | ((input[9 + inpos] & 4194303) << (31 - 22));
    output[10 + outpos] = (input[9 + inpos] >> 22) | ((input[10 + inpos] & 2097151) << (31 - 21));
    output[11 + outpos] = (input[10 + inpos] >> 21) | ((input[11 + inpos] & 1048575) << (31 - 20));
    output[12 + outpos] = (input[11 + inpos] >> 20) | ((input[12 + inpos] & 524287) << (31 - 19));
    output[13 + outpos] = (input[12 + inpos] >> 19) | ((input[13 + inpos] & 262143) << (31 - 18));
    output[14 + outpos] = (input[13 + inpos] >> 18) | ((input[14 + inpos] & 131071) << (31 - 17));
    output[15 + outpos] = (input[14 + inpos] >> 17) | ((input[15 + inpos] & 65535) << (31 - 16));
    output[16 + outpos] = (input[15 + inpos] >> 16) | ((input[16 + inpos] & 32767) << (31 - 15));
    output[17 + outpos] = (input[16 + inpos] >> 15) | ((input[17 + inpos] & 16383) << (31 - 14));
    output[18 + outpos] = (input[17 + inpos] >> 14) | ((input[18 + inpos] & 8191) << (31 - 13));
    output[19 + outpos] = (input[18 + inpos] >> 13) | ((input[19 + inpos] & 4095) << (31 - 12));
    output[20 + outpos] = (input[19 + inpos] >> 12) | ((input[20 + inpos] & 2047) << (31 - 11));
    output[21 + outpos] = (input[20 + inpos] >> 11) | ((input[21 + inpos] & 1023) << (31 - 10));
    output[22 + outpos] = (input[21 + inpos] >> 10) | ((input[22 + inpos] & 511) << (31 - 9));
    output[23 + outpos] = (input[22 + inpos] >> 9) | ((input[23 + inpos] & 255) << (31 - 8));
    output[24 + outpos] = (input[23 + inpos] >> 8) | ((input[24 + inpos] & 127) << (31 - 7));
    output[25 + outpos] = (input[24 + inpos] >> 7) | ((input[25 + inpos] & 63) << (31 - 6));
    output[26 + outpos] = (input[25 + inpos] >> 6) | ((input[26 + inpos] & 31) << (31 - 5));
    output[27 + outpos] = (input[26 + inpos] >> 5) | ((input[27 + inpos] & 15) << (31 - 4));
    output[28 + outpos] = (input[27 + inpos] >> 4) | ((input[28 + inpos] & 7) << (31 - 3));
    output[29 + outpos] = (input[28 + inpos] >> 3) | ((input[29 + inpos] & 3) << (31 - 2));
    output[30 + outpos] = (input[29 + inpos] >> 2) | ((input[30 + inpos] & 1) << (31 - 1));
    output[31 + outpos] = input[30 + inpos] >> 1;
}

fn fast_unpack32(input: &[u32], inpos: usize, output: &mut [u32], outpos: usize) {
    output[outpos..outpos + 32].copy_from_slice(&input[inpos..inpos + 32]);
}
