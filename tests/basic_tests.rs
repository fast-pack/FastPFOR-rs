#![cfg(feature = "rust")]
#![expect(clippy::needless_range_loop)]

use std::io::Cursor;
use std::num::NonZeroU32;

use fastpfor::rust::{
    fast_pack, fast_unpack, Composition, FastPFOR, Integer, VariableByte, BLOCK_SIZE_128,
    DEFAULT_PAGE_SIZE,
};
use rand::RngExt as _;

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
            output_offset.set_position(u64::from(x));

            codec
                .compress(
                    &input,
                    input.len() as u32,
                    &mut input_offset,
                    &mut output,
                    &mut output_offset,
                )
                .unwrap_or_else(|e| {
                    panic!("Failed to compress with {}: {e:?}", codec.name());
                });

            let len = output_offset.position() as u32 - x;
            output_offset.set_position(u64::from(x));

            codec
                .uncompress(
                    &output,
                    len,
                    &mut output_offset,
                    &mut answer,
                    &mut Cursor::new(0),
                )
                .unwrap_or_else(|e| {
                    panic!("Failed to uncompress with {}: {e:?}", codec.name());
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
                    panic!("Failed to compress with {}: {e:?}", codec.name());
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
                    panic!("Failed to uncompress with {}: {e:?}", codec.name());
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
                    panic!("Failed to compress with {}: {e:?}", codec.name());
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
                    panic!("Failed to uncompress with {}: {e:?}", codec.name());
                });
            for k in 1..l {
                if answer[k] != data[k] {
                    assert_eq!(answer[k], data[k]);
                }
            }
        }
    }
}

#[test]
fn verity_bitpacking() {
    let n = 32;
    let times = 1000;
    let mut r = rand::rng();
    let mut data = vec![0; n];
    let mut compressed = vec![0; n];
    let mut uncompressed = vec![0; n];

    for bit in 0..31 {
        for _ in 0..times {
            for k in 0..n {
                data[k] = r.random_range(0..(1 << bit));
            }

            fast_pack(&data, 0, &mut compressed, 0, bit);
            fast_unpack(&compressed, 0, &mut uncompressed, 0, bit);

            assert_eq!(uncompressed, data, "Mismatch for bit {bit}");
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
    let mut rng = rand::rng();

    let mut data = vec![0u32; N];
    let mut compressed = vec![0u32; N];
    let mut uncompressed = vec![0u32; N];

    for bit in 0..31 {
        for _ in 0..TIMES {
            for value in &mut data {
                *value = rng.random();
            }

            fast_pack(&data, 0, &mut compressed, 0, bit);
            fast_unpack(&compressed, 0, &mut uncompressed, 0, bit);

            mask_array(&mut data, (1 << bit) - 1);

            assert_eq!(
                data, uncompressed,
                "Data does not match uncompressed output"
            );
        }
    }
}

fn test_spurious<C: Integer<u32>>(codec: &mut C) {
    let x = vec![0u32; 1024];
    let mut y: Vec<u32> = vec![0; 1024];
    let mut i0 = Cursor::new(0);
    let mut i1 = Cursor::new(0);

    for inlength in 0..32 {
        codec
            .compress(&x, inlength, &mut i0, &mut y, &mut i1)
            .unwrap_or_else(|e| panic!("Compression failed: {e:?}"));

        assert_eq!(
            0,
            i1.position(),
            "Expected output cursor position to be 0, but got {}",
            i1.position()
        );
    }
}

#[test]
fn spurious_out_test() {
    let mut codec1 = FastPFOR::default();
    test_spurious(&mut codec1);

    let mut codec2 = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
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
        .unwrap_or_else(|e| panic!("Compression failed: {e:?}"));
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
        .unwrap_or_else(|e| panic!("Decompression failed: {e:?}"));
    assert_eq!(
        outpos.position(),
        0,
        "Expected output cursor position to be 0 after decompression, but got {}",
        outpos.position()
    );
}

#[test]
fn zero_in_zero_out_test() {
    let mut codec1 = FastPFOR::default();
    test_zero_in_zero_out(&mut codec1);

    let mut codec2 = FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128);
    test_zero_in_zero_out(&mut codec2);

    let mut codec3 = VariableByte;
    test_zero_in_zero_out(&mut codec3);

    let mut codec4 = Composition::new(FastPFOR::default(), VariableByte);
    test_zero_in_zero_out(&mut codec4);

    let mut codec5 = Composition::new(
        FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128),
        VariableByte,
    );
    test_zero_in_zero_out(&mut codec5);
}

#[test]
fn test_increasing_sequence() {
    let n = 256;
    let data: Vec<u32> = (0..n).collect();
    let codecs = vec![
        FastPFOR::default(),
        FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128),
    ];
    for mut codec in codecs {
        // Compress the data
        let mut output_compress = vec![0; data.len() * 4];
        codec
            .compress(
                &data,
                data.len() as u32,
                &mut Cursor::new(0),
                &mut output_compress,
                &mut Cursor::new(0),
            )
            .unwrap_or_else(|e| {
                panic!("Failed to compress: {e:?}");
            });

        // Decompress the data
        let mut decompressed = vec![0; data.len() + 1024];
        codec
            .uncompress(
                &output_compress,
                n,
                &mut Cursor::new(0),
                &mut decompressed,
                &mut Cursor::new(0),
            )
            .unwrap_or_else(|e| {
                panic!("Failed to uncompress: {e:?}");
            });

        // Verify decompressed data matches original
        for (i, &value) in data.iter().enumerate() {
            assert_eq!(value, decompressed[i], "Mismatch at index {i}");
        }
    }
}

#[test]
fn test_random_numbers() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let n = 65536;
    let mut rng = StdRng::seed_from_u64(123456);
    let data: Vec<u32> = (0..n).map(|_| rng.random()).collect(); // Generate random data
    let codecs = vec![
        FastPFOR::default(),
        FastPFOR::new(DEFAULT_PAGE_SIZE, BLOCK_SIZE_128),
    ];
    for mut codec in codecs {
        // Compress the data
        let mut output_compress = vec![0; data.len() * 4];
        codec
            .compress(
                &data,
                data.len() as u32,
                &mut Cursor::new(0),
                &mut output_compress,
                &mut Cursor::new(0),
            )
            .unwrap_or_else(|e| {
                panic!("Failed to compress: {e:?}");
            });

        // Decompress the data
        let mut decompressed = vec![0; data.len() + 1024];
        codec
            .uncompress(
                &output_compress,
                n as u32,
                &mut Cursor::new(0),
                &mut decompressed,
                &mut Cursor::new(0),
            )
            .unwrap_or_else(|e| {
                panic!("Failed to uncompress: {e:?}");
            });

        // Verify decompressed data matches original
        for (i, &value) in data.iter().enumerate() {
            assert_eq!(value, decompressed[i], "Mismatch at index {i}");
        }
    }
}

#[test]
fn test_fastpfor_headless_compress_unfit_pagesize() {
    // The input size is a multiple of 128 but does not fit the page size
    let test_input_size = BLOCK_SIZE_128.checked_add(512).unwrap();
    let page_size = NonZeroU32::new(512).unwrap();

    let input: Vec<u32> = (0..test_input_size.get()).collect();
    let mut output: Vec<u32> = vec![0; input.len()];
    let mut decoded: Vec<u32> = vec![0; input.len()];
    let mut input_offset = Cursor::new(0u32);
    let mut output_offset = Cursor::new(0u32);

    let mut codec = FastPFOR::new(page_size, BLOCK_SIZE_128);
    codec
        .compress(
            &input,
            input.len() as u32,
            &mut input_offset,
            &mut output,
            &mut output_offset,
        )
        .expect("compression failed");

    let compressed_len = output_offset.position() as usize;
    input_offset.set_position(0);

    codec
        .uncompress(
            &output,
            compressed_len as u32,
            &mut input_offset,
            &mut decoded,
            &mut Cursor::new(0u32),
        )
        .expect("decompression failed");

    assert_eq!(input, decoded, "Input and decompressed data do not match");
}

#[test]
fn test_exception_value_vector_resizes() {
    let page_size = NonZeroU32::new(512).unwrap();
    let test_input_size = page_size.get() * 2;

    // every even index value is large which will trigger exception buffer to be resize
    let input: Vec<u32> = (0..test_input_size)
        .map(|i| if i % 2 == 0 { 1 << 30 } else { 3 })
        .collect();

    let mut output: Vec<u32> = vec![0; input.len() * 4];
    let mut decoded: Vec<u32> = vec![0; input.len()];
    let mut input_offset = Cursor::new(0u32);
    let mut output_offset = Cursor::new(0u32);

    let mut codec = FastPFOR::new(page_size, BLOCK_SIZE_128);
    codec
        .compress(
            &input,
            input.len() as u32,
            &mut input_offset,
            &mut output,
            &mut output_offset,
        )
        .expect("compression failed");

    let compressed_len = output_offset.position() as usize;
    input_offset.set_position(0);

    codec
        .uncompress(
            &output,
            compressed_len as u32,
            &mut input_offset,
            &mut decoded,
            &mut Cursor::new(0u32),
        )
        .expect("decompression failed");

    assert_eq!(input, decoded, "Input and decompressed data do not match");
}
