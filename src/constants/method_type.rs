use crate::{Read, Result, buffer::Buffer};

use super::ConstItemIdx;

#[derive(Debug, Clone)]
pub struct ConstMethodType {
    pub descriptor_index: ConstItemIdx,
}

impl Read for ConstMethodType {
    fn read(buf: &mut Buffer, consts_count: u16) -> Result<Self> {
        Ok(Self {
            descriptor_index: ConstItemIdx::read(buf, consts_count)?,
        })
    }
}
