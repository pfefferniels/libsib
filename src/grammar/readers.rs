//! Readers generated from the decompiled `ScoreInStream` readers of Sibelius
//! 8.3.0. Each cites the address of the function it was generated from.

#![allow(unused_imports)]

use super::later::*;
use super::manual::*;
use super::prim::*;
use crate::error::Result;
use crate::reader::Reader;
use crate::tree::Value;

/// 8.3.0 `0x10115c274`.
pub fn score(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x00)?;
    r.node("ScoreOptions", "ScoreOptions", score_options)?;
    pm_part_batch_print_settings(r, "PrintSettings")?;
    if v > 0x1fffff && v < 0x210000 {
        r.tag(0x67)?;
        r.u32("IdeaCount")?;
    }
    r.node("Music", "Music", music)?;
    r.record("record", "Record", |r| {
        if r.since(0x110003) {
            r.node("DefaultPartAppearance", "DefaultPartAppearance", default_part_appearance)?;
        }
        if r.since(0x1a000a) {
            r.node("PartBatchPrintSettings", "PartBatchPrintSettings", part_batch_print_settings)?;
        }
        if r.since(0x2d000e) {
            r.node("FileID", "FileId", file_id)?;
        }
        if r.since(0x380006) {
            r.string("SavedFilePath")?;
        }
        if r.since(0x380007) {
            f_100d926be(r, "SavedFilePathComponents")?;
        }
        Ok(())
    })?;
    r.node("ScorchOptions", "ScorchOptions", scorch_options)?;
    Ok(())
}

/// 8.3.0 `0x100f55060`.
pub(super) fn stave(r: &mut Reader) -> Result<()> {
    r.tag(0x04)?;
    f_100f557ac(r)?;
    r.list("Bars", |r| {
        r.sub("BarsItem", |r| {
            bar(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    r.record("record", "Record", |r| {
        if r.since(0xa0008) {
            pm_bool(r, "ShowInFocusMode")?;
        }
        if r.since(0xa0018) {
            if r.file_version < 0x1e0000 {
                pm_u16(r, "VelocityBarYOffset")?;
            } else {
                pm_u32(r, "VelocityBarYOffset")?;
            }
        }
        if r.since(0x1e0004) {
            pm_bool(r, "DrawBarNumber")?;
        }
        if r.since(0x1f0000) {
            pm_u32(r, "PanoramicViewGapBelow")?;
        }
        stave_tail(r)?;
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100d9236e`.
pub(super) fn list_graphic_data(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListGraphicDataItem", |r| {
            graphic_cc3a(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100fd752c`.
pub(super) fn list_instrument_groups(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListInstrumentGroupsItem", |r| {
            f_100f3c746(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100fdb626`.
pub(super) fn pm_barlines(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("ListBarlines", |r| list_barlines(r, "items")).map(Value::Node))
}

/// 8.3.0 `0x100fda982`.
pub(super) fn pm_brackets(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("ListBrackets", |r| list_brackets(r, "items")).map(Value::Node))
}

/// 8.3.0 `0x100f61006`.
pub(super) fn kontakt_custom_sounds(r: &mut Reader) -> Result<()> {
    r.tag(0x61)?;
    kontakt_custom_sounds_body(r)?;
    Ok(())
}

/// 8.3.0 `0x100ed244a`.
pub(super) fn sync_options(r: &mut Reader) -> Result<()> {
    r.tag(0x63)?;
    f_100ed2492(r)?;
    Ok(())
}

/// 8.3.0 `0x100fd76db`.
pub(super) fn pm_part_staves(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("F100fda178", |r| f_100fda178(r, "items")).map(Value::Node))
}

/// 8.3.0 `0x10108f578`.
pub(super) fn comment_colour_map(r: &mut Reader) -> Result<()> {
    r.tag(0x78)?;
    f_10108f39a(r)?;
    Ok(())
}

/// 8.3.0 `0x100a16666`.
pub(super) fn repulser_settings(r: &mut Reader) -> Result<()> {
    r.tag(0x7a)?;
    f_100a0dc7a(r)?;
    f_100a166f6(r)?;
    r.bool("bIsDodging")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x101002608`.
pub(super) fn text_character_styles(r: &mut Reader) -> Result<()> {
    list_text_character_styles(r, "TextCharacterStyles")?;
    Ok(())
}

/// 8.3.0 `0x100fd70fa`.
pub(super) fn list_custom_chord_names(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListCustomChordNamesItem", |r| {
            f_100d98922(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100eeb450`.
pub(super) fn list_smart_knob_values(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListSmartKnobValuesItem", |r| {
            r.u8("value")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100dcd768`.
pub(super) fn list_live_tempo_data(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListLiveTempoDataItem", |r| {
            f_1010bacea(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100dcd7e6`.
pub(super) fn list_live_tempo_fermata_data(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListLiveTempoFermataDataItem", |r| {
            f_1010baff2(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100dcd882`.
pub(super) fn list_live_tempo_tap_points(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListLiveTempoTapPointsItem", |r| {
            r.u16("value")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100dd7c66`.
pub(super) fn bar_object_07(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    bar_object_base(r)?;
    if v < 0xe0000 {
        r.node("ClefStyle", "F100fa77fa", f_100fa77fa)?;
    } else {
        r.node("ClefStyle", "F100dd8004", f_100dd8004)?;
    }
    r.u8("Flags")?;
    r.record("record", "Record", |r| {
        if r.since(0xdffff) {
            pm_bool(r, "Enabled")?;
        }
        if r.since(0x280003) {
            r.node("TransposingClefStyle", "F100dd8004", f_100dd8004)?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100dff154`.
pub(super) fn bar_object_08(r: &mut Reader) -> Result<()> {
    bar_object_base(r)?;
    f_100fb4506(r)?;
    r.u8("BarlineType")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100efbdf6`.
pub(super) fn bar_object_09(r: &mut Reader) -> Result<()> {
    bar_object_base(r)?;
    r.u8("Top")?;
    r.u8("LogUnitLength")?;
    r.i16("BeatGroups")?;
    r.i16("SymbolNum")?;
    r.string("TimeSignatureString")?;
    r.u8("Flags")?;
    r.record("record", "Record", |r| {
        if r.since(0x80020) {
            f_100df0824(r)?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100fae31e`.
pub(super) fn bar_object_0a(r: &mut Reader) -> Result<()> {
    bar_object_base(r)?;
    r.i8("Sharps")?;
    r.u8("Flags")?;
    r.record("record", "Record", |r| {
        if r.since(0x50007) {
            r.bool("OneStaffOnly")?;
        }
        if r.since(0x1b0014) {
            pm_bool(r, "Major")?;
            pm_bool(r, "Enabled")?;
        }
        if r.since(0x1b0014) {
            pm_i8(r, "PartMapSharps")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100f4c106`.
pub(super) fn bar_object_0c_16_17(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    bar_object_base(r)?;
    r.u32("Duration")?;
    if v < 0x1e0000 {
        pm_u16(r, "RhDx")?;
    } else {
        pm_u32(r, "RhDx")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "RhDy")?;
    } else {
        pm_u32(r, "RhDy")?;
    }
    r.node("LineStyle", "F100fa7a6a", f_100fa7a6a)?;
    r.record("record", "Record", |r| {
        if r.since(0x5ffff) {
            pm_point(r, "value")?;
            pm_point(r, "value")?;
        }
        if r.since(0x240003) {
            r.bool("SnapToBarline")?;
        }
        if r.since(0x330006) {
            pm_bool(r, "AreSegmentYsStaveRelative")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100dd4fce`.
pub(super) fn bar_object_0d(r: &mut Reader) -> Result<()> {
    bar_object_0c_16_17(r)?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100da52a2`.
pub(super) fn bar_object_0e_0f(r: &mut Reader) -> Result<()> {
    bar_object_0c_16_17(r)?;
    r.u8("FinalVolume")?;
    r.record("record", "Record", |r| {
        if r.since(0x50002) {
            r.bool("IsDelta")?;
            r.i8("DeltaVolume")?;
        }
        if r.since(0x240000) {
            pm_u32(r, "ClosedEndAperture")?;
            pm_u32(r, "SmallContinuationAperture")?;
            pm_u32(r, "LargeContinuationAperture")?;
            pm_u32(r, "OpenEndAperture")?;
        }
        if r.since(0x2a0000) {
            r.bool("AutoHairpin")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100dd188a`.
pub(super) fn bar_object_10(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    bar_object_0c_16_17(r)?;
    if v > 0x30009 {
        r.node("Type", "GlissType", gliss_type)?;
        r.node("InstrumentType", "GlissType", gliss_type)?;
        r.node("Curve", "Curve", curve)?;
    }
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100f08c30`.
pub(super) fn bar_object_11(r: &mut Reader) -> Result<()> {
    bar_object_0c_16_17(r)?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100ed3e90`.
pub(super) fn bar_object_12(r: &mut Reader) -> Result<()> {
    bar_object_0c_16_17(r)?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100fa0ea0`.
pub(super) fn bar_object_13(r: &mut Reader) -> Result<()> {
    f_100f6b138(r)?;
    r.record("record", "Record", |r| {
        if r.since(0x80015) {
            r.bool("IsLastTimeEnding")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100e872a8`.
pub(super) fn bar_object_14(r: &mut Reader) -> Result<()> {
    bar_object_0c_16_17(r)?;
    curve_line(r)?;
    Ok(())
}

/// 8.3.0 `0x100f47dd4`.
pub(super) fn bar_object_15(r: &mut Reader) -> Result<()> {
    bar_object_0c_16_17(r)?;
    r.i8("DeltaQuarterTones")?;
    r.i16("TrillsPerSecond")?;
    r.record("record", "Record", |r| {
        if r.since(0x20003) {
            r.bool("PlayStraight")?;
        }
        if r.since(0x30000) {
            r.bool("NoteAbove")?;
        }
        if r.since(0x2a0003) {
            r.bool("Diatonic")?;
        }
        if r.since(0x3d000a) {
            r.u32("TrillType")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100f2cfcc`.
pub(super) fn bar_object_18(r: &mut Reader) -> Result<()> {
    bar_object_0c_16_17(r)?;
    r.node("TextRef", "StyledString", styled_string)?;
    r.u16("Unit")?;
    r.u8("Left")?;
    r.u8("Right")?;
    r.u8("Flags")?;
    r.u16("TupletLength")?;
    r.record("record", "Record", |r| {
        if r.since(0x1b0014) {
            pm_bool(r, "TupletFlipped")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100fc3584`.
pub(super) fn bar_object_19(r: &mut Reader) -> Result<()> {
    bar_object_0c_16_17(r)?;
    r.node("Text", "StyledString", styled_string)?;
    r.u8("SyllableType")?;
    r.u32("NumNotes")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100da0a02`.
pub(super) fn bar_object_1a(r: &mut Reader) -> Result<()> {
    bar_object_0c_16_17(r)?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100e06ac8`.
pub(super) fn bar_object_1b(r: &mut Reader) -> Result<()> {
    f_100f6b138(r)?;
    r.bool("IsRitard")?;
    let using_percent = r.bool("UsingPercent")?;
    if !using_percent {
        r.i32("FinalTempoAbsolute")?;
    } else {
        r.i32("FinalTempoPercent")?;
    }
    r.node("Curve", "Curve698d", curve_698d)?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100ecd044`.
pub(super) fn bar_object_1f(r: &mut Reader) -> Result<()> {
    bar_object_base(r)?;
    r.node("Symbol", "Symbol", symbol)?;
    r.u8("Size")?;
    r.record("record", "Record", |r| {
        if r.since(0x1b001b) {
            r.bool("GraceNoteSize")?;
        }
        if r.since(0x390019) {
            r.string("PlaybackInsertSilenceParams")?;
            r.string("PlaybackShortenNotesParams")?;
            if r.since(0x3d000b) {
                let symbol_item_properties_type = r.u32("SymbolItemPropertiesType")?;
                if symbol_item_properties_type != 0 {
                    r.string("SymbolItemPropertiesDataString")?;
                }
            }
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100f51372`.
pub(super) fn bar_object_21(r: &mut Reader) -> Result<()> {
    bar_object_22(r)?;
    f_100fb4506(r)?;
    r.record("record", "Record", |r| {
        if r.since(0x80018) {
            r.bool("RepeatTextBindsToBarEnd")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100ed8b82`.
pub(super) fn bar_object_22(r: &mut Reader) -> Result<()> {
    bar_object_base(r)?;
    r.node("StyledString", "StyledString", styled_string)?;
    r.bool("DrawAtStartOfSystem")?;
    r.record("record", "Record", |r| {
        if r.since(0x38000e) {
            r.bool("AngleOverriden")?;
            r.f32("Angle")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100ddb4a4`.
pub(super) fn bar_object_23(r: &mut Reader) -> Result<()> {
    bar_object_base(r)?;
    r.u16("Number")?;
    r.bool("Shown")?;
    r.record("record", "Record", |r| {
        if r.since(0x1b001c) {
            r.u16("BNAlphaElementOffset")?;
            r.u32("BNFormat")?;
            r.u32("PrefixSuffix")?;
            r.string("NumberTextPrefixOrSuffix")?;
            r.u32("BarNumberChangeOperation")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100f3b1bc`.
pub(super) fn bar_object_25(r: &mut Reader) -> Result<()> {
    bar_object_base(r)?;
    r.u16("PartDiat")?;
    r.u16("PartChrom2")?;
    r.u16("ScoreDiat")?;
    r.u16("ScoreChrom2")?;
    r.record("record", "Record", |r| {
        if r.since(0x1b0014) {
            pm_bool(r, "Enabled")?;
        }
        if r.since(0x1b0014) {
            pm_u16(r, "TransposingDiat")?;
            pm_u16(r, "TransposingChrom2")?;
            pm_u16(r, "ConcertDiat")?;
            pm_u16(r, "ConcertChrom2")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100effef8`.
pub(super) fn bar_object_26(r: &mut Reader) -> Result<()> {
    bar_object_base(r)?;
    f_100fb4506(r)?;
    r.u32("Mark")?;
    r.bool("Consecutive")?;
    r.record("record", "Record", |r| {
        if r.since(0x240002) {
            r.bool("NewPrefixAndSuffix")?;
            r.string("Prefix")?;
            r.string("Suffix")?;
            if r.since(0x240003) {
                r.bool("CombineDefaults")?;
            }
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x101041774`.
pub(super) fn bar_object_27(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    bar_object_base(r)?;
    if v < 0x260000 {
        if v < 0xe0000 {
            r.node("StaveType", "F100fa7c28", f_100fa7c28)?;
        } else {
            r.node("StaveType", "F101041ca4", f_101041ca4)?;
        }
    }
    r.record("record", "Record", |r| {
        if r.since(0xdffff) {
            pm_bool(r, "Enabled")?;
        }
        if r.file_version < 0x2d0000 {
            if r.since(0x23ffff) {
                r.node("InstrumentInstance", "F100fa4a02", f_100fa4a02)?;
            }
        } else {
            r.node("Instance", "Instance", instance)?;
        }
        if r.since(0x240001) {
            pm_styled_string(r, "InstrumentText")?;
            pm_u32(r, "InstrumentTextDx")?;
            pm_u32(r, "InstrumentTextDy")?;
            r.node("InstrumentTextVisibility", "F10104b1a5", f_10104b1a5)?;
        }
        instrument_change_tail(r)?;
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100f146be`.
pub(super) fn bar_object_28(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    bar_object_base(r)?;
    r.i16("Symbol")?;
    r.u16("Duration")?;
    if v < 0xb0000 {
        r.u8("Flags")?;
    }
    r.record("record", "Record", |r| {
        if r.since(0xaffff) {
            pm_bool(r, "MakesStaveVisible")?;
            if r.file_version < 0x1b001c || r.since(0x1b001e) {
                pm_bool(r, "CueSize")?;
            }
            r.bool("CannotBeInMultirest")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100eab40a`.
pub(super) fn bar_object_29(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    bar_object_base(r)?;
    r.u8("Durn")?;
    r.u8("Trem")?;
    if v < 0xb0000 {
        r.u8("Flags")?;
    }
    r.i16("Artic")?;
    f_100ee0c8e(r)?;
    r.record("record", "Record", |r| {
        if r.since(0x3000f) {
            if r.file_version < 0xb0000 {
                r.u8("Flags2")?;
            }
            pm_i8(r, "ArticDy")?;
        }
        if r.since(0x30011) {
            pm_i8(r, "DotDx")?;
        }
        if r.since(0xaffff) {
            r.u32("CrossStaveType")?;
            r.bool("PreBend")?;
            r.bool("ForcedStem")?;
            pm_bool(r, "FlipStem")?;
            r.bool("SlashThroughStem")?;
            r.bool("GraceNoteSize")?;
            if r.file_version < 0x1b001c || r.since(0x1b001e) {
                pm_bool(r, "CueNoteSize")?;
            }
            pm_bool(r, "FlipArticulation")?;
            r.bool("HideBeam")?;
            r.bool("FlipSlides")?;
        }
        if r.since(0x1a0003) {
            r.bool("FlipFractionalBeam")?;
        }
        if r.since(0x1a0010) {
            r.bool("ZStem")?;
        }
        if r.since(0x1b0012) {
            r.bool("PlayTremolo")?;
        }
        if r.since(0x2d0019) {
            r.bool("BeamTypeSticky")?;
        }
        if r.since(0x2d0022) {
            r.u32("Scoop")?;
            r.u32("Fall")?;
        }
        if r.since(0x2d0023) {
            pm_u32(r, "StemletDy")?;
        }
        if r.since(0x2d0024) {
            r.u32("Arpeggio")?;
        }
        if r.since(0x2d0026) {
            if r.file_version < 0x350000 {
                r.bool("ArpeggioHidden")?;
            } else {
                r.node("ArpeggioVisibility", "F10104b1a5", f_10104b1a5)?;
            }
            pm_u32(r, "ArpeggioTopDy")?;
            pm_u32(r, "ArpeggioBottomDy")?;
        }
        if r.since(0x2d0028) {
            r.u32("Stemlet")?;
        }
        if r.since(0x2d0029) {
            r.u32("BeamFeather")?;
        }
        if r.since(0x310002) {
            pm_i8(r, "ArticBelowDy")?;
        }
        if r.since(0x310007) {
            list_individual_artic_dys(r, "IndividualArticDys")?;
        }
        if r.since(0x310008) {
            pm_i8(r, "ScoopDx")?;
            pm_i8(r, "FallDx")?;
        }
        if r.since(0x330012) {
            r.bool("FermataOverride")?;
            r.u16("FermataPauseFraction")?;
            r.u16("FermataGapFraction")?;
        }
        if r.since(0x350005) {
            pm_u32(r, "RestPosition")?;
        }
        if r.since(0x350007) {
            pm_u32(r, "ArpeggioDx")?;
        }
        if r.since(0x360014) {
            pm_f_100eb87e1(r, "ArticRepulsionState")?;
        }
        if r.since(0x3d0001) {
            r.u32("ArhythmicNotePlaybackMode")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100d98f18`.
pub(super) fn bar_object_2a(r: &mut Reader) -> Result<()> {
    bar_object_base(r)?;
    f_100fa4d2c(r)?;
    pm_f32(r, "XScale")?;
    pm_f32(r, "YScale")?;
    r.record("record", "Record", |r| {
        if r.since(0x370003) {
            pm_f32(r, "Rotation")?;
        }
        if r.since(0x380000) {
            pm_rect(r, "CropRect")?;
        }
        if r.since(0x380001) {
            pm_bool(r, "FlipX")?;
            pm_bool(r, "FlipY")?;
        }
        if r.since(0x380002) {
            pm_u32(r, "Brightness")?;
            pm_u32(r, "Contrast")?;
            pm_u32(r, "Saturation")?;
            pm_u32(r, "Alpha")?;
        }
        if r.since(0x380008) {
            pm_u32(r, "Width")?;
            pm_u32(r, "Height")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100f13666`.
pub(super) fn bar_object_2b(r: &mut Reader) -> Result<()> {
    bar_object_1f(r)?;
    f_100fb4506(r)?;
    r.record("record", "Record", |r| {
        if r.since(0x390016) {
            pm_bool(r, "DrawOnAllStaves")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x10108a66e`.
pub(super) fn bar_object_2c(r: &mut Reader) -> Result<()> {
    bar_object_22(r)?;
    f_10108a828(r)?;
    Ok(())
}

/// 8.3.0 `0x1010eb0d4`.
pub(super) fn bar_object_2d(r: &mut Reader) -> Result<()> {
    bar_object_base(r)?;
    r.record("record", "Record", |r| {
        if r.since(0xdffff) {
            pm_bool(r, "Enabled")?;
        }
        if r.since(0x240001) {
            pm_styled_string(r, "PlayerTextTop")?;
            pm_u32(r, "PlayerTextDxTop")?;
            pm_u32(r, "PlayerTextDyTop")?;
            r.node("PlayerTextVisibilityTop", "F10104b1a5", f_10104b1a5)?;
            pm_styled_string(r, "PlayerTextBottom")?;
            pm_u32(r, "PlayerTextDxBottom")?;
            pm_u32(r, "PlayerTextDyBottom")?;
            r.node("PlayerTextVisibilityBottom", "F10104b1a5", f_10104b1a5)?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100f9a9e6`.
pub(super) fn bar_object_base(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.u16("Sr")?;
    if v < 0x1e0000 {
        pm_u16(r, "Dx")?;
    } else {
        pm_u32(r, "Dx")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "Dy")?;
    } else {
        pm_u32(r, "Dy")?;
    }
    r.u8("Voices")?;
    r.record("record", "Record", |r| {
        if r.since(0x20009) && r.file_version < 0xb0000 {
            r.u8("ItemFlags")?;
        }
        if r.since(0x80012) {
            pm_u32(r, "Colour")?;
        }
        if r.since(0x80015) {
            r.u8("PlayTimes")?;
        }
        if r.since(0xaffff) {
            f_10104b084(r)?;
        }
        if r.since(0x1b001b) {
            pm_bool(r, "CueSize")?;
        }
        if r.since(0x300003) {
            pm_u32(r, "Repulsive")?;
        }
        if r.since(0x300006) {
            pm_u32(r, "DrawLayer")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x101039b36`.
pub(super) fn bar_object_93(r: &mut Reader) -> Result<()> {
    bar_object_base(r)?;
    r.list("Strokes", |r| {
        r.sub("StrokesItem", |r| {
            f_1010398fe(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100fad4ce`.
pub(super) fn keyword_base(r: &mut Reader) -> Result<()> {
    r.string("String")?;
    r.bool("EndOfWord")?;
    r.record("record", "Record", |r| {
        if r.since(0x80013) {
            r.bool("RegExp")?;
        }
        if r.since(0x80016) {
            r.bool("CaseSens")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x1010e7014`.
pub(super) fn swing_settings(r: &mut Reader) -> Result<()> {
    r.tag(0x8c)?;
    f_1010e5668(r)?;
    Ok(())
}

/// 8.3.0 `0x100f5bb52`.
pub(super) fn tie(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x4c)?;
    if v < 0x1a0000 {
        if v < 0xf0000 {
            r.i8("LhDx")?;
            r.i8("RhDx")?;
        } else {
            if v < 0x1e0000 {
                r.u16("LhDx")?;
            } else {
                r.u32("LhDx")?;
            }
            if v < 0x1e0000 {
                r.u16("RhDx")?;
            } else {
                r.u32("RhDx")?;
            }
        }
        r.i8("Dy")?;
        r.i8("MiddleDy")?;
        r.record("record", "Record", |r| {
            if r.since(0x20001) {
                r.u8("TieFlags")?;
            }
            if r.since(0x170001) {
                r.u32("Shoulder")?;
            }
            tie_tail(r)?;
            Ok(())
        })?;
    } else {
        if v < 0x1e0000 {
            pm_u16(r, "LhDx")?;
        } else {
            pm_u32(r, "LhDx")?;
        }
        if v < 0x1e0000 {
            pm_u16(r, "RhDx")?;
        } else {
            pm_u32(r, "RhDx")?;
        }
        pm_i8(r, "Dy")?;
        pm_i8(r, "MiddleDy")?;
        r.record("record", "Record", |r| {
            pm_u32(r, "Shoulder")?;
            r.bool("IsSlide")?;
            pm_bool(r, "Flipped")?;
            if r.since(0x330004) {
                pm_u32(r, "FirstSegmentEndXOffset")?;
                pm_u32(r, "SecondSegmentStartXOffset")?;
            }
            if r.since(0x330009) {
                pm_i8(r, "SecondSegmentMiddleDy")?;
            }
            tie_tail(r)?;
            Ok(())
        })?;
    }
    Ok(())
}

/// 8.3.0 `0x100e93841`.
pub(super) fn list_mid_points(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListMidPointsItem", |r| {
            f_100e116e0(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100e93ae8`.
pub(super) fn pm_bezier_point(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("BezierPoint", bezier_point).map(Value::Node))
}

/// 8.3.0 `0x100de929a`.
pub(super) fn list_frame_string_notes(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListFrameStringNotesItem", |r| {
            f_100de7d34(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100dbe709`.
pub(super) fn list_chord_elements(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListChordElementsItem", |r| {
            f_100de8414(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x1010bc56e`.
pub(super) fn guitar_frame_overrides(r: &mut Reader) -> Result<()> {
    r.tag(0x81)?;
    f_1010bc2d8(r)?;
    Ok(())
}

/// 8.3.0 `0x1010bf5ac`.
pub(super) fn guitar_frame_text(r: &mut Reader) -> Result<()> {
    r.tag(0x82)?;
    f_1010bf3ee(r)?;
    Ok(())
}

/// 8.3.0 `0x1010c593a`.
pub(super) fn guitar_frame_text_formatting(r: &mut Reader) -> Result<()> {
    r.tag(0x83)?;
    f_1010c5756(r)?;
    Ok(())
}

/// 8.3.0 `0x101199a06`.
pub fn version_info(r: &mut Reader) -> Result<()> {
    r.tag(0x76)?;
    r.string("VersionName")?;
    r.u64("StartDate")?;
    r.u64("EndDate")?;
    r.list("NumCommentThreads", |r| {
        r.sub("NumCommentThreadsItem", |r| {
            r.list("NumCommentsInThread", |r| {
                r.sub("NumCommentsInThreadItem", |r| {
                    f_101198d12(r)?;
                    Ok(())
                })
                .map(Value::Node)
            })?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x101174abe`.
pub fn idea_info(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x67)?;
    r.string("Name")?;
    if v < 0x230000 {
        r.i32("DateCreated")?;
        r.i32("DateModified")?;
    } else {
        r.u64("DateCreated")?;
        r.u64("DateModified")?;
    }
    list_keywords(r, "Keywords")?;
    r.record("record", "Record", |r| {
        if r.since(0x210001) {
            r.u32("BackgroundColour")?;
            r.i8("Sharps")?;
            r.bool("Major")?;
            r.f32("TempoBPM")?;
            r.u16("NumBars")?;
            r.string("FullInstrumentNames")?;
            r.string("ShortInstrumentNames")?;
            r.string("TimeSignatureString")?;
        }
        if r.since(0x210003) {
            r.node("Id", "ScoreUniqueId", score_unique_id)?;
        }
        if r.since(0x28ffff) {
            r.bool("SerialisedClipFileContainsHeaders")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100fd7194`.
pub(super) fn list_instrument_instances(r: &mut Reader) -> Result<()> {
    r.list("items", |r| {
        r.sub("ItemsItem", |r| {
            instance(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100eff398`.
pub(super) fn range_description(r: &mut Reader) -> Result<()> {
    r.u32("StartStave")?;
    r.u16("StartBar")?;
    r.u16("StartSr")?;
    r.u32("EndStave")?;
    r.u16("EndBar")?;
    r.u16("EndSr")?;
    r.u32("VerticalRange")?;
    r.u32("HorizontalRange")?;
    r.u32("SafeLength")?;
    r.list("StaveNumbers", |r| {
        r.sub("StaveNumbersItem", |r| {
            r.u32("value")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100ff2fbc`.
pub(super) fn clipboard_context(r: &mut Reader) -> Result<()> {
    r.tag(0x6b)?;
    r.list("StaveContexts", |r| {
        r.sub("StaveContextsItem", |r| {
            f_100ff3b70(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100fe99ea`.
pub(super) fn selection(r: &mut Reader) -> Result<()> {
    r.tag(0x47)?;
    r.u32("BarStart")?;
    r.u16("BarLength")?;
    r.u32("Track")?;
    r.u16("BarNum")?;
    r.node("FragmentList", "FragmentList", fragment_list)?;
    r.u8("OnTablature")?;
    r.u8("OnPercussion")?;
    r.u8("ChangedSr")?;
    r.u8("Top")?;
    r.u8("LogUnitLength")?;
    r.i16("BeatGroups")?;
    r.u8("InTuplet")?;
    r.node("Item", "Item", item)?;
    r.record("record", "Record", |r| {
        if r.since(0x90003) {
            f_100df0824(r)?;
        }
        if r.since(0xa0003) {
            r.u32("OriginalStaveNum")?;
            r.u32("OriginalBarNum")?;
        }
        if r.since(0x2d0009) {
            r.bool("OnBlankPage")?;
        }
        selection_tail(r)?;
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x1010180a1`.
pub(super) fn f_1010180a1(r: &mut Reader) -> Result<()> {
    list_graphic_data(r, "GraphicDataList")?;
    Ok(())
}

/// 8.3.0 `0x100f1bcca`.
pub(super) fn bar_spacing(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x37)?;
    r.u16("BarNum")?;
    if v < 0x1e0000 {
        pm_u16(r, "LhsIndent")?;
    } else {
        pm_u32(r, "LhsIndent")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "RhsIndent")?;
    } else {
        pm_u32(r, "RhsIndent")?;
    }
    if v < 0xb0000 {
        r.u8("Flags")?;
    }
    if v < 0xb0000 {
        r.u8("DrawFlags")?;
    }
    pm_list_part_map_default(r, "Spacings")?;
    pm_u16(r, "NumBarRestExcepItems")?;
    r.record("record", "Record", |r| {
        if r.since(0x3000b) {
            if r.file_version < 0x1e0000 {
                pm_u16(r, "BarNumDx")?;
            } else {
                pm_u32(r, "BarNumDx")?;
            }
            if r.file_version < 0x1e0000 {
                pm_u16(r, "BarNumDy")?;
            } else {
                pm_u32(r, "BarNumDy")?;
            }
        }
        if r.since(0xa001e) {
            if r.file_version < 0x1e0000 {
                pm_u16(r, "ExtraGapAtStartOfSystem")?;
            } else {
                pm_u32(r, "ExtraGapAtStartOfSystem")?;
            }
        }
        if r.since(0xaffff) {
            pm_u32(r, "BreakType")?;
            pm_bool(r, "SectionEnd")?;
            r.u32("PauseType")?;
            pm_bool(r, "DrawBracket")?;
            pm_bool(r, "DrawTimeSig")?;
            pm_bool(r, "DrawKeySig")?;
            pm_bool(r, "DrawClef")?;
            pm_bool(r, "DrawSystemicBarline")?;
            pm_bool(r, "DoesNotEndMultirest")?;
        }
        if r.since(0x1a0008) {
            pm_bool(r, "SuppressAutoLayout")?;
        }
        if r.since(0x1b001d) {
            pm_bool(r, "DrawBarNumber")?;
        }
        if r.since(0x1e0007) {
            r.node("SpecialPageBreakData", "SpecialPageBreakData", special_page_break_data)?;
        }
        if r.since(0x220000) {
            pm_u32(r, "BreakTypeAtStart")?;
        }
        if r.since(0x2d002b) {
            pm_u32(r, "MinimumSrZeroGapAtStartOfSystem")?;
        }
        if r.since(0x300006) {
            pm_u32(r, "Repulsive")?;
            pm_u32(r, "DrawLayer")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100e934fa`.
pub(super) fn pm_mid_points(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("ListMidPoints", |r| list_mid_points(r, "items")).map(Value::Node))
}

/// 8.3.0 `0x101131e76`.
pub(super) fn score_options(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x40)?;
    r.u32("OriginalProgramVersion")?;
    r.f32("JustifyTracking")?;
    pm_f32(r, "MinHeightToJustify")?;
    r.f32("StaveJustifyRatio")?;
    if v < 0x1e0000 {
        pm_u16(r, "DefaultStaveGap")?;
    } else {
        pm_u32(r, "DefaultStaveGap")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "DefaultSystemGap")?;
    } else {
        pm_u32(r, "DefaultSystemGap")?;
    }
    if v < 0x210000 {
        if v < 0x1e0000 {
            pm_u16(r, "AboveTopStaveGap")?;
        } else {
            pm_u32(r, "AboveTopStaveGap")?;
        }
        if v < 0x1e0000 {
            pm_u16(r, "BelowBottomStaveGap")?;
        } else {
            pm_u32(r, "BelowBottomStaveGap")?;
        }
    }
    pm_u32(r, "NameAtStart")?;
    pm_u32(r, "NameSubsequently")?;
    pm_u32(r, "NameAtNewSection")?;
    if v < 0x270000 {
        pm_u32(r, "FirstPageNumber")?;
        pm_bool(r, "DrawPageNumbers")?;
        pm_bool(r, "PageNumbersOnFirstPage")?;
    }
    pm_bool(r, "ShowMultiRests")?;
    r.record("record", "Record", |r| {
        r.bool("GlobalSnap")?;
        if r.since(0x50004) {
            pm_u32(r, "SaveFirstPageXOffset")?;
            pm_u32(r, "SaveFirstPageYOffset")?;
            pm_u32(r, "SaveScoreZoomType")?;
            pm_f32(r, "SaveZoomFactor")?;
        }
        if r.since(0x80019) {
            pm_bool(r, "UseAutoPageBreaks")?;
            pm_bool(r, "AutoPageBreaksEveryPage")?;
            pm_u32(r, "AutoPageBreaksMinBarRests")?;
            pm_f32(r, "AutoPageBreaksMinPageFull")?;
            pm_bool(r, "BadAutoPageBreaksUseSymbol")?;
            pm_i16(r, "BadAutoPageBreaksSymbol")?;
            pm_string(r, "BadAutoPageBreaksText")?;
            pm_bool(r, "PrintBadAutoPageBreaks")?;
        }
        if r.since(0x90001) {
            r.bool("BeamsOverRestsAtStartEndOfGroup")?;
            r.bool("BeamsOverRestsInBetweenNotesInGroup")?;
            r.bool("BeamTupletsToAdjacentNotes")?;
        }
        if r.since(0x110000) {
            pm_bool(r, "AutoPageBreaksAtFinalBarlines")?;
            pm_bool(r, "AutoPageBreaksAtBarRests")?;
            pm_bool(r, "PreferLongerRestBeforeAutoPageBreak")?;
        }
        if r.since(0x110001) {
            pm_bool(r, "AutoSplitMultiRests")?;
            pm_u32(r, "MultiRestSplittingRule")?;
            pm_u16(r, "MultiRestSplittingGroupSize")?;
            pm_u16(r, "MultiRestSplittingMultiple")?;
        }
        if r.since(0x130009) {
            pm_bool(r, "UseAutoSystemBreaks")?;
            pm_u32(r, "AutoSystemBreakRule")?;
            pm_u16(r, "RegularAutoSystemBreakGroupSize")?;
            pm_bool(r, "AutoSystemBreaksAtRehearsalMarks")?;
            pm_bool(r, "AutoSystemBreaksAtTempoText")?;
            pm_bool(r, "AutoSystemBreaksAtDoubleBarlines")?;
            pm_bool(r, "AutoSystemBreaksAtKeyChanges")?;
            pm_bool(r, "AutoSystemBreaksAtMultiRests")?;
            pm_u16(r, "MinAutoSystemBreakMultiRestSize")?;
            pm_f32(r, "MinFullSystemForAutoSystemBreaks")?;
        }
        if r.since(0x13000a) {
            pm_f32(r, "ContractNoteSpacingsFactor")?;
            pm_f32(r, "ContractSystemSpacingsFactor")?;
        }
        if r.since(0x140000) {
            pm_bool(r, "UseTacetMultiRests")?;
        }
        if r.since(0x1a0012) {
            if r.file_version < 0x1e0000 {
                if r.file_version < 0x1e0000 {
                    pm_u16(r, "ExtraSpaceForTenBarMultiRest")?;
                } else {
                    pm_u32(r, "ExtraSpaceForTenBarMultiRest")?;
                }
            } else {
                pm_u32(r, "ExtraSpaceForTenBarMultiRest")?;
            }
        }
        if r.since(0x1affff) {
            pm_u32(r, "PartCopies")?;
        }
        if r.since(0x1e0008) {
            pm_f32(r, "SavePanoramicZoomFactor")?;
            pm_u32(r, "SavePanoramicScoreZoomType")?;
            pm_u32(r, "SavePanoramicFirstPageXOffset")?;
            pm_u32(r, "SavePanoramicFirstPageYOffset")?;
        }
        if r.since(0x200002) && r.file_version < 0x210000 {
            if r.file_version < 0x1e0000 {
                pm_u16(r, "AboveTopStaveGapAfterFirstPage")?;
            } else {
                pm_u32(r, "AboveTopStaveGapAfterFirstPage")?;
            }
            if r.file_version < 0x1e0000 {
                pm_u16(r, "BelowBottomStaveGapAfterFirstPage")?;
            } else {
                pm_u32(r, "BelowBottomStaveGapAfterFirstPage")?;
            }
            pm_bool(r, "UseValuesAfterFirstPage")?;
        }
        if r.since(0x250000) {
            r.bool("ShowIdeasFromThisScoreOnly")?;
        }
        if r.since(0x260001) {
            pm_f_101193236(r, "InitialPageNumberChange")?;
        }
        if r.since(0x280001) {
            pm_bool(r, "UseScoreOtherPageNumberChangesInParts")?;
        }
        if r.since(0x2d0006) {
            pm_bool(r, "HideDefaultPageNumbersInParts")?;
        }
        if r.since(0x2d000c) {
            pm_bool(r, "UseScoreTitlePageNumberChangesInParts")?;
        }
        if r.since(0x2d0016) {
            pm_u32(r, "PageArrangementType")?;
        }
        if r.since(0x300000) {
            pm_bool(r, "JustifyGrandStaffStaves")?;
            pm_bool(r, "JustifyMultiStaffStaves")?;
            pm_u32(r, "DefaultGapBetweenGroups")?;
            pm_u32(r, "DefaultGapBelowVocalStaves")?;
            pm_u32(r, "DefaultGapAboveForSystemObjects")?;
        }
        if r.since(0x300004) {
            pm_u32(r, "MagneticStaveGapX")?;
            pm_u32(r, "MagneticStaveGapY")?;
        }
        if r.since(0x360018) {
            r.bool("BreakAtLastUnlockedBarWhenSystemOverspills")?;
        }
        if r.since(0x36001b) {
            pm_bool(r, "ApplyAutoSystemBreakToLockedFormatAsSoonAsPossible")?;
        }
        if r.since(0x390014) {
            r.string("FileNameSpecForPDFExport")?;
        }
        if r.since(0x3d000d) {
            r.bool("TimelineExpandLandmarkLanes")?;
            r.f32("TimelineZoomFactor")?;
            r.u32("TimelineScrollPosition")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x101162a48`.
pub(super) fn pm_part_batch_print_settings(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("PartBatchPrintSettings", part_batch_print_settings).map(Value::Node))
}

/// 8.3.0 `0x100d926be`.
pub(super) fn f_100d926be(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("F100d926beItem", |r| {
            r.string("value")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x10112d30e`.
pub(super) fn default_part_appearance(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x66)?;
    r.f32("PageWmm")?;
    r.f32("PageHmm")?;
    r.f32("StaveHeightmm")?;
    if v < 0x1e0000 {
        r.u16("LhsIndentOfFirstBar")?;
    } else {
        r.u32("LhsIndentOfFirstBar")?;
    }
    if v < 0x1e0000 {
        r.u16("GapAboveFirstSystem")?;
    } else {
        r.u32("GapAboveFirstSystem")?;
    }
    r.f32("MinHeightToJustify")?;
    r.node("MultiRestStyle", "MultiRestStyle", multi_rest_style)?;
    r.bool("OneBarMultirests")?;
    r.node("BarNumberFreq", "BarNumberFreq", bar_number_freq)?;
    r.bool("CentreBarNumbers")?;
    r.record("record", "Record", |r| {
        if r.since(0x130000) {
            r.bool("UseScorePageBreaksInPart")?;
            r.bool("PageBreaksBecomeSystemBreaksInPart")?;
            r.bool("UseScoreSystemBreaksInPart")?;
            r.bool("UseOtherScoreBreaksInPart")?;
        }
        if r.since(0x130003) {
            r.bool("KeepGapsBeforeCodasInPart")?;
        }
        if r.since(0x130007) {
            r.bool("Landscape")?;
            r.bool("OmitNoLinesStaveTypeChangesInPart")?;
            r.bool("OmitClefsInPart")?;
            r.node("PartClefOmitRule", "PartClefOmitRule", part_clef_omit_rule)?;
            r.bool("ShowPartNameOnFirstPage")?;
            r.bool("ShowPartNameOnSubsequentPages")?;
        }
        if r.since(0x140001) {
            r.u32("TimeCodeDisplay")?;
            r.u32("TimeSignatureStyle")?;
        }
        if r.since(0x1a000d) {
            r.node("BarNumberTextStyleId", "StyleId", style_id)?;
        }
        if r.since(0x1a0011) {
            r.bool("ShowHitPoints")?;
        }
        if r.since(0x1b000c) {
            if r.file_version < 0x1e0000 {
                r.u16("DefaultSystemGap")?;
            } else {
                r.u32("DefaultSystemGap")?;
            }
        }
        if r.since(0x1e0002) {
            r.u16("BarNumberNumericFrequency")?;
        }
        if r.since(0x1e0003) {
            r.bool("OmitBarNumbersAtRehearsalMarks")?;
        }
        if r.since(0x210004) {
            r.bool("UseScoreTitlePagesInPart")?;
            r.node("ScoreSpecialPageBreaksInPart", "ScoreSpecialPageBreaksInPart", score_special_page_breaks_in_part)?;
        }
        if r.since(0x280001) {
            r.u32("FirstPageNumber")?;
            r.bool("UseScoreOtherPageNumberChanges")?;
        }
        if r.since(0x2d0006) {
            r.bool("HideDefaultPageNumberInPart")?;
        }
        if r.since(0x2d000c) {
            r.bool("UseScoreTitlePageNumberChanges")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x101192686`.
pub(super) fn file_id(r: &mut Reader) -> Result<()> {
    r.string("value")?;
    Ok(())
}

/// 8.3.0 `0x100f557ac`.
pub(super) fn f_100f557ac(r: &mut Reader) -> Result<()> {
    pm_bool(r, "Small")?;
    r.node("MIDIData", "MididataE36a", mididata_e36a)?;
    pm_bool(r, "TopOfSection")?;
    Ok(())
}

/// 8.3.0 `0x1010dcc3a`.
pub(super) fn graphic_cc3a(r: &mut Reader) -> Result<()> {
    graphic(r)?;
    Ok(())
}

/// 8.3.0 `0x100f3c746`.
pub(super) fn f_100f3c746(r: &mut Reader) -> Result<()> {
    r.any_tag()?;
    r.list("Staves", |r| {
        r.sub("StavesItem", |r| {
            f_100fa6f66(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    r.record("record", "Record", |r| {
        if r.since(0x3d0006) {
            r.u32("Condensing")?;
        }
        if r.since(0x3d0007) {
            r.bool("HideInFullScore")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100fd75b1`.
pub(super) fn list_barlines(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListBarlinesItem", |r| {
            f_100f3c746(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100fd7641`.
pub(super) fn list_brackets(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListBracketsItem", |r| {
            f_100debf42(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100ed2492`.
pub(super) fn f_100ed2492(r: &mut Reader) -> Result<()> {
    r.string("VideoFileName")?;
    r.node("HitPoints", "HitPoints", hit_points)?;
    r.record("record", "Record", |r| {
        if r.since(0xc0002) {
            r.u64("ScoreStartTime")?;
            r.u64("VideoStartTime")?;
            r.u64("VideoStartOffset")?;
        }
        if r.since(0xd0001) {
            r.u64("CurrentTime")?;
        }
        if r.since(0xe0001) {
            r.u32("VideoVolumePercent")?;
        }
        if r.since(0x1a0007) {
            r.bool("HitPointShowTimeCode")?;
            r.bool("HitPointShowLocation")?;
            r.bool("HitPointShowLabel")?;
        }
        if r.since(0x320004) {
            r.u32("RewireTimeMode")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100fda178`.
pub(super) fn f_100fda178(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("F100fda178Item", |r| {
            r.u32("value")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x10108f39a`.
pub(super) fn f_10108f39a(r: &mut Reader) -> Result<()> {
    r.list("NumMappings", |r| {
        r.sub("NumMappingsItem", |r| {
            r.string("UserName")?;
            r.u32("Colour")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100a0dc7a`.
pub(super) fn f_100a0dc7a(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    f_100a16e70(r, "XTypes")?;
    f_100a16e70(r, "YTypes")?;
    if v > 0x2fffff {
        r.record("record", "Record", |r| {
            f_100a16e70(r, "KnownTypes")?;
            Ok(())
        })?;
    }
    Ok(())
}

/// 8.3.0 `0x100a166f6`.
pub(super) fn f_100a166f6(r: &mut Reader) -> Result<()> {
    f_100a16e70(r, "Types")?;
    list_traits(r, "Traits")?;
    Ok(())
}

/// 8.3.0 `0x1010093b6`.
pub(super) fn list_text_character_styles(r: &mut Reader, name: &'static str) -> Result<()> {
    if r.file_version <= 0xfffff {
        // the count is stored without items
        r.u32(name)?;
        return Ok(());
    }
    r.list(name, |r| {
        r.sub("ListTextCharacterStylesItem", |r| {
            f_1010da9fe(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100d98922`.
pub(super) fn f_100d98922(r: &mut Reader) -> Result<()> {
    r.tag(0x60)?;
    r.u32("Type")?;
    r.string("Name")?;
    Ok(())
}

/// 8.3.0 `0x1010bacea`.
pub(super) fn f_1010bacea(r: &mut Reader) -> Result<()> {
    r.tag(0x80)?;
    r.u16("Sr")?;
    r.u16("Duration")?;
    r.u32("BarRepeatCount")?;
    r.f32("TempoAbsoluteCpm")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x1010baff2`.
pub(super) fn f_1010baff2(r: &mut Reader) -> Result<()> {
    r.tag(0x85)?;
    r.u16("ScorePositionSr")?;
    r.u64("PauseFraction")?;
    r.u64("GapAfterFraction")?;
    r.i32("RepeatPass")?;
    r.record("record", "Record", |r| {
        if r.since(0x390018) {
            r.u32("Type")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100fa77fa`.
pub(super) fn f_100fa77fa(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x100000 {
        old_clef_style_ref(r)?;
    } else {
        style_id(r)?;
    }
    Ok(())
}

/// 8.3.0 `0x100dd8004`.
pub(super) fn f_100dd8004(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x100000 {
        if !r.string_refs {
            pm_u8(r, "value")?;
        } else {
            pm_string(r, "value")?;
        }
    } else {
        pm_style_id(r, "value")?;
    }
    Ok(())
}

/// 8.3.0 `0x100fb4506`.
pub(super) fn f_100fb4506(r: &mut Reader) -> Result<()> {
    f_100f6d77a(r, "YPositions")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100df0824`.
pub(super) fn f_100df0824(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.string("QuaverGroups")?;
    r.string("SemiQuaverGroups")?;
    r.string("SemiQuaverSubGroups")?;
    r.string("DemiSemiQuaverGroups")?;
    r.string("DemiSemiQuaverSubGroups")?;
    if v < 0x90001 && v != 0x80022 && v > 0x8001e {
        r.bool("AllowBeamsOverRests")?;
    }
    r.bool("UseSingleGroupWithinTuplets")?;
    r.bool("SeparateTupletsFromAdjacentNotes")?;
    r.u32("TimeSigTop")?;
    r.u32("TimeSigBottom")?;
    f_100df35ce(r, "QuaverBeamEventList")?;
    f_100df35ce(r, "SemiQuaverBeamEventList")?;
    f_100df35ce(r, "DemiSemiQuaverBeamEventList")?;
    r.u16("SrAfterEndOfBar")?;
    r.record("record", "Record", |r| {
        if r.since(0x90000) && r.file_version < 0x2d0019 {
            r.bool("AllowRestBeamAtGroupStartEnd")?;
            r.bool("AllowRestBeamOverGroupMiddle")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100fa7a6a`.
pub(super) fn f_100fa7a6a(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x100000 {
        old_style_ref_828c(r)?;
    } else {
        style_id(r)?;
    }
    Ok(())
}

/// 8.3.0 `0x100dd16d9`.
pub(super) fn gliss_type(r: &mut Reader) -> Result<()> {
    r.string("value")?;
    Ok(())
}

/// 8.3.0 `0x100dd173d`.
pub(super) fn curve(r: &mut Reader) -> Result<()> {
    r.string("value")?;
    Ok(())
}

/// 8.3.0 `0x100f6b138`.
pub(super) fn f_100f6b138(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    bar_object_0c_16_17(r)?;
    f_100fb4506(r)?;
    if v < 0x390000 {
        pm_f_100f6d77a(r, "RhYPositions")?;
    } else {
        f_100f6d77a(r, "RhYPositions")?;
    }
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100e5a966`.
pub(super) fn styled_string(r: &mut Reader) -> Result<()> {
    r.tag(0x20)?;
    r.string("Text")?;
    r.node("TextStyle", "F100fa75d2", f_100fa75d2)?;
    r.record("record", "Record", |r| {
        if r.since(0x2d001b) {
            r.u32("FixedWidth")?;
            r.u32("FixedHeight")?;
        }
        if r.since(0x390001) {
            r.u8("Leading")?;
            r.bool("IsLeadingSet")?;
        }
        if r.since(0x390002) {
            r.bool("EraseBackground")?;
            r.bool("IsEraseBackgroundSet")?;
            r.u32("BackgroundColour")?;
        }
        if r.since(0x390004) {
            r.u32("HorizontalAlignment")?;
            r.bool("IsHorizontalAlignmentSet")?;
            r.u32("VerticalAlignment")?;
        }
        if r.since(0x39000b) {
            r.bool("IsVerticalAlignmentSet")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100e0698d`.
pub(super) fn curve_698d(r: &mut Reader) -> Result<()> {
    r.string("value")?;
    Ok(())
}

/// 8.3.0 `0x100e5a589`.
pub(super) fn symbol(r: &mut Reader) -> Result<()> {
    r.i16("value")?;
    Ok(())
}

/// 8.3.0 `0x100fa7c28`.
pub(super) fn f_100fa7c28(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x150000 {
        old_style_ref_804a(r)?;
    } else {
        style_id(r)?;
    }
    Ok(())
}

/// 8.3.0 `0x100fa75d2`.
pub(super) fn f_100fa75d2(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x100000 {
        old_text_style_ref(r)?;
    } else {
        style_id(r)?;
    }
    Ok(())
}

/// 8.3.0 `0x1010ecf48`.
pub(super) fn pm_styled_string(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("StyledString", styled_string).map(Value::Node))
}

/// 8.3.0 `0x101041ca4`.
pub(super) fn f_101041ca4(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x150000 {
        if !r.string_refs {
            pm_i32(r, "value")?;
        } else {
            pm_string(r, "value")?;
        }
    } else {
        pm_style_id(r, "value")?;
    }
    Ok(())
}

/// 8.3.0 `0x100fa4a02`.
pub(super) fn f_100fa4a02(r: &mut Reader) -> Result<()> {
    style_id(r)?;
    Ok(())
}

/// 8.3.0 `0x10104b1a5`.
pub(super) fn f_10104b1a5(r: &mut Reader) -> Result<()> {
    r.tag(0x70)?;
    f_10104b084(r)?;
    Ok(())
}

/// 8.3.0 `0x100ee0c8e`.
pub(super) fn f_100ee0c8e(r: &mut Reader) -> Result<()> {
    r.list("Notes", |r| {
        r.sub("NotesItem", |r| {
            r.tag(0x4b)?;
            note(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100eb9d0a`.
pub(super) fn list_individual_artic_dys(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListIndividualArticDysItem", |r| {
            f_100eb8756(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100eb9db0`.
pub(super) fn pm_f_100eb87e1(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("F100eb87e1", f_100eb87e1).map(Value::Node))
}

/// 8.3.0 `0x100fa4d2c`.
pub(super) fn f_100fa4d2c(r: &mut Reader) -> Result<()> {
    r.i32("GraphicData")?;
    Ok(())
}

/// 8.3.0 `0x10108a828`.
pub(super) fn f_10108a828(r: &mut Reader) -> Result<()> {
    r.u64("Date")?;
    r.string("UserName")?;
    r.record("record", "Record", |r| {
        if r.since(0x330005) {
            pm_bool(r, "Maximised")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x10104b084`.
pub(super) fn f_10104b084(r: &mut Reader) -> Result<()> {
    r.bool("HideInScore")?;
    pm_bool(r, "HideInPart")?;
    Ok(())
}

/// 8.3.0 `0x1010398fe`.
pub(super) fn f_1010398fe(r: &mut Reader) -> Result<()> {
    r.tag(0x94)?;
    r.list("Points", |r| {
        r.sub("PointsItem", |r| {
            f_1010396cc(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x1010e5668`.
pub(super) fn f_1010e5668(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.u32("SwingType")?;
    r.node("LegacySwingStyle", "StyleId", style_id)?;
    r.u16("RhythmicOffsetSrs")?;
    r.u32("SwingBeatSubdivision")?;
    if v < 0x3c0000 {
        r.bool("ApplyToShorterNoteDurations")?;
    } else {
        r.u32("ShorterNoteDurationBehaviour")?;
    }
    r.f32("FirstSubdivisionSwingRatio")?;
    r.f32("SecondSubdivisionSwingRatio")?;
    r.f32("ThirdSubdivisionSwingRatio")?;
    r.f32("FirstSubdivisionLevelEmphasis")?;
    r.f32("SecondSubdivisionLevelEmphasis")?;
    r.f32("ThirdSubdivisionLevelEmphasis")?;
    r.bool("IsTripleTimeSwing")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100e116e0`.
pub(super) fn f_100e116e0(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x44)?;
    if v < 0x180000 {
        r.u8("X")?;
        r.u16("Y")?;
    } else {
        pm_u8(r, "X")?;
        pm_u16(r, "Y")?;
    }
    r.record("record", "Record", |r| {
        if r.since(0x36000f) {
            pm_u8(r, "XSecondSegment")?;
            pm_u16(r, "YSecondSegment")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100e92f56`.
pub(super) fn bezier_point(r: &mut Reader) -> Result<()> {
    r.tag(0x84)?;
    r.u32("X")?;
    r.u32("Y")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100de7d34`.
pub(super) fn f_100de7d34(r: &mut Reader) -> Result<()> {
    r.tag(0x79)?;
    r.u8("Pitch")?;
    r.u32("Symbol")?;
    r.node("Text", "StyledString", styled_string)?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100de8414`.
pub(super) fn f_100de8414(r: &mut Reader) -> Result<()> {
    r.tag(0x5a)?;
    chord_element(r)?;
    Ok(())
}

/// 8.3.0 `0x1010bc2d8`.
pub(super) fn f_1010bc2d8(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x81)?;
    r.node("ChordSuffixOverrides", "SuffixSettings", suffix_settings)?;
    r.node("ChordSuffixExtensionArrangement", "ChordSuffixExtensionArrangement", chord_suffix_extension_arrangement)?;
    if v > 0x320001 {
        r.u32("ChordRootAccidentalPosition")?;
    }
    if v > 0x33000e {
        r.node("OrderOfAlterations", "OrderOfAlterations", order_of_alterations)?;
    }
    r.record("record", "Record", |r| {
        if r.since(0x360004) {
            r.u32("ExtensionNumberPosition")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x1010bf3ee`.
pub(super) fn f_1010bf3ee(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x82)?;
    r.string("Text")?;
    r.bool("Recognised")?;
    if v >= 0x320003 {
        r.bool("IsLegacyChord")?;
    }
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x1010c5756`.
pub(super) fn f_1010c5756(r: &mut Reader) -> Result<()> {
    r.tag(0x83)?;
    pm_u32(r, "Size")?;
    pm_u32(r, "Bold")?;
    pm_u32(r, "Italic")?;
    pm_u32(r, "Underline")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x101198d12`.
pub(super) fn f_101198d12(r: &mut Reader) -> Result<()> {
    r.tag(0x77)?;
    r.string("Comment")?;
    r.u64("TimeStamp")?;
    r.string("UserName")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x10117b059`.
pub(super) fn list_keywords(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListKeywordsItem", |r| {
            r.string("value")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100ff3b70`.
pub(super) fn f_100ff3b70(r: &mut Reader) -> Result<()> {
    r.tag(0x6c)?;
    r.string("InstrumentShortName")?;
    r.node("ClefStyleAtStart", "F100fa77fa", f_100fa77fa)?;
    r.node("KeySignatureAtStart", "BarObject", bar_object)?;
    r.node("TimeSignatureAtStart", "BarObject", bar_object)?;
    r.record("record", "Record", |r| {
        if r.since(0x210001) {
            r.string("InstrumentFullName")?;
        }
        if r.since(0x210005) {
            if r.file_version < 0x2a0000 {
                r.node("Instrument", "F100fa793c", f_100fa793c)?;
            } else if r.since(0x29ffff) {
                if r.file_version < 0x2d0000 {
                    r.node("InstrumentInstance", "F100fa4a02", f_100fa4a02)?;
                } else {
                    r.node("InstrumentInstance", "Instance", instance)?;
                }
            } else {
                r.node("InstrumentInstance", "Instance", instance)?;
            }
        } else if r.since(0x29ffff) {
            if r.file_version < 0x2d0000 {
                r.node("InstrumentInstance", "F100fa4a02", f_100fa4a02)?;
            } else {
                r.node("InstrumentInstance", "Instance", instance)?;
            }
        } else {
            r.node("InstrumentInstance", "Instance", instance)?;
        }
        stave_start_tail(r)?;
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100da7f74`.
pub(super) fn fragment_list(r: &mut Reader) -> Result<()> {
    r.tag(0x59)?;
    r.list("Masks", |r| {
        r.sub("MasksItem", |r| {
            r.i32("value")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100f29145`.
pub(super) fn pm_list_part_map_default(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("ListPartMapDefault", |r| list_part_map_default(r, "items")).map(Value::Node))
}

/// 8.3.0 `0x101018f66`.
pub(super) fn special_page_break_data(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v > 0x20ffff {
        r.tag(0x68)?;
    }
    r.record("record", "Record", |r| {
        if r.since(0x200000) {
            pm_list_part_map_default_36dc(r, "ItemPageOffsetData")?;
            pm_u32(r, "BlankPageTypeData")?;
            pm_u32(r, "NumBlankPages")?;
        }
        if r.since(0x200001) {
            pm_u32(r, "BlankPageTypeDataBefore")?;
            pm_u32(r, "NumBlankPagesBefore")?;
        }
        if r.since(0x210000) {
            pm_bool(r, "HasMarginModifier")?;
            pm_f_1010199af(r, "MarginModifier")?;
        }
        if r.since(0x220000) {
            pm_f_1010199af(r, "MarginModifierPreFirstPage")?;
        }
        if r.since(0x260000) {
            pm_list_part_map_default_24b6(r, "PageNumberChangeData")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x10114293a`.
pub(super) fn pm_f_101193236(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("F101193236", f_101193236).map(Value::Node))
}

/// 8.3.0 `0x100fd7229`.
pub(super) fn list_midiconnections(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListMidiconnectionsItem", |r| {
            f_100f997b8(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x101003932`.
pub(super) fn stave_types(r: &mut Reader) -> Result<()> {
    list_stave_types(r, "StaveTypes")?;
    Ok(())
}

/// 8.3.0 `0x100fd73cb`.
pub(super) fn f_100fd73cb(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("F100fd73cbItem", |r| {
            r.u32("value")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100fd6d6b`.
pub(super) fn list_line_playback_style_list(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListLinePlaybackStyleListItem", |r| {
            f_10104dda3(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100fdc0e0`.
pub(super) fn list_swing_styles(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListSwingStylesItem", |r| {
            f_1010e8278(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x101003a9c`.
pub(super) fn instrument_types(r: &mut Reader) -> Result<()> {
    list_instrument_list(r, "InstrumentList")?;
    Ok(())
}

/// 8.3.0 `0x100ef1539`.
pub(super) fn summary(r: &mut Reader) -> Result<()> {
    r.tag(0x3f)?;
    f_100eedb54(r)?;
    Ok(())
}

/// 8.3.0 `0x101002cac`.
pub(super) fn clef_styles(r: &mut Reader) -> Result<()> {
    list_clef_styles(r, "ClefStyles")?;
    Ok(())
}

/// 8.3.0 `0x100fd72a5`.
pub(super) fn f_100fd72a5(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("F100fd72a5Item", |r| {
            r.string("value")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x1010021ba`.
pub(super) fn text_styles(r: &mut Reader) -> Result<()> {
    list_text_styles(r, "TextStyles")?;
    Ok(())
}

/// 8.3.0 `0x101002b30`.
pub(super) fn pen_styles(r: &mut Reader) -> Result<()> {
    list_pen_styles(r, "PenStyles")?;
    Ok(())
}

/// 8.3.0 `0x100fd6e51`.
pub(super) fn list_articulation_playback_style_list(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListArticulationPlaybackStyleListItem", |r| {
            f_1010533f7(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100e24f62`.
pub(super) fn options(r: &mut Reader) -> Result<()> {
    r.tag(0x3e)?;
    options_body(r)?;
    Ok(())
}

/// 8.3.0 `0x100f0aa7c`.
pub(super) fn playing_options(r: &mut Reader) -> Result<()> {
    r.tag(0x41)?;
    f_100f09a16(r)?;
    Ok(())
}

/// 8.3.0 `0x100fd732c`.
pub(super) fn list_deleted_line_pb_styles(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListDeletedLinePbStylesItem", |r| {
            style_id(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100fd6f2d`.
pub(super) fn list_notehead_playback_style_list(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListNoteheadPlaybackStyleListItem", |r| {
            f_10104f731(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100fd742b`.
pub(super) fn list_symbol_categories(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListSymbolCategoriesItem", |r| {
            f_100f37244(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x101003688`.
pub(super) fn indexed_note_styles(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v <= 0x21ffff {
        list_note_styles(r, "NoteStyles")?;
    } else {
        list_indexed_note_styles(r, "IndexedNoteStyles")?;
    }
    Ok(())
}

/// 8.3.0 `0x101003c2a`.
pub(super) fn ensembles(r: &mut Reader) -> Result<()> {
    list_ensemble_list(r, "EnsembleList")?;
    Ok(())
}

/// 8.3.0 `0x100ef982a`.
pub(super) fn spacing_table(r: &mut Reader) -> Result<()> {
    r.tag(0x58)?;
    f_100ef299e(r)?;
    Ok(())
}

/// 8.3.0 `0x10100275a`.
pub(super) fn indexed_symbols(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v <= 0x21ffff {
        list_symbols(r, "Symbols")?;
    } else {
        list_indexed_symbols(r, "IndexedSymbols")?;
    }
    Ok(())
}

/// 8.3.0 `0x100fd7009`.
pub(super) fn list_symbol_playback_style_list(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListSymbolPlaybackStyleListItem", |r| {
            f_10105257d(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x101002e10`.
pub(super) fn line_styles(r: &mut Reader) -> Result<()> {
    list_line_styles(r, "LineStyles")?;
    Ok(())
}

/// 8.3.0 `0x100fd74a7`.
pub(super) fn list_bar_spacings(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListBarSpacingsItem", |r| {
            bar_spacing(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100fa8e64`.
pub(super) fn system_stave(r: &mut Reader) -> Result<()> {
    r.tag(0x03)?;
    r.node("MIDIData", "Mididata", mididata)?;
    r.list("Bars", |r| {
        r.sub("BarsItem", |r| {
            bar(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100f3ec2f`.
pub(super) fn style_id(r: &mut Reader) -> Result<()> {
    r.string("value")?;
    Ok(())
}

/// 8.3.0 `0x100e145f0`.
pub(super) fn multi_rest_style(r: &mut Reader) -> Result<()> {
    r.u32("value")?;
    Ok(())
}

/// 8.3.0 `0x100e14680`.
pub(super) fn bar_number_freq(r: &mut Reader) -> Result<()> {
    r.u32("value")?;
    Ok(())
}

/// 8.3.0 `0x100e14938`.
pub(super) fn part_clef_omit_rule(r: &mut Reader) -> Result<()> {
    r.u32("value")?;
    Ok(())
}

/// 8.3.0 `0x100e14a58`.
pub(super) fn score_special_page_breaks_in_part(r: &mut Reader) -> Result<()> {
    r.u32("value")?;
    Ok(())
}

/// 8.3.0 `0x100f3e36a`.
pub(super) fn mididata_e36a(r: &mut Reader) -> Result<()> {
    r.tag(0x3b)?;
    f_100f3e2aa(r)?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100fa6f66`.
pub(super) fn f_100fa6f66(r: &mut Reader) -> Result<()> {
    r.u32("value")?;
    Ok(())
}

/// 8.3.0 `0x100debf42`.
pub(super) fn f_100debf42(r: &mut Reader) -> Result<()> {
    f_100f3c746(r)?;
    r.u8("BracketType")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100e1335f`.
pub(super) fn hit_points(r: &mut Reader) -> Result<()> {
    r.tag(0x64)?;
    f_100e133a8(r)?;
    Ok(())
}

/// 8.3.0 `0x100a16e70`.
pub(super) fn f_100a16e70(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("F100a16e70Item", |r| {
            r.string("value")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100a16ff1`.
pub(super) fn list_traits(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListTraitsItem", |r| {
            f_100a0cf26(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x1010da9fe`.
pub(super) fn f_1010da9fe(r: &mut Reader) -> Result<()> {
    r.tag(0x89)?;
    style_header(r)?;
    f_1010d9754(r)?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100fa8450`.
pub(super) fn old_clef_style_ref(r: &mut Reader) -> Result<()> {
    if !r.string_refs {
        r.u8("value")?;
    } else {
        r.string("value")?;
    }
    Ok(())
}

/// 8.3.0 `0x1010331ac`.
pub(super) fn pm_style_id(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("StyleId", style_id).map(Value::Node))
}

/// 8.3.0 `0x100f6d77a`.
pub(super) fn f_100f6d77a(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("F100f6d77aItem", |r| {
            f_100f539f0(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100df35ce`.
pub(super) fn f_100df35ce(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("F100df35ceItem", |r| {
            f_100f3fb74(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100fa828c`.
pub(super) fn old_style_ref_828c(r: &mut Reader) -> Result<()> {
    if !r.string_refs {
        r.u8("value")?;
    } else {
        r.string("value")?;
    }
    Ok(())
}

/// 8.3.0 `0x100f6d80a`.
pub(super) fn pm_f_100f6d77a(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("F100f6d77a", |r| f_100f6d77a(r, "items")).map(Value::Node))
}

/// 8.3.0 `0x100fa804a`.
pub(super) fn old_style_ref_804a(r: &mut Reader) -> Result<()> {
    if !r.string_refs {
        r.u32("value")?;
    } else {
        r.string("value")?;
    }
    Ok(())
}

/// 8.3.0 `0x100fa858e`.
pub(super) fn old_text_style_ref(r: &mut Reader) -> Result<()> {
    if !r.string_refs {
        r.u8("value")?;
    } else {
        r.string("value")?;
    }
    Ok(())
}

/// 8.3.0 `0x100eb8756`.
pub(super) fn f_100eb8756(r: &mut Reader) -> Result<()> {
    r.i16("Artic")?;
    pm_i8(r, "Dy")?;
    Ok(())
}

/// 8.3.0 `0x100eb87e1`.
pub(super) fn f_100eb87e1(r: &mut Reader) -> Result<()> {
    r.i16("First")?;
    r.i16("Second")?;
    Ok(())
}

/// 8.3.0 `0x1010396cc`.
pub(super) fn f_1010396cc(r: &mut Reader) -> Result<()> {
    r.tag(0x95)?;
    r.u32("X")?;
    r.u32("Y")?;
    r.f32("P")?;
    Ok(())
}

/// 8.3.0 `0x1010b6e01`.
pub(super) fn suffix_settings(r: &mut Reader) -> Result<()> {
    r.tag(0x7e)?;
    suffix_appearances(r)?;
    Ok(())
}

/// 8.3.0 `0x1010b4600`.
pub(super) fn chord_suffix_extension_arrangement(r: &mut Reader) -> Result<()> {
    r.u32("value")?;
    Ok(())
}

/// 8.3.0 `0x1010b4648`.
pub(super) fn order_of_alterations(r: &mut Reader) -> Result<()> {
    r.u32("value")?;
    Ok(())
}

/// 8.3.0 `0x100fa793c`.
pub(super) fn f_100fa793c(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v > 0xfffff {
        style_id(r)?;
    }
    Ok(())
}

/// 8.3.0 `0x100f293e6`.
pub(super) fn list_part_map_default(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListPartMapDefaultItem", |r| {
            spacing(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x1010236dc`.
pub(super) fn pm_list_part_map_default_36dc(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("ListPartMapDefault9861", |r| list_part_map_default_9861(r, "items")).map(Value::Node))
}

/// 8.3.0 `0x101022eec`.
pub(super) fn pm_f_1010199af(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("F1010199af", f_1010199af).map(Value::Node))
}

/// 8.3.0 `0x1010224b6`.
pub(super) fn pm_list_part_map_default_24b6(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("ListPartMapDefault2750", |r| list_part_map_default_2750(r, "items")).map(Value::Node))
}

/// 8.3.0 `0x101193236`.
pub(super) fn f_101193236(r: &mut Reader) -> Result<()> {
    r.tag(0x71)?;
    r.record("record", "Record", |r| {
        r.u32("PageNumber")?;
        r.u32("PageNumberFormat")?;
        r.u32("DisplayRule")?;
        r.u32("BlankPageOffset")?;
        if r.since(0x280002) {
            r.bool("FormatOnly")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100f997b8`.
pub(super) fn f_100f997b8(r: &mut Reader) -> Result<()> {
    r.tag(0x4e)?;
    r.string("Name")?;
    r.string("Model")?;
    r.string("DeviceTypeName")?;
    r.string("EquipmentName")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x10100dd2e`.
pub(super) fn list_stave_types(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListStaveTypesItem", |r| {
            f_100f44324(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x10104dda3`.
pub(super) fn f_10104dda3(r: &mut Reader) -> Result<()> {
    r.any_tag()?;
    f_10104ddea(r)?;
    Ok(())
}

/// 8.3.0 `0x1010e8278`.
pub(super) fn f_1010e8278(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x5d)?;
    style_header(r)?;
    if v < 0x100000 {
        r.u32("PresetType")?;
    }
    r.bool("SwingAsShorthand")?;
    r.u32("BeatUnit")?;
    r.u16("BeatDuration")?;
    r.i32("NumBeats")?;
    r.node("Offsets", "Offsets", offsets)?;
    r.node("Durations", "Offsets", offsets)?;
    r.node("Stresses", "Offsets", offsets)?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x1010084ec`.
pub(super) fn list_instrument_list(r: &mut Reader, name: &'static str) -> Result<()> {
    if r.file_version <= 0xfffff {
        // the count is stored without items
        r.u32(name)?;
        return Ok(());
    }
    r.list(name, |r| {
        r.sub("ListInstrumentListItem", |r| {
            f_100f73b5e(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100eedb54`.
pub(super) fn f_100eedb54(r: &mut Reader) -> Result<()> {
    pm_string(r, "Title")?;
    pm_string(r, "Composer")?;
    pm_string(r, "Arranger")?;
    pm_string(r, "Lyricist")?;
    pm_string(r, "MoreInfo")?;
    r.record("record", "Record", |r| {
        if r.since(0x20000) {
            pm_string(r, "Artist")?;
            pm_string(r, "Copyright")?;
        }
        if r.since(0xa0019) {
            pm_string(r, "Publisher")?;
        }
        if r.since(0xa001c) {
            r.string("Serial")?;
        }
        if r.since(0xd0000) {
            if r.file_version < 0xf0001 {
                r.bool("DummyIsWorksheetOrExercise")?;
            }
            r.string("WorksheetName")?;
            r.bool("IsDatafile")?;
            r.string("WorksheetDescr")?;
            if r.file_version < 0xd0004 {
                r.string("DummyWorksheetInstr")?;
                r.bool("DummyHasSourceBox")?;
            }
            if r.file_version < 0xd0003 {
                r.u16("BarsPerQuestionDummy")?;
            } else if r.file_version < 0xd0004 {
                r.u32("DummyBarsPerQuestion")?;
            }
            r.string("DifficultyLevel")?;
            if r.file_version < 0xd0004 {
                r.bool("DummyStudentVoicesSeen.vc1")?;
                r.bool("DummyStudentVoicesSeen.vc2")?;
                r.bool("DummyStudentVoicesSeen.vc3")?;
                r.bool("DummyStudentVoicesSeen.vc4")?;
                r.bool("DummyTeacherVoicesSeen.vc1")?;
                r.bool("DummyTeacherVoicesSeen.vc2")?;
                r.bool("DummyTeacherVoicesSeen.vc3")?;
                r.bool("DummyTeacherVoicesSeen.vc4")?;
            }
            r.bool("StudentCompletionOptionsAtComputer")?;
            r.bool("StudentCompletionOptionsOnPaper")?;
            r.bool("StudentCompletionOptionsPerform")?;
            r.bool("TeacherOptionsInvolved")?;
            r.bool("TeacherOptionsAccessToSibelius")?;
            r.u32("TimeToComplete")?;
            if r.file_version < 0xd0004 {
                r.bool("DummyIsWorksheet")?;
            }
            r.bool("AnySize")?;
            r.bool("Individual")?;
            r.bool("LimitedGroupSize")?;
        }
        if r.since(0xe0000) {
            r.u32("NumQuestions")?;
        }
        if r.since(0xf0000) {
            pm_string(r, "VirtualPartName")?;
        }
        if r.since(0xf0001) {
            r.u32("WorksheetType")?;
        }
        if r.since(0x1a0001) {
            r.bool("HasSourceBox")?;
        }
        if r.since(0x1a0004) {
            r.bool("HasAnswerScore")?;
        }
        if r.since(0x1a0009) {
            pm_bool(r, "PartNameIsDefault")?;
        }
        if r.since(0x280004) {
            pm_string(r, "InstrumentChanges")?;
        }
        if r.since(0x280004) {
            pm_bool(r, "InstrumentChangesIsDefault")?;
        }
        if r.since(0x280005) {
            pm_string(r, "Subtitle")?;
            pm_string(r, "Copyist")?;
            pm_string(r, "Dedication")?;
            pm_string(r, "OpusNumber")?;
            pm_string(r, "YearOfComposition")?;
            pm_string(r, "ComposerDates")?;
        }
        if r.since(0x2d0010) {
            r.u64("MasterScoreCreationDate")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x101013186`.
pub(super) fn list_clef_styles(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListClefStylesItem", |r| {
            f_100deb984(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x1010149a4`.
pub(super) fn list_text_styles(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListTextStylesItem", |r| {
            f_100ec03bc(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x10100f9e2`.
pub(super) fn list_pen_styles(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListPenStylesItem", |r| {
            pen_style(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x1010533f7`.
pub(super) fn f_1010533f7(r: &mut Reader) -> Result<()> {
    r.any_tag()?;
    f_10105343e(r)?;
    Ok(())
}

/// 8.3.0 `0x100e1729c`.
pub(super) fn options_body(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    pm_bool(r, "ShowBreaks")?;
    r.bool("DrawMIDICodes")?;
    r.bool("CautionaryNaturals")?;
    pm_bool(r, "TransposingScore")?;
    if v < 0x1e0000 {
        r.u16("SpaceAfterLyric")?;
    } else {
        r.u32("SpaceAfterLyric")?;
    }
    if v < 0x1e0000 {
        r.u16("SpaceBetweenLyricHyphens")?;
    } else {
        r.u32("SpaceBetweenLyricHyphens")?;
    }
    r.i16("ArticulationAbove")?;
    r.i16("ArticulationInStave")?;
    if v < 0x1e0000 {
        r.u16("ArticulationHeadGap")?;
    } else {
        r.u32("ArticulationHeadGap")?;
    }
    if v < 0x1e0000 {
        r.u16("ArticulationStemGap")?;
    } else {
        r.u32("ArticulationStemGap")?;
    }
    if v < 0x1e0000 {
        r.u16("ArticulationSeparation")?;
    } else {
        r.u32("ArticulationSeparation")?;
    }
    r.bool("ArticulationCentredOnStem")?;
    if v < 0x1e0000 {
        r.u16("AccidentalHGap")?;
    } else {
        r.u32("AccidentalHGap")?;
    }
    if v < 0x1e0000 {
        r.u16("AccidentalVGap")?;
    } else {
        r.u32("AccidentalVGap")?;
    }
    if v < 0x1e0000 {
        r.u16("AccidentalFirstHGap")?;
    } else {
        r.u32("AccidentalFirstHGap")?;
    }
    r.bool("TopAccidentalAtRight")?;
    r.u8("BeamMaxGradient")?;
    if v < 0x1e0000 {
        r.u16("AngledBeamThreshold")?;
    } else {
        r.u32("AngledBeamThreshold")?;
    }
    r.u8("BeamSeparation")?;
    if v < 0x1e0000 {
        r.u16("FlagWidth")?;
    } else {
        r.u32("FlagWidth")?;
    }
    if v < 0x1e0000 {
        r.u16("SingleTremoloSeparation")?;
    } else {
        r.u32("SingleTremoloSeparation")?;
    }
    if v < 0x1e0000 {
        r.u16("NoteDotGap")?;
    } else {
        r.u32("NoteDotGap")?;
    }
    if v < 0x1e0000 {
        r.u16("RestDotGap")?;
    } else {
        r.u32("RestDotGap")?;
    }
    if v < 0x1e0000 {
        r.u16("DotDotGap")?;
    } else {
        r.u32("DotDotGap")?;
    }
    if v < 0x1e0000 {
        r.u16("NoteTieGap")?;
    } else {
        r.u32("NoteTieGap")?;
    }
    if v < 0x1e0000 {
        r.u16("DotTieGap")?;
    } else {
        r.u32("DotTieGap")?;
    }
    if v < 0xf0005 {
        if v < 0x1e0000 {
            r.u16("TieTieGap")?;
        } else {
            r.u32("TieTieGap")?;
        }
        if v < 0x1e0000 {
            r.u16("NoteTieVGap")?;
        } else {
            r.u32("NoteTieVGap")?;
        }
    } else {
        if v < 0x1e0000 {
            r.u16("TieAboveVertGap")?;
        } else {
            r.u32("TieAboveVertGap")?;
        }
        if v < 0x1e0000 {
            r.u16("TieAboveFirstGap")?;
        } else {
            r.u32("TieAboveFirstGap")?;
        }
        if v < 0x1e0000 {
            r.u16("TieAboveSecondGap")?;
        } else {
            r.u32("TieAboveSecondGap")?;
        }
        if v < 0x1e0000 {
            r.u16("TieBelowVertGap")?;
        } else {
            r.u32("TieBelowVertGap")?;
        }
        if v < 0x1e0000 {
            r.u16("TieBelowFirstGap")?;
        } else {
            r.u32("TieBelowFirstGap")?;
        }
        if v < 0x1e0000 {
            r.u16("TieBelowSecondGap")?;
        } else {
            r.u32("TieBelowSecondGap")?;
        }
    }
    r.u8("VoiceVersion")?;
    if v < 0x1e0000 {
        r.u16("HairpinMiddleSmallH")?;
    } else {
        r.u32("HairpinMiddleSmallH")?;
    }
    if v < 0x1e0000 {
        r.u16("HairpinMiddleLargeH")?;
    } else {
        r.u32("HairpinMiddleLargeH")?;
    }
    if v < 0x1e0000 {
        r.u16("HairpinLargeH")?;
    } else {
        r.u32("HairpinLargeH")?;
    }
    if v < 0x1e0000 {
        r.u16("HairpinSmallH")?;
    } else {
        r.u32("HairpinSmallH")?;
    }
    if v < 0x1e0000 {
        r.u16("HairpinThresholdW")?;
    } else {
        r.u32("HairpinThresholdW")?;
    }
    if v < 0x1e0000 {
        r.u16("LineDxFromNoteLhs")?;
    } else {
        r.u32("LineDxFromNoteLhs")?;
    }
    if v < 0x1e0000 {
        r.u16("LineDxFromNoteRhs")?;
    } else {
        r.u32("LineDxFromNoteRhs")?;
    }
    if v < 0x1e0000 {
        r.u16("SlurDxFromNoteCenter")?;
    } else {
        r.u32("SlurDxFromNoteCenter")?;
    }
    if v < 0x1e0000 {
        r.u16("SlurDxFromStem")?;
    } else {
        r.u32("SlurDxFromStem")?;
    }
    if v < 0x1e0000 {
        r.u16("SlurDyOutsideHeadOnSpace")?;
    } else {
        r.u32("SlurDyOutsideHeadOnSpace")?;
    }
    if v < 0x1e0000 {
        r.u16("SlurDyOutsideHeadOnLine")?;
    } else {
        r.u32("SlurDyOutsideHeadOnLine")?;
    }
    if v < 0x1e0000 {
        r.u16("SlurDyOutsideTie")?;
    } else {
        r.u32("SlurDyOutsideTie")?;
    }
    if v < 0x1e0000 {
        r.u16("SlurDyOutsideQuaverStem")?;
    } else {
        r.u32("SlurDyOutsideQuaverStem")?;
    }
    if v < 0x1e0000 {
        r.u16("SlurDyOutsideCrotchetStem")?;
    } else {
        r.u32("SlurDyOutsideCrotchetStem")?;
    }
    r.u32("SlurShoulder")?;
    r.bool("ClipSlurs")?;
    if v < 0x1e0000 {
        pm_u16(r, "StaveBracketDx")?;
    } else {
        pm_u32(r, "StaveBracketDx")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "StaveSubBracketDx")?;
    } else {
        pm_u32(r, "StaveSubBracketDx")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "BraceDx")?;
    } else {
        pm_u32(r, "BraceDx")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "BraceMinWidth")?;
    } else {
        pm_u32(r, "BraceMinWidth")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "BraceMaxWidth")?;
    } else {
        pm_u32(r, "BraceMaxWidth")?;
    }
    pm_bool(r, "SubAsBrace")?;
    if v < 0x1e0000 {
        r.u16("DoubleBarlineDx")?;
    } else {
        r.u32("DoubleBarlineDx")?;
    }
    if v < 0x1e0000 {
        r.u16("FinalBarlineDx")?;
    } else {
        r.u32("FinalBarlineDx")?;
    }
    if v < 0x210000 {
        pm_f32(r, "PageWmm")?;
        pm_f32(r, "PageHmm")?;
        pm_f32(r, "PageLMarginmm")?;
        pm_f32(r, "PageRMarginmm")?;
        pm_f32(r, "PageTMarginmm")?;
        pm_f32(r, "PageBMarginmm")?;
        if v < 0x1e0000 {
            pm_u16(r, "PageW")?;
        } else {
            pm_u32(r, "PageW")?;
        }
        if v < 0x1e0000 {
            pm_u16(r, "PageH")?;
        } else {
            pm_u32(r, "PageH")?;
        }
        if v < 0x1e0000 {
            pm_u16(r, "PageLMargin")?;
        } else {
            pm_u32(r, "PageLMargin")?;
        }
        if v < 0x1e0000 {
            pm_u16(r, "PageRMargin")?;
        } else {
            pm_u32(r, "PageRMargin")?;
        }
        if v < 0x1e0000 {
            pm_u16(r, "PageTMargin")?;
        } else {
            pm_u32(r, "PageTMargin")?;
        }
        if v < 0x1e0000 {
            pm_u16(r, "PageBMargin")?;
        } else {
            pm_u32(r, "PageBMargin")?;
        }
    }
    if v < 0x1e0000 {
        r.u16("PreInitClefW")?;
    } else {
        r.u32("PreInitClefW")?;
    }
    if v < 0x1e0000 {
        r.u16("InitClefW")?;
    } else {
        r.u32("InitClefW")?;
    }
    if v < 0x1e0000 {
        r.u16("PreInitKeyW")?;
    } else {
        r.u32("PreInitKeyW")?;
    }
    if v < 0x1e0000 {
        r.u16("KeySigFlatW")?;
    } else {
        r.u32("KeySigFlatW")?;
    }
    if v < 0x1e0000 {
        r.u16("KeySigNaturalW")?;
    } else {
        r.u32("KeySigNaturalW")?;
    }
    if v < 0x1e0000 {
        r.u16("KeySigSharpW")?;
    } else {
        r.u32("KeySigSharpW")?;
    }
    if v < 0x1e0000 {
        r.u16("PostInitsW")?;
    } else {
        r.u32("PostInitsW")?;
    }
    pm_u8(r, "SmallStaveScale")?;
    pm_u8(r, "CueNoteScale")?;
    pm_u8(r, "GraceNoteScale")?;
    if v < 0x210000 {
        pm_f32(r, "StaveHeightmm")?;
    }
    pm_u32(r, "BarNumberFreq")?;
    pm_bool(r, "BarNumbersAtSectionStart")?;
    r.bool("ShowHiddenBarNumbers")?;
    r.u32("RehearsalMarksStyle")?;
    if v < 0x100000 {
        pm_f_100fa28f4(r, "TimeSignatureTextStyle")?;
    } else {
        pm_style_id(r, "TimeSignatureTextStyle")?;
    }
    if v < 0x1e0000 {
        r.u16("GapBeforeTimeSig")?;
    } else {
        r.u32("GapBeforeTimeSig")?;
    }
    if v < 0x1e0000 {
        r.u16("RangeSelectionGap")?;
    } else {
        r.u32("RangeSelectionGap")?;
    }
    if v < 0x1e0000 {
        r.u16("FrameDy")?;
    } else {
        r.u32("FrameDy")?;
    }
    if v < 0x1e0000 {
        r.u16("FrameCellWidth")?;
    } else {
        r.u32("FrameCellWidth")?;
    }
    if v < 0x1e0000 {
        r.u16("FrameCellHeight")?;
    } else {
        r.u32("FrameCellHeight")?;
    }
    r.string("FretnumStr")?;
    r.u32("NumFrets")?;
    r.u32("MaxSeparation")?;
    if v < 0x1e0000 {
        r.u16("OpenSymbolRadius")?;
    } else {
        r.u32("OpenSymbolRadius")?;
    }
    if v < 0x1e0000 {
        r.u16("CrossLineLength")?;
    } else {
        r.u32("CrossLineLength")?;
    }
    if v < 0x1e0000 {
        r.u16("ExtendedLength")?;
    } else {
        r.u32("ExtendedLength")?;
    }
    r.bool("ShowTopLine")?;
    r.bool("LeftOfFrame")?;
    r.bool("ExtendStrings")?;
    if v < 0x1e0000 {
        r.u16("NameYOffset")?;
    } else {
        r.u32("NameYOffset")?;
    }
    if v < 0x140000 {
        r.u32("MultiRestStyle")?;
    } else {
        pm_u32(r, "MultiRestStyle")?;
    }
    if v < 0x1e0000 {
        r.u16("MultiRestGap")?;
    } else {
        r.u32("MultiRestGap")?;
    }
    if v < 0x1e0000 {
        r.u16("PauseDyAboveStave")?;
    } else {
        r.u32("PauseDyAboveStave")?;
    }
    pm_bool(r, "DefaultSections")?;
    pm_bool(r, "BottomSection")?;
    r.node("ExtractOptions", "ExtractOptions", extract_options)?;
    r.u32("MinStavesForBreak")?;
    r.bool("LeftBreak")?;
    r.bool("RightBreak")?;
    if v < 0x1e0000 {
        r.u16("LeftBreakDx")?;
    } else {
        r.u32("LeftBreakDx")?;
    }
    if v < 0x1e0000 {
        r.u16("RightBreakDx")?;
    } else {
        r.u32("RightBreakDx")?;
    }
    if v < 0x1e0000 {
        if v < 0x1e0000 {
            pm_u16(r, "InstrumentNameDx")?;
        } else {
            pm_u32(r, "InstrumentNameDx")?;
        }
    } else {
        pm_u32(r, "InstrumentNameDx")?;
    }
    if v < 0x1e0000 {
        r.u16("SpaceBetweenLyrics")?;
    } else {
        r.u32("SpaceBetweenLyrics")?;
    }
    r.u8("ShowBarRestsPlus")?;
    r.record("record", "Record", |r| {
        pm_bool(r, "CentreBarNumbers")?;
        r.bool("UseTextHyphenInLyrics")?;
        r.bool("DrawHBarWithSymbol")?;
        if r.file_version < 0x1e0000 {
            r.u16("SlurDyOutsideArticulation")?;
        } else {
            r.u32("SlurDyOutsideArticulation")?;
        }
        if r.file_version < 0x1e0000 {
            r.u16("ArticulationDyOutsideSlur")?;
        } else {
            r.u32("ArticulationDyOutsideSlur")?;
        }
        if r.file_version < 0x1e0000 {
            r.u16("SlurDyMinFromStaveLine")?;
        } else {
            r.u32("SlurDyMinFromStaveLine")?;
        }
        r.bool("RotateTupletNumbers")?;
        if r.file_version < 0x1e0000 {
            r.u16("SlurDyOutsideHead")?;
        } else {
            r.u32("SlurDyOutsideHead")?;
        }
        r.f32("MaxSlurGradient")?;
        r.bool("NewStemLengths")?;
        pm_bool(r, "OneBarMultirests")?;
        r.bool("OneLyricHyphen")?;
        r.bool("LyricHyphensAtStart")?;
        r.bool("ClipTies")?;
        if r.since(0x20003) {
            r.u32("TupletPosWithBracket")?;
            r.u32("TupletPosWithoutBracket")?;
        }
        if r.since(0x20005) && r.file_version < 0x210000 {
            pm_f32(r, "RightPageLMarginmm")?;
            pm_f32(r, "RightPageRMarginmm")?;
            if r.file_version < 0x1e0000 {
                pm_u16(r, "RightPageLMargin")?;
            } else {
                pm_u32(r, "RightPageLMargin")?;
            }
            if r.file_version < 0x1e0000 {
                pm_u16(r, "RightPageRMargin")?;
            } else {
                pm_u32(r, "RightPageRMargin")?;
            }
            pm_u8(r, "MarginType")?;
        }
        if r.since(0x20006) {
            if r.file_version < 0x1e0000 {
                r.u16("TupletDistanceFromHead")?;
            } else {
                r.u32("TupletDistanceFromHead")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("TupletDistanceFromStem")?;
            } else {
                r.u32("TupletDistanceFromStem")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("TupletHorizontalDistanceFromNote")?;
            } else {
                r.u32("TupletHorizontalDistanceFromNote")?;
            }
        }
        if r.since(0x20008) {
            r.bool("IgnoreHangingPunctuation")?;
        }
        if r.since(0x20009) {
            r.bool("IsAPart")?;
        }
        if r.since(0x2000a) {
            pm_bool(r, "TimecodeAboveEveryBar")?;
            pm_bool(r, "DurationAtEndOfScore")?;
            r.u32("TimeCodeDisplay")?;
            r.u32("TimeCodeFrameRate")?;
            r.u32("TimeCodeStartTimeInMilliSecs")?;
        }
        if r.since(0x30003) {
            if r.file_version < 0x1e0000 {
                r.u16("FullNameWidthDelta")?;
            } else {
                r.u32("FullNameWidthDelta")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("ShortNameWidthDelta")?;
            } else {
                r.u32("ShortNameWidthDelta")?;
            }
        }
        if r.since(0x3000b) {
            r.bool("UseFullInTabBends")?;
        }
        if r.since(0x30010) {
            r.u32("DoubleTremoloStyle")?;
        }
        if r.since(0x40000) {
            r.u32("ArticulationXPos")?;
        }
        if r.since(0x50000) {
            r.node("DefaultBarlineType", "DefaultBarlineType", default_barline_type)?;
        }
        if r.since(0x50001) {
            pm_bool(r, "WingedRepeats")?;
        }
        if r.since(0x60002) {
            r.bool("TupletAvoidStaff")?;
        }
        if r.since(0x60003) {
            list_editable_options_list(r, "EditableOptionsList")?;
        }
        if r.since(0x70005) {
            if r.file_version < 0x1e0000 {
                if r.file_version < 0x1e0000 {
                    pm_u16(r, "TimeCodeVerticalPosition")?;
                } else {
                    pm_u32(r, "TimeCodeVerticalPosition")?;
                }
            } else {
                pm_u32(r, "TimeCodeVerticalPosition")?;
            }
        }
        if r.since(0x130001) {
            r.bool("TiesHeadsideOffsetOnSingleNotes")?;
            r.bool("TiesHeadsideOffsetOnChords")?;
        }
        if r.file_version < 0xf0005 {
            if r.since(0x7000a) {
                r.bool("TiesBetweenNotes")?;
            }
        } else {
            r.bool("TiesStemsideOffsetOnSingleNotes")?;
            r.bool("TiesStemsideOffsetOnChords")?;
            if r.file_version < 0x1e0000 {
                r.u16("TiesStemsideOffsetValue")?;
            } else {
                r.u32("TiesStemsideOffsetValue")?;
            }
            if r.file_version < 0x130002 {
                r.u32("TiesBetweenNotes")?;
            }
            r.bool("TiesInChordsAlignLeft")?;
            r.bool("TiesInChordsAlignRight")?;
            r.bool("TiesBetweenNotesAfterDots")?;
        }
        if r.since(0x7000b) {
            r.u8("TieVersion")?;
        }
        if r.since(0x7000c) {
            if r.file_version < 0x1e0000 {
                r.u16("TieOnLineDy")?;
            } else {
                r.u32("TieOnLineDy")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("TieOnSpaceDy")?;
            } else {
                r.u32("TieOnSpaceDy")?;
            }
        }
        if r.since(0x80003) {
            r.bool("UseArrowsInTabBends")?;
        }
        if r.since(0x80004) {
            r.bool("TopVoiceAccidentalsAtRight")?;
        }
        if r.since(0x80006) {
            if r.file_version < 0x1e0000 {
                r.u16("MinStemLength")?;
            } else {
                r.u32("MinStemLength")?;
            }
        }
        if r.since(0x80008) {
            r.bool("BrokenSystemLineBeforeKeySig")?;
            if r.file_version < 0x1e0000 {
                r.u16("BrokenSystemLineRhsSpacing")?;
            } else {
                r.u32("BrokenSystemLineRhsSpacing")?;
            }
        }
        if r.since(0x8000c) {
            r.bool("CentreSyllablesWithLyricLines")?;
        }
        if r.since(0x8000d) {
            r.bool("DrawGraceNotesWithTabLetters")?;
        }
        if r.since(0x90002) {
            if r.file_version < 0x1e0000 {
                r.u16("NoNameWidthDelta")?;
            } else {
                r.u32("NoNameWidthDelta")?;
            }
        }
        if r.since(0x90008) {
            if r.file_version < 0x1e0000 {
                r.u16("GapForTabStemsOutsideStave")?;
            } else {
                r.u32("GapForTabStemsOutsideStave")?;
            }
        }
        if r.since(0xa0006) {
            r.u32("MixerNumVisibleStrips")?;
        }
        if r.since(0xa0007) {
            pm_bool(r, "FocusOnStaves")?;
        }
        if r.since(0xa0007) {
            pm_bool(r, "ShowBreaksFocusMode")?;
        }
        if r.since(0xa0008) {
            r.u8("LegerLineProtrusion")?;
        }
        if r.since(0xa0009) {
            pm_bool(r, "InitialBarlineOnSingleStaves")?;
        }
        if r.since(0xa000c) {
            r.bool("AllowBeamsAfterRests")?;
        }
        if r.since(0xa0011) {
            r.bool("StaccatosHalfCentred")?;
        }
        if r.since(0xa001a) {
            r.u32("FingeringsPosition")?;
            if r.file_version < 0x1e0000 {
                r.u16("FingeringsDistance")?;
            } else {
                r.u32("FingeringsDistance")?;
            }
            if r.file_version < 0x170000 {
                if r.file_version < 0x1e0000 {
                    r.u16("FingeringsFontSize")?;
                } else {
                    r.u32("FingeringsFontSize")?;
                }
            }
        }
        if r.since(0xc0000) {
            pm_bool(r, "TimecodeAtStartOfSystem")?;
            if r.file_version < 0x1e0000 {
                if r.file_version < 0x1e0000 {
                    pm_u16(r, "HitPointVerticalPosition")?;
                } else {
                    pm_u32(r, "HitPointVerticalPosition")?;
                }
            } else {
                pm_u32(r, "HitPointVerticalPosition")?;
            }
        }
        if r.since(0xc0003) {
            pm_bool(r, "TimecodeAboveFirstBar")?;
        }
        if r.since(0x110003) {
            pm_bool(r, "UseScorePageBreaksInPart")?;
            pm_bool(r, "PageBreaksBecomeSystemBreaksInPart")?;
            pm_bool(r, "UseScoreSystemBreaksInPart")?;
            pm_bool(r, "UseOtherScoreBreaksInPart")?;
        }
        if r.since(0x130002) {
            pm_bool(r, "KeepGapsBeforeCodasInPart")?;
        }
        if r.since(0x130005) {
            pm_bool(r, "OmitNoLinesStaveTypeChangesInPart")?;
            pm_bool(r, "OmitClefsInPart")?;
            pm_u32(r, "PartClefOmitRule")?;
        }
        if r.since(0x130006) {
            pm_bool(r, "ShowPartNameOnFirstPage")?;
            pm_bool(r, "ShowPartNameOnSubsequentPages")?;
        }
        if r.since(0x140000) {
            pm_string(r, "TacetMultiRestText")?;
        }
        if r.since(0x150001) {
            r.bool("TiesSpacePerLine")?;
            r.u8("TiesMaxShiftHead")?;
            r.u8("TiesMaxShiftStem")?;
        }
        if r.since(0x150002) {
            r.bool("TiesArrangeByChord")?;
        }
        if r.since(0x170001) {
            r.u32("TieShoulder")?;
        }
        if r.since(0x170002) {
            r.bool("ScaleTieShoulderByLength")?;
            if r.file_version < 0x1e0000 {
                r.u16("TieShoulderScaleFrom")?;
            } else {
                r.u32("TieShoulderScaleFrom")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("TieShoulderScaleTo")?;
            } else {
                r.u32("TieShoulderScaleTo")?;
            }
            r.u32("TieMaxShoulder")?;
        }
        if r.since(0x190000) {
            r.bool("AdjustTranspositionIfKeySigWraps")?;
        }
        if r.since(0x190001) {
            pm_u32(r, "TimecodeStave")?;
        }
        if r.since(0x1a0000) {
            pm_bool(r, "RespellRemoteKeysInTransposingScore")?;
        }
        if r.since(0x1a0002) {
            r.bool("NewBeamPositionRule")?;
        }
        if r.since(0x1a0005) {
            if r.file_version < 0x1e0000 {
                r.u16("BeamSlant2nd")?;
            } else {
                r.u32("BeamSlant2nd")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("BeamSlant3rd")?;
            } else {
                r.u32("BeamSlant3rd")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("BeamSlant4th")?;
            } else {
                r.u32("BeamSlant4th")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("BeamSlant5th")?;
            } else {
                r.u32("BeamSlant5th")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("BeamSlant6th")?;
            } else {
                r.u32("BeamSlant6th")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("BeamSlant7th")?;
            } else {
                r.u32("BeamSlant7th")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("BeamSlant8ve")?;
            } else {
                r.u32("BeamSlant8ve")?;
            }
        }
        if r.since(0x1a0006) {
            if r.file_version < 0x1e0000 {
                r.u16("MinTieSpacing")?;
            } else {
                r.u32("MinTieSpacing")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("MinTieSpacingChords")?;
            } else {
                r.u32("MinTieSpacingChords")?;
            }
        }
        if r.since(0x1a000c) {
            pm_u32(r, "AlignmentOfBarNumbersAtStartOfSystem")?;
        }
        if r.since(0x1a000d) {
            pm_style_id(r, "BarNumberTextStyleId")?;
        }
        if r.since(0x1a000e) {
            if r.file_version < 0x1e0000 {
                r.u16("BeamedQuaverStemLength")?;
            } else {
                r.u32("BeamedQuaverStemLength")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("BeamedSemiquaverStemLength")?;
            } else {
                r.u32("BeamedSemiquaverStemLength")?;
            }
        }
        if r.since(0x1a000f) {
            r.u8("BeamMaxGradientUpTo8ve")?;
        }
        if r.since(0x1a0011) {
            pm_bool(r, "ShowHitPoints")?;
        }
        if r.since(0x1b0001) {
            r.bool("AvoidWedges")?;
        }
        if r.since(0x1b000b) {
            r.f32("BeamMaxGradientFloat")?;
            r.f32("BeamMaxGradientUpTo8veFloat")?;
        }
        if r.since(0x1b0010) {
            if r.file_version < 0x1e0000 {
                r.u16("TiesStemsideOffsetDy")?;
            } else {
                r.u32("TiesStemsideOffsetDy")?;
            }
        }
        if r.since(0x1b0016) {
            pm_u32(r, "MultiRestBarNumberStyle")?;
        }
        if r.since(0x1b0017) {
            r.u32("NewTimeCodeFrameRate")?;
        }
        if r.since(0x1b0018) {
            pm_bool(r, "CentredMultiRestBarNumbers")?;
        }
        if r.since(0x1e0000) {
            r.bool("NewConflictingVoicesRule")?;
        }
        if r.since(0x1e0001) {
            r.u32("BarNumberRepeatCountFormat")?;
        }
        if r.since(0x1e0002) {
            pm_u16(r, "BarNumberNumericFrequency")?;
        }
        if r.since(0x1e0003) {
            pm_bool(r, "OmitBarNumbersAtRehearsalMarks")?;
        }
        if r.since(0x1e0004) {
            pm_bool(r, "BottomSectionForBarNumbers")?;
        }
        if r.since(0x1e0005) {
            pm_u32(r, "BarNumberStavePolicy")?;
        }
        if r.since(0x1e0006) {
            pm_bool(r, "TopStaveForBarNumbers")?;
        }
        if r.since(0x1e0008) {
            pm_u32(r, "ScoreViewType")?;
        }
        if r.since(0x20ffff) {
            r.node("PageAndStaveMargins", "PageAndStaveMargins", page_and_stave_margins)?;
        }
        if r.since(0x210004) {
            pm_bool(r, "UseScoreTitlePagesInPart")?;
            pm_u32(r, "ScoreSpecialPageBreaksInPart")?;
        }
        if r.since(0x240003) {
            r.string("DefaultRehearsalMarkPrefix")?;
            r.string("DefaultRehearsalMarkSuffix")?;
        }
        if r.since(0x2c0002) {
            r.bool("V5CrossStaffBeaming")?;
        }
        if r.since(0x2d0003) {
            pm_bool(r, "OnlyDisplayInitialName")?;
        }
        if r.since(0x2d0005) {
            r.bool("TextVerticalPositionRelativeToStave")?;
        }
        if r.since(0x2d000b) {
            pm_u32(r, "CentredMultiRestBarNumbersDistanceBelowStaff")?;
        }
        if r.since(0x2d000d) {
            r.string("TextForInstrumentChangeWarnings")?;
        }
        if r.since(0x2d0012) {
            pm_bool(r, "NumberSuccessiveSingleRepeatBars")?;
        }
        if r.since(0x2d0014) {
            pm_u16(r, "RepeatBarNumberFrequency")?;
        }
        if r.since(0x2d0015) {
            pm_bool(r, "ShowRepeatBarNumbersInParentheses")?;
        }
        if r.since(0x2d0017) {
            pm_bool(r, "ShouldResetNumberingRepeatBarsAtRehearsalMarks")?;
            pm_bool(r, "ShouldResetNumberingRepeatBarsAtSpecialBarLines")?;
        }
        if r.since(0x2d0018) {
            r.bool("AllowRestBeamAtGroupStartEnd")?;
            r.bool("AllowRestBeamOverGroupMiddle")?;
        }
        if r.since(0x2d001a) {
            r.bool("UseNormalStemLengthsForBeamedRests")?;
        }
        if r.since(0x2d001b) {
            r.bool("AllowSecondaryBeamBreakOverGroupMiddle")?;
        }
        if r.since(0x2d001d) {
            pm_bool(r, "ShowBreveBarRests")?;
        }
        if r.since(0x2d001f) {
            r.bool("AllowStemletsOnBeamedRests")?;
            r.bool("UseHorizontalBeamsForStemlets")?;
            if r.file_version < 0x2d0022 {
                r.u32("ShortenStemletsBy")?;
            }
            r.u32("MinimumStemletValue")?;
        }
        if r.since(0x2d0021) {
            r.bool("ExtendStemletsIntoStaff")?;
        }
        if r.since(0x2d0022) {
            r.u32("DistanceBetweenRightOfRhythmDotAndFall")?;
            r.u32("DistanceBetweenRightOfNoteOnLineAndFall")?;
            r.u32("DistanceBetweenRightOfNoteOnSpaceAndFall")?;
            r.u32("DistanceBetweenLeftOfNoteOnLineAndScoop")?;
            r.u32("DistanceBetweenLeftOfNoteOnSpaceAndScoop")?;
            r.u32("DistanceBetweenCentreOfNoteOnLineAndFall")?;
            r.u32("DistanceBetweenCentreOfNoteOnSpaceAndFall")?;
            r.u32("DistanceBetweenBottomOfNoteOnLineAndScoop")?;
            r.u32("DistanceBetweenBottomOfNoteOnSpaceAndScoop")?;
        }
        if r.since(0x2d0025) {
            r.u32("DyForTopOfNoteAttachedArpeggio")?;
            r.u32("DyForBottomOfNoteAttachedArpeggio")?;
            r.u32("DxForNoteAttachedArpeggio")?;
            r.u32("ExtraYForArrowOnNoteAttachedArpeggio")?;
        }
        if r.since(0x2d0027) {
            r.u32("DistanceBetweenCentreOfNoteOnLineAndDoit")?;
            r.u32("DistanceBetweenCentreOfNoteOnSpaceAndDoit")?;
        }
        if r.since(0x2d002a) {
            pm_bool(r, "ShouldDrawAutoCautionaryAccidentals")?;
            pm_bool(r, "ShouldBracketAutoCautionaryAccidentals")?;
            pm_bool(r, "ShouldApplyAutoCautionaryAccidentalsToAllOctaves")?;
        }
        if r.since(0x2d002b) {
            r.bool("RestateTiedAccidentalsAtStartOfSystem")?;
            r.bool("BracketRestatedTiedAccidentals")?;
        }
        if r.since(0x2d002c) {
            pm_bool(r, "OnlyApplyAutoCautionariesToFirstNoteOfNextBar")?;
        }
        if r.since(0x2d002d) {
            r.bool("ResetAccidentalsOnClefChange")?;
        }
        if r.since(0x2d002e) {
            pm_bool(r, "RestateAccidentalsWhenSeenInNewVoice")?;
            pm_bool(r, "BracketRestatedAccidentalsInNewVoice")?;
        }
        if r.since(0x2d002f) {
            r.i16("ArticulationInsideSlur")?;
        }
        if r.since(0x2d0030) {
            r.bool("AllowFlippingOfAboveArticulations")?;
        }
        if r.since(0x2d0031) {
            r.bool("NewV6ArticulationPositionRule")?;
        }
        if r.since(0x2d0032) {
            r.u32("ArticulationStaveGap")?;
        }
        if r.since(0x2e0003) {
            r.bool("ShowStringPitchesAtLeftHandSide")?;
            r.bool("NumberEveryThirdFret")?;
            r.u32("DistanceBetweenFrameAndStringPitches")?;
            r.u32("DistanceBetweenFrameAndFretNumbers")?;
        }
        if r.since(0x310001) {
            r.node("ChordSymbolOptions", "ChordSymbolOptions", chord_symbol_options)?;
        }
        if r.since(0x310003) {
            r.bool("AllowExtraSpaceForAccentOrMarcatoInStave")?;
        }
        if r.since(0x310004) {
            r.i16("ArticulationInsideTuplet")?;
            r.u32("ArticulationDyOutsideTuplet")?;
        }
        if r.since(0x320006) {
            pm_bool(r, "UseDodger")?;
        }
        if r.since(0x320007) {
            r.u32("TieMiddleThickness")?;
            r.u32("SlurMiddleThickness")?;
        }
        if r.since(0x320008) {
            r.bool("ViewLiveTempo")?;
        }
        if r.since(0x320009) {
            r.u32("DefaultSlurHeightScale")?;
            r.u32("FixedMaximumSlurHeight")?;
            r.bool("UseFixedMaximumSlurHeight")?;
        }
        if r.since(0x32000a) {
            r.u32("MaxSlurShoulder")?;
        }
        if r.since(0x330001) {
            r.bool("DetermineSlurDirectionSolelyByLHSNote")?;
        }
        if r.since(0x340001) {
            r.u32("DistanceBetweenLeftOfNoteOnLineAndPlop")?;
            r.u32("DistanceBetweenLeftOfNoteOnSpaceAndPlop")?;
            r.u32("DistanceBetweenTopOfNoteOnLineAndPlop")?;
            r.u32("DistanceBetweenTopOfNoteOnSpaceAndPlop")?;
        }
        if r.since(0x350000) {
            r.bool("ApplyAngledBeamThresholdToRests")?;
        }
        if r.since(0x350001) {
            r.bool("ResetAutoCautionariesOnKeySignatureChange")?;
        }
        if r.since(0x350002) {
            r.bool("TupletsAlwaysAboveOnVocalStaves")?;
        }
        if r.since(0x360001) {
            r.u32("TupletHorizontalDistanceFromFirstNoteHead")?;
            r.u32("TupletHorizontalDistanceFromFirstNoteStem")?;
            r.u32("TupletHorizontalDistanceFromLastNoteHead")?;
            r.u32("TupletHorizontalDistanceFromLastNoteStem")?;
            r.u32("TupletHorizontalDistanceFromNoteAfter")?;
        }
        if r.since(0x360002) {
            r.bool("TupletDirectionDeterminedByImpliedBeam")?;
        }
        if r.since(0x360005) {
            r.bool("TupletAutoBracketAllowsNotesToBeJoinedBySecondaryBeam")?;
        }
        if r.since(0x360006) {
            r.bool("AllowCentredStaccatosWithOtherArticsAtSameEnd")?;
            r.bool("AllowCentredStaccatosWithOtherArticsAtOppositeEnd")?;
        }
        if r.since(0x360007) {
            pm_bool(r, "NumberSuccessiveTwoBarRepeats")?;
            pm_bool(r, "NumberSuccessiveFourBarRepeats")?;
        }
        if r.since(0x360008) {
            pm_bool(r, "NumberMultipleRepeats")?;
            pm_u32(r, "MultipleRepeatNumberingFormat")?;
        }
        if r.since(0x360009) {
            r.i16("ArticulationInsideSlurMiddle")?;
            r.bool("AllowArticulationsToSplitBelowMiddleOfTuplet")?;
        }
        if r.since(0x36000c) {
            r.u8("LegerLineProtrusionOnRest")?;
        }
        if r.since(0x36000d) {
            r.bool("AvoidCollisionsBetweenSlursAndOtherObjects")?;
            r.u32("MinimumGapAroundSlurs")?;
            r.u32("MagneticLayoutSlurHeight")?;
        }
        if r.since(0x360010) {
            r.bool("UsePreciseMagneticLayoutBoundingBoxes")?;
        }
        if r.since(0x360015) {
            r.f32("MagneticLayoutSlurLoopiness")?;
        }
        if r.since(0x360016) {
            r.u32("MagneticLayoutSlurHorizontalEndThreshold")?;
        }
        if r.since(0x360019) {
            r.bool("AdjustTextAlignmentOnSmallStaves")?;
        }
        if r.since(0x36001a) {
            r.bool("OmitHyphensWhenSpaceInsufficient")?;
        }
        if r.since(0x38000a) {
            r.bool("ViewOptions.m_ViewBarInfo")?;
            r.bool("ViewOptions.m_ViewBarInfoPanoramic")?;
            r.bool("ViewOptions.m_ViewHiddenObjects")?;
            r.bool("ViewOptions.m_ViewHandles")?;
            r.bool("ViewOptions.m_ViewPageMargins")?;
            r.bool("ViewOptions.m_ViewAttachmentInScores")?;
            r.bool("ViewOptions.m_ViewAttachmentInParts")?;
            r.bool("ViewOptions.m_ShowPlaybackLineWhenNotPlaying")?;
            r.bool("ViewOptions.m_ViewComments")?;
            r.bool("ViewOptions.m_ViewAutoHighlights")?;
            r.bool("ViewOptions.m_ViewPartPositionsInScores")?;
            r.bool("ViewOptions.m_ViewPartPositionsInParts")?;
            r.bool("ViewOptions.m_ViewDifferences")?;
            r.bool("ViewOptions.m_ViewMagneticLayoutCollisions")?;
            r.bool("ViewOptions.m_ViewMagneticLayoutBuckets")?;
            r.bool("ViewOptions.m_ViewMagneticLayoutGhosts")?;
            r.u32("ViewOptions.m_NoteColoring")?;
            r.bool("ViewOptions.m_ViewDynamicRulers")?;
            r.bool("ViewOptions.m_ViewObjectRulers")?;
            r.bool("ViewOptions.m_ViewSelectionRulers")?;
            r.bool("ViewOptions.m_HideAllInvisibleViewOptions")?;
        }
        if r.since(0x39000f) {
            r.bool("ExtendOldStyleMultirests")?;
        }
        if r.since(0x39001a) {
            r.string("FullNameFormat")?;
            r.string("ShortNameFormat")?;
            r.string("InstrumentChangeFormat")?;
            r.string("ChangeWarningFormat")?;
            r.string("UnisonPrefix")?;
            r.string("PlayerJoin")?;
            r.string("PlayerLastJoin")?;
            pm_u32(r, "InstrumentBraceDx")?;
            r.bool("StackPlayerNumbers")?;
            r.bool("VCentreInstrumentName")?;
        }
        if r.since(0x3b0000) {
            r.bool("ShowHiddenStavesInFocusOnStaves")?;
        }
        if r.since(0x3d000c) {
            r.bool("PreferNotesToBarRestsWhenPairingVoices")?;
        }
        if r.since(0x3d000f) {
            r.bool("ViewOptions.m_ViewAnnotations")?;
        }
        if r.since(0x3dffff) {
            r.bool("RestsAvoidOthers")?;
            r.bool("RestsFollowNotes")?;
            r.node("DefaultRestPhraseGrouping", "DefaultRestPhraseGrouping", default_rest_phrase_grouping)?;
            r.bool("RestsContinue")?;
        }
        options_tail(r)?;
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100f09a16`.
pub(super) fn f_100f09a16(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.f32("DefaultVoiceVolume")?;
    r.u8("TremoloNoteLength")?;
    r.u8("SlurNoteLength")?;
    r.u8("NormalNoteLength")?;
    r.f32("ArpeggiateTime")?;
    r.u8("Espressivo")?;
    r.bool("RandomVolumes")?;
    r.bool("AccentBeams")?;
    r.f32("GraceNoteDurationPerSu")?;
    if v < 0x3d0000 {
        if v < 0x100000 {
            r.u16("Swing")?;
        } else {
            r.node("Swing", "StyleId", style_id)?;
        }
    } else {
        r.node("SwingPattern", "SwingPattern", swing_pattern)?;
    }
    list_articulations(r, "Articulations")?;
    r.bool("SendReverbData")?;
    r.u8("AverageReverb")?;
    r.bool("SendPanData")?;
    r.record("record", "Record", |r| {
        if r.since(0x20007) {
            r.f32("MinTremoloTime")?;
            r.bool("SendDetuneData")?;
            if r.since(0x30001) {
                r.bool("PlayRepeats")?;
                r.bool("ShortenDotted")?;
                r.bool("ResetControllers")?;
                r.bool("SendBankChangesMSB")?;
                r.bool("SendBankChangesLSB")?;
                r.bool("SendProgramChanges")?;
                r.bool("SwingAsShorthand")?;
            }
            if r.since(0x50005) {
                r.i32("SoloState")?;
                r.u32("SoloedStave")?;
                r.u32("FirstStaveOnMixer")?;
            }
            if r.since(0x60001) {
                r.u32("RubatoLevel")?;
            }
            if r.since(0x60005) {
                r.f32("MaxLengthAdjustment")?;
            }
            if r.since(0x80011) {
                r.bool("DynamicsAffectAdjacentStaves")?;
                r.u32("SpacesForMultistaveDynamics")?;
            }
            if r.since(0x80013) {
                r.bool("LiveMode")?;
            }
            if r.since(0x8001b) {
                r.u32("FineLengthInBeats")?;
            }
            if r.since(0xa001b) {
                r.bool("TiesBetweenVoices")?;
            }
            if r.since(0x230000) {
                r.string("ConfigurationName")?;
            }
            if r.since(0x280007) {
                r.f32("DefaultVoiceAttack")?;
            }
            if r.since(0x2a0002) {
                r.bool("ConvertLiveVelocitiesToDynamics")?;
            }
            if r.since(0x2c0001) {
                r.bool("ShowGroupStripsInMixer")?;
                r.bool("ShowStaffStripsInMixer")?;
            }
            if r.since(0x330008) {
                r.bool("LiveTempoBeatInIsInBars")?;
                r.u32("LiveTempoBeatIn")?;
                r.bool("UseLiveTempoPlayback")?;
                r.bool("FactoriseLiveTempoRecording")?;
                r.f32("LiveTempoRecordingSmoothingFactor")?;
            }
            if r.since(0x340004) {
                r.u32("ReverbType")?;
                if r.file_version < 0x340007 {
                    r.bool("UseConvolutionReverb")?;
                    r.bool("UseConvolutionReverbInFlexiTime")?;
                }
                r.u8("SibeliusPlayerReverbLevel")?;
                r.u8("SibeliusPlayerChorusLevel")?;
            }
            if r.since(0x340005) {
                r.f32("MasterVolumedB")?;
            }
            if r.since(0x340007) {
                r.node("PlaybackConfigParams", "PlaybackConfigParams", playback_config_params)?;
            }
            if r.since(0x360000) {
                r.bool("PlayGapAfterSectionEnds")?;
            }
            if r.since(0x360003) {
                r.bool("PlayRepeatsAfterDSDC")?;
                r.bool("UseManualBarPlaySequence")?;
                r.string("ManualBarPlaySequenceString")?;
            }
            if r.since(0x36001c) {
                r.bool("MixerFadersScaleNoteVelocity")?;
            }
            if r.since(0x3d0000) {
                r.node("MeterEmphasisSettings", "MeterEmphasisSettings", meter_emphasis_settings)?;
            }
            if r.since(0x3d0002) {
                r.bool("AppoggiaturasPlayOnTheBeat")?;
            }
            if r.since(0x3d0003) {
                r.u32("ShortAppoggiaturaDuration_ms")?;
            }
            if r.since(0x3d0004) {
                r.bool("UseNewEspressivo")?;
                r.u32("NewEspressivoPreset")?;
            }
            if r.since(0x3d0005) {
                r.node("PlaybackNuanceSettings", "PlaybackNuanceSettings", playback_nuance_settings)?;
            }
            if r.since(0x3d0009) {
                r.bool("PlayBackMordents")?;
            }
        }
        playing_options_tail(r)?;
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x10104f731`.
pub(super) fn f_10104f731(r: &mut Reader) -> Result<()> {
    r.any_tag()?;
    f_10104f778(r)?;
    Ok(())
}

/// 8.3.0 `0x100f37244`.
pub(super) fn f_100f37244(r: &mut Reader) -> Result<()> {
    r.tag(0x4f)?;
    r.string("Name")?;
    r.i32("StartIndex")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x1010056ae`.
pub(super) fn list_note_styles(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListNoteStylesItem", |r| {
            note_style(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x10100689f`.
pub(super) fn list_indexed_note_styles(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListIndexedNoteStylesItem", |r| {
            f_101008c06(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x101007c7c`.
pub(super) fn list_ensemble_list(r: &mut Reader, name: &'static str) -> Result<()> {
    if r.file_version <= 0xfffff {
        // the count is stored without items
        r.u32(name)?;
        return Ok(());
    }
    r.list(name, |r| {
        r.sub("ListEnsembleListItem", |r| {
            f_100d975c4(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100ef299e`.
pub(super) fn f_100ef299e(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x1e0000 {
        pm_u16(r, "DefaultBarRestWidth")?;
    } else {
        pm_u32(r, "DefaultBarRestWidth")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "DefaultStartOfBarGap")?;
    } else {
        pm_u32(r, "DefaultStartOfBarGap")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "Minimum")?;
    } else {
        pm_u32(r, "Minimum")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "Sixteenth")?;
    } else {
        pm_u32(r, "Sixteenth")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "Eighth")?;
    } else {
        pm_u32(r, "Eighth")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "Quarter")?;
    } else {
        pm_u32(r, "Quarter")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "Half")?;
    } else {
        pm_u32(r, "Half")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "Whole")?;
    } else {
        pm_u32(r, "Whole")?;
    }
    if v < 0x1e0000 {
        pm_u16(r, "DoubleWhole")?;
    } else {
        pm_u32(r, "DoubleWhole")?;
    }
    r.record("record", "Record", |r| {
        if r.since(0x30006) {
            pm_bool(r, "IncludeLyrics")?;
        }
        if r.since(0xa0004) {
            if r.file_version < 0x1e0000 {
                pm_u16(r, "HookWidth")?;
            } else {
                pm_u32(r, "HookWidth")?;
            }
        }
        if r.since(0xa0008) {
            if r.file_version < 0x1e0000 {
                pm_u16(r, "MinSpaceAroundNote")?;
            } else {
                pm_u32(r, "MinSpaceAroundNote")?;
            }
            if r.file_version < 0x1e0000 {
                pm_u16(r, "MinSpaceBeforeAccidental")?;
            } else {
                pm_u32(r, "MinSpaceBeforeAccidental")?;
            }
            if r.file_version < 0x1e0000 {
                pm_u16(r, "MinSpaceAroundLegerLine")?;
            } else {
                pm_u32(r, "MinSpaceAroundLegerLine")?;
            }
            if r.file_version < 0x1e0000 {
                pm_u16(r, "MinSpaceAtStartOfBar")?;
            } else {
                pm_u32(r, "MinSpaceAtStartOfBar")?;
            }
            if r.file_version < 0x1e0000 {
                pm_u16(r, "MinSpaceAtEndOfBar")?;
            } else {
                pm_u32(r, "MinSpaceAtEndOfBar")?;
            }
            if r.file_version < 0x1e0000 {
                pm_u16(r, "SpaceAroundGraceNote")?;
            } else {
                pm_u32(r, "SpaceAroundGraceNote")?;
            }
            if r.file_version < 0x1e0000 {
                pm_u16(r, "ExtraSpaceAfterLastGraceNote")?;
            } else {
                pm_u32(r, "ExtraSpaceAfterLastGraceNote")?;
            }
        }
        if r.since(0xa000a) {
            pm_bool(r, "AllowFirstLyricOverhang")?;
        }
        if r.since(0xa000b) {
            pm_bool(r, "AllowSpaceForHyphen")?;
        }
        if r.since(0xa0010) {
            pm_bool(r, "AllowSpaceForVoiceConflicts")?;
        }
        if r.since(0x300005) {
            pm_bool(r, "IncludeChordSymbols")?;
        }
        if r.since(0x310009) {
            pm_bool(r, "DetermineEmptyBarWidthBySrLength")?;
        }
        if r.since(0x33000b) {
            pm_u32(r, "ExtraSpaceBetweenGuitarFrames")?;
        }
        if r.since(0x340000) {
            pm_u32(r, "MinSpaceBeforeArpeggio")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x101005042`.
pub(super) fn list_symbols(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListSymbolsItem", |r| {
            editable(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x10100682f`.
pub(super) fn list_indexed_symbols(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListIndexedSymbolsItem", |r| {
            f_101008f00(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x10105257d`.
pub(super) fn f_10105257d(r: &mut Reader) -> Result<()> {
    r.any_tag()?;
    f_1010525c4(r)?;
    Ok(())
}

/// 8.3.0 `0x1010118f2`.
pub(super) fn list_line_styles(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListLineStylesItem", |r| {
            f_100f328d6(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100d8d7d8`.
pub(super) fn mididata(r: &mut Reader) -> Result<()> {
    r.tag(0x3d)?;
    f_100d8d6e2(r)?;
    Ok(())
}

/// 8.3.0 `0x100f3e2aa`.
pub(super) fn f_100f3e2aa(r: &mut Reader) -> Result<()> {
    midi_settings(r)?;
    r.u32("MuteMode")?;
    Ok(())
}

/// 8.3.0 `0x100e133a8`.
pub(super) fn f_100e133a8(r: &mut Reader) -> Result<()> {
    list_array(r, "Array")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100a0cf26`.
pub(super) fn f_100a0cf26(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x7b)?;
    r.u32("mask")?;
    r.bool("fixed")?;
    r.bool("repulsive")?;
    r.bool("splittable")?;
    r.bool("cohesive")?;
    r.bool("invisible")?;
    r.u32("leftFreedom")?;
    r.u32("rightFreedom")?;
    r.u32("topFreedom")?;
    r.u32("bottomFreedom")?;
    r.u32("priority")?;
    if v < 0x300000 {
        r.u32("bucketTypeHorz")?;
        r.u32("bucketTypeVert")?;
    }
    r.u32("bucketXLeniency")?;
    r.u32("bucketYLeniency")?;
    r.u32("leftBoundary")?;
    r.u32("rightBoundary")?;
    r.u32("topBoundary")?;
    r.u32("bottomBoundary")?;
    r.u32("drawLayer")?;
    r.record("record", "Record", |r| {
        if r.since(0x2fffff) {
            r.string("xBucket")?;
            r.string("yBucket")?;
        }
        if r.since(0x320005) {
            r.u32("bucketXExtensibility")?;
            r.u32("bucketYExtensibility")?;
        }
        if r.since(0x330011) {
            r.u32("handicap")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100e06002`.
pub(super) fn style_header(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x120000 {
        if v > 0xfffff && v < 0x120000 {
            f_100e6aed2(r)?;
        } else {
            style_name(r)?;
        }
    } else {
        style_name(r)?;
        r.node("Id", "StyleId", style_id)?;
        r.bool("Compulsory")?;
        r.record("record", "Record", |_| Ok(()))?;
    }
    Ok(())
}

/// 8.3.0 `0x1010d9754`.
pub(super) fn f_1010d9754(r: &mut Reader) -> Result<()> {
    r.u32("Height")?;
    r.string("FontName")?;
    r.bool("Bold")?;
    r.bool("Italic")?;
    r.bool("Underline")?;
    r.u8("Leading")?;
    r.u32("PartHeight")?;
    r.f32("OriginalStaveHeightInScore")?;
    r.bool("FixedHeightInScore")?;
    r.bool("FixedHeightInParts")?;
    r.record("record", "Record", |r| {
        if r.since(0x380004) {
            r.bool("IsHeightSet")?;
            r.bool("IsFontNameSet")?;
            r.bool("IsBoldFlagSet")?;
            r.bool("IsItalicFlagSet")?;
            r.bool("IsUnderlineFlagSet")?;
            r.bool("IsLeadingSet")?;
        }
        if r.since(0x390000) {
            r.u32("VerticalScale")?;
            r.u32("HorizontalScale")?;
            r.u32("Tracking")?;
            r.u32("NamedBaseline")?;
            r.bool("IsVerticalScaleSet")?;
            r.bool("IsHorizontalScaleSet")?;
            r.bool("IsTrackingSet")?;
            r.bool("IsNamedBaselineSet")?;
        }
        if r.since(0x39000a) {
            r.string("FontStyleName")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100f539f0`.
pub(super) fn f_100f539f0(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x43)?;
    pm_bool(r, "Visible")?;
    if v < 0x1e0000 {
        pm_u16(r, "Y")?;
    } else {
        pm_u32(r, "Y")?;
    }
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100f3fb74`.
pub(super) fn f_100f3fb74(r: &mut Reader) -> Result<()> {
    r.u32("BeamType")?;
    r.u16("Sr")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100dfc04a`.
pub(super) fn spacing(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x38)?;
    if v < 0x1e0000 {
        r.u16("GapBefore")?;
    } else {
        r.u32("GapBefore")?;
    }
    r.u16("Sr")?;
    r.u16("Count")?;
    r.record("record", "Record", |r| {
        if r.since(0xa0005) {
            if r.file_version < 0x1e0000 {
                r.u16("SolidAtStart")?;
            } else {
                r.u32("SolidAtStart")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("SolidAtEnd")?;
            } else {
                r.u32("SolidAtEnd")?;
            }
        }
        if r.since(0xa000f) {
            if r.file_version < 0x1e0000 {
                r.u16("LastNoteSpace")?;
            } else {
                r.u32("LastNoteSpace")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("LastAutoSpace")?;
            } else {
                r.u32("LastAutoSpace")?;
            }
        }
        spacing_tail(r)?;
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x101019861`.
pub(super) fn list_part_map_default_9861(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListPartMapDefault9861Item", |r| {
            f_100fda178(r, "__Pair[]")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x1010199af`.
pub(super) fn f_1010199af(r: &mut Reader) -> Result<()> {
    r.tag(0x6d)?;
    r.u8("MMMarginTypeLabel")?;
    r.f32("MMPageLeftMarginmmLabel")?;
    r.f32("MMPageRightMarginmmLabel")?;
    r.f32("MMPageTopMarginmmLabel")?;
    r.f32("MMPageBottomMarginmmLabel")?;
    r.f32("MMRightPageLeftMarginmmLabel")?;
    r.f32("MMRightPageRightMarginmmLabel")?;
    r.u32("MMAboveTopStaveGapSuLabel")?;
    r.u32("MMBelowBottomStaveGapSuLabel")?;
    r.u32("MMAboveTopStaveGapAfterFirstPageSuLabel")?;
    r.u32("MMBelowBottomStaveGapAfterFirstPageSuLabel")?;
    r.bool("MMUseValuesAfterFirstPageLabel")?;
    r.u32("MMStaffLeftMarginNoNamesSuLabel")?;
    r.u32("MMStaffLeftMarginShortNamesSuLabel")?;
    r.u32("MMStaffLeftMarginFullNamesSuLabel")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x101022750`.
pub(super) fn list_part_map_default_2750(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListPartMapDefault2750Item", |r| {
            f_101193236(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100f44324`.
pub(super) fn f_100f44324(r: &mut Reader) -> Result<()> {
    r.tag(0x53)?;
    f_100f44586(r)?;
    Ok(())
}

/// 8.3.0 `0x10104ddea`.
pub(super) fn f_10104ddea(r: &mut Reader) -> Result<()> {
    r.node("StyleId", "StyleId", style_id)?;
    r.string("SoundIDChange")?;
    r.node("StyleType", "StyleType", style_type)?;
    r.string("ParamString")?;
    r.record("record", "Record", |r| {
        if r.since(0x2d0007) {
            r.bool("ExtendSoundIDChangeToEndOfLastNote")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x1010e8e00`.
pub(super) fn offsets(r: &mut Reader) -> Result<()> {
    r.list("items", |r| {
        r.sub("ItemsItem", |r| {
            r.u16("value")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100f73b5e`.
pub(super) fn f_100f73b5e(r: &mut Reader) -> Result<()> {
    r.tag(0x32)?;
    style_header(r)?;
    r.string("FullName")?;
    r.string("ShortName")?;
    r.string("DialogName")?;
    r.string("NativeFullName")?;
    r.string("NativeShortName")?;
    r.string("NativeDialogName")?;
    r.string("BracketGroup")?;
    r.u8("TopNote")?;
    r.u8("BottomNote")?;
    r.u8("ComfortableTopNote")?;
    r.u8("ComfortableBottomNote")?;
    r.node("StaveTypeId", "F100fa7c28", f_100fa7c28)?;
    r.u8("NumStaves")?;
    r.node("Clef", "F100fa77fa", f_100fa77fa)?;
    r.node("OtherClef", "F100fa77fa", f_100fa77fa)?;
    r.node("PartClef", "F100fa77fa", f_100fa77fa)?;
    r.node("GlissType", "GlissType", gliss_type)?;
    r.u16("PartDiat")?;
    r.u16("PartChrom2")?;
    r.u16("ScoreDiat")?;
    r.u16("ScoreChrom2")?;
    r.i8("Pan")?;
    r.u8("Balance")?;
    r.u8("Distance")?;
    r.u8("MIDIFileProb")?;
    r.string("DefaultSoundID")?;
    r.record("record", "Record", |r| {
        if r.since(0x2d000d) && r.file_version < 0x3a0000 {
            r.string("InstrumentChangeName")?;
            r.string("InstrumentChangeWarningName")?;
            r.string("NativeInstrumentChangeName")?;
            r.string("NativeInstrumentChangeWarningName")?;
        }
        if r.since(0x300001) {
            r.bool("IsVocal")?;
        }
        if r.since(0x330007) {
            r.node("TabInstrumentStyleIdToUseForStringTunings", "StyleId", style_id)?;
        }
        if r.since(0x360011) {
            r.bool("ShowGuitarDiagramsByDefault")?;
        }
        if r.since(0x39001b) {
            r.bool("ShowTransposition")?;
        }
        if r.since(0x39001c) {
            r.string("TranspositionName")?;
        }
        instrument_type_tail(r)?;
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100deb984`.
pub(super) fn f_100deb984(r: &mut Reader) -> Result<()> {
    r.tag(0x31)?;
    style_header(r)?;
    r.node("Symbol", "Symbol", symbol)?;
    r.u8("Diat")?;
    r.i8("CentrePos")?;
    r.u8("MaxSharpPos")?;
    r.u8("MaxFlatPos")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100ec03bc`.
pub(super) fn f_100ec03bc(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x380000 {
        f_100ec102c(r)?;
    } else {
        r.tag(0x35)?;
        style_header(r)?;
        f_1010d9754(r)?;
        r.u32("Angle")?;
        r.i32("BBits")?;
        r.u8("BoxTop")?;
        r.u8("BoxBottom")?;
        r.u8("BoxLeft")?;
        r.u8("BoxRight")?;
        r.i32("Sizes")?;
        r.u8("Alignment")?;
        r.u8("SnapType")?;
        r.u8("RepeatType")?;
        r.string("MusicTerms")?;
        r.bool("NotOn1stPage")?;
        r.node("TypeOptions", "TypeOptions", type_options)?;
        r.bool("UseMarginsRelativeToFirstPageX")?;
        r.bool("UseMarginsRelativeToFirstPageY")?;
        r.record("record", "Record", |r| {
            if r.since(0x38000c) {
                r.node("ParentTextStyleId", "StyleId", style_id)?;
            }
            if r.since(0x38000d) {
                r.bool("IsCircledSet")?;
                r.bool("IsBoxedSet")?;
                r.bool("IsWhiteBackgroundSet")?;
                r.bool("AreSectionsSet")?;
                r.bool("IsSpaceForNextNoteSet")?;
                r.bool("IsSpaceForNextBeatSet")?;
                r.bool("IsHyphenForNextNoteSet")?;
                r.bool("IsSnapToMarginSet")?;
                r.bool("IsAlignToStaffMarginsSet")?;
                r.bool("IsAlignXToPageSet")?;
                r.bool("IsTransposableSet")?;
                r.bool("IsPlayableSet")?;
                r.bool("IsAngleSet")?;
                r.bool("IsBoxSet")?;
                r.bool("IsAlignmentSet")?;
                r.bool("IsSnapTypeSet")?;
                r.bool("IsRepeatTypeSet")?;
                r.bool("IsNotOn1stPageSet")?;
                r.bool("IsMusicTermsSet")?;
                r.bool("AreEditableTypeOptionsSet")?;
                r.bool("IsUseMarginsRelativeToFirstPageXSet")?;
                r.bool("IsUseMarginsRelativeToFirstPageYSet")?;
            }
            if r.since(0x38000f) {
                r.bool("HasFrame")?;
                r.u32("FrameWidth")?;
                r.u32("FrameHeight")?;
                r.bool("IsFrameSet")?;
            }
            if r.since(0x390002) {
                r.u32("BackgroundColour")?;
                r.bool("IsBackgroundColourSet")?;
            }
            if r.since(0x390003) {
                r.u32("ForegroundColour")?;
                r.bool("IsForegroundColourSet")?;
            }
            if r.since(0x390006) {
                r.u32("IndentOfFirstLine")?;
                r.u32("IndentAfterFirstLine")?;
                r.u32("GapAfterParagraph")?;
                r.bool("IsIndentOfFirstLineSet")?;
                r.bool("IsIndentAfterFirstLineSet")?;
                r.bool("IsGapAfterParagraphSet")?;
            }
            if r.since(0x39000b) {
                r.node("FrameVerticalAlignment", "FrameVerticalAlignment", frame_vertical_alignment)?;
                r.bool("IsFrameVerticalAlignmentSet")?;
            }
            if r.since(0x390015) {
                r.node("TextAlignment", "TextAlignment", text_alignment)?;
                r.bool("IsTextAlignmentSet")?;
            }
            text_style_tail(r)?;
            Ok(())
        })?;
    }
    Ok(())
}

/// 8.3.0 `0x100f0836c`.
pub(super) fn pen_style(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x39)?;
    style_header(r)?;
    if v < 0x1e0000 {
        r.u16("Width")?;
    } else {
        r.u32("Width")?;
    }
    r.u8("Sizes")?;
    r.bool("Alias")?;
    r.bool("LineStylePen")?;
    r.u8("ScaledSizes")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x10105343e`.
pub(super) fn f_10105343e(r: &mut Reader) -> Result<()> {
    r.u32("StyleNum")?;
    r.string("SoundIDChange")?;
    r.node("StyleType", "StyleType3363", style_type_3363)?;
    r.string("ParamString")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100e503c4`.
pub(super) fn pm_f_100fa28f4(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, |r| r.sub("F100fa28f4", f_100fa28f4).map(Value::Node))
}

/// 8.3.0 `0x100dd7592`.
pub(super) fn extract_options(r: &mut Reader) -> Result<()> {
    r.tag(0x57)?;
    f_100dd70ac(r)?;
    Ok(())
}

/// 8.3.0 `0x100dff26b`.
pub(super) fn default_barline_type(r: &mut Reader) -> Result<()> {
    r.u8("value")?;
    Ok(())
}

/// 8.3.0 `0x100e509eb`.
pub(super) fn list_editable_options_list(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListEditableOptionsListItem", |r| {
            type_options(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x10118ce8c`.
pub(super) fn page_and_stave_margins(r: &mut Reader) -> Result<()> {
    r.tag(0x69)?;
    r.record("record", "Record", |r| {
        pm_f32(r, "PageWidthmm")?;
        pm_f32(r, "PageHeightmm")?;
        pm_f32(r, "StaveHeightmm")?;
        pm_u8(r, "MarginType")?;
        pm_u32(r, "PageWidthSu")?;
        pm_u32(r, "PageHeightSu")?;
        pm_f32(r, "PageLeftMarginmm")?;
        pm_f32(r, "PageRightMarginmm")?;
        pm_f32(r, "PageTopMarginmm")?;
        pm_f32(r, "PageBottomMarginmm")?;
        pm_f32(r, "RightPageLeftMarginmm")?;
        pm_f32(r, "RightPageRightMarginmm")?;
        pm_u32(r, "StaffLeftMarginNoNamesSu")?;
        pm_u32(r, "StaffLeftMarginShortNamesSu")?;
        pm_u32(r, "StaffLeftMarginFullNamesSu")?;
        pm_u32(r, "AboveTopStaveGapSu")?;
        pm_u32(r, "BelowBottomStaveGapSu")?;
        pm_u32(r, "AboveTopStaveGapAfterFirstPageSu")?;
        pm_u32(r, "BelowBottomStaveGapAfterFirstPageSu")?;
        pm_bool(r, "UseValuesAfterFirstPage")?;
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x1010b5e6a`.
pub(super) fn chord_symbol_options(r: &mut Reader) -> Result<()> {
    r.tag(0x7d)?;
    f_1010b5900(r)?;
    Ok(())
}

/// 8.3.0 `0x100eb87b0`.
pub(super) fn default_rest_phrase_grouping(r: &mut Reader) -> Result<()> {
    r.u8("value")?;
    Ok(())
}

/// 8.3.0 `0x1010ea87d`.
pub(super) fn swing_pattern(r: &mut Reader) -> Result<()> {
    r.any_tag()?;
    f_1010ea71e(r)?;
    Ok(())
}

/// 8.3.0 `0x100f11dc2`.
pub(super) fn list_articulations(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListArticulationsItem", |r| {
            f_100dec698(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100bb335b`.
pub(super) fn playback_config_params(r: &mut Reader) -> Result<()> {
    r.tag(0x88)?;
    f_100bb2d20(r)?;
    Ok(())
}

/// 8.3.0 `0x1009b42a2`.
pub(super) fn meter_emphasis_settings(r: &mut Reader) -> Result<()> {
    r.any_tag()?;
    f_1009b4168(r)?;
    Ok(())
}

/// 8.3.0 `0x1009b9ebe`.
pub(super) fn playback_nuance_settings(r: &mut Reader) -> Result<()> {
    r.any_tag()?;
    f_1009b9f12(r)?;
    Ok(())
}

/// 8.3.0 `0x10104f778`.
pub(super) fn f_10104f778(r: &mut Reader) -> Result<()> {
    r.u32("StyleNum")?;
    r.string("SoundIDChange")?;
    r.node("StyleType", "StyleTypeF69d", style_type_f69d)?;
    r.string("ParamString")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x101008c06`.
pub(super) fn f_101008c06(r: &mut Reader) -> Result<()> {
    r.tag(0x34)?;
    f_101008c4e(r)?;
    Ok(())
}

/// 8.3.0 `0x100d975c4`.
pub(super) fn f_100d975c4(r: &mut Reader) -> Result<()> {
    r.tag(0x6e)?;
    style_header(r)?;
    r.string("FamilySelectedByDefault")?;
    r.string("InstrumentSelectedByDefault")?;
    list_families(r, "Families")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100f37634`.
pub(super) fn editable(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x36)?;
    if v < 0x1d0000 {
        r.string("Name")?;
    } else {
        style_name(r)?;
    }
    r.u8("Character")?;
    if v < 0x1e0000 {
        r.u16("Dx")?;
    } else {
        r.u32("Dx")?;
    }
    if v < 0x1e0000 {
        r.u16("Dy")?;
    } else {
        r.u32("Dy")?;
    }
    r.node("TextStyleNum", "F100fa75d2", f_100fa75d2)?;
    list_sub_symbols(r, "SubSymbols")?;
    r.record("record", "Record", |r| {
        if r.since(0x3000a) {
            r.u16("UnicodeChar")?;
        }
        if r.since(0x370002) {
            let is_graphic = r.bool("IsGraphic")?;
            if is_graphic {
                r.node("Graphic", "GraphicCc3a", graphic_cc3a)?;
            }
        }
        if r.since(0x38000b) {
            r.f32("Scale")?;
            r.f32("Rotation")?;
        }
        if r.since(0x390008) {
            r.u32("DefaultAttachment")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x101008f00`.
pub(super) fn f_101008f00(r: &mut Reader) -> Result<()> {
    r.tag(0x34)?;
    f_101008f48(r)?;
    Ok(())
}

/// 8.3.0 `0x1010525c4`.
pub(super) fn f_1010525c4(r: &mut Reader) -> Result<()> {
    r.u32("StyleNum")?;
    r.string("SoundIDChange")?;
    r.record("record", "Record", |r| {
        if r.since(0x2d0002) {
            r.node("SoundIDChangeDuration", "SoundIdchangeDuration", sound_idchange_duration)?;
        }
        if r.since(0x390017) {
            r.node("StyleType", "StyleType24e9", style_type_24e9)?;
            r.string("ParamString")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100f328d6`.
pub(super) fn f_100f328d6(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x3a)?;
    style_header(r)?;
    r.node("PenStyle", "PenStyle", pen_style)?;
    r.u8("LineType")?;
    r.node("StartSymbol", "StartSymbol", start_symbol)?;
    r.node("MidSymbol", "StartSymbol", start_symbol)?;
    r.node("EndSymbol", "StartSymbol", start_symbol)?;
    r.bool("Horizontal")?;
    r.bool("Vertical")?;
    r.bool("LineXAfterSymbol")?;
    if v < 0x1e0000 {
        r.u16("DashLength")?;
    } else {
        r.u32("DashLength")?;
    }
    if v < 0x1e0000 {
        r.u16("GapLength")?;
    } else {
        r.u32("GapLength")?;
    }
    r.u8("ArrowType")?;
    if v < 0x1e0000 {
        r.u16("StartCapNotchDy")?;
    } else {
        r.u32("StartCapNotchDy")?;
    }
    r.u8("EndCapArrowType")?;
    if v < 0x1e0000 {
        r.u16("EndCapNotchDy")?;
    } else {
        r.u32("EndCapNotchDy")?;
    }
    if v < 0x1e0000 {
        r.u16("MinimumLength")?;
    } else {
        r.u32("MinimumLength")?;
    }
    r.node("Category", "Category", category)?;
    r.u8("RepeatTimeLineType")?;
    r.i8("OctavaLineDeltaQuarterTones")?;
    r.u8("ArpeggioLineType")?;
    if v < 0x1e0000 {
        r.u16("SlurMidPointDy")?;
    } else {
        r.u32("SlurMidPointDy")?;
    }
    let caption_enabled = r.bool("CaptionEnabled")?;
    if caption_enabled {
        r.node("CaptionRef", "StyledString", styled_string)?;
        if v < 0x1e0000 {
            r.u16("CaptionDy")?;
        } else {
            r.u32("CaptionDy")?;
        }
    }
    r.record("record", "Record", |r| {
        if caption_enabled && r.since(0x20001) {
            r.bool("CaptionAlignment")?;
        }
        if r.since(0x30004) {
            r.bool("IsRitard")?;
        }
        if r.since(0x60003) {
            r.node("TypeOptions", "TypeOptions", type_options)?;
        }
        if r.since(0x60006) {
            if r.file_version < 0x1e0000 {
                r.u16("DefaultRhsDx")?;
            } else {
                r.u32("DefaultRhsDx")?;
            }
        }
        if caption_enabled && r.since(0x70004) {
            if r.file_version < 0x1e0000 {
                r.u16("CaptionDx")?;
            } else {
                r.u32("CaptionDx")?;
            }
            r.bool("CaptionOn2ndSystem")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100d8d6e2`.
pub(super) fn f_100d8d6e2(r: &mut Reader) -> Result<()> {
    midi_settings(r)?;
    r.u8("ClickPitch")?;
    r.u16("ClickPeriod")?;
    r.u16("ClickLength")?;
    r.bool("FollowBeat")?;
    r.u16("OldBarsIntro")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100e13784`.
pub(super) fn list_array(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListArrayItem", |r| {
            f_100e13492(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100e6aed2`.
pub(super) fn f_100e6aed2(r: &mut Reader) -> Result<()> {
    r.string("Name")?;
    r.node("Id", "StyleId", style_id)?;
    r.bool("Compulsory")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100e6ae7c`.
pub(super) fn style_name(r: &mut Reader) -> Result<()> {
    r.string("Name")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100f44586`.
pub(super) fn f_100f44586(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x150000 {
        r.string("Name")?;
    } else {
        style_header(r)?;
    }
    let category = r.u32("Category")?;
    r.bool("AsNumbers")?;
    r.bool("NotesOnSpaces")?;
    r.bool("ShowRhythms")?;
    r.bool("Hiding")?;
    r.bool("Default")?;
    let stave_lines = r.i8("StaveLines")?;
    if v < 0x1e0000 {
        r.u16("StaveLineSeparation")?;
    } else {
        r.u32("StaveLineSeparation")?;
    }
    if v < 0x1e0000 {
        r.u16("value")?;
    } else {
        r.u32("value")?;
    }
    r.bool("LegerLines")?;
    r.bool("KeySignatures")?;
    r.bool("InitialClef")?;
    r.bool("Bracket")?;
    r.bool("InitBarline")?;
    r.bool("BarRests")?;
    r.bool("Barlines")?;
    r.list_n("RestY", 10, |r| {
        r.sub("RestY", |r| {
            if v < 0x1e0000 {
                r.u16("RestY")?;
            } else {
                r.u32("RestY")?;
            }
            Ok(())
        })
        .map(Value::Node)
    })?;
    r.list_n("TopRestY", 10, |r| {
        r.sub("TopRestY", |r| {
            if v < 0x1e0000 {
                r.u16("TopRestY")?;
            } else {
                r.u32("TopRestY")?;
            }
            Ok(())
        })
        .map(Value::Node)
    })?;
    r.list_n("BottomRestY", 10, |r| {
        r.sub("BottomRestY", |r| {
            if v < 0x1e0000 {
                r.u16("BottomRestY")?;
            } else {
                r.u32("BottomRestY")?;
            }
            Ok(())
        })
        .map(Value::Node)
    })?;
    if stave_lines != 0 {
        if category == 2 {
            f_100f47537(r, "Pitches")?;
        } else if category == 1 {
            f_100dd2272(r)?;
        }
    }
    r.record("record", "Record", |r| {
        if r.since(0x3000e) {
            r.bool("StemsUp")?;
        }
        if r.since(0x50009) {
            r.bool("ArticulationsAbove")?;
        }
        if r.since(0x90007) {
            r.bool("LevelBeams")?;
        }
        if r.since(0x90007) {
            r.bool("StemsAboveOrBelowStave")?;
        }
        if r.since(0x90009) {
            r.bool("WhiteBehindTabNotes")?;
        }
        if r.since(0xa0000) {
            if r.file_version < 0x1e0000 {
                r.u16("NumBeamsToAllowOnExtendedStem")?;
            } else {
                r.u32("NumBeamsToAllowOnExtendedStem")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("ExtraGapToAllowOnExtendedStem")?;
            } else {
                r.u32("ExtraGapToAllowOnExtendedStem")?;
            }
            r.bool("ClipExtendedStemsInsideExtraGap")?;
        }
        if r.since(0xa0001) {
            r.bool("AllVoice1StemsUp")?;
            r.bool("AllVoice1StemsDown")?;
        }
        if r.since(0xa0002) {
            r.bool("DrawStemsBetweenTabChordNotes")?;
        }
        if r.since(0x1b000d) && (category == 2) {
            f_100f47537(r, "Diats")?;
        }
        if r.since(0x2d0004) {
            r.u32("BarlineBottomDy")?;
        }
        if r.since(0x330003) && r.file_version < 0x330008 {
            r.node("TabInstrumentStyleIdToUseForStringTunings", "StyleId", style_id)?;
        }
        stave_type_tail(r)?;
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x10104dd0f`.
pub(super) fn style_type(r: &mut Reader) -> Result<()> {
    r.string("value")?;
    Ok(())
}

/// 8.3.0 `0x100ec102c`.
pub(super) fn f_100ec102c(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x35)?;
    style_header(r)?;
    if v < 0x1e0000 {
        r.u16("Height")?;
    } else {
        r.u32("Height")?;
    }
    r.u32("Angle")?;
    r.string("FontName")?;
    r.i32("BBits")?;
    r.u8("BoxTop")?;
    r.u8("BoxBottom")?;
    r.u8("BoxLeft")?;
    r.u8("BoxRight")?;
    r.i32("Sizes")?;
    if v < 0x1e0000 {
        r.u16("DefaultY")?;
    } else {
        r.u32("DefaultY")?;
    }
    if v < 0x1e0000 {
        r.u16("DefaultX")?;
    } else {
        r.u32("DefaultX")?;
    }
    r.u8("Alignment")?;
    r.u8("SnapType")?;
    r.u8("RepeatType")?;
    r.u8("Leading")?;
    r.string("MusicTerms")?;
    r.bool("NotOn1stPage")?;
    r.record("record", "Record", |r| {
        if r.since(0x60003) {
            r.node("TypeOptions", "TypeOptions", type_options)?;
        }
        if r.since(0xf0003) {
            if r.file_version < 0x1e0000 {
                r.u16("PartHeight")?;
            } else {
                r.u32("PartHeight")?;
            }
        }
        if r.since(0x100000) {
            r.f32("OriginalStaveHeightInScore")?;
            r.bool("FixedHeightInScore")?;
            r.bool("FixedHeightInParts")?;
        }
        if r.since(0x220001) {
            r.bool("UseMarginsRelativeToFirstPageX")?;
            r.bool("UseMarginsRelativeToFirstPageY")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100da0343`.
pub(super) fn type_options(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x1e0000 {
        r.u16("DefaultDx")?;
    } else {
        r.u32("DefaultDx")?;
    }
    if v < 0x1e0000 {
        r.u16("DefaultDy")?;
    } else {
        r.u32("DefaultDy")?;
    }
    r.u32("OptionFlags")?;
    r.record("record", "Record", |r| {
        if r.since(0x1a000b) {
            if r.file_version < 0x1e0000 {
                r.u16("DefaultDxInParts")?;
            } else {
                r.u32("DefaultDxInParts")?;
            }
            if r.file_version < 0x1e0000 {
                r.u16("DefaultDyInParts")?;
            } else {
                r.u32("DefaultDyInParts")?;
            }
        }
        if r.since(0x300002) {
            r.bool("UsePositionAboveStaff")?;
            r.u32("DefaultDyAboveStaffInScore")?;
            r.u32("DefaultDyAboveStaffInParts")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100e5a65c`.
pub(super) fn frame_vertical_alignment(r: &mut Reader) -> Result<()> {
    r.u32("value")?;
    Ok(())
}

/// 8.3.0 `0x100e5a62c`.
pub(super) fn text_alignment(r: &mut Reader) -> Result<()> {
    r.u32("value")?;
    Ok(())
}

/// 8.3.0 `0x101053363`.
pub(super) fn style_type_3363(r: &mut Reader) -> Result<()> {
    r.string("value")?;
    Ok(())
}

/// 8.3.0 `0x100fa28f4`.
pub(super) fn f_100fa28f4(r: &mut Reader) -> Result<()> {
    r.u8("value")?;
    Ok(())
}

/// 8.3.0 `0x100dd70ac`.
pub(super) fn f_100dd70ac(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.bool("NameOnStaves")?;
    r.bool("NameTopLeft1stPage")?;
    r.bool("NameCentreOtherPages")?;
    r.bool("Transpose")?;
    r.bool("ResetSpacing")?;
    r.bool("SaveToDirectory")?;
    r.u32("Format")?;
    r.bool("PageToSystemBreak")?;
    r.bool("DeleteSystemBreak")?;
    r.bool("DeleteOtherBreaks")?;
    r.bool("ChangeStaveHeight")?;
    r.f32("NewStaveHeightmm")?;
    r.bool("ChangePageSize")?;
    r.f32("NewPageWidthmm")?;
    r.f32("NewPageHeightmm")?;
    r.f32("NewPageLeftmm")?;
    r.f32("NewPageRightmm")?;
    r.f32("NewPageTopmm")?;
    r.f32("NewPageBottommm")?;
    if v < 0x1e0000 {
        r.u16("RehearsalMarkSize")?;
    } else {
        r.u32("RehearsalMarkSize")?;
    }
    r.u32("FilenameFormat")?;
    r.record("record", "Record", |r| {
        r.bool("OneBarMultirestsInParts")?;
        if r.since(0x70001) {
            r.bool("LandscapePage")?;
        }
        if r.since(0x70003) {
            r.bool("ResetSystemText")?;
        }
        if r.since(0x70003) {
            r.bool("ResetRehearsalMarks")?;
        }
        if r.since(0x8001a) {
            r.bool("UseAutoPageBreaks")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x1010b5900`.
pub(super) fn f_1010b5900(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if v < 0x350009 {
        f_1010b5c56(r)?;
    } else {
        f_1010b5934(r)?;
    }
    Ok(())
}

/// 8.3.0 `0x1010ea71e`.
pub(super) fn f_1010ea71e(r: &mut Reader) -> Result<()> {
    r.bool("IsLegacySwingStyle")?;
    r.node("LegacySwingId", "StyleId", style_id)?;
    r.string("CustomSwingKeywordText")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100dec698`.
pub(super) fn f_100dec698(r: &mut Reader) -> Result<()> {
    r.tag(0x45)?;
    r.u8("InstScaleLength")?;
    r.f32("InstDeltaVolume")?;
    r.f32("InstPauseLength")?;
    r.record("record", "Record", |r| {
        if r.since(0x20007) {
            r.f32("InstStressAmount")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100bb2d20`.
pub(super) fn f_100bb2d20(r: &mut Reader) -> Result<()> {
    r.list("NumVirtualInstruments", |r| {
        r.sub("NumVirtualInstrumentsItem", |r| {
            r.u32("VirtualInstrumentSlotNum")?;
            r.node("VSTInstrumentParameterSettings", "VstinstrumentParameterSettings", vstinstrument_parameter_settings)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    r.list("NumEffectBusses", |r| {
        r.sub("NumEffectBussesItem", |r| {
            r.u32("EffectBusNum")?;
            r.node("VSTEffectParameterSettings", "VsteffectParameterSettings", vsteffect_parameter_settings)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x1009b4168`.
pub(super) fn f_1009b4168(r: &mut Reader) -> Result<()> {
    r.u32("PitchedInstrumentPreset")?;
    r.u32("UnpitchedPercussionPreset")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x1009b9f12`.
pub(super) fn f_1009b9f12(r: &mut Reader) -> Result<()> {
    r.f32("OptimalFlamDragDynamicPerNoteReductionRatio")?;
    r.f32("MaximumFlamDragDynamicGraceNoteGroupReductionRatio")?;
    r.f32("MinimumFlamDragDynamicReductionRatioForLastGraceNote")?;
    r.record("record", "Record", |r| {
        if r.since(0x3d0008) {
            r.u32("MordentNoteDuration_ms")?;
            r.u16("MordentNoteDurationSrs")?;
            r.f32("MaxTimeToStealFromRhythmicNoteForMordents")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x10104f69d`.
pub(super) fn style_type_f69d(r: &mut Reader) -> Result<()> {
    r.string("value")?;
    Ok(())
}

/// 8.3.0 `0x101008c4e`.
pub(super) fn f_101008c4e(r: &mut Reader) -> Result<()> {
    r.u32("Index")?;
    r.node("Editable", "NoteStyle", note_style)?;
    Ok(())
}

/// 8.3.0 `0x100d98274`.
pub(super) fn list_families(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListFamiliesItem", |r| {
            f_100d8c5a0(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100f3a61c`.
pub(super) fn list_sub_symbols(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListSubSymbolsItem", |r| {
            f_100ee50ce(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x101008f48`.
pub(super) fn f_101008f48(r: &mut Reader) -> Result<()> {
    r.u32("Index")?;
    r.node("Editable", "Editable", editable)?;
    Ok(())
}

/// 8.3.0 `0x1010563e2`.
pub(super) fn sound_idchange_duration(r: &mut Reader) -> Result<()> {
    r.u32("value")?;
    Ok(())
}

/// 8.3.0 `0x1010524e9`.
pub(super) fn style_type_24e9(r: &mut Reader) -> Result<()> {
    r.string("value")?;
    Ok(())
}

/// 8.3.0 `0x100f31c82`.
pub(super) fn start_symbol(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x4a)?;
    r.node("Symbol", "Symbol", symbol)?;
    if v < 0x1e0000 {
        r.u16("Dx")?;
    } else {
        r.u32("Dx")?;
    }
    if v < 0x1e0000 {
        r.u16("Dy")?;
    } else {
        r.u32("Dy")?;
    }
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100f31fcb`.
pub(super) fn category(r: &mut Reader) -> Result<()> {
    r.string("value")?;
    Ok(())
}

/// 8.3.0 `0x100dfab18`.
pub(super) fn midi_patch(r: &mut Reader) -> Result<()> {
    r.u8("BankCoarse")?;
    r.u8("BankFine")?;
    r.i8("Program")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100e13492`.
pub(super) fn f_100e13492(r: &mut Reader) -> Result<()> {
    r.tag(0x65)?;
    f_100e134da(r)?;
    Ok(())
}

/// 8.3.0 `0x100f47537`.
pub(super) fn f_100f47537(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("F100f47537Item", |r| {
            r.u8("value")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100dd2272`.
pub(super) fn f_100dd2272(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    list_drum_kit_sounds(r, "DrumKitSounds")?;
    if v > 0x60004 {
        r.record("record", "Record", |r| {
            if r.since(0x60004) {
                r.bool("PitchMapInitialised")?;
                r.i32("CentralPosIndex")?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

/// 8.3.0 `0x1010b5c56`.
pub(super) fn f_1010b5c56(r: &mut Reader) -> Result<()> {
    r.tag(0x7d)?;
    f_1010b5934(r)?;
    Ok(())
}

/// 8.3.0 `0x1010b5934`.
pub(super) fn f_1010b5934(r: &mut Reader) -> Result<()> {
    r.u32("DefaultChordSymbolElements")?;
    r.u32("ArrangementOfExtensions")?;
    r.u32("RootNoteAccidentalPosition")?;
    r.node("SuffixSettings", "SuffixSettings", suffix_settings)?;
    r.record("record", "Record", |r| {
        if r.since(0x310006) {
            r.u32("Language")?;
        }
        if r.since(0x320000) {
            r.bool("ShowMaj7SuffixInMinorChordInParentheses")?;
        }
        if r.since(0x33000c) {
            r.u32("OrderOfAlterations")?;
        }
        if r.since(0x33000f) {
            r.u32("AugmentedChordOrder")?;
        }
        if r.since(0x330010) {
            r.i32("MinNumAlterationsToWarrantParentheses")?;
        }
        if r.since(0x350004) && r.file_version < 0x350007 {
            r.bool("PreventConfusingPositionOfAlterations")?;
        }
        if r.since(0x350006) {
            r.u32("PositionOfAlterations")?;
        }
        if r.since(0x360013) {
            r.bool("MoveAlterationsAfterNumbers")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100bb19e5`.
pub(super) fn vstinstrument_parameter_settings(r: &mut Reader) -> Result<()> {
    r.tag(0x86)?;
    f_100bb1778(r)?;
    Ok(())
}

/// 8.3.0 `0x100bb1d9a`.
pub(super) fn vsteffect_parameter_settings(r: &mut Reader) -> Result<()> {
    r.tag(0x87)?;
    f_100bb1cbc(r)?;
    Ok(())
}

/// 8.3.0 `0x100d8c5a0`.
pub(super) fn f_100d8c5a0(r: &mut Reader) -> Result<()> {
    r.tag(0x6f)?;
    r.string("Name")?;
    list_instrument_style_ids(r, "InstrumentStyleIds")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100ee50ce`.
pub(super) fn f_100ee50ce(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x49)?;
    r.i16("Symbol")?;
    if v < 0x1e0000 {
        r.u16("Dx")?;
    } else {
        r.u32("Dx")?;
    }
    if v < 0x1e0000 {
        r.u16("Dy")?;
    } else {
        r.u32("Dy")?;
    }
    r.record("record", "Record", |r| {
        if r.since(0x38000b) {
            r.f32("Scale")?;
            r.f32("Rotation")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100e134da`.
pub(super) fn f_100e134da(r: &mut Reader) -> Result<()> {
    r.u64("Time")?;
    r.string("Name")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100dd3066`.
pub(super) fn list_drum_kit_sounds(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListDrumKitSoundsItem", |r| {
            f_100dee849(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100bb1778`.
pub(super) fn f_100bb1778(r: &mut Reader) -> Result<()> {
    r.f32("Gain")?;
    r.f32("Pan")?;
    r.list("NumEffectSends", |r| {
        r.sub("NumEffectSendsItem", |r| {
            r.u32("EffectBusNum")?;
            r.f32("EffectSendLevel")?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100bb1cbc`.
pub(super) fn f_100bb1cbc(r: &mut Reader) -> Result<()> {
    r.f32("OutputLevel")?;
    r.record("record", "Record", |_| Ok(()))?;
    Ok(())
}

/// 8.3.0 `0x100d8ce20`.
pub(super) fn list_instrument_style_ids(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("ListInstrumentStyleIdsItem", |r| {
            style_id(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100dee849`.
pub(super) fn f_100dee849(r: &mut Reader) -> Result<()> {
    f_100deea60(r, "items")?;
    Ok(())
}

/// 8.3.0 `0x100deea60`.
pub(super) fn f_100deea60(r: &mut Reader, name: &'static str) -> Result<()> {
    r.list(name, |r| {
        r.sub("F100deea60Item", |r| {
            f_100deca3c(r)?;
            Ok(())
        })
        .map(Value::Node)
    })?;
    Ok(())
}

/// 8.3.0 `0x100deca3c`.
pub(super) fn f_100deca3c(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    r.tag(0x54)?;
    r.string("Sound")?;
    r.node("NoteHead", "F100fa5494", f_100fa5494)?;
    if v < 0x230000 {
        r.bool("UseHardWiredPitch")?;
        r.u8("HardWiredPitch")?;
    }
    r.record("record", "Record", |r| {
        if r.since(0x50008) {
            r.u8("MIDIInputPitch")?;
        }
        if r.since(0x280009) {
            r.string("SoundID")?;
            r.u32("ArticulationType")?;
        }
        Ok(())
    })?;
    Ok(())
}

/// 8.3.0 `0x100fa5494`.
pub(super) fn f_100fa5494(r: &mut Reader) -> Result<()> {
    let v = r.file_version;
    if (0x100000..0x130000).contains(&v) {
        style_id(r)?;
    } else if !r.string_refs {
        r.i32("value")?;
    } else {
        r.string("value")?;
    }
    Ok(())
}
