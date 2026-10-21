//! Readers whose control flow depends on values read earlier, translated by
//! hand from the Sibelius 8.3.0 decompilation.

#![allow(unused_imports)]

use super::later::*;
use super::prim::*;
use super::readers::*;
use crate::error::Result;
use crate::reader::Reader;
use crate::tree::Value;

/// 8.3.0 `0x10116dcda`.
pub(super) fn part_batch_print_settings(r: &mut Reader) -> Result<()> {
    r.tag(0x42)?;
    r.u32("Flags")?;
    r.u32("PaperSize")?;
    r.u32("Orientation")?;
    r.u32("NormalSizeOrientation")?;
    r.u16("PaperWidth")?;
    r.u16("PaperLength")?;
    r.u16("Copies")?;
    r.bool("Collate")?;
    r.bool("AllPages")?;
    r.bool("CropMarks")?;
    r.bool("Border")?;
    r.u32("Scale")?;
    r.u32("NormalSizeScale")?;
    r.string("Range")?;
    r.u32("Format")?;
    r.u32("NormalSubRange")?;
    r.u32("BookletSubRange")?;
    let size = r.i32("PlatformDataSize")?;
    r.bytes("PlatformData", size.max(0) as usize)?;
    r.record("record", "Record", |r| {
        if r.since(0x1b_0006) {
            r.u32("NUpNumber")?;
        }
        if r.since(0x39_0005) {
            r.u32("PrintRange")?;
        }
        if r.since(0x39_0007) {
            r.u32("DuplexingType")?;
        }
        if r.since(0x39_000c) {
            r.string("PaperName")?;
        }
        if r.since(0x39_000d) {
            r.string("PrinterName")?;
        }
        if r.since(0x39_000e) {
            r.f32("PaperWidthF")?;
            r.f32("PaperLengthF")?;
        }
        if r.since(0x39_0011) {
            r.bool("DraftFooter")?;
        }
        if r.since(0x39_0012) {
            r.f32("MarginLeft")?;
            r.f32("MarginTop")?;
            r.f32("MarginRight")?;
            r.f32("MarginBottom")?;
        }
        if r.since(0x39_0013) {
            r.u32("DuplexingTypeBoundsChecked")?;
        }
        Ok(())
    })
}

/// First version that widened coordinates from u16 to u32.
const WIDE_COORDINATES: u32 = 0x001e_0000;

/// Reads a coordinate: a sign-extended u16 in files before 0x1e0000.
fn coordinate(r: &mut Reader, name: &'static str) -> Result<()> {
    if r.file_version < WIDE_COORDINATES {
        r.i16(name).map(drop)
    } else {
        r.u32(name).map(drop)
    }
}

/// 8.3.0 `0x100f58a1e`: notehead style with four notehead symbols and per-symbol offsets.
pub(super) fn note_style(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x33)?;
    if (0x10_0000..0x13_0000).contains(&v) {
        if v < 0x12_0000 {
            r.string("Name")?;
        } else {
            style_name(r)?;
        }
        r.node("Id", "StyleId", style_id)?;
        r.bool("Compulsory")?;
        r.record("record", "Record", |_| Ok(()))?;
    } else {
        style_name(r)?;
    }
    r.list_n("NoteHeads", 4, |r| r.read_u16().map(|v| Value::I16(v as i16)))?;
    coordinate(r, "StemShortening")?;
    r.record("record", "Record", |r| {
        if !r.since(0x3_000c) {
            return Ok(());
        }
        r.bool("Playable")?;
        r.bool("Transposable")?;
        r.bool("HasAccidentals")?;
        r.bool("HasStem")?;
        r.bool("HasLegerLines")?;
        r.list_n("NoteheadOptions", 4, |r| {
            r.sub("NoteheadOptions", |r| {
                [
                    "StemUpStemOffset",
                    "StemUpNoteheadRightOffset",
                    "StemUpNoteheadUpOffset",
                    "StemDownStemOffset",
                    "StemDownNoteheadLeftOffset",
                    "StemDownNoteheadDownOffset",
                ]
                .into_iter()
                .try_for_each(|name| coordinate(r, name))
            })
            .map(Value::Node)
        })
    })
}

/// 8.3.0 `0x100fd7194`: a list of instrument instances (files before 0x2d0000).
fn instance_list(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| r.sub("Instance", instance).map(Value::Node)).map(drop)
}


/// 8.3.0 `0x100fcb520`: the whole musical content. Written for a normal file
/// load: not a part being linked to a master score, never cancelled.
pub(super) fn music(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x01)?;
    r.node("Summary", "Summary", summary)?;
    list_midiconnections(r, "MIDIConnections")?;
    r.node("SpacingTable", "SpacingTable", spacing_table)?;
    if v > 0x2bffff {
        f_100fd72a5(r, "DeletedStaveKeywords")?;
        f_100fd72a5(r, "DeletedSystemKeywords")?;
        list_deleted_line_pb_styles(r, "DeletedLinePbStyles")?;
        f_100fd73cb(r, "DeletedArticulationPbStyles")?;
        f_100fd73cb(r, "DeletedNoteheadPbStyles")?;
        f_100fd73cb(r, "DeletedSymbolPbStyles")?;
    }
    if v > 0x2affff {
        list_line_playback_style_list(r, "LinePlaybackStyleList")?;
        list_articulation_playback_style_list(r, "ArticulationPlaybackStyleList")?;
        list_notehead_playback_style_list(r, "NoteheadPlaybackStyleList")?;
        list_symbol_playback_style_list(r, "SymbolPlaybackStyleList")?;
    }
    text_styles(r)?;
    list_symbol_categories(r, "SymbolCategories")?;
    indexed_symbols(r)?;
    pen_styles(r)?;
    line_styles(r)?;
    clef_styles(r)?;
    indexed_note_styles(r)?;
    stave_types(r)?;
    if v > 0x6ffff {
        list_swing_styles(r, "SwingStyles")?;
    }
    if v > 0x220002 {
        instrument_types(r)?;
        ensembles(r)?;
    }
    if v > 0x23ffff && v < 0x2d0000 {
        instance_list(r, "InstrumentInstanceList")?;
    }
    r.node("Options", "Options", options)?;
    r.node("PlayingOptions", "PlayingOptions", playing_options)?;
    if v > 0x3ffff {
        list_graphic_data(r, "GraphicDataList")?;
    }
    list_bar_spacings(r, "BarSpacings")?;
    r.u16("NumBars")?;
    r.node("SystemStave", "SystemStave", system_stave)?;
    r.list("SystemKeywordList", |r| r.sub("Keyword", keyword).map(Value::Node))?;
    r.list("StaveKeywordList", |r| r.sub("Keyword", keyword).map(Value::Node))?;
    let mut flagged = Vec::new();
    // staves listed here are virtual staves (2025.3 `0x1013370f0`)
    if v > 0x40_ffff {
        r.list("VStaveNums", |r| {
            let index = r.read_u32()?;
            flagged.push(index);
            Ok(Value::U32(index))
        })?;
    }
    let mut index = 0u32;
    r.list("Staves", |r| {
        let node = r.sub("Stave", |r| {
            stave(r)?;
            if flagged.contains(&index) {
                vstave(r)?;
            }
            Ok(())
        })?;
        index += 1;
        Ok(Value::Node(node))
    })?;
    if v < 0x240000 {
        instance_list(r, "Instruments")?;
    } else if v >= 0x2d0000 {
        list_instrument_groups(r, "InstrumentGroups")?;
    }
    if v < 0xd0000 {
        list_barlines(r, "Barlines")?;
        list_brackets(r, "Brackets")?;
    } else {
        pm_barlines(r, "Barlines")?;
        pm_brackets(r, "Brackets")?;
    }
    r.i8("ChannelForNextInstrument")?;
    r.string("MusicTextFontName")?;
    r.bool("MusicTextFontBold")?;
    r.bool("MusicTextFontItalic")?;
    r.record("record", "Record", |r| {
        if r.since(0x8_0021) {
            r.string("BaseTextFontName")?;
        }
        if r.since(0xa_000e) && r.file_version < 0x330000 {
            list_custom_chord_names(r, "CustomChordNameList")?;
        }
        if r.since(0xa_0016) {
            r.node("KontaktCustomSounds", "KontaktCustomSounds", kontakt_custom_sounds)?;
        }
        if r.since(0xc_0001) {
            r.node("SyncOptions", "SyncOptions", sync_options)?;
        }
        if r.since(0xc_0004) {
            pm_part_staves(r, "PartStaves")?;
        }
        if r.since(0xd_0003) {
            r.string("WorksheetExerciseInstructions")?;
            if r.file_version < 0x1a0002 {
                r.bool("WorksheetHasSourceBox")?;
            }
            r.u32("WorksheetBarsPerQuestion")?;
            if r.since(0x1b_0000) {
                r.u32("WorksheetQuestionsPerSystem")?;
            }
            ["Student", "Teacher"].iter().try_for_each(|_| {
                r.list_n("WorksheetVoicesVisible", 4, |r| r.read_u8().map(Value::Bool))
            })?;
        }
        if r.since(0xf_0002) {
            r.string("PartExtractionFileSpec")?;
            list_parts_to_extract(r, "PartsToExtract")?;
        }
        if r.since(0xf_0005) && r.file_version < 0x1b0000 {
            pm_u32(r, "PartCopies")?;
        }
        if r.since(0x27_ffff) && r.file_version < 0x2b0000 {
            list_line_playback_style_list(r, "LinePlaybackStyleList")?;
            list_articulation_playback_style_list(r, "ArticulationPlaybackStyleList")?;
            list_notehead_playback_style_list(r, "NoteheadPlaybackStyleList")?;
            list_symbol_playback_style_list(r, "SymbolPlaybackStyleList")?;
        }
        if r.since(0x2d_0020) {
            r.node("CommentColourMap", "CommentColourMap", comment_colour_map)?;
        }
        if r.since(0x2f_0000) {
            r.node("RepulserSettings", "RepulserSettings", repulser_settings)?;
        }
        if r.since(0x37_ffff) {
            text_character_styles(r)?;
        }
        music_tail(r)?;
        Ok(())
    })
}

/// 8.3.0 `0x1010b694a`: chord-suffix appearances with a parallel array of
/// parenthesis options in the trailing record.
pub(super) fn suffix_appearances(r: &mut Reader) -> Result<()> {
    r.tag(0x7e)?;
    let n = r.list("Appearances", |r| {
        r.sub("Appearance", |r| {
            r.u32("Category")?;
            r.u32("AppearanceId")?;
            r.bool("Parenthesise")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    r.record("record", "Record", |r| {
        if r.since(0x36_000a) && n != 0 {
            r.list_n("ParenthesiseOptions", n, |r| r.read_u32().map(Value::U32))?;
        }
        Ok(())
    })
}

/// 8.3.0 `0x100ee9234`: MIDI routing of an instrument. Files before 0x30000
/// carry the legacy program/bank pair and MIDI flags instead of a `MIDIPatch`.
pub(super) fn midi_settings(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x3_0000 {
        r.i8("Program")?;
        r.i16("Bank")?;
    } else {
        r.node("MIDIPatch", "MidiPatch", midi_patch)?;
    }
    r.u8("Channel")?;
    r.i32("Connection")?;
    r.i8("Pan")?;
    r.u8("Balance")?;
    r.u8("Distance")?;
    if v < 0x3_0000 {
        r.u8("MIDIflags")?;
    }
    r.record("record", "Record", |r| {
        if r.since(0xa_0015) {
            r.u8("Priority")?;
        }
        if r.since(0x2a_0004) {
            r.string("SoundIDOverride")?;
        }
        if r.since(0x2c_0001) {
            r.bool("MixerStripIsOpen")?;
        }
        if r.since(0x2c_0003) {
            r.string("PreferredDevice")?;
        }
        if r.since(0x2d_0000) {
            r.string("DrumMapOverrideSoundSetName")?;
            r.string("DrumMapOverrideDrumMapName")?;
        }
        if r.since(0x2d_0008) {
            r.u32("LaneOverride")?;
        }
        if r.since(0x34_0004) {
            r.u8("ReverbLevel")?;
            r.u8("ChorusLevel")?;
            r.string("CurrentDeviceName")?;
            r.string("CurrentPatchName")?;
            list_smart_knob_values(r, "SibeliusPlayerSmartKnobValues")?;
        }
        if r.since(0x35_0003) {
            r.bool("HasSibeliusSmartKnobDataBeenOverriden")?;
        }
        Ok(())
    })
}

/// 8.3.0 `0x100d8f25c`: an image, stored either as ARGB pixels or as an
/// embedded (or linked) file.
pub(super) fn graphic(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x5c)?;
    let embedded = v >= 0x370001 && r.bool("FileEmbedded")?;
    if v < 0x370001 || (v < 0x380006 && !embedded) {
        r.list("Bits", |r| r.read_u32().map(Value::U32))?;
        r.u32("Width")?;
        r.list_n("Checksum", 4, |r| r.read_u32().map(|c| Value::I32(c as i32)))?;
    } else {
        r.string("Filename")?;
        if embedded {
            let len = r.i32("GraphicFileLength")?;
            r.bytes("GraphicFile", len.max(0) as usize)?;
        }
    }
    if v >= 0x8_0000 {
        r.f32("XPixelsPerMM")?;
        r.f32("YPixelsPerMM")?;
    }
    r.record("record", "Record", |r| {
        if r.since(0x38_0003) {
            r.bool("DimensionsKnown")?;
            r.u32("Width")?;
            r.u32("Height")?;
        }
        if r.since(0x38_0005) {
            r.bool("IsLinkedToFile")?;
            r.bool("IsRelativeFilename")?;
        }
        if r.since(0x38_0007) {
            r.list("OriginalFileComponents", |r| r.read_string())?;
        }
        Ok(())
    })
}


/// Whether `tag` starts a bar object (8.3.0 `0x100d8e29a`).
fn is_bar_object_tag(tag: u8) -> bool {
    (7..0x2e).contains(&tag) || tag & !1 == 0x92
}

/// 8.3.0 `0x100f77766`: one bar object, dispatched on its tag. Unknown tags
/// carry a varint-length record that Sibelius skips.
pub(super) fn bar_object(r: &mut Reader) -> Result<()> {
    match r.any_tag()? {
        0x07 => bar_object_07(r),
        0x08 => bar_object_08(r),
        0x09 => bar_object_09(r),
        0x0a => bar_object_0a(r),
        0x0c => bar_object_0c_16_17(r),
        0x0d => bar_object_0d(r),
        0x0e => bar_object_0e_0f(r),
        0x0f => bar_object_0e_0f(r),
        0x10 => bar_object_10(r),
        0x11 => bar_object_11(r),
        0x12 => bar_object_12(r),
        0x13 => bar_object_13(r),
        0x14 => bar_object_14(r),
        0x15 => bar_object_15(r),
        0x16 => bar_object_0c_16_17(r),
        0x17 => bar_object_0c_16_17(r),
        0x18 => bar_object_18(r),
        0x19 => bar_object_19(r),
        0x1a => bar_object_1a(r),
        0x1b => bar_object_1b(r),
        0x1c => staff_filter(r),
        0x1f => bar_object_1f(r),
        0x21 => bar_object_21(r),
        0x22 => bar_object_22(r),
        0x23 => bar_object_23(r),
        0x24 => chord_symbol(r),
        0x25 => bar_object_25(r),
        0x26 => bar_object_26(r),
        0x27 => bar_object_27(r),
        0x28 => bar_object_28(r),
        0x29 => bar_object_29(r),
        0x2a => bar_object_2a(r),
        0x2b => bar_object_2b(r),
        0x2c => bar_object_2c(r),
        0x2d => bar_object_2d(r),
        0x92 => bar_object_base(r),
        0x93 => bar_object_93(r),
        _ => r.record("unknown", "Unknown", |_| Ok(())),
    }
}

/// 8.3.0 `0x100dc260e`. Before 0x31000d the item count is not trusted and
/// objects run until the next byte is not an object tag.
pub(super) fn bar(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x05)?;
    if r.file_version < WIDE_COORDINATES {
        pm_u16(r, "GapBelowStave")?;
    } else {
        pm_u32(r, "GapBelowStave")?;
    }
    pm_u8(r, "MakesStaveVisible")?;
    r.bool("HasStaveTypeChanges")?;
    r.u8("NumDrawStateChanges")?;
    if v < 0x31000d {
        r.u32("ItemCount")?;
        let mut items = Vec::new();
        while r.peek_u8().is_ok_and(is_bar_object_tag) {
            items.push(Value::Node(r.sub("BarObject", bar_object)?));
        }
        r.push("Items", Value::List(crate::tree::List { count: crate::tree::Count::Implied, items }));
    } else {
        r.list("Items", |r| {
            let tag = r.peek_u8()?;
            r.expect(is_bar_object_tag(tag), "a bar object tag")?;
            r.sub("BarObject", bar_object).map(Value::Node)
        })?;
    }
    r.record("record", "Record", |r| {
        if r.since(0x31_000c) {
            list_live_tempo_data(r, "LiveTempoData")?;
        }
        if r.since(0x34_0003) {
            list_live_tempo_fermata_data(r, "LiveTempoFermataData")?;
        }
        if r.since(0x36_000b) {
            list_live_tempo_tap_points(r, "LiveTempoTapPoints")?;
        }
        bar_tail(r)?;
        Ok(())
    })
}

/// Playback dictionary categories, in the order of the table at 8.3.0 `0x102c3cff0`.
const KEYWORD_CATEGORIES: [&str; 19] = [
    "VolumeKeyword",
    "DeltaVolumeKeyword",
    "LengthScaleKeyword",
    "TempoKeyword",
    "DeltaTempoKeyword",
    "PatchKeyword",
    "MetronomeKeyword",
    "PressureKeyword",
    "RubatoKeyword",
    "SwingKeyword",
    "RitardKeyword",
    "NamedPatchKeyword",
    "RepeatBehaviourKeyword",
    "ControlChangeKeyword",
    "DynamicKeyword",
    "DynamicEnvelopeKeyword",
    "SoundIDChangeKeyword",
    "None",
    "ResetTempo",
];

/// 8.3.0 `0x100fc871c`: a playback-dictionary entry (tag 0x52) or a plain
/// keyword (tag 0x51).
pub(super) fn keyword(r: &mut Reader) -> Result<()> {
    match r.any_tag()? {
        0x52 => playback_keyword(r),
        0x51 => {
            keyword_base(r)?;
            r.record("record", "Record", |_| Ok(()))
        }
        _ => Ok(()),
    }
}

/// 8.3.0 `0x100e08286`. The category is stored by name; unknown names act as "None".
fn playback_keyword(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    keyword_base(r)?;
    let category = r.read_string()?;
    let index = match &category {
        Value::Str(units) => {
            let name = String::from_utf16_lossy(units);
            KEYWORD_CATEGORIES.iter().position(|c| *c == name)
        }
        _ => None,
    };
    r.push("Category", category);
    // before 0x2c0001 deltas and percentages are single bytes
    let narrow = v < 0x2c_0001;
    match index {
        Some(0) => drop(r.u8("Volume")?),
        Some(1) => {
            if narrow {
                r.i8("DeltaVolume")?;
            } else {
                r.i32("DeltaVolume")?;
            }
            r.bool("Instantaneous")?;
        }
        Some(2) => drop(r.u8("LengthScale")?),
        Some(3) => drop(r.f32("CrotchetsPerMinute")?),
        Some(4) if narrow => drop(r.u8("DeltaTempo")?),
        Some(4) => drop(r.i32("DeltaTempo")?),
        Some(5) => {
            r.u16("Bank")?;
            r.i8("Program")?;
        }
        Some(6) => drop(r.u16("BeatLength")?),
        Some(7) => drop(r.u8("Pressure")?),
        Some(8) => drop(r.u16("RubatoLength")?),
        Some(9) if v < 0x10_0000 => drop(r.u32("SwingPatternType")?),
        Some(9) if v < 0x3b_0000 => r.node("SwingPatternId", "StyleId", style_id)?,
        Some(9) => r.node("SwingSettings", "SwingSettings", swing_settings)?,
        Some(10) if narrow => {
            r.u8("TempoPercent")?;
            r.u8("Duration")?;
        }
        Some(10) => {
            r.i32("TempoPercent")?;
            r.i32("Duration")?;
        }
        Some(11) if v > 0x5_0006 => r.string("Instrument")?,
        Some(12) if v > 0x8_0013 && v < 0x8_0020 => drop(r.u8("RepeatBehaviourIndex")?),
        Some(18) if v >= 0x3a_0001 => drop(r.u32("ResetTempoReferencePoint")?),
        _ => {}
    }
    r.record("record", "Record", |r| {
        match index {
            Some(1) => {
                if r.file_version >= 0x2c_0001 {
                    r.i32("DeltaAttack")?;
                } else if r.since(0x28_0006) {
                    r.i8("DeltaAttack")?;
                }
                if r.since(0x2a_0002) {
                    r.string("SoundIDChange")?;
                }
            }
            Some(12) if r.since(0x8_001f) => drop(r.u8("RepeatBehaviourIndex")?),
            Some(13) if r.since(0x1b_0015) => {
                r.u8("Controller")?;
                r.u8("ControlValue")?;
            }
            Some(14) => {
                if r.since(0x28_0000) {
                    r.u8("DynamicLevel")?;
                    r.u8("DynamicAttack")?;
                }
                if r.since(0x2a_0002) {
                    r.string("SoundIDChange")?;
                }
            }
            Some(15) => {
                if r.since(0x28_0000) {
                    r.u8("DynamicEnvelopeBoost")?;
                    r.u8("DynamicEnvelopeDecay")?;
                }
                if r.since(0x2a_0001) {
                    r.bool("DynamicEnvelopeUseFinalDynamic")?;
                    r.u8("DynamicEnvelopeFinalDynamic")?;
                }
            }
            Some(16) => {
                if r.since(0x28_0000) {
                    r.string("SoundIDChange")?;
                }
                if r.since(0x2d_0007) {
                    r.u32("SoundIDChangeDuration")?;
                }
            }
            _ => {}
        }
        keyword_tail(r)
    })
}

/// The names an instrument instance shows and their offsets (2025.3
/// `0x1011a04b0`, shared with instrument changes and stave starts).
pub(super) fn instance_names(r: &mut Reader) -> Result<()> {
    pm_styled_string(r, "FullName")?;
    pm_styled_string(r, "ShortName")?;
    ["FullNameOffsetX", "FullNameOffsetY", "ShortNameOffsetX", "ShortNameOffsetY"]
        .into_iter()
        .try_for_each(|name| if r.file_version < WIDE_COORDINATES { pm_u16(r, name) } else { pm_u32(r, name) })
}

/// 8.3.0 `0x10102b908`: an instrument instance (names, name offsets, type id).
/// Files before 0x1f0000 describe the instrument inline instead of naming a type.
pub(super) fn instance(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x02)?;
    if v > 0x23_ffff && v < 0x2d_0000 {
        r.string("Name")?;
        r.record("record", "Record", |_| Ok(()))?;
        r.node("Id", "StyleId", style_id)?;
        r.bool("Compulsory")?;
        r.record("record", "Record", |_| Ok(()))?;
    }
    instance_names(r)?;
    if v < 0x1f_0000 {
        r.i32("Order")?;
        r.string("Group")?;
        r.u8("TopNote")?;
        r.u8("BottomNote")?;
        if v < 0x11_0000 {
            r.u8("PartClef")?;
        } else {
            r.node("PartClef", "StyleId", style_id)?;
        }
        if v > 0x3_0009 {
            r.string("GlissType")?;
        }
        if v > 5 && v < 0x1e_000a {
            r.string("DefaultInstrumentName")?;
        }
    }
    if v < 0x2d_0000 {
        r.list("Staves", |r| r.read_u32().map(Value::U32))?;
    }
    r.record("record", "Record", |r| {
        if r.since(0x24_ffff) {
            part_map(r, "InstrumentId", |r| r.sub("StyleId", style_id).map(Value::Node))
        } else if r.file_version < 0x1f_0000 {
            if r.since(0x7_0000) {
                r.u8("ComfTopNote")?;
                r.u8("ComfBottomNote")?;
            }
            Ok(())
        } else {
            r.string("InstrumentId")
        }
    })
}

/// 8.3.0 `0x100ec6028`: one note of a NoteRest. Bit 7 of `Flags` adds a tie;
/// bits 4–6 of `Flags2` select the recorded performance values.
pub(super) fn note(r: &mut Reader) -> Result<()> {
    let flags = r.u8("Flags")?;
    if flags & 0x80 != 0 {
        r.node("Tie", "Tie", tie)?;
    }
    if r.file_version < 0x8_0010 && flags & 0x40 != 0 {
        r.u8("OriginalVelocity")?;
        r.u16("OriginalDeltaSr")?;
        r.u16("OriginalDuration")?;
    }
    r.u8("Diat")?;
    r.u8("Chrom2")?;
    r.i8("Acc")?;
    r.record("record", "Record", |r| {
        let flags2 = if r.since(0x2_0001) { r.u8("Flags2")? } else { 0 };
        if r.since(0x8_000f) {
            if flags2 & 0x10 != 0 {
                r.u8("OriginalVelocity")?;
            }
            r.u16("OriginalDeltaSr")?;
            if flags2 & 0x20 != 0 {
                r.u16("OriginalDuration")?;
            }
            if flags2 & 0x40 != 0 {
                r.u16("OriginalQuantizedDuration")?;
            }
        }
        if r.since(0x9_0006) {
            r.i8("StringNum")?;
        }
        if r.since(0x3e_0000) {
            pm_u8(r, "partDiat")?;
        }
        if r.since(0x3e_0001) {
            pm_u32(r, "NoteColour")?;
        }
        note_tail(r)?;
        Ok(())
    })
}

/// Stream flag: the payload ends with Scorch options.
const FLAG_SCORCH_OPTIONS: u32 = 0x20;

/// 8.3.0 `0x1011689ec`.
pub(super) fn scorch_options(r: &mut Reader) -> Result<()> {
    if r.flags & FLAG_SCORCH_OPTIONS != 0 {
        r.tag(0x5b)?;
        r.string("Data")?;
    }
    Ok(())
}

/// 8.3.0 `0x100e877fa`: slur-like curved lines (mid points, magnetism, Béziers).
pub(super) fn curve_line(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if (0xc_0000..0x18_0000).contains(&v) {
        pm_mid_points(r, "MidPoints")?;
    } else {
        list_mid_points(r, "MidPoints")?;
    }
    if v > 0xffff {
        r.u8("SIFFlags")?;
    }
    r.record("record", "Record", |r| {
        if r.since(0x1_0000) && r.file_version < 0x19_0000 {
            r.u8("Flags")?;
        }
        if r.since(0x1_0004) {
            r.u8("StartNoteIndex")?;
            r.u8("EndNoteIndex")?;
        }
        if r.since(0x18_ffff) {
            r.bool("IsMagnetic")?;
            pm_bool(r, "MagneticSlurFlipped")?;
            r.bool("IsBend")?;
            r.bool("IsSlide")?;
            r.bool("IsSlideFlipped")?;
        }
        let segment = |r: &mut Reader, suffix: bool| -> Result<()> {
            let names = if suffix {
                ["BezierControlPtLeft2ndSegment", "BezierControlPtRight2ndSegment"]
            } else {
                ["BezierControlPtLeft", "BezierControlPtRight"]
            };
            let edited = if suffix {
                ["LeftBezierControlPtEdited2ndSegment", "RightBezierControlPtEdited2ndSegment"]
            } else {
                ["LeftBezierControlPtEdited", "RightBezierControlPtEdited"]
            };
            if r.file_version < 0x340000 {
                pm_point(r, names[0])?;
                pm_point(r, names[1])?;
            } else {
                pm_bezier_point(r, names[0])?;
                pm_bezier_point(r, names[1])?;
            }
            pm_bool(r, edited[0])?;
            pm_bool(r, edited[1])
        };
        if r.since(0x31_0005) {
            segment(r, false)?;
        }
        if r.since(0x32_0007) {
            pm_u32(r, "MiddleThickness")?;
        }
        if r.since(0x33_000d) {
            segment(r, true)?;
        }
        if r.since(0x36_000e) {
            pm_bool(r, "AutoRoute")?;
        }
        Ok(())
    })
}

/// 8.3.0 `0x100de7f24`: an element of a chord symbol. The two type bytes are
/// named through a table Sibelius fills at run time, so their names are ours.
pub(super) fn chord_element(r: &mut Reader) -> Result<()> {
    r.u8("Chrom2")?;
    r.u8("ElementType")?;
    r.record("record", "Record", |r| {
        if r.file_version == 0x2d0012 || r.file_version == 0x2d0013 {
            let n = r.i32("LegacyByteCount")?;
            r.bytes("LegacyBytes", n.max(0) as usize)?;
        }
        if r.since(0x2d_0013) && r.file_version < 0x2f0000 {
            let n = r.i32("FrameStringNotes")?;
            r.list_n("LegacyFrameStringNotes", n.max(0) as usize, |r| {
                r.sub("LegacyFrameStringNote", |r| {
                    r.u8("String")?;
                    r.u32("Fret")?;
                    r.string("Text")?;
                    Ok(())
                })
                .map(Value::Node)
            })?;
        }
        if r.since(0x2e_0000) {
            r.u8("ElementSubType")?;
        }
        if r.since(0x2e_ffff) {
            list_frame_string_notes(r, "FrameStringNotes")?;
        }
        if r.since(0x2f_0001) {
            r.node("TextForOpenSymbol", "StyledString", styled_string)?;
        }
        Ok(())
    })
}

/// 8.3.0 `0x10109770a` (virtual `Read` of the chord in a chord symbol).
pub(super) fn chord(r: &mut Reader) -> Result<()> {
    r.tag(0x7c)?;
    r.i8("Root")?;
    r.i8("Bass")?;
    r.string("ChordTypeId")?;
    r.record("record", "Record", |_| Ok(()))
}

/// 8.3.0 `0x1010b7818` (virtual `Read` of the scale in a chord symbol).
pub(super) fn scale(r: &mut Reader) -> Result<()> {
    r.tag(0x7f)?;
    r.i8("Root")?;
    r.u32("ScaleType")?;
    r.record("record", "Record", |_| Ok(()))
}

/// 8.3.0 `0x100dab5f8` (bar object 0x24): chord symbol / guitar frame.
pub(super) fn chord_symbol(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    bar_object_base(r)?;
    r.node("StaveTypeIndex", "StyleId", style_id)?;
    list_chord_elements(r, "temp")?;
    r.node("ChordName", "StyledString", styled_string)?;
    r.node("FretNumber", "StyledString", styled_string)?;
    if v < 0x31000b {
        r.bool("ShowName")?;
    }
    r.u32("LowestFret")?;
    r.bool("LowestFretResetOnDraw")?;
    if v > 0x16_ffff {
        r.node("FingeringTextStyle", "StyleId", style_id)?;
    }
    r.record("record", "Record", |r| {
        if r.since(0x1_000f) {
            r.bool("ShowFrame")?;
        }
        if r.since(0x9_0005) && r.file_version < 0x31_0000 {
            r.u32("DescriptionRoot")?;
            r.u32("DescriptionType")?;
            r.u32("DescriptionBass")?;
        }
        if r.since(0xa_0012) {
            r.bool("CustomChordName")?;
        }
        if r.since(0xa_001a) {
            r.string("Fingerings")?;
            if r.file_version < 0x31_000c {
                r.bool("ShowFingerings")?;
            }
        }
        if r.since(0x2d_001e) {
            r.bool("Horizontal")?;
            if r.file_version < 0x31000b {
                r.bool("ScaleMode")?;
            }
            r.u32("NumFrets")?;
        }
        if r.since(0x2e_0001) {
            r.u32("EnforceLowestFret")?;
        }
        if r.since(0x2e_0002) {
            pm_f32(r, "ScaleFactor")?;
        }
        if r.file_version < 0x310000 {
            return Ok(());
        }
        let is_scale = r.since(0x31_000a) && r.bool("IsScale")?;
        if is_scale {
            return r.node("Scale", "Scale", scale);
        }
        r.node("Chord", "Chord", chord)?;
        if r.file_version < 0x310001 {
            r.u32("ChordSymbolElements")?;
        } else {
            pm_u32(r, "ChordSymbolElements")?;
        }
        pm_i8(r, "RootSpelling")?;
        pm_i8(r, "BassSpelling")?;
        pm_bool(r, "SpellAutomatically")?;
        if r.since(0x31_000d) {
            r.node("GuitarFrameOverrides", "GuitarFrameOverrides", guitar_frame_overrides)?;
        }
        if r.since(0x32_0001) {
            r.node("GuitarFrameText", "GuitarFrameText", guitar_frame_text)?;
        }
        if r.since(0x32_0003) {
            r.bool("bImportProblem")?;
        }
        if r.since(0x33_0000) {
            r.node("GuitarFrameTextFormatting", "GuitarFrameTextFormatting", guitar_frame_text_formatting)?;
        }
        if r.since(0x33_0002) {
            r.bool("InPurgatory")?;
        }
        if r.since(0x33_000a) {
            r.bool("HasDummyFrame")?;
        }
        if r.since(0x36_0012) {
            pm_bool(r, "RootAndChordTypeImplied")?;
        }
        if r.since(0x36_0017) {
            r.bool("ShowExplicitNaturalOnAlteredBassNote")?;
        }
        Ok(())
    })
}

/// 8.3.0 `0x100f6104e`: Kontakt sounds; every sound carries `NumParams` values.
pub(super) fn kontakt_custom_sounds_body(r: &mut Reader) -> Result<()> {
    let params = r.i32("NumParams")?.max(0) as usize;
    r.string("CollectionName")?;
    let sounds = r.i32("NumSounds")?.max(0) as usize;
    r.list_n("Sounds", sounds, |r| {
        r.sub("KontaktCustomSound", |r| {
            r.tag(0x62)?;
            r.i32("SoundNum")?;
            r.list_n("Params", params, |r| {
                r.sub("Param", |r| {
                    r.bool("Changed")?;
                    r.u32("Value")?;
                    Ok(())
                })
                .map(Value::Node)
            })?;
            r.record("record", "Record", |_| Ok(()))
        })
        .map(Value::Node)
    })?;
    r.record("record", "Record", |_| Ok(()))
}

/// 8.3.0 `0x1011605b2`: the house-style part of an Idea clip. Clips are read
/// in "styles only" mode, which skips score options and part appearance.
pub(super) fn clip_styles(r: &mut Reader) -> Result<()> {
    r.tag(0x00)?;
    let styles = r.u32("StylesVersion")?;
    if styles > 4 {
        r.bool("FromDynamicPart")?;
    }
    r.node("Music", "ClipMusic", |r| clip_music(r, styles))
}

/// 8.3.0 `0x100fca458` in "styles only" mode; `styles` is the clip's own
/// `StylesVersion`, not the container version.
fn clip_music(r: &mut Reader, styles: u32) -> Result<()> {
    r.tag(0x01)?;
    if styles > 7 {
        list_line_playback_style_list(r, "LinePlaybackStyleList")?;
        list_notehead_playback_style_list(r, "NoteheadPlaybackStyleList")?;
        list_symbol_playback_style_list(r, "SymbolPlaybackStyleList")?;
    }
    text_styles(r)?;
    indexed_symbols(r)?;
    clef_styles(r)?;
    line_styles(r)?;
    indexed_note_styles(r)?;
    stave_types(r)?;
    if styles > 5 {
        instrument_types(r)?;
        ensembles(r)?;
        if styles > 6 && styles < 9 {
            r.node("InstrumentInstanceList", "InstrumentInstanceList", list_instrument_instances)?;
        }
    }
    if styles > 0xb {
        text_character_styles(r)?;
    }
    Ok(())
}

/// 8.3.0 `0x101192555`: four words identifying the score a clip came from.
pub(super) fn score_unique_id(r: &mut Reader) -> Result<()> {
    r.list_n("Words", 4, |r| r.read_u32().map(Value::U32))
}

/// 8.3.0 `0x100dd108c`: the object a selection holds. Sibelius peeks at the
/// tag and lets the object's reader consume it; other tags leave it empty.
pub(super) fn item(r: &mut Reader) -> Result<()> {
    match r.peek_u8()? {
        0x04 => r.node("Stave", "Stave", stave),
        0x37 => r.node("BarSpacing", "BarSpacing", bar_spacing),
        0x38 => r.node("Spacing", "Spacing", spacing),
        0x07..=0x0a | 0x0c..=0x1b | 0x1f | 0x21..=0x2d | 0x93 => bar_object(r),
        _ => Ok(()),
    }
}

/// 8.3.0 `0x10118b93c`: an Idea or clipboard selection. Clips name their
/// styles instead of indexing them (`ScoreInStream` flag at +0x132).
pub fn clip(r: &mut Reader) -> Result<()> {
    r.tag(0x6a)?;
    r.node("ScoreUniqueID", "ScoreUniqueId", score_unique_id)?;
    r.string_refs = true;
    if r.bool("IsRange")? {
        r.node("RangeDescription", "RangeDescription", range_description)?;
        r.list("OriginalStaveNums", |r| r.read_u32().map(Value::U32))?;
    }
    // the clipboard context moved into the trailing record in 0x2a0000
    if r.file_version < 0x2a_0000 {
        r.node("ClipboardContext", "ClipboardContext", clipboard_context)?;
    }
    clip_styles(r)?;
    f_1010180a1(r)?;
    r.list("Selections", |r| r.sub("Selection", selection).map(Value::Node))?;
    r.record("record", "Record", |r| {
        if r.since(0x29_ffff) {
            r.node("ClipboardContext", "ClipboardContext", clipboard_context)?;
        }
        Ok(())
    })
}

/// 8.3.0 `0x100fd796b`: the parts to extract. From 0x42 each part takes a
/// second word, as part keys do (2025.3 `0x1013407c0`).
pub(super) fn list_parts_to_extract(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("Part", |r| {
            r.u32("value")?;
            if r.file_version > 0x41_ffff {
                r.u32("word2")?;
            }
            Ok(())
        })
        .map(Value::Node)
    })
    .map(drop)
}
