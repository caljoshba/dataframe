// https://github.com/pola-rs/polars/blob/master/polars/polars-core/src/datatypes.rs

use super::{
    bool::Boolean,
    utf8::Utf8,
    u8::UInt8,
    u16::UInt16,
    u32::UInt32,
    u64::UInt64,
    i8::Int8,
    i16::Int16,
    i32::Int32,
    i64::Int64,
    f32::Float32,
    f64::Float64,
};
use std::hash::{Hash, Hasher};

pub enum DataType {
    Null,
    Boolean(bool),
    Utf8(String),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    // Date,
    // Datetime,
    // Duration,
    // Time,
}

impl Hash for DataType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use DataType::*;
        match self {
            Null => state.write_u64(u64::MAX / 2 + 135123),
            Int8(v) => state.write_i8(*v),
            Int16(v) => state.write_i16(*v),
            Int32(v) => state.write_i32(*v),
            Int64(v) => state.write_i64(*v),
            UInt8(v) => state.write_u8(*v),
            UInt16(v) => state.write_u16(*v),
            UInt32(v) => state.write_u32(*v),
            UInt64(v) => state.write_u64(*v),
            Utf8(s) => state.write(s.as_bytes()),
            Boolean(v) => state.write_u8(*v as u8),
            _ => unimplemented!(),
        }
    }
}

impl Eq for AnyValue {}

// #[derive(Debug, Clone)]
// pub struct Date {}
// #[derive(Debug, Clone)]
// pub struct Datetime {}
// #[derive(Debug, Clone)]
// pub struct Duration {}
// #[derive(Debug, Clone)]
// pub struct Time {}

pub trait DType {
    fn dtype() -> DataType where Self: Sized
}