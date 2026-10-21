//! A lossless, self-encoding parse tree for score payloads.
//!
//! Every leaf remembers its own wire encoding, so writing a tree back needs no
//! grammar: serialization is a pre-order concatenation of leaf encodings. The
//! grammar is only needed to *read*, which keeps reader and writer from drifting.

use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Value {
    U8(u8),
    I8(i8),
    /// One byte; Sibelius treats any non-zero value as true.
    Bool(u8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    /// Two big-endian words, high word first.
    U64(u64),
    /// Little-endian IEEE 754 single precision, kept as raw bits for exact round trips.
    F32(u32),
    /// Big-endian u32 byte length followed by UTF-16BE code units.
    Str(Vec<u16>),
    /// Big-endian u32 byte length followed by 8-bit characters (old files).
    Str8(Vec<u8>),
    Bytes(Vec<u8>),
    Node(Node),
    List(List),
    Record(Record),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Count {
    /// Big-endian u32 element count, multiplied by a constant on the wire.
    U32 { scale: u32 },
    U8,
    /// No count on the wire; the length is implied by context.
    Implied,
}

#[derive(Clone, PartialEq)]
pub struct List {
    pub count: Count,
    pub items: Vec<Value>,
}

/// A body preceded by its byte length as a varint. Readers skip whatever they do
/// not understand at the end of the body; those bytes are kept as `padding`.
#[derive(Clone, PartialEq)]
pub struct Record {
    pub body: Node,
    pub padding: Vec<u8>,
    /// Bytes the length varint occupied; Sibelius sometimes pads it.
    pub length_width: usize,
}

#[derive(Clone, PartialEq)]
pub struct Node {
    pub kind: &'static str,
    pub fields: Vec<(&'static str, Value)>,
}

impl Node {
    pub fn new(kind: &'static str) -> Self {
        Self { kind, fields: Vec::new() }
    }

    pub fn push(&mut self, name: &'static str, value: Value) {
        self.fields.push((name, value));
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.fields.iter().find(|(n, _)| *n == name).map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.fields.iter_mut().find(|(n, _)| *n == name).map(|(_, v)| v)
    }

    pub fn node(&self, name: &str) -> Option<&Node> {
        match self.get(name)? {
            Value::Node(n) => Some(n),
            Value::Record(r) => Some(&r.body),
            _ => None,
        }
    }

    pub fn node_mut(&mut self, name: &str) -> Option<&mut Node> {
        match self.get_mut(name)? {
            Value::Node(n) => Some(n),
            Value::Record(r) => Some(&mut r.body),
            _ => None,
        }
    }

    pub fn list(&self, name: &str) -> Option<&List> {
        match self.get(name)? {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn int(&self, name: &str) -> Option<i64> {
        self.get(name).and_then(Value::as_int)
    }

    pub fn text(&self, name: &str) -> Option<String> {
        match self.get(name)? {
            Value::Str(units) => Some(String::from_utf16_lossy(units)),
            Value::Str8(bytes) => Some(bytes.iter().map(|&b| b as char).collect()),
            _ => None,
        }
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        self.fields.iter().for_each(|(_, v)| v.write(out));
    }

    /// Bytes inside records that the grammar skipped as unknown.
    pub fn padding_len(&self) -> usize {
        self.fields.iter().map(|(_, v)| v.padding_len()).sum()
    }

    /// Every non-empty record padding below this node, with the path of node
    /// kinds that leads to it.
    pub fn paddings(&self) -> Vec<(String, &[u8])> {
        self.paddings_at(self.kind)
    }

    fn paddings_at(&self, path: &str) -> Vec<(String, &[u8])> {
        self.fields.iter().flat_map(|(_, v)| v.paddings_at(path)).collect()
    }
}

pub fn write_varint(value: u32, out: &mut Vec<u8>) {
    write_varint_padded(value, 1, out);
}

/// Writes `value` in at least `width` bytes, using continuation bits over
/// zero groups for any padding.
pub fn write_varint_padded(mut value: u32, width: usize, out: &mut Vec<u8>) {
    let mut written = 0;
    loop {
        written += 1;
        let more = value >> 7 != 0 || written < width;
        out.push(((value & 0x7f) as u8) << 1 | more as u8);
        value >>= 7;
        if !more {
            break;
        }
    }
}

impl Value {
    pub fn as_int(&self) -> Option<i64> {
        match *self {
            Value::U8(v) | Value::Bool(v) => Some(v.into()),
            Value::I8(v) => Some(v.into()),
            Value::U16(v) => Some(v.into()),
            Value::I16(v) => Some(v.into()),
            Value::U32(v) => Some(v.into()),
            Value::I32(v) => Some(v.into()),
            Value::U64(v) => i64::try_from(v).ok(),
            _ => None,
        }
    }

    pub fn str(text: &str) -> Self {
        Value::Str(text.encode_utf16().collect())
    }

    pub fn padding_len(&self) -> usize {
        match self {
            Value::Node(n) => n.padding_len(),
            Value::List(l) => l.items.iter().map(Value::padding_len).sum(),
            Value::Record(r) => r.padding.len() + r.body.padding_len(),
            _ => 0,
        }
    }

    fn paddings_at(&self, path: &str) -> Vec<(String, &[u8])> {
        match self {
            Value::Node(n) => n.paddings_at(&format!("{path}/{}", n.kind)),
            Value::List(l) => l.items.iter().flat_map(|v| v.paddings_at(path)).collect(),
            Value::Record(r) => {
                let inner = format!("{path}/{}", r.body.kind);
                let own = (!r.padding.is_empty()).then(|| (inner.clone(), r.padding.as_slice()));
                own.into_iter().chain(r.body.paddings_at(&inner)).collect()
            }
            _ => Vec::new(),
        }
    }

    pub fn encoded_len(&self) -> usize {
        let mut out = Vec::new();
        self.write(&mut out);
        out.len()
    }

    pub fn write(&self, out: &mut Vec<u8>) {
        match self {
            Value::U8(v) | Value::Bool(v) => out.push(*v),
            Value::I8(v) => out.push(*v as u8),
            Value::U16(v) => out.extend_from_slice(&v.to_be_bytes()),
            Value::I16(v) => out.extend_from_slice(&v.to_be_bytes()),
            Value::U32(v) => out.extend_from_slice(&v.to_be_bytes()),
            Value::I32(v) => out.extend_from_slice(&v.to_be_bytes()),
            Value::U64(v) => out.extend_from_slice(&v.to_be_bytes()),
            Value::F32(bits) => out.extend_from_slice(&bits.to_le_bytes()),
            Value::Str(units) => {
                out.extend_from_slice(&(2 * units.len() as u32).to_be_bytes());
                units.iter().for_each(|u| out.extend_from_slice(&u.to_be_bytes()));
            }
            Value::Str8(bytes) => {
                out.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
                out.extend_from_slice(bytes);
            }
            Value::Bytes(bytes) => out.extend_from_slice(bytes),
            Value::Node(node) => node.write(out),
            Value::List(list) => {
                match list.count {
                    Count::U32 { scale } => {
                        out.extend_from_slice(&(scale * list.items.len() as u32).to_be_bytes())
                    }
                    Count::U8 => out.push(list.items.len() as u8),
                    Count::Implied => {}
                }
                list.items.iter().for_each(|v| v.write(out));
            }
            Value::Record(record) => {
                let mut body = Vec::new();
                record.body.write(&mut body);
                write_varint_padded((body.len() + record.padding.len()) as u32, record.length_width, out);
                out.extend(body);
                out.extend_from_slice(&record.padding);
            }
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::U8(v) => write!(f, "{v}u8"),
            Value::I8(v) => write!(f, "{v}i8"),
            Value::Bool(v) => write!(f, "{}", *v != 0),
            Value::U16(v) => write!(f, "{v}u16"),
            Value::I16(v) => write!(f, "{v}i16"),
            Value::U32(v) => write!(f, "{v}"),
            Value::I32(v) => write!(f, "{v}i"),
            Value::U64(v) => write!(f, "{v}u64"),
            Value::F32(bits) => write!(f, "{}f", f32::from_bits(*bits)),
            Value::Str(units) => write!(f, "{:?}", String::from_utf16_lossy(units)),
            Value::Str8(bytes) => write!(f, "{:?}", bytes.iter().map(|&b| b as char).collect::<String>()),
            Value::Bytes(b) => write!(f, "bytes[{}]", hex(b)),
            Value::Node(n) => n.fmt(f),
            Value::List(l) => f.debug_list().entries(&l.items).finish(),
            Value::Record(r) if r.padding.is_empty() => r.body.fmt(f),
            Value::Record(r) => write!(f, "{:?} + padding[{}]", r.body, hex(&r.padding)),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct(self.kind);
        self.fields.iter().for_each(|(name, value)| {
            s.field(name, value);
        });
        s.finish()
    }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varint_uses_low_bit_as_continuation() {
        let encode = |v| {
            let mut out = Vec::new();
            write_varint(v, &mut out);
            out
        };
        assert_eq!(encode(0), [0x00]);
        assert_eq!(encode(0x7f), [0xfe]);
        assert_eq!(encode(0x80), [0x01, 0x02]);
        assert_eq!(encode(549), [0x4b, 0x08]);
    }

    #[test]
    fn padded_varint_keeps_its_width() {
        let mut out = Vec::new();
        write_varint_padded(0, 2, &mut out);
        assert_eq!(out, [0x01, 0x00]);
    }
}
