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

    // TODO: To make the encoder not crash ->  Skip inputs smaller than block size
    let block_size = NonZeroU32::from(data.codec);
    let bs = block_size.get() as usize;
    if input.len() < bs {
        return;
    }

    // TODO: To make the encoder not crash ->  Truncate to block size multiple
    let last_block_size_multiple = input.len() / bs * bs;
    let input = &input[..last_block_size_multiple];

    // Allocate output buffers with generous size
    let mut rust_compressed = vec![0u32; input.len() * 2 + 1024];
    let mut cpp_compressed = vec![0u32; input.len() * 2 + 1024];

    // Compress with Rust implementation using Codec wrapper
    let fastpfor = FastPFOR::new(DEFAULT_PAGE_SIZE, block_size);
    let mut rust_codec = Codec::from(fastpfor);
    let rust_result = rust_codec
        .compress_to_slice(input, &mut rust_compressed)
        .expect("Rust compression failed");

    // Compress with C++ implementation
    let cpp_result = match data.codec {
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

    // Compare compressed outputs
    assert_eq!(
        rust_result.len(),
        cpp_result.len(),
        "Compressed length mismatch: Rust={}, C++={}",
        rust_result.len(),
        cpp_result.len()
    );

    for (i, (&rust_val, &cpp_val)) in rust_result.iter().zip(cpp_result.iter()).enumerate() {
        assert_eq!(
            rust_val, cpp_val,
            "Compressed data mismatch at position {}: Rust={}, C++={}",
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
