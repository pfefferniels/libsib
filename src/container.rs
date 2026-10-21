//! The outer chunk tree of a `.sib` file.
//!
//! Every chunk starts with `0x0F "SIBELIUS\0"`, a big-endian header and an optional
//! zlib payload, followed by a directory of `(kind, absolute offset)` pairs that
//! point at child chunks. Children follow their parent's directory in pre-order.
//! See `FORMAT.md` for the byte layout.

use std::io::{self, BufReader, Read, Write};

use flate2::Compression;
use flate2::bufread::ZlibDecoder;
use flate2::write::ZlibEncoder;

use crate::cipher::{self, Keystream, Variant};
use crate::error::{Error, Result};

pub const MAGIC: &[u8; 10] = b"\x0fSIBELIUS\0";
const FLAG_ENCRYPTED: u32 = 0x02;
const FLAG_COMPRESSED: u32 = 0x08;
const STAMP_OFFSET: usize = 14;
const KEY_BLOCK_LEN: usize = 62;
const SEED_OFFSET: usize = 0x17;
const ZLIB_LEVEL: u32 = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChunkKind(pub u32);

impl ChunkKind {
    pub const SCORE: Self = Self(1);
    pub const LIBRARY: Self = Self(2);
    pub const IDEA: Self = Self(4);
    pub const IDEA_TEXT: Self = Self(5);
    pub const IDEA_SCORE: Self = Self(6);
    pub const IDEA_IMAGE: Self = Self(7);
    pub const VERSIONS: Self = Self(11);
    pub const VERSION: Self = Self(12);
    pub const VERSION_INFO: Self = Self(13);

    pub fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::SCORE => "score",
            Self::LIBRARY => "library",
            Self::IDEA => "idea",
            Self::IDEA_TEXT => "idea-text",
            Self::IDEA_SCORE => "idea-score",
            Self::IDEA_IMAGE => "idea-image",
            Self::VERSIONS => "versions",
            Self::VERSION => "version",
            Self::VERSION_INFO => "version-info",
            _ => return None,
        })
    }
}

/// 62 opaque bytes stored ahead of an encrypted payload; only the big-endian
/// word at offset 0x17 (the keystream seed) is known to be read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyBlock(pub [u8; KEY_BLOCK_LEN]);

impl KeyBlock {
    pub fn seed(&self) -> u32 {
        let word = &self.0[SEED_OFFSET..SEED_OFFSET + 4];
        u32::from_be_bytes(word.try_into().expect("four bytes"))
    }

    /// A key block whose filler bytes are derived from `seed`.
    pub fn from_seed(seed: u32) -> Self {
        let mut bytes: [u8; KEY_BLOCK_LEN] = [0; KEY_BLOCK_LEN];
        bytes
            .iter_mut()
            .zip(Keystream::new(!seed, Variant::Sibelius))
            .for_each(|(byte, key)| *byte = key);
        bytes[SEED_OFFSET..SEED_OFFSET + 4].copy_from_slice(&seed.to_be_bytes());
        Self(bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Payload {
    pub data: Vec<u8>,
    pub key: Option<KeyBlock>,
    pub compress_info: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub kind: ChunkKind,
    /// `None` for a directory slot with offset 0.
    pub chunk: Option<Chunk>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chunk {
    pub version: u32,
    /// In the root chunk of a saved score this is the byte sum of the file after
    /// the field (see `stamp_is_checksum`); nested chunks and clips store 0.
    pub stamp: u32,
    /// Whether `stamp` held the checksum, and so must be recomputed on write.
    pub stamp_is_checksum: bool,
    /// Header flags other than the compression and encryption bits.
    pub other_flags: u32,
    pub payload: Option<Payload>,
    pub entries: Vec<Entry>,
    /// Uncompressed body following the header, as in Idea clips: the rest of
    /// the file after a root chunk without payload or directory.
    pub raw: Vec<u8>,
}

impl Chunk {
    fn has_stamp(&self) -> bool {
        self.version > 6
    }

    fn has_entry_count(&self) -> bool {
        self.version > 0x0021_0000
    }

    fn flags(&self) -> u32 {
        let compressed = self.payload.as_ref().map_or(0, |_| FLAG_COMPRESSED);
        let encrypted = self.payload.as_ref().and_then(|p| p.key).map_or(0, |_| FLAG_ENCRYPTED);
        self.other_flags | compressed | encrypted
    }

    /// Depth-first iteration over this chunk and its descendants, with the kind
    /// path leading to each.
    pub fn walk(&self) -> Vec<(Vec<ChunkKind>, &Chunk)> {
        let mut out = vec![(Vec::new(), self)];
        self.entries.iter().for_each(|entry| {
            if let Some(child) = &entry.chunk {
                out.extend(child.walk().into_iter().map(|(mut path, chunk)| {
                    path.insert(0, entry.kind);
                    (path, chunk)
                }));
            }
        });
        out
    }

    /// The decompressed payload or, failing that, the raw body.
    pub fn body(&self) -> Option<&[u8]> {
        match (&self.payload, self.raw.is_empty()) {
            (Some(p), _) => Some(&p.data),
            (None, false) => Some(&self.raw),
            (None, true) => None,
        }
    }

    pub fn child(&self, kind: ChunkKind) -> Option<&Chunk> {
        self.entries
            .iter()
            .find(|e| e.kind == kind)
            .and_then(|e| e.chunk.as_ref())
    }

    pub fn child_mut(&mut self, kind: ChunkKind) -> Option<&mut Chunk> {
        self.entries
            .iter_mut()
            .find(|e| e.kind == kind)
            .and_then(|e| e.chunk.as_mut())
    }
}

pub fn parse(file: &[u8]) -> Result<Chunk> {
    let (mut root, end) = parse_at(file, 0)?;
    root.stamp_is_checksum = root.has_stamp() && root.stamp == byte_sum(&file[STAMP_OFFSET + 4..]);
    if end < file.len() {
        if root.payload.is_some() || !root.entries.is_empty() {
            return Err(Error::Malformed {
                offset: end,
                message: "data after the chunk tree".into(),
            });
        }
        root.raw = file[end..].to_vec();
    }
    Ok(root)
}

pub fn serialize(root: &Chunk) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    write_chunk(root, &mut out)?;
    if root.has_stamp() && root.stamp_is_checksum {
        let sum = byte_sum(&out[STAMP_OFFSET + 4..]);
        out[STAMP_OFFSET..STAMP_OFFSET + 4].copy_from_slice(&sum.to_be_bytes());
    }
    Ok(out)
}

/// The value stored in the root chunk's stamp field.
pub fn byte_sum(bytes: &[u8]) -> u32 {
    bytes.iter().fold(0u32, |sum, &b| sum.wrapping_add(b as u32))
}

pub fn stored_stamp(file: &[u8]) -> Option<u32> {
    file.get(STAMP_OFFSET..STAMP_OFFSET + 4)
        .map(|w| u32::from_be_bytes(w.try_into().expect("four bytes")))
}

struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let bytes = self
            .data
            .get(self.pos..self.pos + n)
            .ok_or(Error::UnexpectedEof { offset: self.pos })?;
        self.pos += n;
        Ok(bytes)
    }

    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes(self.take(4)?.try_into().expect("four bytes")))
    }
}

/// Parses the chunk at `offset`; also returns where the chunk and its
/// descendants end.
fn parse_at(file: &[u8], offset: usize) -> Result<(Chunk, usize)> {
    let mut cur = Cursor {
        data: file,
        pos: offset,
    };
    if cur.take(MAGIC.len())? != MAGIC {
        return Err(Error::BadMagic { offset });
    }
    let version = cur.u32()?;
    let stamp = if version > 6 { cur.u32()? } else { 0 };
    let flags = cur.u32()?;
    let compressed = flags & FLAG_COMPRESSED != 0;
    let encrypted = flags & FLAG_ENCRYPTED != 0;
    if encrypted && !compressed {
        return Err(Error::Unsupported(format!(
            "encrypted but uncompressed chunk at {offset:#x}"
        )));
    }
    let compress_info = if compressed { cur.u32()? } else { 0 };
    let key = if encrypted {
        Some(KeyBlock(cur.take(KEY_BLOCK_LEN)?.try_into().expect("key block length")))
    } else {
        None
    };
    let count = if version > 0x0021_0000 { cur.u32()? as usize } else { 0 };
    let payload = if compressed {
        let (data, used) = inflate(&file[cur.pos..], key.map(|k| k.seed()), cur.pos)?;
        cur.pos += used;
        Some(Payload {
            data,
            key,
            compress_info,
        })
    } else {
        None
    };
    let slots = (0..count)
        .map(|_| Ok((ChunkKind(cur.u32()?), cur.u32()? as usize)))
        .collect::<Result<Vec<_>>>()?;
    let mut end = cur.pos;
    let entries = slots
        .into_iter()
        .map(|(kind, child)| {
            let chunk = (child != 0)
                .then(|| parse_at(file, child))
                .transpose()?
                .map(|(chunk, child_end)| {
                    end = end.max(child_end);
                    chunk
                });
            Ok(Entry { kind, chunk })
        })
        .collect::<Result<Vec<_>>>()?;
    let chunk = Chunk {
        version,
        stamp,
        stamp_is_checksum: false,
        other_flags: flags & !(FLAG_COMPRESSED | FLAG_ENCRYPTED),
        payload,
        entries,
        raw: Vec::new(),
    };
    Ok((chunk, end))
}

/// Decrypts on the fly so that inflating stops exactly at the end of the zlib stream.
struct Decrypting<'a> {
    src: &'a [u8],
    keys: Option<Keystream>,
}

impl Read for Decrypting<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.src.read(buf)?;
        if let Some(keys) = &mut self.keys {
            buf[..n].iter_mut().zip(keys).for_each(|(byte, key)| *byte ^= key);
        }
        Ok(n)
    }
}

fn inflate(src: &[u8], seed: Option<u32>, offset: usize) -> Result<(Vec<u8>, usize)> {
    let keys = seed.map(|s| Keystream::new(s, Variant::Sibelius));
    let mut decoder = ZlibDecoder::new(BufReader::new(Decrypting { src, keys }));
    let mut data = Vec::new();
    decoder
        .read_to_end(&mut data)
        .map_err(|source| Error::Inflate { offset, source })?;
    Ok((data, decoder.total_in() as usize))
}

fn deflate(data: &[u8], seed: Option<u32>) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::new(ZLIB_LEVEL));
    encoder.write_all(data)?;
    let packed = encoder.finish()?;
    Ok(match seed {
        Some(seed) => cipher::apply(&packed, seed),
        None => packed,
    })
}

fn write_chunk(chunk: &Chunk, out: &mut Vec<u8>) -> Result<()> {
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&chunk.version.to_be_bytes());
    if chunk.has_stamp() {
        out.extend_from_slice(&chunk.stamp.to_be_bytes());
    }
    out.extend_from_slice(&chunk.flags().to_be_bytes());
    if let Some(payload) = &chunk.payload {
        out.extend_from_slice(&payload.compress_info.to_be_bytes());
        if let Some(key) = &payload.key {
            out.extend_from_slice(&key.0);
        }
    }
    if chunk.has_entry_count() {
        out.extend_from_slice(&(chunk.entries.len() as u32).to_be_bytes());
    }
    if let Some(payload) = &chunk.payload {
        out.extend(deflate(&payload.data, payload.key.map(|k| k.seed()))?);
    }
    let directory = out.len();
    out.resize(directory + 8 * chunk.entries.len(), 0);
    out.extend_from_slice(&chunk.raw);
    chunk.entries.iter().enumerate().try_for_each(|(i, entry)| {
        let child_offset = match &entry.chunk {
            Some(child) => {
                let at = out.len() as u32;
                write_chunk(child, out)?;
                at
            }
            None => 0,
        };
        let slot = directory + 8 * i;
        out[slot..slot + 4].copy_from_slice(&entry.kind.0.to_be_bytes());
        out[slot + 4..slot + 8].copy_from_slice(&child_offset.to_be_bytes());
        Ok(())
    })
}
