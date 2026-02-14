#![no_main]

use std::io::Cursor;

use fastpfor::rust::{FastPFOR, Integer, BLOCK_SIZE_128, BLOCK_SIZE_256, DEFAULT_PAGE_SIZE};
use libfuzzer_sys::fuzz_target;
use std::num::NonZeroU32;

fuzz_target!(|data: FuzzInput| {
    let input = data.data;
    let block_size = NonZeroU32::from(data.codec);
    let mut codec = FastPFOR::new(DEFAULT_PAGE_SIZE, block_size);

    // TODO: empty input is encoded as empty, which does not match the CPP version
    if input.is_empty() {
        return;
    }

    // TODO: only multiples of block size seem to be exported from the compress - decompress cycle
    let bs = block_size.get() as usize;
    if input.len() < bs {
        return;
    }
    let last_block_size_multiple = input.len() / bs * bs;
    let input = input
        .into_iter()
        .take(last_block_size_multiple as usize)
        .collect::<Vec<_>>();

    // Allocate output buffer with generous size
    let mut compressed = vec![0u32; input.len() * 2 + 1024];

    // Compress the data
    let mut output_offset = Cursor::new(0);
    codec
        .compress(
            &input,
            input.len() as u32,
            &mut Cursor::new(0),
            &mut compressed,
            &mut output_offset,
        )
        .unwrap();
    let compressed_size = output_offset.position() as u32;
    assert!(compressed_size != 0, "compression should not be empty");

    // Now decompress
    let mut decompressed = vec![0u32; input.len()];
    let mut output_offset = Cursor::new(0);

    codec
        .uncompress(
            &compressed,
            compressed_size,
            &mut Cursor::new(0),
            &mut decompressed,
            &mut output_offset,
        )
        .unwrap();
    let decompressed_length = output_offset.position() as usize;

    // Verify roundtrip
    assert_eq!(decompressed_length, input.len());
    if decompressed_length + input.len() < 200 {
        assert_eq!(
            input,
            decompressed[..decompressed_length],
            "Decompressed mismatch: expected {}, got {decompressed_length}",
            input.len()
        );
    } else {
        for (i, (&original, &decoded)) in input.iter().zip(decompressed.iter()).enumerate() {
            assert_eq!(
                original, decoded,
                "Mismatch at position {}: expected {}, got {}",
                i, original, decoded
            );
        }
    }
});

#[derive(arbitrary::Arbitrary)]
struct FuzzInput {
    data: Vec<u32>,
    codec: FuzzCodec,
}

impl std::fmt::Debug for FuzzInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FuzzInput")
            .field("data_length", &self.data.len())
            .field("codec", &self.codec)
            .finish()
    }
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
