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
}
