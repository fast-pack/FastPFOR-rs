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

#[cfg(test)]
mod tests {
    use rand::RngExt as _;

    use super::fast_pack;
    use crate::rust::integer_compression::bitunpacking::fast_unpack;

    #[test]
    fn pack_unpack_roundtrip() {
        let n = 32;
        let times = 1000;
        let mut r = rand::rng();
        let mut data = vec![0u32; n];
        let mut compressed = vec![0u32; n];
        let mut uncompressed = vec![0u32; n];

        for bit in 0..31u8 {
            for _ in 0..times {
                for value in &mut data {
                    *value = r.random_range(0..(1 << bit));
                }
                fast_pack(&data, 0, &mut compressed, 0, bit);
                fast_unpack(&compressed, 0, &mut uncompressed, 0, bit);
                assert_eq!(uncompressed, data, "Mismatch for bit {bit}");
            }
        }
    }

    #[test]
    fn pack_unpack_with_masking() {
        const N: usize = 32;
        const TIMES: usize = 1000;
        let mut rng = rand::rng();
        let mut data = vec![0u32; N];
        let mut compressed = vec![0u32; N];
        let mut uncompressed = vec![0u32; N];

        for bit in 0..31u8 {
            for _ in 0..TIMES {
                for value in &mut data {
                    *value = rng.random();
                }
                fast_pack(&data, 0, &mut compressed, 0, bit);
                fast_unpack(&compressed, 0, &mut uncompressed, 0, bit);
                for value in &mut data {
                    *value &= (1 << bit) - 1;
                }
                assert_eq!(
                    data, uncompressed,
                    "Data does not match uncompressed output"
                );
            }
        }
    }
}
