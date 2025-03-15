use crate::{Read, Result, buffer::Buffer};

use super::{ConstItem, ConstItemIdx};

#[derive(Debug, Clone)]
pub struct ConstInvokeDynamic {
    // TODO
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: ConstItemIdx,
}

impl Read for ConstInvokeDynamic {
    fn read(buf: &mut Buffer, consts_count: u16, empty_const_slots: &[u16]) -> Result<Self> {
        let bootstrap_method_attr_index = buf.read_u16()?;
        let name_and_type_index = ConstItemIdx::read(buf, consts_count, empty_const_slots)?;

        Ok(Self {
            bootstrap_method_attr_index,
            name_and_type_index,
        })
    }
}

impl ConstItem {
    pub fn is_invoke_dynamic(&self) -> bool {
        matches!(self, Self::InvokeDynamic(_))
    }

    pub fn as_invoke_dynamic(&self) -> Option<&ConstInvokeDynamic> {
        if let Self::InvokeDynamic(item) = self {
            Some(item)
        } else {
            None
        }
    }
}
