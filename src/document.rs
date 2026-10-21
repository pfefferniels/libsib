//! A whole `.sib` file: the chunk tree plus the decoded content of every chunk.

use std::path::Path;

use crate::container::{self, Chunk, ChunkKind};
use crate::error::{Error, Result};
use crate::grammar::{self, Entry, Parsed};

/// Decoded payload of one chunk, chosen by where the chunk sits in the tree.
#[derive(Clone, Debug, PartialEq)]
pub enum Content {
    Score(Parsed),
    VersionInfo(Parsed),
    IdeaInfo(Parsed),
    Clip(Parsed),
    /// A complete nested file (a saved version or an idea), length-prefixed.
    Embedded(Box<Document>),
    /// A length-prefixed preview image (PNG).
    Image(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    pub root: Chunk,
}

impl Document {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_bytes(&std::fs::read(path)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Self {
            root: container::parse(bytes)?,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        container::serialize(&self.root)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(std::fs::write(path, self.to_bytes()?)?)
    }

    /// The main score payload: the score chunk's, or the root's in the oldest formats.
    pub fn score(&self) -> Result<Parsed> {
        let (path, chunk) = self.score_chunk()?;
        match decode(path, chunk)? {
            Content::Score(parsed) => Ok(parsed),
            _ => Err(missing("score payload")),
        }
    }

    /// Replaces the main score payload, e.g. after editing its tree.
    pub fn set_score(&mut self, score: &Parsed) -> Result<()> {
        let chunk = match self.root.child(ChunkKind::SCORE) {
            Some(_) => self.root.child_mut(ChunkKind::SCORE).expect("checked"),
            None => &mut self.root,
        };
        let payload = chunk.payload.as_mut().ok_or_else(|| missing("score payload"))?;
        payload.data = score.to_bytes();
        Ok(())
    }

    fn score_chunk(&self) -> Result<(&'static [ChunkKind], &Chunk)> {
        match self.root.child(ChunkKind::SCORE) {
            Some(chunk) => Ok((&[ChunkKind::SCORE], chunk)),
            None if self.root.payload.is_some() => Ok((&[], &self.root)),
            None => Err(missing("score chunk")),
        }
    }

    /// Every chunk with a payload, decoded, together with its kind path.
    pub fn contents(&self) -> Result<Vec<(Vec<ChunkKind>, Content)>> {
        self.root
            .walk()
            .into_iter()
            .filter(|(_, chunk)| chunk.body().is_some())
            .map(|(path, chunk)| decode(&path, chunk).map(|content| (path, content)))
            .collect()
    }
}

fn missing(what: &str) -> Error {
    Error::Unsupported(format!("file has no {what}"))
}

const CLIP_TAG: u8 = 0x6a;

/// Decodes the payload of `chunk`, found at `path` below the root.
pub fn decode(path: &[ChunkKind], chunk: &Chunk) -> Result<Content> {
    let data = chunk.body().ok_or_else(|| missing("payload"))?;
    let parse = |entry| grammar::parse(data, entry, chunk.version, chunk.other_flags, false);
    Ok(match path {
        [ChunkKind::SCORE] => Content::Score(parse(Entry::Score)),
        // Clips (tag 0x6a) and scores of the oldest formats keep their payload in the root.
        [] if data.first() == Some(&CLIP_TAG) => Content::Clip(parse(Entry::Clip)),
        [] => Content::Score(parse(Entry::Score)),
        [.., ChunkKind::VERSION, ChunkKind::VERSION_INFO] => Content::VersionInfo(parse(Entry::VersionInfo)),
        [.., ChunkKind::IDEA, ChunkKind::IDEA_TEXT] => Content::IdeaInfo(parse(Entry::IdeaInfo)),
        [.., ChunkKind::VERSION, ChunkKind::SCORE] | [.., ChunkKind::IDEA, ChunkKind::IDEA_SCORE] => {
            Content::Embedded(Box::new(Document::from_bytes(length_prefixed(data)?)?))
        }
        [.., ChunkKind::IDEA, ChunkKind::IDEA_IMAGE] => Content::Image(length_prefixed(data)?.to_vec()),
        _ => return Err(Error::Unsupported(format!("payload of a chunk at {path:?}"))),
    })
}

/// Encodes `content` back into payload bytes.
pub fn encode(content: &Content) -> Result<Vec<u8>> {
    Ok(match content {
        Content::Score(p) | Content::VersionInfo(p) | Content::IdeaInfo(p) | Content::Clip(p) => p.to_bytes(),
        Content::Embedded(doc) => with_length(&doc.to_bytes()?),
        Content::Image(bytes) => with_length(bytes),
    })
}

fn length_prefixed(data: &[u8]) -> Result<&[u8]> {
    let len = data
        .get(..4)
        .map(|w| u32::from_be_bytes(w.try_into().expect("four bytes")) as usize)
        .ok_or(Error::UnexpectedEof { offset: 0 })?;
    if data.len() != 4 + len {
        return Err(Error::Malformed {
            offset: 0,
            message: format!("length prefix {len} for {} bytes", data.len() - 4),
        });
    }
    Ok(&data[4..])
}

fn with_length(bytes: &[u8]) -> Vec<u8> {
    let mut out = (bytes.len() as u32).to_be_bytes().to_vec();
    out.extend_from_slice(bytes);
    out
}
