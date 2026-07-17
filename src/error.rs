use thiserror::Error;

/// Alias for the result type of `FastPFor` operations.
pub type FastPForResult<T> = Result<T, FastPForError>;

/// Errors that can occur when using the `FastPFor` codecs.
#[non_exhaustive]
#[derive(Error, Debug)]
#[expect(missing_docs)]
pub enum FastPForError {
    #[error("Unsupported operation")]
    Unimplemented,

    #[error("Not enough data in the input buffer")]
    NotEnoughData,

    #[error("Output buffer too small")]
    OutputBufferTooSmall,

    #[error("Invalid input length {0}")]
    InvalidInputLength(usize),

    #[error("Input length {input_len} is not a multiple of block size {block_size}")]
    InputMustBeMultipleOfBlockSize { input_len: usize, block_size: usize },

    #[error("Page size {page_size} is not a multiple of block size {block_size}")]
    InvalidPageSize {
        /// The page size that was provided
        page_size: u32,
        /// The block size that the page size must be a multiple of
        block_size: u32,
    },

    #[cfg(feature = "cpp")]
    #[error("C++ exception: {0}")]
    CppError(#[from] cxx::Exception),

    #[error("Expected element count {expected} exceeds maximum {max}")]
    ExpectedCountExceedsMax {
        /// The expected count provided by the caller
        expected: usize,
        /// The maximum allowed based on input size
        max: usize,
    },

    #[error("Decoded {actual} elements, expected {expected}")]
    DecodedCountMismatch {
        /// Number of elements actually decoded
        actual: usize,
        /// Expected count provided by the caller
        expected: usize,
    },
}
