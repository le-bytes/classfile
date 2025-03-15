use crate::{Read, Result, buffer::Buffer};

use super::ConstItemIdx;

#[derive(Debug, Clone)]
pub struct ConstInterfaceMethodRef {
    pub interface_index: ConstItemIdx,
    pub name_and_type_index: ConstItemIdx,
}

impl Read for ConstInterfaceMethodRef {
    fn read(buf: &mut Buffer, consts_count: u16, empty_const_slots: &[u16]) -> Result<Self> {
        let interface_index = ConstItemIdx::read(buf, consts_count, empty_const_slots)?;
        let name_and_type_index = ConstItemIdx::read(buf, consts_count, empty_const_slots)?;

        Ok(Self {
            interface_index,
            name_and_type_index,
        })
    }
}
