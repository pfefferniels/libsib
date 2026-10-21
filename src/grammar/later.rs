//! Fields that releases after 8.3.0 added, read as Sibelius 2025.3 reads them
//! (addresses cite that build). Nearly all sit at the ends of records, so they
//! hook into the 8.3.0 grammar there. Field names are the element names 2025.3
//! passes to `SIS_Enter`; the few it leaves unnamed are marked as such.

use super::manual::{bar_object, instance_names};
use super::prim::*;
use super::readers::{music_line, style_id, tie};
use crate::error::Result;
use crate::reader::Reader;
use crate::tree::Value;

/// End of a tie's record (2025.3 `0x10127f980`): the line it is drawn with
/// and, behind the feature flags `EnableTieStyles` and `EnableLVTies`, dash
/// and dot settings. A writer with a flag off leaves its fields out.
pub(super) fn tie_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x3e_0007) {
        r.node("LineStyleId", "StyleId", style_id)?;
        r.string("GlissType")?;
        r.string("GlissCurve")?;
    }
    if r.since(0x3e_0009) {
        pm_u8(r, "TextDisplayState")?;
    }
    if r.since(0x3e_000a) {
        pm_u32(r, "RhDy")?;
    }
    if r.since(0x3f_0008) && r.more() {
        pm_u8(r, "Style")?;
    }
    if r.since(0x3f_0009) && r.more() {
        ["DashLength", "DashGapLength", "DotRadius", "DotGapLength"]
            .into_iter()
            .try_for_each(|name| pm_u32(r, name))?;
    }
    if r.since(0x3f_ffff) && r.more() {
        pm_bool(r, "LvTie")?;
    }
    Ok(())
}

/// End of a note's record (2025.3 `0x101195f10`), after `NoteColour`.
pub(super) fn note_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x3f_0007) {
        r.bool("TieIntoNoteBool")?;
        r.node("TieIntoNote", "Tie", tie)?;
    }
    if r.since(0x42_0000) {
        pm_bool(r, "Bracketed")?;
    }
    if r.since(0x42_0001) {
        hidden(r)?;
    }
    Ok(())
}

/// 2025.3 `0x1013df110`.
fn hidden(r: &mut Reader) -> Result<()> {
    r.bool("HideInScore")?;
    pm_bool(r, "HideInPart")
}

/// End of a bar's record (2025.3 `0x10101f660`): whether the stave gap is
/// automatic and, from 0x430006, the bar rest Sibelius shows when a bar is empty.
pub(super) fn bar_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x3f_0007) {
        pm_bool(r, "AutoStaveGap")?;
    }
    if r.since(0x43_0005) {
        r.node("FakeBarRest", "BarObject", bar_object)?;
    }
    Ok(())
}

/// End of a spacing entry's record (tag 0x38, 2025.3 `0x1010744f0`).
pub(super) fn spacing_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x43_0004) {
        r.bool("UseThisSpacingForLayout")?;
    }
    Ok(())
}

/// End of a stave type's record (2025.3 `0x101253b50`).
pub(super) fn stave_type_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x3e_0005) {
        r.i32("StaveScaleFactor")?;
    }
    Ok(())
}

/// End of a text style's record (2025.3 `0x10118e900`).
pub(super) fn text_style_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x40_0002) {
        r.bool("RepeatUntilSectionEnd")?;
        r.bool("IsRepeatUntilSectionEndSet")?;
    }
    Ok(())
}

/// End of a stave's record (2025.3 `0x10126c000`).
pub(super) fn stave_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x3e_0005) {
        pm_u32(r, "ScaleFactor")?;
    }
    if r.since(0x3e_0006) {
        part_map(r, "ScaleFactorChanges", scale_factor_changes)?;
    }
    if r.since(0x40_ffff) {
        r.list("VStaveNums", u32)?;
    }
    if r.since(0x44_0000) {
        r.u32("NextTrackFilterID")?;
    }
    Ok(())
}

/// 2025.3 `0x10126d040`: changes of a stave's scale, a u16 and a u32 each
/// (both unnamed in the binary).
fn scale_factor_changes(r: &mut Reader) -> Result<Value> {
    r.sub("ScaleFactorChanges", |r| {
        r.list("Changes", |r| {
            r.sub("Change", |r| {
                r.u16("Position")?;
                r.u32("Factor").map(drop)
            })
            .map(Value::Node)
        })
        .map(drop)
    })
    .map(Value::Node)
}

/// End of the playing options' record (2025.3 `0x101202120`).
pub(super) fn playing_options_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x43_0001) {
        r.bool("PlayBackHiddenNotes")?;
    }
    Ok(())
}

/// End of an instrument type's record (2025.3 `0x1012a88d0`). The instrument
/// change names, dropped at 0x3a0000, return in 0x3f0002.
pub(super) fn instrument_type_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x3f_0001) {
        r.string("InstrumentChangeName")?;
        r.string("InstrumentChangeWarningName")?;
        r.string("NativeInstrumentChangeName")?;
        r.string("NativeInstrumentChangeWarningName")?;
    }
    if r.since(0x40_ffff) {
        r.bool("IsDynamic")?;
    }
    Ok(())
}

/// End of an instrument change's record (tag 0x27, 2025.3 `0x1013d2610`): the
/// names shown after the change, as an object of tag 0x96 (2025.3 `0x101293040`).
pub(super) fn instrument_change_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x41_0000) {
        r.node("InstrumentNames", "InstrumentNames", |r| {
            r.tag(0x96)?;
            instance_names(r)
        })?;
    }
    Ok(())
}

/// End of a selection's record (2025.3 `0x1013621d0`).
pub(super) fn paste_selection_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x3f_0005) {
        r.u8("OriginalMiddleLinePitch")?;
    }
    if r.since(0x45_0001) {
        r.u32("BarPauseType")?;
    }
    Ok(())
}

/// End of a stave start's record (tag 0x6c, 2025.3 `0x1013704f0`).
pub(super) fn stave_start_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x40_ffff) {
        r.node("InstrumentInstance", "InstrumentNames", |r| {
            r.tag(0x02)?;
            instance_names(r)
        })?;
    }
    if r.since(0x42_0002) {
        r.u32("SourcePartID")?;
        if r.since(0x41_ffff) {
            r.u32("SourcePartIDWord2")?;
        }
    }
    Ok(())
}

/// End of a playback keyword's record (2025.3 `0x101086a20`).
pub(super) fn keyword_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x45_0000) {
        r.bool("StaffFilterDirectiveFlag")?;
        r.u32("StaffFilterChange")?;
    }
    Ok(())
}

/// End of the music's record (2025.3 `0x1013370f0`).
pub(super) fn music_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x3f_0002) {
        r.bool("LockedByUser")?;
    }
    Ok(())
}

/// The fields a stave listed in `VStaveNums` carries after its body
/// (2025.3 `0x1014913d0`).
pub(super) fn vstave(r: &mut Reader) -> Result<()> {
    r.u32("VStaveID")?;
    r.u32("SourceStaveNum").map(drop)
}

/// End of the options' record (2025.3 `0x10109a420`), after `RestsContinue`.
pub(super) fn options_tail(r: &mut Reader) -> Result<()> {
    if r.since(0x3e_0005) {
        pm_u8(r, "MediumStaveScale")?;
        pm_u8(r, "XSmallStaveScale")?;
    }
    bools(
        r,
        &[
            (0x3f_0003, "ViewOptions.m_ViewReplayMarker"),
            (0x3f_0004, "JoinBarlinesAtEndOfSystems"),
            (0x3f_0006, "AutoOptimize"),
        ],
    )?;
    if r.since(0x3f_000a) {
        ["TieDashLength", "TieDashGapLength", "TieDotRadius", "TieDotGapLength"]
            .into_iter()
            .try_for_each(|name| r.u32(name).map(drop))?;
    }
    if r.since(0x3f_ffff) {
        r.u32("LvTieLength")?;
    }
    bools(
        r,
        &[
            (0x40_0000, "RespaceAfterUpdatingCautionaryAccidentals"),
            (0x40_0001, "PanoramaViewDisablesFocusOnStaves"),
        ],
    )?;
    if r.since(0x41_ffff) {
        pm_bool(r, "DoubleThickForCoincidentRepeats")?;
        pm_bool(r, "RepeatDotsInAllSpaces")?;
        r.u32("DottedBarlineGap")?;
        r.u32("DashedBarlineGap")?;
        r.u32("DashedBarlineLength")?;
    }
    bools(
        r,
        &[
            (0x42_0003, "AllowHiddenNotesToUpdateAccidentals"),
            (0x43_0003, "BarRestOrMultiRestWhenAllNotesHiddenInBar"),
            (0x43_0006, "UseProperSlideOverBarlineCalculation"),
        ],
    )?;
    if r.since(0x44_0001) {
        pm_bool(r, "ShowBarRests")?;
    }
    bools(r, &[(0x44_0002, "ViewOptions.m_ViewTrackFilters")])
}

/// Single bool fields, each added after the version it is paired with.
fn bools(r: &mut Reader, fields: &[(u32, &'static str)]) -> Result<()> {
    fields
        .iter()
        .try_for_each(|&(v, name)| if r.since(v) { r.bool(name).map(drop) } else { Ok(()) })
}

/// A staff filter (bar object of tag 0x1c, 2025.3 `0x1013e9ab0`): a line
/// whose span shows only some notes of a stave.
pub(super) fn staff_filter(r: &mut Reader) -> Result<()> {
    music_line(r)?;
    r.u32("FilterID")?;
    part_map(r, "FilterInfo", filter_info)?;
    r.u32("PartID")?;
    if r.file_version > 0x41_ffff {
        r.u32("PartIDWord2")?;
    }
    r.u32("VStaveID")?;
    pm_u32(r, "FillColor")?;
    pm_string(r, "Description")?;
    pm_bool(r, "Maximized")?;
    pm_bool(r, "Activated")?;
    r.record("record", "Record", |_| Ok(()))
}

/// 2025.3 `0x1013ee9a0`.
fn filter_info(r: &mut Reader) -> Result<Value> {
    r.sub("FilterInfo", |r| {
        r.u8("HiddenVoices")?;
        ["SingleNotes", "Rests", "Player1", "Player2"]
            .into_iter()
            .try_for_each(|name| r.bool(name).map(drop))?;
        r.u8("NotePositionBits")?;
        r.u8("ActiveCategoryBits")?;
        r.record("record", "Record", |_| Ok(()))
    })
    .map(Value::Node)
}
