use crate::{Read, Result, buffer::Buffer};

#[derive(Debug, Clone)]
pub struct ConstUtf8 {
    pub string: String,
}

impl Read for ConstUtf8 {
    fn read(buf: &mut Buffer, _consts_count: u16, _empty_const_slots: &[u16]) -> Result<Self> {
        let len = buf.read_u16()?;
        Ok(Self {
            string: buf.read_utf8(len as usize)?,
        })
    }
}
