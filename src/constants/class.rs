use crate::{Read, Result, buffer::Buffer};

use super::ConstItemIdx;

#[derive(Debug, Clone)]
pub struct ConstClass {
    pub name_index: ConstItemIdx,
}

impl Read for ConstClass {
    fn read(buf: &mut Buffer, consts_count: u16) -> Result<Self> {
        Ok(Self {
            name_index: ConstItemIdx::read(buf, consts_count)?,
        })
    }
}
