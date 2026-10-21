//! Round-trip checks over the test files (see `common`).

mod common;

use common::corpus;
use libsib::container;
use libsib::{Document, check};

#[test]
fn container_round_trips_byte_for_byte() {
    corpus().iter().for_each(|path| {
        let original = std::fs::read(path).unwrap();
        let rewritten = container::serialize(&container::parse(&original).unwrap()).unwrap();
        assert!(original == rewritten, "{} differs after rewrite", path.display());
    });
}

#[test]
fn root_stamp_is_byte_sum_of_remainder() {
    corpus().iter().for_each(|path| {
        let file = std::fs::read(path).unwrap();
        assert_eq!(
            container::stored_stamp(&file),
            Some(container::byte_sum(&file[18..])),
            "{}",
            path.display()
        );
    });
}

#[test]
fn every_chunk_decodes_completely_and_round_trips() {
    corpus().iter().for_each(|path| {
        let doc = Document::open(path).unwrap();
        check::check(&doc).unwrap().iter().for_each(|f| {
            assert!(f.problem.is_none(), "{} {:?}: {:?}", path.display(), f.path, f.problem);
        });
    });
}
