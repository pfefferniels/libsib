//! Grammar-side reader: reads primitives and assembles a [`Node`] tree.
//!
//! The primitives mirror Sibelius' own `ScoreInStream`: big-endian integers,
//! raw little-endian floats, length-prefixed strings, one-byte tags and
//! varint-length records whose unread tail is skipped.

use crate::error::{Error, Result};
use crate::tree::{Count, List, Node, Record, Value, hex};

/// Stream flag: strings are UTF-16 rather than 8-bit.
pub const FLAG_UTF16: u32 = 0x10;
/// Stream flag: every tag is followed by a u32 size.
pub const FLAG_SIZED_TAGS: u32 = 0x01;

pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
    /// Version whose field layout the readers follow: the container version
    /// of the chunk, raised to `newest` while a record is reread.
    pub version: u32,
    /// Container version of the chunk, which fixes encodings such as part keys.
    pub file_version: u32,
    /// Newest field layout the grammar knows.
    newest: u32,
    /// Container flags of that chunk, without the compression and encryption bits.
    pub flags: u32,
    /// Style references are written as names instead of indices (Idea clips).
    pub string_refs: bool,
    /// End of the innermost record being read; no read may cross it.
    limit: usize,
    stack: Vec<Node>,
    trace: bool,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8], version: u32, flags: u32) -> Self {
        Self { data, pos: 0, version, file_version: version, newest: version, flags, string_refs: false, limit: data.len(), stack: vec![Node::new("Payload")], trace: false }
    }

    /// Lets records that end in unread bytes be reread with the field layout
    /// of `version` (see [`Reader::record`]).
    pub fn with_newest_layout(mut self, version: u32) -> Self {
        self.newest = version;
        self
    }

    pub fn with_trace(mut self, trace: bool) -> Self {
        self.trace = trace;
        self
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn rewind(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Whether the layout being read has the fields added after version `v`.
    /// While a record is reread with a newer layout, fields count only while
    /// bytes are left: the writer stopped where its own layout ended.
    pub fn since(&self, v: u32) -> bool {
        self.version > v && (self.version == self.file_version || self.pos < self.limit)
    }

    /// Whether the innermost record has bytes left, for fields that Sibelius
    /// writes only when a feature is enabled.
    pub fn more(&self) -> bool {
        self.pos < self.limit
    }

    /// Bytes left before the end of the innermost record (or the payload).
    pub fn remaining(&self) -> usize {
        self.limit - self.pos
    }

    pub fn at_end(&self) -> bool {
        self.pos == self.data.len()
    }

    pub fn peek(&self, n: usize) -> &'a [u8] {
        &self.data[self.pos..(self.pos + n).min(self.data.len())]
    }

    pub fn peek_u8(&self) -> Result<u8> {
        self.data[..self.limit].get(self.pos).copied().ok_or_else(|| self.fail("unexpected end of record"))
    }

    pub fn path(&self) -> String {
        self.stack.iter().map(|n| n.kind).collect::<Vec<_>>().join("/")
    }

    pub fn fail(&self, message: impl Into<String>) -> Error {
        Error::Malformed {
            offset: self.pos,
            message: format!("{} in {} (next: {})", message.into(), self.path(), hex(self.peek(24))),
        }
    }

    /// Fails unless `cond` holds; used to pin down hypotheses about constant fields.
    pub fn expect(&self, cond: bool, what: &str) -> Result<()> {
        if cond { Ok(()) } else { Err(self.fail(format!("expected {what}"))) }
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let bytes = self.data[..self.limit]
            .get(self.pos..self.pos + n)
            .ok_or_else(|| self.fail(format!("need {n} bytes")))?;
        self.pos += n;
        Ok(bytes)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N]> {
        Ok(self.take(N)?.try_into().expect("length checked by take"))
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        Ok(u16::from_be_bytes(self.array()?))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes(self.array()?))
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        Ok(u64::from_be_bytes(self.array()?))
    }

    pub fn read_f32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.array()?))
    }

    pub fn read_varint(&mut self) -> Result<u32> {
        let mut value = 0u32;
        let mut shift = 0;
        loop {
            let byte = self.read_u8()?;
            if shift > 28 {
                return Err(self.fail("varint longer than five bytes"));
            }
            value |= u32::from(byte >> 1) << shift;
            shift += 7;
            if byte & 1 == 0 {
                return Ok(value);
            }
        }
    }

    /// Reads a string in the encoding the stream flags select.
    pub fn read_string(&mut self) -> Result<Value> {
        let len = self.read_u32()? as usize;
        let utf16 = self.flags & FLAG_UTF16 != 0;
        if (utf16 && len % 2 != 0) || len > self.remaining() {
            self.pos -= 4;
            return Err(self.fail(format!("implausible string length {len}")));
        }
        let bytes = self.take(len)?;
        Ok(if utf16 {
            Value::Str(bytes.chunks_exact(2).map(|p| u16::from_be_bytes([p[0], p[1]])).collect())
        } else {
            Value::Str8(bytes.to_vec())
        })
    }

    pub fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        Ok(self.take(n)?.to_vec())
    }

    fn current(&mut self) -> &mut Node {
        self.stack.last_mut().expect("root node is never popped")
    }

    pub fn push(&mut self, name: &'static str, value: Value) {
        if self.trace {
            let at = self.pos - value.encoded_len();
            eprintln!("{at:#08x} {}.{name} = {value:?}", self.path());
        }
        self.current().push(name, value);
    }

    pub fn u8(&mut self, name: &'static str) -> Result<u8> {
        let v = self.read_u8()?;
        self.push(name, Value::U8(v));
        Ok(v)
    }

    pub fn i8(&mut self, name: &'static str) -> Result<i8> {
        let v = self.read_u8()? as i8;
        self.push(name, Value::I8(v));
        Ok(v)
    }

    pub fn bool(&mut self, name: &'static str) -> Result<bool> {
        let v = self.read_u8()?;
        self.push(name, Value::Bool(v));
        Ok(v != 0)
    }

    pub fn u16(&mut self, name: &'static str) -> Result<u16> {
        let v = self.read_u16()?;
        self.push(name, Value::U16(v));
        Ok(v)
    }

    pub fn i16(&mut self, name: &'static str) -> Result<i16> {
        let v = self.read_u16()? as i16;
        self.push(name, Value::I16(v));
        Ok(v)
    }

    pub fn u32(&mut self, name: &'static str) -> Result<u32> {
        let v = self.read_u32()?;
        self.push(name, Value::U32(v));
        Ok(v)
    }

    pub fn i32(&mut self, name: &'static str) -> Result<i32> {
        let v = self.read_u32()? as i32;
        self.push(name, Value::I32(v));
        Ok(v)
    }

    pub fn u64(&mut self, name: &'static str) -> Result<u64> {
        let v = self.read_u64()?;
        self.push(name, Value::U64(v));
        Ok(v)
    }

    pub fn f32(&mut self, name: &'static str) -> Result<f32> {
        let bits = self.read_f32()?;
        self.push(name, Value::F32(bits));
        Ok(f32::from_bits(bits))
    }

    pub fn string(&mut self, name: &'static str) -> Result<()> {
        let v = self.read_string()?;
        self.push(name, v);
        Ok(())
    }

    pub fn bytes(&mut self, name: &'static str, n: usize) -> Result<Vec<u8>> {
        let v = self.read_bytes(n)?;
        self.push(name, Value::Bytes(v.clone()));
        Ok(v)
    }

    /// `ScoreInStream::ReadTag`: a one-byte type tag that must match.
    pub fn tag(&mut self, expected: u8) -> Result<()> {
        let tag = self.read_u8()?;
        if tag != expected {
            self.pos -= 1;
            return Err(self.fail(format!("tag {tag:#04x}, expected {expected:#04x}")));
        }
        self.push("tag", Value::U8(tag));
        self.tag_size()
    }

    /// A tag whose value selects what follows.
    pub fn any_tag(&mut self) -> Result<u8> {
        let tag = self.u8("tag")?;
        self.tag_size()?;
        Ok(tag)
    }

    fn tag_size(&mut self) -> Result<()> {
        if self.flags & FLAG_SIZED_TAGS != 0 {
            self.u32("tag_size")?;
        }
        Ok(())
    }

    /// Reads a nested node without attaching it.
    pub fn sub(&mut self, kind: &'static str, body: impl FnOnce(&mut Self) -> Result<()>) -> Result<Node> {
        self.stack.push(Node::new(kind));
        let outcome = body(self);
        let node = self.stack.pop().expect("pushed above");
        outcome.map(|()| node)
    }

    pub fn node(&mut self, name: &'static str, kind: &'static str, body: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {
        let node = self.sub(kind, body)?;
        self.current().push(name, Value::Node(node));
        Ok(())
    }

    /// `BeginRecord`/`EndRecord`: a varint byte length, the body, then any bytes
    /// the body did not consume (fields newer than this grammar).
    ///
    /// A newer Sibelius that saves in an older format still writes its newer
    /// fields at the ends of records, where older readers skip them. So when
    /// bytes are left over, the body is reread with the newest layout and that
    /// reading is kept if it explains more of the record.
    pub fn record(&mut self, name: &'static str, kind: &'static str, mut body: impl FnMut(&mut Self) -> Result<()>) -> Result<()> {
        let varint_start = self.pos;
        let len = self.read_varint()? as usize;
        if len > self.remaining() {
            self.pos = varint_start;
            return Err(self.fail(format!("record of {len} bytes")));
        }
        let start = self.pos;
        let length_width = start - varint_start;
        let outer = std::mem::replace(&mut self.limit, start + len);
        let mut node = self.sub(kind, &mut body);
        if node.is_ok() && self.pos < self.limit {
            node = Ok(self.reread_as_newest(start, kind, &mut body).unwrap_or_else(|| node.expect("checked")));
        }
        self.limit = outer;
        let node = node?;
        let padding = self.read_bytes(start + len - self.pos)?;
        self.current().push(name, Value::Record(Record { body: node, padding, length_width }));
        Ok(())
    }

    /// Rereads the record body that began at `start` with the newest layout.
    /// Returns it only if it reads further; otherwise the reader is left as the
    /// first reading left it.
    fn reread_as_newest(&mut self, start: usize, kind: &'static str, body: &mut impl FnMut(&mut Self) -> Result<()>) -> Option<Node> {
        if self.version >= self.newest {
            return None;
        }
        let first_end = self.pos;
        let version = std::mem::replace(&mut self.version, self.newest);
        self.pos = start;
        let second = self.sub(kind, body);
        self.version = version;
        match second {
            Ok(node) if self.pos > first_end => Some(node),
            _ => {
                self.pos = first_end;
                None
            }
        }
    }

    /// A list preceded by a big-endian count (`EnterReadI32` in Sibelius).
    pub fn list(&mut self, name: &'static str, mut item: impl FnMut(&mut Self) -> Result<Value>) -> Result<usize> {
        let n = self.read_u32()? as usize;
        if n > self.remaining() {
            self.pos -= 4;
            return Err(self.fail(format!("implausible count {n}")));
        }
        let items = (0..n).map(|_| item(self)).collect::<Result<Vec<_>>>()?;
        self.current().push(name, Value::List(List { count: Count::U32 { scale: 1 }, items }));
        Ok(n)
    }

    /// A list whose count is known from earlier fields.
    pub fn list_n(&mut self, name: &'static str, n: usize, mut item: impl FnMut(&mut Self) -> Result<Value>) -> Result<()> {
        let items = (0..n).map(|_| item(self)).collect::<Result<Vec<_>>>()?;
        self.current().push(name, Value::List(List { count: Count::Implied, items }));
        Ok(())
    }

    pub fn finish(mut self) -> Node {
        self.stack.truncate(1);
        self.stack.pop().expect("root")
    }
}
