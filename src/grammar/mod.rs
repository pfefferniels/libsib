//! Grammar for decompressed chunk payloads, translated from the
//! `ScoreInStream` readers of Sibelius 8.3.0 (addresses cited per function).

mod later;
mod manual;
mod prim;
mod readers;

use crate::error::Result;
use crate::reader::Reader;
use crate::tree::Node;

/// Oldest container version the grammar supports: the first entry of 8.3.0's
/// table of export formats (`0x102cebcf0`).
pub const MIN_VERSION: u32 = 0x0002_0000;

/// Newest container version seen in files; its field layout is the one
/// records are reread with when bytes are left over.
pub const NEWEST_VERSION: u32 = 0x0045_0004;


/// What a payload holds, and so which reader starts it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entry {
    Score,
    VersionInfo,
    IdeaInfo,
    /// An Idea: a clipboard selection with the styles it needs.
    Clip,
}

impl Entry {
    fn reader(self) -> (&'static str, fn(&mut Reader) -> Result<()>) {
        match self {
            Entry::Score => ("Score", readers::score),
            Entry::VersionInfo => ("VersionInfo", readers::version_info),
            Entry::IdeaInfo => ("IdeaInfo", readers::idea_info),
            Entry::Clip => ("Clip", manual::clip),
        }
    }
}

/// A payload parsed as far as the grammar reaches. Whatever could not be parsed
/// is kept verbatim in `rest`, so writing stays lossless either way.
#[derive(Clone, Debug, PartialEq)]
pub struct Parsed {
    pub root: Node,
    pub rest: Vec<u8>,
    /// Why parsing stopped before the end, if it did.
    pub stopped: Option<String>,
}

impl Parsed {
    pub fn is_complete(&self) -> bool {
        self.stopped.is_none()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.root.write(&mut out);
        out.extend_from_slice(&self.rest);
        out
    }
}

pub fn parse(data: &[u8], entry: Entry, version: u32, flags: u32, trace: bool) -> Parsed {
    let mut r = Reader::new(data, version, flags).with_newest_layout(NEWEST_VERSION.max(version)).with_trace(trace);
    let (kind, body) = entry.reader();
    let stopped = if version < MIN_VERSION {
        Some(format!("container version {version:#x} predates the oldest supported format"))
    } else {
        match r.node(kind, kind, body) {
            Err(e) => {
                r.rewind(0);
                Some(e.to_string())
            }
            Ok(()) if !r.at_end() => Some(format!("{} bytes after the end of the {kind}", r.remaining())),
            Ok(()) => None,
        }
    };
    // A failed reader leaves no fields behind, and the rewind put every byte in `rest`.
    let rest = data[r.pos()..].to_vec();
    Parsed { root: r.finish(), rest, stopped }
}
