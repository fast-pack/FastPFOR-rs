use std::io::Cursor;

use crate::rust::FastPForResult;

/// Integer compression/decompression interface with length headers.
///
/// Implementations write output length as a header before compressed data,
/// enabling self-describing compressed streams.
pub trait Integer<T> {
    /// Compresses integers with length header.
    ///
    /// # Arguments
    /// * `input_length` - Number of integers to compress
    /// * `input_offset` - Read position cursor, advanced by `input_length`
    /// * `output_offset` - Write position cursor, advanced by bytes written
    fn compress(
        &mut self,
        input: &[u32],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [T],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()>;

    /// Decompresses integers using length header.
    ///
    /// # Arguments
    /// * `input_length` - Total compressed data length
    /// * `input_offset` - Read position cursor, advanced by bytes read
    /// * `output_offset` - Write position cursor, advanced by integers written
    fn uncompress(
        &mut self,
        input: &[T],
        input_length: u32,
        input_offset: &mut Cursor<u32>,
        output: &mut [u32],
        output_offset: &mut Cursor<u32>,
    ) -> FastPForResult<()>;
}
