use crate::{Read, Result, buffer::Buffer};

#[derive(Debug, Clone)]
pub struct ConstInteger {
    pub integer: i32,
}

impl Read for ConstInteger {
    fn read(buf: &mut Buffer, _consts_count: u16, _empty_const_slots: &[u16]) -> Result<Self> {
        Ok(Self {
            integer: buf.read_i32()?,
        })
    }
}
