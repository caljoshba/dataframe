use super::datatypes::{
    DType
};

#[derive(Debug, Clone)]
pub struct Boolean {}

impl DType for Boolean {
    fn dtype() -> DataType {
        DataType::Boolean
    }
}

impl From<bool> for DataType {
    fn from(value: bool) -> Self {
        DataType::Boolean(value)
    }
}