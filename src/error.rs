use thiserror::Error;

pub type FastPForResult<T> = Result<T, FastPForError>;

#[derive(Error, Debug)]
pub enum FastPForError {
    #[error("Unable to uncompress data: {0}")]
    UncompressError(String),
    #[error("Unable to compress data: {0}")]
    CompressError(String),
    #[error("Invalid block size: {0}")]
    InvalidBlockSizeError(String),
}
