mod class;
pub use class::*;
mod double;
pub use double::*;
mod field_ref;
pub use field_ref::*;
mod float;
pub use float::*;
mod integer;
pub use integer::*;
mod interface_method_ref;
pub use interface_method_ref::*;
mod invoke_dynamic;
pub use invoke_dynamic::*;
mod long;
pub use long::*;
mod method_handle;
pub use method_handle::*;
mod method_ref;
pub use method_ref::*;
mod method_type;
pub use method_type::*;
mod name_and_type;
pub use name_and_type::*;
mod string;
pub use string::*;
mod utf8;
pub use utf8::*;

use index_vec::{IndexVec, define_index_type};

use crate::{Read, Result, buffer::Buffer, error::ClassReaderError};

define_index_type! {
    pub struct ConstItemIdx = u16;
}

impl Read for ConstItemIdx {
    fn read(buf: &mut Buffer, consts_count: u16, empty_consts_slots: &[u16]) -> Result<Self> {
        let idx = buf.read_u16()?;
        if idx == 0 {
            return Err(ClassReaderError::InvalidConstantPoolIdx(idx));
        }
        let idx = idx - 1;
        let mut offset = 0;
        for slot in empty_consts_slots {
            if &idx == slot {
                return Err(ClassReaderError::InvalidConstantPoolIdx(idx));
            } else if &idx < slot {
                break;
            } else {
                offset += 1;
            }
        }
        let idx = idx - offset;
        if idx >= consts_count {
            Err(ClassReaderError::InvalidConstantPoolIdx(idx))
        } else {
            Ok(ConstItemIdx::from_raw(idx))
        }
    }
}

impl Read for Option<ConstItemIdx> {
    fn read(buf: &mut Buffer, consts_count: u16, empty_consts_slots: &[u16]) -> Result<Self> {
        let idx = buf.read_u16()?;
        if idx == 0 {
            return Ok(None);
        }
        let idx = idx - 1;
        let mut offset = 0;
        for slot in empty_consts_slots {
            if &idx == slot {
                return Err(ClassReaderError::InvalidConstantPoolIdx(idx));
            } else if &idx < slot {
                break;
            } else {
                offset += 1;
            }
        }
        let idx = idx - offset;
        if idx >= consts_count {
            Err(ClassReaderError::InvalidConstantPoolIdx(idx))
        } else {
            Ok(Some(ConstItemIdx::from_raw(idx)))
        }
    }
}

pub type Constants = IndexVec<ConstItemIdx, ConstItem>;

#[derive(Debug, Clone)]
pub enum ConstItem {
    Class(ConstClass),
    FieldRef(ConstFieldRef),
    MethodRef(ConstMethodRef),
    InterfaceMethodRef(ConstInterfaceMethodRef),
    String(ConstString),
    Integer(ConstInteger),
    Float(ConstFloat),
    Long(ConstLong),
    Double(ConstDouble),
    NameAndType(ConstNameAndType),
    Utf8(ConstUtf8),
    MethodHandle(ConstMethodHandle),
    MethodType(ConstMethodType),
    InvokeDynamic(ConstInvokeDynamic),
}

impl ConstItem {
    pub fn is_8bit(&self) -> bool {
        matches!(self, Self::Double(_) | Self::Long(_))
    }
}

impl Read for ConstItem {
    fn read(buf: &mut Buffer, consts_count: u16, empty_const_slots: &[u16]) -> Result<Self> {
        let tag = buf.read_u8()?;
        Ok(match tag {
            1 => Self::Utf8(ConstUtf8::read(buf, consts_count, empty_const_slots)?),
            3 => Self::Integer(ConstInteger::read(buf, consts_count, empty_const_slots)?),
            4 => Self::Float(ConstFloat::read(buf, consts_count, empty_const_slots)?),
            5 => Self::Long(ConstLong::read(buf, consts_count, empty_const_slots)?),
            6 => Self::Double(ConstDouble::read(buf, consts_count, empty_const_slots)?),
            7 => Self::Class(ConstClass::read(buf, consts_count, empty_const_slots)?),
            8 => Self::String(ConstString::read(buf, consts_count, empty_const_slots)?),
            9 => Self::FieldRef(ConstFieldRef::read(buf, consts_count, empty_const_slots)?),
            10 => Self::MethodRef(ConstMethodRef::read(buf, consts_count, empty_const_slots)?),
            11 => Self::InterfaceMethodRef(ConstInterfaceMethodRef::read(
                buf,
                consts_count,
                empty_const_slots,
            )?),
            12 => Self::NameAndType(ConstNameAndType::read(
                buf,
                consts_count,
                empty_const_slots,
            )?),
            15 => Self::MethodHandle(ConstMethodHandle::read(
                buf,
                consts_count,
                empty_const_slots,
            )?),
            16 => Self::MethodType(ConstMethodType::read(buf, consts_count, empty_const_slots)?),
            18 => Self::InvokeDynamic(ConstInvokeDynamic::read(
                buf,
                consts_count,
                empty_const_slots,
            )?),
            tag => return Err(ClassReaderError::InvalidConstItemTag(tag)),
        })
    }
}
