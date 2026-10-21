# libsib

A reader and writer for Sibelius `.sib` files in Rust. A file is decoded into
its chunk tree and, for every payload, a tree of named fields that encodes
itself back to the same bytes. An edited tree is written by the same path.
[`FORMAT.md`](FORMAT.md) describes the format.

## Status

- All 363 test files decode completely and are written back byte for byte,
  nested versions, Ideas and clips included. They span container versions
  0x20001 (Sibelius 1.x) to 0x450005 (Sibelius 2025).
- Field names are the ones Sibelius itself uses. The grammar follows the
  readers of Sibelius 8.3.0, extended with the fields that Sibelius 2025.3
  reads.
- In the test files, 16 bytes are kept as raw bytes instead of fields: fields
  added after 2025.3 (see *Open questions* in `FORMAT.md`).
- Container versions between 0x20001 and 0x2d0010 (Sibelius 2 to 4) are
  supported but untested.
- There is no typed music model yet. Editing works on the field tree.

## Use

```rust
use libsib::Document;
use libsib::tree::Value;

fn main() -> libsib::Result<()> {
    let mut doc = Document::open("score.sib")?;
    let mut score = doc.score()?;
    let summary = score.root.node_mut("Score").and_then(|s| s.node_mut("Music")).and_then(|m| m.node_mut("Summary")).unwrap();
    *summary.node_mut("Title").unwrap().get_mut("default").unwrap() = Value::str("New title");
    doc.set_score(&score)?;
    doc.save("edited.sib")
}
```

The `sibtool` binary inspects files:

```
sibtool tree <file.sib>...            print the chunk tree
sibtool unpack <file.sib> <dir>       write each decompressed payload to <dir>
sibtool roundtrip <file.sib>...       check that parse + serialize reproduces each file
sibtool parse [--trace] <file.sib>... parse the score payload and report how far the grammar gets
sibtool check <file.sib>...           decode every chunk (nested files too) and verify the round trip
sibtool padding [--dump <path>] <file.sib>...  sum the unread record bytes by record path
```

`cargo test` checks the round trip of every `.sib` file found in the
directories listed in `SIB_CORPUS` (separated like `PATH`). No test files are
distributed; without the variable those tests pass vacuously.

## Layout

- `src/container.rs`, `src/cipher.rs`: chunk tree, keystream, zlib.
- `src/reader.rs`, `src/tree.rs`: field reader and the self-encoding tree.
- `src/grammar/readers.rs`: readers generated from the Sibelius 8.3.0 readers.
- `src/grammar/manual.rs`: readers too irregular to generate.
- `src/grammar/later.rs`: fields added after 8.3.0, from Sibelius 2025.3.
- `src/document.rs`, `src/check.rs`: whole files, nested files, verification.

## Notes

The format was studied for interoperability. This repository contains neither
Sibelius code nor decompiler output; the generator and its inputs are kept
outside it. The grammar records the field names and read order observed in
Sibelius 8.3.0 and 2025.3.

## Credits

- The payload cipher was first described by cw2k in
  [SCORA](https://github.com/cw2k/SCORA).
- Decompilation used [Ghidra](https://ghidra-sre.org/).
- Many test files are the MIT-licensed test scores of
  [sibmei](https://github.com/music-encoding/sibmei).

## Licence

Licensed under either of the [Apache License, Version 2.0](LICENSE-APACHE) or
the [MIT licence](LICENSE-MIT), at your option.
