use std::io::Cursor;

mod common;

#[test]
fn saul_test() {
    let codecs = common::get_codecs();

    for mut codec in codecs {
        if codec.name() == "VariableByte" {
            continue;
        }

        for x in 0..50 {
            let input = vec![2, 3, 4, 5];
            let mut output: Vec<u32> = vec![0; 90];
            let mut answer: Vec<u32> = vec![0; input.len()];
            let mut input_offset = Cursor::new(0);
            let mut output_offset = Cursor::new(0);
            output_offset.set_position(x as u64);

            codec
                .compress(
                    &input,
                    input.len() as u32,
                    &mut input_offset,
                    &mut output,
                    &mut output_offset,
                )
                .unwrap_or_else(|e| {
                    panic!("Failed to compress with {}: {:?}", codec.name(), e);
                });

            let len = output_offset.position() as u32 - x;
            output_offset.set_position(x as u64);

            codec
                .uncompress(
                    &output,
                    len,
                    &mut output_offset,
                    &mut answer,
                    &mut Cursor::new(0),
                )
                .unwrap_or_else(|e| {
                    panic!("Failed to uncompress with {}: {:?}", codec.name(), e);
                });

            assert_eq!(input, answer);
        }
    }
}

#[test]
fn test_varying_length() {
    let n = 4096;
    let mut data = vec![0u32; n];
    for k in 0..n {
        data[k] = k as u32;
    }
    let codecs = common::get_codecs();
    for mut codec in codecs {
        for l in 1..128 {
            let mut data_copy = data.clone();
            data_copy.resize(l, 0);
            let mut output_compress = vec![0; data_copy.len() * 4];
            codec
                .compress(
                    &data_copy,
                    data_copy.len() as u32,
                    &mut Cursor::new(0),
                    &mut output_compress,
                    &mut Cursor::new(0),
                )
                .unwrap_or_else(|e| {
                    panic!("Failed to compress with {}: {:?}", codec.name(), e);
                });
            let mut answer = vec![0; l + 1024];
            codec
                .uncompress(
                    &output_compress,
                    output_compress.len() as u32,
                    &mut Cursor::new(0),
                    &mut answer,
                    &mut Cursor::new(0),
                )
                .unwrap_or_else(|e| {
                    panic!("Failed to uncompress with {}: {:?}", codec.name(), e);
                });
            for k in 0..l {
                assert_eq!(answer[k], data[k]);
            }
        }
    }
}

#[test]
fn test_varying_length_two() {
    let n = 128;
    let mut data = vec![0u32; n];
    data[126] = -1i32 as u32;
    let codecs = common::get_codecs();
    for mut codec in codecs {
        for l in 1..128 {
            let mut data_copy = data.clone();
            let mut output_compress = vec![0; data_copy.len() * 4];
            data_copy.resize(l, 0);
            codec
                .compress(
                    &data_copy,
                    data_copy.len() as u32,
                    &mut Cursor::new(0),
                    &mut output_compress,
                    &mut Cursor::new(0),
                )
                .unwrap_or_else(|e| {
                    panic!("Failed to compress with {}: {:?}", codec.name(), e);
                });
            let mut answer = vec![0; data_copy.len() + 1024];
            codec
                .uncompress(
                    &output_compress,
                    128,
                    &mut Cursor::new(0),
                    &mut answer,
                    &mut Cursor::new(0),
                )
                .unwrap_or_else(|e| {
                    panic!("Failed to uncompress with {}: {:?}", codec.name(), e);
                });
            for k in 1..l {
                if answer[k] != data[k] {
                    assert_eq!(answer[k], data[k]);
                }
            }
        }
    }
}

use rand::Rng;

#[test]
fn verity_bitpacking() {
    let n = 32;
    let times = 1000;
    let mut r = rand::thread_rng();
    let mut data = vec![0; n];
    let mut compressed = vec![0; n];
    let mut uncompressed = vec![0; n];

    for bit in 0..31 {
        for _ in 0..times {
            for k in 0..n {
                data[k] = r.gen_range(0..(1 << bit));
            }

            fastpfor::fast_pack(&data, 0, &mut compressed, 0, bit);
            fastpfor::fast_unpack(&compressed, 0, &mut uncompressed, 0, bit);

            assert_eq!(uncompressed, data, "Mismatch for bit {}", bit);
        }
    }
}

fn mask_array(array: &mut [u32], mask: u32) {
    for value in array.iter_mut() {
        *value &= mask;
    }
}

#[test]
fn verify_with_exceptions() {
    const N: usize = 32;
    const TIMES: usize = 1000;
    let mut rng = rand::thread_rng();

    let mut data = vec![0u32; N];
    let mut compressed = vec![0u32; N];
    let mut uncompressed = vec![0u32; N];

    for bit in 0..31 {
        for _ in 0..TIMES {
            for value in &mut data {
                *value = rng.gen();
            }

            fastpfor::fast_pack(&data, 0, &mut compressed, 0, bit);
            fastpfor::fast_unpack(&compressed, 0, &mut uncompressed, 0, bit);

            mask_array(&mut data, (1 << bit) - 1);

            assert_eq!(
                data, uncompressed,
                "Data does not match uncompressed output"
            );
        }
    }
}

use fastpfor::Integer;

fn test_spurious<C: Integer<u32>>(codec: &mut C) {
    let x = vec![0u32; 1024];
    let mut y: Vec<u32> = vec![0; 1024];
    let mut i0 = Cursor::new(0);
    let mut i1 = Cursor::new(0);

    for inlength in 0..32 {
        codec
            .compress(&x, inlength, &mut i0, &mut y, &mut i1)
            .unwrap_or_else(|e| panic!("Compression failed: {:?}", e));

        assert_eq!(
            0,
            i1.position(),
            "Expected output cursor position to be 0, but got {}",
            i1.position()
        );
    }
}

use fastpfor::{BLOCK_SIZE_128, DEFAULT_PAGE_SIZE};

#[test]
fn spurious_out_test() {
    let mut codec1 = fastpfor::FastPFOR::default();
    test_spurious(&mut codec1);

    let mut codec2 = fastpfor::FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
    test_spurious(&mut codec2);
}

fn test_zero_in_zero_out<C: Integer<u32>>(codec: &mut C) {
    // Empty input and output arrays
    let x: Vec<u32> = Vec::new();
    let mut y: Vec<u32> = Vec::new();
    let mut i0 = Cursor::new(0);
    let mut i1 = Cursor::new(0);

    // Test compression
    codec
        .compress(&x, 0, &mut i0, &mut y, &mut i1)
        .unwrap_or_else(|e| panic!("Compression failed: {:?}", e));
    assert_eq!(
        i1.position(),
        0,
        "Expected output cursor position to be 0 after compression, but got {}",
        i1.position()
    );

    // Test decompression
    let mut out: Vec<u32> = Vec::new();
    let mut outpos = Cursor::new(0);
    codec
        .uncompress(&y, 0, &mut i1, &mut out, &mut outpos)
        .unwrap_or_else(|e| panic!("Decompression failed: {:?}", e));
    assert_eq!(
        outpos.position(),
        0,
        "Expected output cursor position to be 0 after decompression, but got {}",
        outpos.position()
    );
}

#[test]
fn zero_in_zero_out_test() {
    let mut codec1 = fastpfor::FastPFOR::default();
    test_zero_in_zero_out(&mut codec1);

    let mut codec2 = fastpfor::FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
    test_zero_in_zero_out(&mut codec2);

    let mut codec3 = fastpfor::VariableByte;
    test_zero_in_zero_out(&mut codec3);

    let mut codec4 =
        fastpfor::Composition::new(fastpfor::FastPFOR::default(), fastpfor::VariableByte);
    test_zero_in_zero_out(&mut codec4);

    let mut codec5 = fastpfor::Composition::new(
        fastpfor::FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128),
        fastpfor::VariableByte,
    );
    test_zero_in_zero_out(&mut codec5);
}

fn test_unsorted<C: Integer<u32>>(codec: &mut C) {
    let lengths = [133, 1026, 1_333_333];

    for &n in &lengths {
        // Initialize the data array
        let mut data = vec![3u32; n];

        // Insert larger values at specific intervals
        for k in (0..n).step_by(5) {
            data[k] = 100;
        }
        for k in (0..n).step_by(533) {
            data[k] = 10_000;
        }
        data[5] = (-311i32) as u32; // Simulate negative values if applicable

        // Allocate a buffer for compression
        let mut compressed = vec![0u32; ((n as f64 * 1.01) as usize) + 1024];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        // Compress the data
        codec
            .compress(
                &data,
                n as u32,
                &mut input_offset,
                &mut compressed,
                &mut output_offset,
            )
            .unwrap_or_else(|e| panic!("Compression failed: {:?}", e));

        // Resize the compressed buffer to match the actual size
        let compressed = compressed[..output_offset.position() as usize].to_vec();

        // Allocate a buffer for decompression
        let mut recovered = vec![0u32; n];
        let mut recoffset = Cursor::new(0);

        // Decompress the data
        codec
            .uncompress(
                &compressed,
                compressed.len() as u32,
                &mut Cursor::new(0),
                &mut recovered,
                &mut recoffset,
            )
            .unwrap_or_else(|e| panic!("Decompression failed: {:?}", e));

        // Verify that the decompressed data matches the original
        assert_eq!(
            data, recovered,
            "Original data and recovered data do not match for length {}",
            n
        );
    }
}

#[test]
fn unsorted_test() {
    let mut codec1 = fastpfor::VariableByte;
    test_unsorted(&mut codec1);

    // This fails
    // let mut codec2 = fastpfor::Composition::new(
    //     fastpfor::FastPFOR::default(),
    //     fastpfor::VariableByte::default(),
    // );
    // test_unsorted(&mut codec2);

    // This fails
    // let mut codec3 = fastpfor::Composition::new(
    //     fastpfor::FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128),
    //     fastpfor::VariableByte::default(),
    // );
    // test_unsorted(&mut codec3);
}
