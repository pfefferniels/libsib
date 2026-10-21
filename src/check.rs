//! Whole-file verification: every chunk decodes completely and re-encodes to
//! the exact original bytes.

use crate::container::ChunkKind;
use crate::document::{self, Content, Document};
use crate::error::Result;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// Chunk kinds from the outermost file down, nested files included.
    pub path: Vec<ChunkKind>,
    pub version: u32,
    pub bytes: usize,
    /// Record bytes the grammar skipped as unknown (newer than its source).
    pub skipped: usize,
    pub problem: Option<String>,
}

pub fn check(doc: &Document) -> Result<Vec<Finding>> {
    let mut out = Vec::new();
    check_into(doc, &[], &mut out)?;
    Ok(out)
}

fn check_into(doc: &Document, prefix: &[ChunkKind], out: &mut Vec<Finding>) -> Result<()> {
    doc.root
        .walk()
        .into_iter()
        .filter(|(_, c)| c.body().is_some())
        .try_for_each(|(path, chunk)| {
            let data = chunk.body().expect("filtered");
            let full: Vec<ChunkKind> = prefix.iter().chain(&path).copied().collect();
            let content = document::decode(&path, chunk)?;
            let mut problem = (document::encode(&content)? != *data).then(|| "re-encoding differs".to_owned());
            let skipped = match &content {
                Content::Score(p) | Content::VersionInfo(p) | Content::IdeaInfo(p) | Content::Clip(p) => {
                    problem = problem.or_else(|| p.stopped.clone());
                    p.root.padding_len()
                }
                Content::Embedded(inner) => {
                    check_into(inner, &full, out)?;
                    0
                }
                Content::Image(_) => 0,
            };
            out.push(Finding {
                path: full,
                version: chunk.version,
                bytes: data.len(),
                skipped,
                problem,
            });
            Ok(())
        })
}
