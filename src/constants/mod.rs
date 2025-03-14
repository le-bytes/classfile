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
    fn read(buf: &mut Buffer, consts_count: u16) -> Result<Self> {
        let idx = buf.read_u16()?;
        if idx == 0 || idx > consts_count {
            Err(ClassReaderError::InvalidConstantPoolIdx(idx))
        } else {
            Ok(ConstItemIdx::from_raw(idx - 1))
        }
    }
}

impl Read for Option<ConstItemIdx> {
    fn read(buf: &mut Buffer, consts_count: u16) -> Result<Self> {
        let idx = buf.read_u16()?;
        if idx > consts_count {
            Err(ClassReaderError::InvalidConstantPoolIdx(idx))
        } else if idx == 0 {
            Ok(None)
        } else {
            Ok(Some(ConstItemIdx::from_raw(idx - 1)))
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
    fn read(buf: &mut Buffer, consts_count: u16) -> Result<Self> {
        let tag = buf.read_u8()?;
        Ok(match tag {
            1 => Self::Utf8(ConstUtf8::read(buf, consts_count)?),
            3 => Self::Integer(ConstInteger::read(buf, consts_count)?),
            4 => Self::Float(ConstFloat::read(buf, consts_count)?),
            5 => Self::Long(ConstLong::read(buf, consts_count)?),
            6 => Self::Double(ConstDouble::read(buf, consts_count)?),
            7 => Self::Class(ConstClass::read(buf, consts_count)?),
            8 => Self::String(ConstString::read(buf, consts_count)?),
            9 => Self::FieldRef(ConstFieldRef::read(buf, consts_count)?),
            10 => Self::MethodRef(ConstMethodRef::read(buf, consts_count)?),
            11 => Self::InterfaceMethodRef(ConstInterfaceMethodRef::read(buf, consts_count)?),
            12 => Self::NameAndType(ConstNameAndType::read(buf, consts_count)?),
            15 => Self::MethodHandle(ConstMethodHandle::read(buf, consts_count)?),
            16 => Self::MethodType(ConstMethodType::read(buf, consts_count)?),
            18 => Self::InvokeDynamic(ConstInvokeDynamic::read(buf, consts_count)?),
            tag => return Err(ClassReaderError::InvalidConstItemTag(tag)),
        })
    }
}
