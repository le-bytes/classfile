use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::buffer::BufferError;

/// Models the possible errors returned when reading a .class file
#[derive(Debug, PartialEq, Eq)]
pub enum ClassReaderError {
    InvalidMagicBytes(u32),
    InvalidConstantPoolIdx(u16),
    InvalidMethodHandleReferenceKind(u8),
    InvalidConstItemTag(u8),
    InvalidClassAccessFlags(u16),
    InvalidFieldAccessFlags(u16),
    InvalidMethodAccessFlags(u16),
    UnexpectedEndOfData,
    InvalidCesu8String,
    UnsupportedVersion(u16, u16),
}

impl Display for ClassReaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassReaderError::InvalidMagicBytes(bytes) => {
                write!(
                    f,
                    "Invalid magic bytes `0x{:08X}` (expected 0xCAFEBABE)",
                    bytes
                )
            }
            ClassReaderError::InvalidConstantPoolIdx(idx) => {
                write!(f, "Invalid ConstantPool index `{}`", idx)
            }
            ClassReaderError::InvalidMethodHandleReferenceKind(val) => {
                write!(
                    f,
                    "Invalid value `{}` for MethodHandle reference_kind (not in range 1..=9)",
                    val
                )
            }
            ClassReaderError::InvalidConstItemTag(tag) => {
                write!(
                    f,
                    "Invalid ConstItem tag `{}` (expected one of 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 15, 16, 18)",
                    tag
                )
            }
            ClassReaderError::InvalidClassAccessFlags(flags) => {
                write!(f, "Invalid class access flags: {}", flags)
            }
            ClassReaderError::InvalidFieldAccessFlags(flags) => {
                write!(f, "Invalid field access flags: {}", flags)
            }
            ClassReaderError::InvalidMethodAccessFlags(flags) => {
                write!(f, "Invalid method access flags: {}", flags)
            }
            ClassReaderError::UnexpectedEndOfData => {
                write!(f, "Unexpected end of data")
            }
            ClassReaderError::InvalidCesu8String => {
                write!(f, "Invalid cesu8 string")
            }
            ClassReaderError::UnsupportedVersion(major, minor) => {
                write!(f, "Unsupported class file version {major}.{minor}")
            }
        }
    }
}

impl Error for ClassReaderError {}

pub type Result<T> = std::result::Result<T, ClassReaderError>;

impl From<BufferError> for ClassReaderError {
    fn from(err: BufferError) -> Self {
        match err {
            BufferError::UnexpectedEndOfData => Self::UnexpectedEndOfData,
            BufferError::InvalidCesu8String => Self::InvalidCesu8String,
        }
    }
}
