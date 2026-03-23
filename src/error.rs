use thiserror::Error;

/// Alias for the result type of `FastPFor` operations.
pub type FastPForResult<T> = Result<T, FastPForError>;

/// Errors that can occur when using the `FastPFor` codecs.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum FastPForError {
    /// Unsupported operation
    #[error("Unsupported operation")]
    Unimplemented,

    /// Not enough data in the input buffer
    #[error("Not enough data in the input buffer")]
    NotEnoughData,

    /// Output buffer too small
    #[error("Output buffer too small")]
    OutputBufferTooSmall,

    /// Invalid input length
    #[error("Invalid input length {0}")]
    InvalidInputLength(usize),

    /// Page size is not a multiple of the block size
    #[error("Page size {page_size} is not a multiple of block size {block_size}")]
    InvalidPageSize {
        /// The page size that was provided
        page_size: u32,
        /// The block size that the page size must be a multiple of
        block_size: u32,
    },

    /// Error propagated from the C++ `FastPFOR` library
    #[cfg(feature = "cpp")]
    #[error("C++ exception: {0}")]
    CppError(#[from] cxx::Exception),

    /// Expected element count exceeds maximum allowed (possible corrupt or untrusted input)
    #[error("Expected element count {expected} exceeds maximum {max}")]
    ExpectedCountExceedsMax {
        /// The expected count provided by the caller
        expected: usize,
        /// The maximum allowed based on input size
        max: usize,
    },

    /// Decoded element count did not match the expected count
    #[error("Decoded {actual} elements, expected {expected}")]
    DecodedCountMismatch {
        /// Number of elements actually decoded
        actual: usize,
        /// Expected count provided by the caller
        expected: usize,
    },
}
