//! Editing a decoded score and writing it back.

mod common;

use libsib::Document;
use libsib::tree::Value;

const TITLE: &str = "A considerably longer title than any of the originals";

fn title(doc: &Document) -> Option<String> {
    let score = doc.score().ok()?;
    let summary = score.root.node("Score")?.node("Music")?.node("Summary")?;
    summary.node("Title")?.text("default")
}

#[test]
fn edited_title_survives_a_save() {
    let Some(mut doc) = common::corpus()
        .iter()
        .filter_map(|p| Document::open(p).ok())
        .find(|d| title(d).is_some())
    else {
        return;
    };
    let mut score = doc.score().unwrap();
    let summary = score
        .root
        .node_mut("Score")
        .and_then(|s| s.node_mut("Music"))
        .and_then(|m| m.node_mut("Summary"))
        .unwrap();
    *summary.node_mut("Title").unwrap().get_mut("default").unwrap() = Value::str(TITLE);
    doc.set_score(&score).unwrap();

    let reread = Document::from_bytes(&doc.to_bytes().unwrap()).unwrap();
    assert_eq!(title(&reread).as_deref(), Some(TITLE));
    let findings = libsib::check::check(&reread).unwrap();
    assert!(findings.iter().all(|f| f.problem.is_none()), "{findings:?}");
}
