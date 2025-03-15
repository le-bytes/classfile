use bitflags::bitflags;

use crate::{
    Read, Result, attribute::Attribute, buffer::Buffer, constants::ConstItemIdx,
    error::ClassReaderError,
};

bitflags! {
    /// Flags of a class method
    #[derive(Debug, Clone)]
    pub struct MethodAccessFlags: u16 {
        const PUBLIC       = 0x0001;
        const PRIVATE      = 0x0002;
        const PROTECTED    = 0x0004;
        const STATIC       = 0x0008;
        const FINAL        = 0x0010;
        const SYNCHRONIZED = 0x0020;
        const BRIDGE       = 0x0040;
        const VARARGS      = 0x0080;
        const NATIVE       = 0x0100;
        const ABSTRACT     = 0x0400;
        const STRICT       = 0x0800;
        const SYNTHETIC    = 0x1000;
    }
}

impl Default for MethodAccessFlags {
    fn default() -> MethodAccessFlags {
        MethodAccessFlags::empty()
    }
}

impl MethodAccessFlags {
    fn read(buf: &mut Buffer) -> Result<Self> {
        let num = buf.read_u16()?;
        Self::from_bits(num).ok_or(ClassReaderError::InvalidMethodAccessFlags(num))
    }
}

#[derive(Debug, Clone)]
pub struct Method {
    pub access_flags: MethodAccessFlags,
    pub name_index: ConstItemIdx,
    pub descriptor_index: ConstItemIdx,
    pub attributes: Vec<Attribute>,
}

impl Read for Method {
    fn read(buf: &mut Buffer, consts_count: u16, empty_const_slots: &[u16]) -> Result<Self> {
        let access_flags = MethodAccessFlags::read(buf)?;
        let name_index = ConstItemIdx::read(buf, consts_count, empty_const_slots)?;
        let descriptor_index = ConstItemIdx::read(buf, consts_count, empty_const_slots)?;
        let attributes = Vec::read(buf, consts_count, empty_const_slots)?;

        Ok(Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}
