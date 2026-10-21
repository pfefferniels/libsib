# The `.sib` file format

Notes on the format as far as this crate understands it. They describe what the
code in `src/` implements and was tested against. Where a statement rests on
inference rather than on code read from a Sibelius binary, it says so.

## Sources

- **Sibelius 8.3.0 for macOS** (official Avid installer). The payload grammar is
  translated from its `ScoreInStream` readers, decompiled with Ghidra. Every
  generated reader cites the address of the function it comes from
  (`/// 8.3.0 0x…`), and field names are the element names those readers pass
  to `SIS_Enter`, which Sibelius also uses for its XML form.
- **Sibelius 2025.3 for macOS** (Avid-signed and notarised installer). Its
  readers were matched to the 8.3.0 ones by the literals and calls they share,
  and every field they read beyond 8.3.0 was ported by hand
  (`src/grammar/later.rs`, citing `2025.3 0x…`).
- **SCORA** by cw2k (<https://github.com/cw2k/SCORA>). The keystream cipher
  below was first published there, in `CryptedStream.java`.
- **Test files**: 363 files with container versions 0x20001 (Sibelius 1.x) to
  0x450005 (Sibelius 2025). They come from the author's own scores, the examples
  shipped with Sibelius 8.3.0 and 2025.3, the MIT-licensed test files of
  [sibmei](https://github.com/music-encoding/sibmei) and a few public GitHub
  repositories. None of them is distributed with the crate.

## Container

A file is a tree of chunks. Every chunk has the same layout, all integers
big-endian:

| Field | Size | Present when |
|---|---|---|
| magic `0F 53 49 42 45 4C 49 55 53 00` (`\x0fSIBELIUS\0`) | 10 | always |
| version | 4 | always |
| stamp | 4 | version > 6 |
| flags | 4 | always |
| compress info | 4 | flags & 0x08 |
| key block | 62 | flags & 0x02 |
| entry count | 4 | version > 0x210000 |
| payload: zlib stream, XORed with the keystream | – | flags & 0x08 |
| directory: entry count × (kind u32, absolute offset u32) | 8 × n | entry count present |
| child chunks, in pre-order | – | always |

- **Flags.** 0x08 compressed, 0x02 encrypted, 0x10 strings are UTF-16
  (otherwise 8-bit), 0x20 the payload ends with Scorch options, 0x01 every tag
  is followed by a u32 size (not seen in files).
- **Stamp.** In the root chunk of a saved file it is the wrapping byte sum of
  everything after offset 18. Nested chunks and clips store 0.
- **Key block.** 62 bytes. Only the u32 at offset 0x17, the keystream seed, is
  known to be used.
- **Compression.** zlib at level 6 with memory level 8. The system zlib
  reproduces Sibelius' streams byte for byte. The compress-info word was 0 in
  every file seen.
- **Raw bodies.** An uncompressed root chunk can carry its body directly after
  the directory (Idea clips do).
- **Kinds.** 1 score, 2 library, 4 idea, 5 idea text, 6 idea score, 7 idea
  image, 11 versions, 12 version, 13 version info. Version and idea scores hold
  a complete nested file, idea images a PNG, both preceded by a u32 length.
- **Oldest formats.** Files of version 0x20001 have no child chunks. The score
  payload sits in the root chunk. A root payload that starts with tag 0x6a is
  a clip instead.

## Cipher

The keystream mixes two linear congruential generators (the MSVC and ANSI C
`rand` constants) under control of a third, 15-bit generator. With
`base = seed & !7`: `key1 = base ^ 0x1F30D27B`, `key2 = base ^ 0x3A4C80B2`,
`selector = (seed & 7) << 4`. For each byte:

1. `selector = selector × 13821 & 0x7FFF`, `mode = (selector >> 8) & 3`;
2. unless mode is 2, `key1 = key1 × 214013 + 2531011`;
3. unless mode is 1, `key2 = key2 × 1103515245 + 12345`;
4. the key byte is the low byte of `POST[mode] ^ ((mixed >> 16) & 0x7FFF)`,
   where `mixed` is the XOR of the generators that stepped and
   `POST = [27811, 12742, 37606, 5005]`.

Scorch files use the multiplier 47485 and the mask 0xFFFF for the selector.
See `src/cipher.rs`.

## Payload primitives

- Integers are big-endian. Floats are IEEE 754 single precision stored
  little-endian (Sibelius writes them raw).
- A string is a u32 byte length followed by UTF-16BE code units (or 8-bit
  characters when flag 0x10 is clear).
- A tag is one byte that names the object that follows. Readers usually require
  a specific value.
- A **record** is a varint byte length followed by a body. The varint keeps 7
  bits per byte in the high bits, with bit 0 as the continuation flag. Sibelius
  sometimes pads it (length 0 as `01 00`), so the width has to be kept to write
  the same bytes. Readers skip what they do not understand at the end of a
  record. This is how a format version stays readable by older releases, and
  most format changes after Sibelius 2 sit inside records.
- A **list** is a u32 count followed by its items.
- A **part map** (`PartMap<T>`) is a score-wide value followed by per-part
  overrides: a u32 that counts keys and values separately (twice the number of
  overrides), then key/value pairs. Files before 0xc0000 store the score-wide
  value alone.
- A **part key** is a u32, widened by one word from 0x41 and by another from
  0x42 (2025.3 `PM_U32`).
- Coordinates and many sizes are u16 (sign-extended) before 0x1e0000 and u32
  from then on.

## Versions

8.3.0 contains a table of release ids and file versions (initialised by
`0x100dd4e20`, the two halves of each word swapped), which it consults for
version-specific behaviour:

| Release id | Version | Release id | Version |
|---|---|---|---|
| 1 | 0x020000 | 8 | 0x390016 |
| 2 | 0x080022 | 9 | 0x3d000e |
| 3 | 0x0a001f | 10 | 0x3d0010 |
| 4 | 0x1b001f | 11 | 0x3e0000 |
| 5 | 0x280006 | 12 | 0x3e0001 |
| 6 | 0x2d0010 | 13 | 0x3e0002 |
| 7 | 0x36001e | | |

8.3.0 itself writes 0x3d000e. In the test files, 0x2d0010 scores were created
by Sibelius 5.2 and some 0x390016 scores by 7.5.1, which suggests that ids 6
and 8 stand for Sibelius 5 and 7. The table does not name the releases.

- `ScoreOptions.OriginalProgramVersion` names the release that *created* a
  score, e.g. 7510 for 7.5.1 or 20240300 for 2024.3.
- **Saved-down files.** A newer Sibelius that saves in an older format, e.g.
  Sibelius 2024 writing 0x390016, still appends its newer fields to the ends of
  records, where older readers skip them. When a record has bytes left over,
  the reader reads it a second time with the newest layout it knows, and keeps
  that reading if it explains more of the record (`Reader::record`). Inside
  records, fields *added* at a version are read when the layout has them and,
  during such a second reading, only while the record has bytes left
  (`Reader::since`). Fields *removed* at a version, and everything outside
  records, follow the file's own version.
- Some tie fields depend on feature flags of the writing application
  (`EnableTieStyles`, `EnableLVTies`). Files written with a flag off lack
  them, so they are read only while their record has bytes left.

## Payload structure

The score payload is tag 0x00, then `ScoreOptions` (tag 0x40), part print
settings and `Music` (tag 0x01). `Music` holds, in order: the summary (title,
composer, …), MIDI connections, spacing table, playback and house styles (text,
symbols, lines, clefs, noteheads, stave types, instrument types, ensembles),
the large `Options` object, playback options, graphics, bar spacings, the
system stave and its bars, the keyword dictionaries, the staves with their bars
and bar objects, instrument groups, barlines and brackets, and a closing record
with fonts, worksheet settings and part settings. Each part's version gates are
in the readers.

Bar objects start with a tag in 0x07–0x2d (plus 0x92 and 0x93) that selects
the reader. Unknown tags are skipped as a record. The table gives the class
each reader constructs, as named by the run-time type information of 8.3.0
(0x1c only exists in 2025.3).

| Tag | Class | Tag | Class |
|---|---|---|---|
| 07 | Clef | 1a | Highlight |
| 08 | SpecialBarline | 1b | RitardLine |
| 09 | TimeSignature | 1c | staff filter |
| 0a | KeySignature | 1f | SymbolItem |
| 0c | MusicLine | 21 | SystemTextItem |
| 0d | ArpeggioLine | 22 | TextItem |
| 0e 0f | Hairpin (crescendo, diminuendo) | 23 | BarNumber |
| 10 | GlissandoLine | 24 | GuitarFrame |
| 11 | OctavaLine | 25 | Transposition |
| 12 | PedalLine | 26 | RehearsalMark |
| 13 | RepeatTimeLine | 27 | InstrumentChange |
| 14 | Slur | 28 | BarRest |
| 15 | Trill | 29 | NoteRest |
| 16 | Box | 2a | Graphic |
| 17 | BeamLine | 2b | SystemSymbolItem |
| 18 | Tuplet | 2c | CommentItem |
| 19 | LyricItem | 2d | PlayerChange |
| 92 | Item | 93 | AnnotationItem |

Readers in `src/grammar/readers.rs` are named after these classes where the
binary names them, and otherwise after the fields they read.

Other payloads: version info and idea info (kinds 13 and 5), and Idea clips
(tag 0x6a), which name the styles they need where scores use indices.

## Fields newer than 8.3.0

In the test files, the 8.3.0 readers explain every byte up to 0x3d000e. The
fields that later releases added come from the 2025.3 readers
(`src/grammar/later.rs`). Nearly all were appended to records:

- notes: `TieIntoNoteBool` and a second tie (`TieIntoNote`), `Bracketed`,
  `HideInScore`, `HideInPart`;
- ties: `LineStyleId` (e.g. `line.staff.tie`), `GlissType`, `GlissCurve`,
  `TextDisplayState`, `RhDy`, and behind feature flags `Style`, dash and dot
  lengths and `LvTie`;
- bars: `AutoStaveGap` and, from 0x430006, a `FakeBarRest` bar object;
- staves: `ScaleFactor`, `ScaleFactorChanges`, `VStaveNums`,
  `NextTrackFilterID`. Staves listed in the music's `VStaveNums` are virtual
  staves with a `VStaveID` and a `SourceStaveNum` after their body;
- options: stave scales, tie dash and dot defaults, barline gaps, bar-rest and
  view settings;
- instrument types (the instrument change names return in 0x3f0002, then
  `IsDynamic`), instrument changes (the names shown afterwards), stave starts,
  selections, keywords, text styles, spacings, playing options and the music
  record gained a few fields each;
- a new bar object, tag 0x1c, describes a staff filter (`FilterID`,
  `FilterInfo`, `PartID`, `VStaveID`, `FillColor`, `Description`, …);
- `PartsToExtract` items take a second word from 0x42, as part keys do.

## Meaning of the music fields

The grammar names fields but does not say what their values mean. The
following was inferred from the test files while writing a MusicXML converter
([sib2mxl](https://github.com/pfefferniels/sib2mxl)); it holds for all of them
as far as I can see, but rests on observation rather than on the binary.

- Positions (`Sr`) count ticks, 256 to a quarter note, from the start of the
  bar. Bar 0 of every stave holds the initial clef, key signature and
  instrument. `Sr` 0xffff means the start of a bar (time signatures, starting
  repeats), 0x7f00 its end (a clef or key change before the barline).
- Bars store no length. The spacing table does: in `BarSpacings`, the entries
  whose `Count` has bit 0x8000 set mark the bar's edges, and the largest of
  their positions below 0x7f00 is where the bar ends. This gives pickups and
  irregular bars; time signatures carry only their nominal length
  (`SrAfterEndOfBar`).
- `Durn` of a note or rest: the low four bits are log2 of the undotted length
  in ticks (10 is a whole note, 8 a quarter), bits 4–5 the number of dots, bit
  6 marks a grace note. Notes in tuplets keep their written value; the tuplet
  (tag 0x18) gives `Left`:`Right` (3:2 for a triplet), its full span in
  `TupletLength` and the distance to its last note in `Duration`. Hidden
  tuplets still apply.
- `Trem`: the low two bits are the beam mode (0 none, 1 start a beam, 2
  continue it, 3 continue with the primary beam only), bits 2–4 the strokes of
  a one-note tremolo.
- `Artic` is a bit set in the order of ManuScript's articulation constants:
  bit 1 staccato, 2 staccatissimo, 3 wedge, 4 tenuto, 5 accent, 6 marcato,
  7 harmonic, 8 plus, 9 up-bow, 10 down-bow, 12 square pause, 13 pause,
  14 triangular pause.
- `Diat` counts diatonic steps from C in octave −1 and `Chrom2` quarter tones
  from the same C, both at sounding pitch; their difference is the accidental.
  The instrument type's `PartDiat` and `PartChrom2` give the interval from
  written to sounding pitch. On tablature staves of the first versions `Diat`
  holds the string instead.
- Bit 0x80 of a note's `Flags` means it is tied to the next note of the same
  pitch in its voice. A slur's `Duration` runs from the onset of its first note
  to the onset of its last. `Voices` is a bit set, one bit per voice.
- A symbol (tag 0x1f) refers to the `Index` of an entry in `IndexedSymbols`
  (not its position in the list); the indices are the same in every language,
  for instance 32 trill, 36 inverted mordent, 37 mordent, 38 turn, 221 fermata.
- Stave types have a `Category` (0 pitched, 1 percussion, 2 tablature) and
  say whether the stave shows key signatures (`KeySignatures`).
- The low two bits of a note's `Acc` give how its accidental is shown (0 as
  needed, 1 hidden, 2 cautionary, 3 bracketed); the higher bits seem to be a
  horizontal offset.
- `CrossStaveType` 2 draws a chord on the staff above, 3 on the staff below.
  It takes effect only where that staff belongs to the same instrument; on
  other staves the value has no visible effect.
- `FlipStem` flips a stem from the direction it would have by default.
- `Arpeggio` on a chord: 1 an arpeggio, 2 upwards, 3 downwards.
- `BreakType` of a bar in `BarSpacings` is a bit set of where the music may
  break after it: 1 within a system, 2 at the end of a system, 4 at the end of
  a page. The usual value is 7; 2 forces a system break, 4 a page break.
- Text style `Height` and the size command in text (`\s80\`) count 32nds of a
  staff space; a text style's `FontName`, `Bold`, `Italic` and `Underline` set
  the rest of its font. The document's page and staff size are in the options'
  `PageAndStaveMargins`, in millimetres.
- Line styles that a file uses are stored with it: `LineType` 1 solid, 2 dotted,
  3 dashed; `StartCapNotchDy` and `EndCapNotchDy` the hooks at either end
  (negative pointing down); `ArrowType` and `EndCapArrowType` arrowheads; the
  caption in `CaptionRef`. Built-in styles that a file does not use are left out.
- A chord symbol (tag 0x24) names its chord by root and bass pitch class
  (`Root`, `Bass`), their spelling as a diatonic step (`RootSpelling`,
  `BassSpelling`) and a chord type id such as `m9` or `13b9b5`. Its diagram has
  one element per string from the lowest: `ElementType` 0 muted, 1 fretted,
  2 under the barre, at the pitch `Chrom2`. Tablature stave types list their
  open strings in `Pitches`.
- Instrument changes carry a style clef for concert pitch (`ClefStyle`) and
  one for transposed display (`TransposingClefStyle`).
- Lyrics with `SyllableType` 0 are followed by a hyphen; 1 ends a word.
- Figured bass is written in Sibelius' figured bass font, where letters stand
  for figures with accidentals (`h` is 6♯, `B` is ♭5, `´` a slashed 6).
- Files of the first versions refer to text, line and clef styles by their
  index in the style lists; later ones by id, such as `clef.treble`.

## Open questions

- Text objects store offsets (`Dx`, `Dy`) that do not match positions measured
  from the staff; they seem to be relative to the text style's default
  position.
- Files written by Sibelius 2025.4 and later (0x450004, 0x450005) end their
  options record with one to three bytes that 2025.3 does not read. They
  round-trip as raw bytes. Naming them needs a later binary.
- A few names 2025.3 leaves unnamed are provisional (the two numbers in each
  `ScaleFactorChanges` entry, the second word of `SourcePartID` and of a staff
  filter's `PartID`).
- Versions between 0x20001 and 0x2d0010 (release ids 2 to 5) are translated
  from 8.3.0, but no test file was available.
- XML-only branches are folded away because binary files never take them.
