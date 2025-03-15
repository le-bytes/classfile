use crate::{Read, Result, buffer::Buffer};

use super::ConstItemIdx;

#[derive(Debug, Clone)]
pub struct ConstNameAndType {
    pub name_index: ConstItemIdx,
    pub descriptor_index: ConstItemIdx,
}

impl Read for ConstNameAndType {
    fn read(buf: &mut Buffer, consts_count: u16, empty_const_slots: &[u16]) -> Result<Self> {
        let name_index = ConstItemIdx::read(buf, consts_count, empty_const_slots)?;
        let descriptor_index = ConstItemIdx::read(buf, consts_count, empty_const_slots)?;

        Ok(Self {
            name_index,
            descriptor_index,
        })
    }
}
