use crate::{Read, Result, buffer::Buffer};

#[derive(Debug, Clone)]
pub struct ConstDouble {
    pub double: f64,
}

impl Read for ConstDouble {
    fn read(buf: &mut Buffer, _consts_count: u16) -> Result<Self> {
        Ok(Self {
            double: buf.read_f64()?,
        })
    }
}
