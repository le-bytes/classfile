use bitflags::bitflags;

use crate::{
    Read, Result, attribute::Attribute, buffer::Buffer, constants::ConstItemIdx,
    error::ClassReaderError,
};

bitflags! {
    /// Possible flags of a class field
    #[derive(Debug, Clone)]
    pub struct FieldAccessFlags: u16 {
        const PUBLIC    = 0x0001;
        const PRIVATE   = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC    = 0x0008;
        const FINAL     = 0x0010;
        const VOLATILE  = 0x0040;
        const TRANSIENT = 0x0080;
        const SYNTHETIC = 0x1000;
        const ENUM      = 0x4000;
    }
}

impl Default for FieldAccessFlags {
    fn default() -> FieldAccessFlags {
        FieldAccessFlags::empty()
    }
}

impl FieldAccessFlags {
    fn read(buf: &mut Buffer) -> Result<Self> {
        let num = buf.read_u16()?;
        Self::from_bits(num).ok_or(ClassReaderError::InvalidFieldAccessFlags(num))
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub access_flag: FieldAccessFlags,
    pub name_index: ConstItemIdx,
    pub descriptor_index: ConstItemIdx,
    pub attributes: Vec<Attribute>,
}

impl Read for Field {
    fn read(buf: &mut Buffer, consts_count: u16, empty_const_slots: &[u16]) -> Result<Self> {
        let access_flag = FieldAccessFlags::read(buf)?;
        let name_index = ConstItemIdx::read(buf, consts_count, empty_const_slots)?;
        let descriptor_index = ConstItemIdx::read(buf, consts_count, empty_const_slots)?;
        let attributes = Vec::read(buf, consts_count, empty_const_slots)?;

        Ok(Self {
            access_flag,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}
