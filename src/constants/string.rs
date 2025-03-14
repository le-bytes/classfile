use crate::{Read, Result, buffer::Buffer};

use super::ConstItemIdx;

#[derive(Debug, Clone)]
pub struct ConstString {
    pub string_index: ConstItemIdx,
}

impl Read for ConstString {
    fn read(buf: &mut Buffer, consts_count: u16) -> Result<Self> {
        Ok(Self {
            string_index: ConstItemIdx::read(buf, consts_count)?,
        })
    }
}
