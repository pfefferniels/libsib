use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected end of data at offset {offset:#x}")]
    UnexpectedEof { offset: usize },
    #[error("missing SIBELIUS magic at offset {offset:#x}")]
    BadMagic { offset: usize },
    #[error("zlib stream at offset {offset:#x}: {source}")]
    Inflate { offset: usize, source: io::Error },
    #[error("unsupported: {0}")]
    Unsupported(String),
    #[error("malformed payload at {offset:#x}: {message}")]
    Malformed { offset: usize, message: String },
    #[error(transparent)]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
