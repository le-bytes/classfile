use crate::{Read, Result, buffer::Buffer, constants::ConstItemIdx};

// TODO
pub enum AttributeKind {
    ConstantValue,
    Code,
    StackMapTable,
    Exceptions,
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    SourceFile,
    SourceDebugExtension,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    BootstrapMethods,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub attribute_name_index: ConstItemIdx,
    pub info: Vec<u8>,
}

impl Read for Attribute {
    fn read(buf: &mut Buffer, consts_count: u16) -> Result<Self> {
        let attribute_name_index = ConstItemIdx::read(buf, consts_count)?;
        let len = buf.read_u32()?;
        let bytes = buf.read_bytes(len as usize)?;

        Ok(Self {
            attribute_name_index,
            info: Vec::from(bytes),
        })
    }
}
