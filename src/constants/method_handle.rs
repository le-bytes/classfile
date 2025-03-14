use crate::{Read, Result, buffer::Buffer, error::ClassReaderError};

use super::ConstItemIdx;

#[derive(Debug, Clone)]
pub enum CPMethodHandleReferenceKind {
    GetField,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
}

impl TryFrom<u8> for CPMethodHandleReferenceKind {
    type Error = ();

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::GetField),
            2 => Ok(Self::GetStatic),
            3 => Ok(Self::PutField),
            4 => Ok(Self::PutStatic),
            5 => Ok(Self::InvokeVirtual),
            6 => Ok(Self::InvokeStatic),
            7 => Ok(Self::InvokeSpecial),
            8 => Ok(Self::NewInvokeSpecial),
            9 => Ok(Self::InvokeInterface),
            _ => Err(()),
        }
    }
}

impl Into<u8> for CPMethodHandleReferenceKind {
    fn into(self) -> u8 {
        match self {
            Self::GetField => 1,
            Self::GetStatic => 2,
            Self::PutField => 3,
            Self::PutStatic => 4,
            Self::InvokeVirtual => 5,
            Self::InvokeStatic => 6,
            Self::InvokeSpecial => 7,
            Self::NewInvokeSpecial => 8,
            Self::InvokeInterface => 9,
        }
    }
}

impl Read for CPMethodHandleReferenceKind {
    fn read(buf: &mut Buffer, _consts_count: u16) -> Result<Self> {
        let tag = buf.read_u8()?;
        Self::try_from(tag).map_err(|_| ClassReaderError::InvalidMethodHanldeReferenceKind(tag))
    }
}

#[derive(Debug, Clone)]
pub struct ConstMethodHandle {
    pub reference_kind: CPMethodHandleReferenceKind,
    pub reference_index: ConstItemIdx,
}

impl Read for ConstMethodHandle {
    fn read(buf: &mut Buffer, consts_count: u16) -> Result<Self> {
        let reference_kind = CPMethodHandleReferenceKind::read(buf, consts_count)?;
        let reference_index = ConstItemIdx::read(buf, consts_count)?;

        Ok(Self {
            reference_kind,
            reference_index,
        })
    }
}
