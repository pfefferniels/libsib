//! Reader and writer for Avid Sibelius `.sib` score files.
//!
//! [`Document`] opens a file and decodes every chunk; [`grammar`] turns a chunk
//! payload into a lossless [`tree::Node`] tree that writes back byte for byte.

pub mod check;
pub mod cipher;
pub mod container;
pub mod document;
pub mod error;
pub mod grammar;
pub mod reader;
pub mod tree;

pub use container::{Chunk, ChunkKind};
pub use document::{Content, Document};
pub use error::{Error, Result};
