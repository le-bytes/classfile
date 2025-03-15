use crate::{Read, Result, buffer::Buffer};

#[derive(Debug, Clone)]
pub struct ConstLong {
    pub long: i64,
}

impl Read for ConstLong {
    fn read(buf: &mut Buffer, _consts_count: u16, _empty_const_slots: &[u16]) -> Result<Self> {
        Ok(Self {
            long: buf.read_i64()?,
        })
    }
}
