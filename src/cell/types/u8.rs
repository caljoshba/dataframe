use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct UInt8 {}

impl DType for UInt8 {
    fn dtype() -> DataType {
        DataType::UInt8
    }
}

impl From<u8> for AnyType {
    fn from(value: u8) -> Self {
        AnyType::UInt8(value)
    }
}

// impl Into<AnyType> for u8 {
//     fn into(self) -> AnyType {
//         AnyType::UInt8(self)
//     }
// }