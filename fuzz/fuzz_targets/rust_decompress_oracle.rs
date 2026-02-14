#![no_main]

use std::num::NonZeroU32;

use fastpfor::cpp::{FastPFor128Codec, FastPFor256Codec};
use fastpfor::rust::{Codec, FastPFOR, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE};
use fastpfor::CodecToSlice;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: FuzzInput| {
    let input = data.data;

    // TODO: Behaviour differs
    if input.is_empty() {
        return;
    }

    // TODO: To make the decder not crash ->  Skip inputs smaller than block size
    let block_size = NonZeroU32::from(data.codec);
    let bs = block_size.get() as usize;
    if input.len() < bs {
        return;
    }

    // TODO: To make the decder not crash -> Truncate to block size multiple
    let last_block_size_multiple = input.len() / bs * bs;
    let input = &input[..last_block_size_multiple];

    // First, compress with C++ implementation to get valid compressed data
    let mut cpp_compressed = vec![0u32; input.len() * 2 + 1024];
    let compressed_data = match data.codec {
        FuzzCodec::FastPFOR256 => {
            let mut cpp_codec = FastPFor256Codec::new();
            cpp_codec
                .compress_to_slice(input, &mut cpp_compressed)
                .expect("C++ compression failed")
        }
        FuzzCodec::FastPFOR128 => {
            let mut cpp_codec = FastPFor128Codec::new();
            cpp_codec
                .compress_to_slice(input, &mut cpp_compressed)
                .expect("C++ compression failed")
        }
    };

    // Now decompress with rust
    let mut rust_decompressed = vec![0u32; input.len()];
    let fastpfor = FastPFOR::new(DEFAULT_PAGE_SIZE, block_size);
    let mut rust_codec = Codec::from(fastpfor);
    let rust_result = rust_codec
        .decompress_to_slice(compressed_data, &mut rust_decompressed)
        .expect("Rust decompression failed");

    // Compare decompressed outputs
    assert_eq!(
        rust_result.len(),
        input.len(),
        "Decompressed length mismatch: Rust={}, C++={}",
        rust_result.len(),
        cpp_result.len()
    );

    for (i, (&rust_val, &cpp_val)) in rust_result.iter().zip(input.iter()).enumerate() {
        assert_eq!(
            rust_val, cpp_val,
            "Decompressed data mismatch at position {}: Rust={}, C++={}",
            i, rust_val, cpp_val
        );
    }
});

#[derive(arbitrary::Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u32>,
    codec: FuzzCodec,
}

#[derive(arbitrary::Arbitrary, Debug, Clone, Copy, PartialEq, Eq)]
enum FuzzCodec {
    FastPFOR256,
    FastPFOR128,
}

impl From<FuzzCodec> for NonZeroU32 {
    fn from(codec: FuzzCodec) -> Self {
        match codec {
            FuzzCodec::FastPFOR256 => BLOCK_SIZE_256,
            FuzzCodec::FastPFOR128 => BLOCK_SIZE_128,
        }
    }
}
