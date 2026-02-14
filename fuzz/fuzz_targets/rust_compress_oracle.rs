#![no_main]

use fastpfor::{CodecToSlice, cpp, rust};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: FuzzInput| {
    let input = data.data;

    // TODO: Behaviour differs
    if input.is_empty() {
        return;
    }

    // TODO: Behaviour differs
    if data.codec == FuzzCodec::VariableByte {
        return;
    }

    // TODO: To make the encoder not crash ->  Skip inputs smaller than block size
    let block_size = match data.codec {
        FuzzCodec::FastPFOR256 => 256,
        FuzzCodec::FastPFOR128 => 128,
        FuzzCodec::VariableByte => 1,
        FuzzCodec::JustCopy => 1,
    };
    if input.len() < block_size {
        return;
    }

    // TODO: To make the encoder not crash ->  Truncate to block size multiple
    let last_block_size_multiple = input.len() / block_size * block_size;
    let input = &input[..last_block_size_multiple];

    // Allocate output buffers with generous size
    let mut rust_compressed = vec![0u32; input.len() * 2 + 1024];
    let mut cpp_compressed = vec![0u32; input.len() * 2 + 1024];

    // Compress with Rust implementation using Codec wrapper
    let mut rust_codec = rust::Codec::from(data.codec);
    let rust_result = rust_codec
        .compress_to_slice(input, &mut rust_compressed)
        .expect("Rust compression failed");

    // Compress with C++ implementation
    let cpp_result = match data.codec {
        FuzzCodec::FastPFOR256 => {
            let mut cpp_codec = cpp::FastPFor256Codec::new();
            cpp_codec
                .compress_to_slice(input, &mut cpp_compressed)
                .expect("C++ compression failed")
        }
        FuzzCodec::FastPFOR128 => {
            let mut cpp_codec = cpp::FastPFor128Codec::new();
            cpp_codec
                .compress_to_slice(input, &mut cpp_compressed)
                .expect("C++ compression failed")
        }
        FuzzCodec::VariableByte => {
            let mut cpp_codec = cpp::VByteCodec::new();
            cpp_codec
                .compress_to_slice(input, &mut cpp_compressed)
                .expect("C++ compression failed")
        }
        FuzzCodec::JustCopy => {
            let mut cpp_codec = cpp::CopyCodec::new();
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
    VariableByte,
    JustCopy,
}

impl From<FuzzCodec> for rust::Codec {
    fn from(codec: FuzzCodec) -> Self {
        match codec {
            FuzzCodec::FastPFOR256 => rust::Codec::from(rust::FastPFOR::new(
                rust::DEFAULT_PAGE_SIZE,
                rust::BLOCK_SIZE_256,
            )),
            FuzzCodec::FastPFOR128 => rust::Codec::from(rust::FastPFOR::new(
                rust::DEFAULT_PAGE_SIZE,
                rust::BLOCK_SIZE_128,
            )),
            FuzzCodec::VariableByte => rust::Codec::from(rust::VariableByte::new()),
            FuzzCodec::JustCopy => rust::Codec::from(rust::JustCopy::new()),
        }
    }
}
