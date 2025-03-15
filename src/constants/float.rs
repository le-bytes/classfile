use crate::{Read, Result, buffer::Buffer};

#[derive(Debug, Clone)]
pub struct ConstFloat {
    pub float: f32,
}

impl Read for ConstFloat {
    fn read(buf: &mut Buffer, _consts_count: u16, _empty_const_slots: &[u16]) -> Result<Self> {
        Ok(Self {
            float: buf.read_f32()?,
        })
    }
}
