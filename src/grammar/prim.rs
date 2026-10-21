//! Building blocks shared by many record types.

use crate::error::Result;
use crate::reader::Reader;
use crate::tree::{Count, List, Value};

pub fn u8(r: &mut Reader) -> Result<Value> {
    r.read_u8().map(Value::U8)
}

pub fn i8(r: &mut Reader) -> Result<Value> {
    r.read_u8().map(|v| Value::I8(v as i8))
}

pub fn bool(r: &mut Reader) -> Result<Value> {
    r.read_u8().map(Value::Bool)
}

pub fn u16(r: &mut Reader) -> Result<Value> {
    r.read_u16().map(Value::U16)
}

pub fn i16(r: &mut Reader) -> Result<Value> {
    r.read_u16().map(|v| Value::I16(v as i16))
}

pub fn u32(r: &mut Reader) -> Result<Value> {
    r.read_u32().map(Value::U32)
}

pub fn i32(r: &mut Reader) -> Result<Value> {
    r.read_u32().map(|v| Value::I32(v as i32))
}

pub fn f32(r: &mut Reader) -> Result<Value> {
    r.read_f32().map(Value::F32)
}

pub fn string(r: &mut Reader) -> Result<Value> {
    r.read_string()
}

/// Identifies a dynamic part. Versions after 8.3.0 widen the key by a word
/// from 0x41 and another from 0x42 (2025.3 `PM_U32`, `0x100fe6560`).
pub fn part_key(r: &mut Reader) -> Result<Value> {
    r.sub("PartKey", |r| {
        r.u32("part")?;
        if r.file_version >= 0x0041_0000 {
            r.i32("extra1")?;
        }
        if r.file_version >= 0x0042_0000 {
            r.i32("extra2")?;
        }
        Ok(())
    })
    .map(Value::Node)
}

/// First version whose part maps carry per-part overrides (8.3.0 `PM_U32`).
const PART_MAP_OVERRIDES: u32 = 0x000c_0000;

/// `PartMap<T>`: a score-wide value followed by per-part overrides. The count
/// on the wire is twice the number of overrides (keys and values are counted
/// separately). Older files store the score-wide value alone.
pub fn part_map(r: &mut Reader, name: &'static str, value: impl Fn(&mut Reader) -> Result<Value>) -> Result<()> {
    r.node(name, "PartMap", |r| {
        let default = value(r)?;
        r.push("default", default);
        if r.file_version < PART_MAP_OVERRIDES {
            return Ok(());
        }
        let raw = r.read_u32()?;
        if raw % 2 != 0 || raw as usize > r.remaining() {
            return Err(r.fail(format!("implausible part-map count {raw}")));
        }
        let items = (0..raw / 2)
            .map(|_| {
                r.sub("Override", |r| {
                    let key = part_key(r)?;
                    r.push("key", key);
                    let v = value(r)?;
                    r.push("value", v);
                    Ok(())
                })
                .map(Value::Node)
            })
            .collect::<Result<Vec<_>>>()?;
        r.push(
            "overrides",
            Value::List(List {
                count: Count::U32 { scale: 2 },
                items,
            }),
        );
        Ok(())
    })
}

pub fn pm_bool(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, bool)
}

pub fn pm_u16(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, u16)
}

pub fn pm_i16(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, i16)
}

pub fn pm_u32(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, u32)
}

pub fn pm_i32(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, i32)
}

pub fn pm_f32(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, f32)
}

pub fn pm_i8(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, i8)
}

pub fn pm_u8(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, u8)
}

pub fn pm_string(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, string)
}

/// `ScoreInStream::ReadPoint`: two big-endian i32 (two i16 before 0x1e0000).
pub fn point(r: &mut Reader) -> Result<Value> {
    r.sub("Point", |r| {
        r.i32("x")?;
        r.i32("y")?;
        Ok(())
    })
    .map(Value::Node)
}

/// Four big-endian u32, e.g. a crop rectangle.
pub fn rect(r: &mut Reader) -> Result<Value> {
    r.sub("Rect", |r| {
        r.u32("left")?;
        r.u32("top")?;
        r.u32("right")?;
        r.u32("bottom")?;
        Ok(())
    })
    .map(Value::Node)
}

pub fn pm_point(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, point)
}

pub fn pm_rect(r: &mut Reader, name: &'static str) -> Result<()> {
    part_map(r, name, rect)
}
