pub mod attribute;
pub mod buffer;
pub mod constants;
pub mod error;
pub mod field;
pub mod method;
pub mod reader;
pub mod version;

use attribute::Attribute;
use bitflags::bitflags;
use buffer::Buffer;
use constants::{ConstItem, ConstItemIdx, Constants};
use error::ClassReaderError;
use field::Field;
use index_vec::IndexVec;
use method::Method;

pub use error::Result;
use version::ClassFileVersion;

pub trait Read: Sized {
    fn read(buf: &mut Buffer, consts_count: u16, empty_const_slots: &[u16]) -> Result<Self>;
}

impl<T: Read> Read for Vec<T> {
    fn read(buf: &mut Buffer, consts_count: u16, empty_const_slots: &[u16]) -> Result<Self> {
        let count = buf.read_u16()?;
        let mut vec = Vec::with_capacity(count as usize);
        for _ in 0..count {
            vec.push(T::read(buf, consts_count, empty_const_slots)?);
        }
        Ok(vec)
    }
}

bitflags! {
    /// Class flags
    #[derive(Debug, Clone)]
    pub struct ClassAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const FINAL = 0x0010;
        const SUPER = 0x0020;
        const INTERFACE = 0x0200;
        const ABSTRACT = 0x0400;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
    }
}

impl Default for ClassAccessFlags {
    fn default() -> ClassAccessFlags {
        ClassAccessFlags::empty()
    }
}

#[derive(Debug, Clone)]
pub struct ClassFile {
    pub version: ClassFileVersion,
    pub constants: Constants,
    pub access_flag: ClassAccessFlags,
    pub this_class: ConstItemIdx,
    pub super_class: Option<ConstItemIdx>,
    pub interfaces: Vec<ConstItemIdx>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
    pub empty_const_slots: Vec<u16>,
}

impl ClassFile {
    pub fn read(buf: &[u8]) -> Result<Self> {
        let mut buf = Buffer::new(buf);
        Self::check_magic_number(&mut buf)?;
        let version = Self::read_version(&mut buf)?;
        let (constants, empty_const_slots) = Self::read_constants(&mut buf)?;
        let consts_count = constants.len() as u16;
        let access_flag = Self::read_access_flags(&mut buf)?;
        let this_class = ConstItemIdx::read(&mut buf, consts_count, &empty_const_slots)?;
        let super_class = Option::<ConstItemIdx>::read(&mut buf, consts_count, &empty_const_slots)?;
        let interfaces = Vec::read(&mut buf, consts_count, &empty_const_slots)?;
        let fields = Vec::read(&mut buf, consts_count, &empty_const_slots)?;
        let methods = Vec::read(&mut buf, consts_count, &empty_const_slots)?;
        let attributes = Vec::read(&mut buf, consts_count, &empty_const_slots)?;

        Ok(Self {
            version,
            constants,
            access_flag,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
            empty_const_slots,
        })
    }

    fn check_magic_number(buf: &mut Buffer) -> Result<()> {
        match buf.read_u32() {
            Ok(0xCAFEBABE) => Ok(()),
            Ok(bytes) => Err(ClassReaderError::InvalidMagicBytes(bytes)),
            Err(err) => Err(err.into()),
        }
    }

    fn read_version(buf: &mut Buffer) -> Result<ClassFileVersion> {
        let minor = buf.read_u16()?;
        let major = buf.read_u16()?;

        ClassFileVersion::from(major, minor)
    }

    fn read_constants(buf: &mut Buffer) -> Result<(Constants, Vec<u16>)> {
        let consts_count = buf.read_u16()? - 1;
        let mut empty_const_slots = Vec::new();
        let mut real_len = 0;
        let mut i = 0;
        let pos = buf.get_pos();
        while i < consts_count {
            let item = ConstItem::read(buf, consts_count, &[])?;
            if item.is_8bit() {
                i += 1;
                empty_const_slots.push(i);
            }
            real_len += 1;
            i += 1;
        }
        buf.set_pos(pos);
        let mut consts = IndexVec::with_capacity(real_len);
        for _ in 0..real_len {
            let item = ConstItem::read(buf, real_len as u16, &empty_const_slots)?;
            consts.push(item);
        }
        Ok((consts, empty_const_slots))
    }

    fn read_access_flags(buf: &mut Buffer) -> Result<ClassAccessFlags> {
        let num = buf.read_u16()?;
        ClassAccessFlags::from_bits(num).ok_or(ClassReaderError::InvalidClassAccessFlags(num))
    }
}
