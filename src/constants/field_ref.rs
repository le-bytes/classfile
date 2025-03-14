use crate::{Read, Result, buffer::Buffer};

use super::ConstItemIdx;

#[derive(Debug, Clone)]
pub struct ConstFieldRef {
    pub class_index: ConstItemIdx,
    pub name_and_type_index: ConstItemIdx,
}

impl Read for ConstFieldRef {
    fn read(buf: &mut Buffer, consts_count: u16) -> Result<Self> {
        let class_index = ConstItemIdx::read(buf, consts_count)?;
        let name_and_type_index = ConstItemIdx::read(buf, consts_count)?;

        Ok(Self {
            class_index,
            name_and_type_index,
        })
    }
}
