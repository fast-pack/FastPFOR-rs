#[cfg(feature = "rust")]
mod tests {
    use fastpfor::rust::{FastPFOR, Integer, BLOCK_SIZE_128, BLOCK_SIZE_256};
    use std::io::Cursor;

    /// Bug: FastPFOR silently loses data when input length < block_size
    ///
    /// Discovered by fuzzer: Input [0] with block_size 128 compresses to empty output
    ///
    /// Root cause: greatest_multiple(1, 128) = 0, so codec writes header with length=0
    /// and skips compression. On decompression, it correctly reads 0 and returns empty array.
    ///
    /// This is a critical data corruption bug - the codec claims success but loses data.
    #[test]
    fn test_data_loss_single_element_block_128() {
        let input = vec![0u32];
        let mut codec = FastPFOR::new(65536, BLOCK_SIZE_128);

        let mut compressed = vec![0u32; 1024];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        codec
            .compress(
                &input,
                1,
                &mut input_offset,
                &mut compressed,
                &mut output_offset,
            )
            .expect("Compression should succeed");

        let compressed_size = output_offset.position() as u32;

        // Decompress
        let mut decompressed = vec![0u32; 1024];
        let mut decode_input_offset = Cursor::new(0);
        let mut decode_output_offset = Cursor::new(0);

        codec
            .uncompress(
                &compressed,
                compressed_size,
                &mut decode_input_offset,
                &mut decompressed,
                &mut decode_output_offset,
            )
            .expect("Decompression should succeed");

        let decompressed_length = decode_output_offset.position() as usize;

        assert_eq!(
            input.len(),
            decompressed_length,
            "Decompressed length mismatch: expected {}, got {}",
            input.len(),
            decompressed_length
        );

        for (i, (&original, &decoded)) in input.iter().zip(decompressed.iter()).enumerate() {
            assert_eq!(
                original, decoded,
                "Data mismatch at position {}: expected {}, got {}",
                i, original, decoded
            );
        }
    }

    /// Same bug with block_size 256
    #[test]
    fn test_data_loss_single_element_block_256() {
        let input = vec![42u32];
        let mut codec = FastPFOR::new(65536, BLOCK_SIZE_256);

        let mut compressed = vec![0u32; 1024];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        codec
            .compress(
                &input,
                1,
                &mut input_offset,
                &mut compressed,
                &mut output_offset,
            )
            .expect("Compression should succeed");

        let compressed_size = output_offset.position() as u32;

        let mut decompressed = vec![0u32; 1024];
        let mut decode_input_offset = Cursor::new(0);
        let mut decode_output_offset = Cursor::new(0);

        codec
            .uncompress(
                &compressed,
                compressed_size,
                &mut decode_input_offset,
                &mut decompressed,
                &mut decode_output_offset,
            )
            .expect("Decompression should succeed");

        let decompressed_length = decode_output_offset.position() as usize;
        assert_eq!(input.len(), decompressed_length);
    }

    /// Bug affects any input smaller than block size
    #[test]
    fn test_data_loss_partial_block() {
        let input = vec![1u32, 2, 3, 4, 5]; // 5 elements < 128
        let mut codec = FastPFOR::new(65536, BLOCK_SIZE_128);

        let mut compressed = vec![0u32; 1024];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        codec
            .compress(
                &input,
                input.len() as u32,
                &mut input_offset,
                &mut compressed,
                &mut output_offset,
            )
            .expect("Compression should succeed");

        let compressed_size = output_offset.position() as u32;

        let mut decompressed = vec![0u32; 1024];
        let mut decode_input_offset = Cursor::new(0);
        let mut decode_output_offset = Cursor::new(0);

        codec
            .uncompress(
                &compressed,
                compressed_size,
                &mut decode_input_offset,
                &mut decompressed,
                &mut decode_output_offset,
            )
            .expect("Decompression should succeed");

        let decompressed_length = decode_output_offset.position() as usize;
        assert_eq!(input.len(), decompressed_length);
    }

    /// Verify that inputs >= block_size work correctly (sanity check)
    #[test]
    fn test_full_block_works() {
        let input: Vec<u32> = (0..128).collect();
        let mut codec = FastPFOR::new(65536, BLOCK_SIZE_128);

        let mut compressed = vec![0u32; 2048];
        let mut input_offset = Cursor::new(0);
        let mut output_offset = Cursor::new(0);

        codec
            .compress(
                &input,
                input.len() as u32,
                &mut input_offset,
                &mut compressed,
                &mut output_offset,
            )
            .expect("Compression should succeed");

        let compressed_size = output_offset.position() as u32;

        let mut decompressed = vec![0u32; 2048];
        let mut decode_input_offset = Cursor::new(0);
        let mut decode_output_offset = Cursor::new(0);

        codec
            .uncompress(
                &compressed,
                compressed_size,
                &mut decode_input_offset,
                &mut decompressed,
                &mut decode_output_offset,
            )
            .expect("Decompression should succeed");

        let decompressed_length = decode_output_offset.position() as usize;
        assert_eq!(input.len(), decompressed_length);

        for (i, (&original, &decoded)) in input.iter().zip(decompressed.iter()).enumerate() {
            assert_eq!(
                original, decoded,
                "Data mismatch at position {}: expected {}, got {}",
                i, original, decoded
            );
        }
    }
}
