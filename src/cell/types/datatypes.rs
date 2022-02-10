// https://github.com/pola-rs/polars/blob/master/polars/polars-core/src/datatypes.rs

use std::hash::{Hash, Hasher};
use std::fmt::{
    Display,
    Formatter,
    Result
};
use std::ops::{
    Add,
    Div,
    Mul,
    Sub,
};

#[derive(Debug, Copy, Clone)]
pub enum AnyType {
    Null,
    Boolean(bool),
    Utf8(&'static str),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    USize(usize),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    ISize(isize),
    Float32(f32),
    Float64(f64),
    // Date,
    // Datetime,
    // Duration,
    // Time,
}

impl<T> From<Option<T>> for AnyType
where
    T: Into<AnyType>,
{
    fn from(a: Option<T>) -> Self {
        match a {
            None => AnyType::Null,
            Some(v) => v.into(),
        }
    }
}

// impl From<AnyType> for dyn Any {
//     fn from(a: AnyType) -> Any {
//         None
//     }
// }

// impl AnyType {
//     pub fn to_value(&self) -> AllowedTypes {
//         match self {
//             AnyType::Null => 0u8,
//             AnyType::Boolean(val) => val,
//             AnyType::Utf8(val) => val,
//             AnyType::UInt8(val) => val,
//             AnyType::UInt16(val) => val,
//             AnyType::UInt32(val) => val,
//             AnyType::UInt64(val) => val,
//             AnyType::USize(val) => val,
//             AnyType::Int8(val) => val,
//             AnyType::Int16(val) => val,
//             AnyType::Int32(val) => val,
//             AnyType::Int64(val) => val,
//             AnyType::ISize(val) => val,
//             AnyType::Float32(val) => val,
//             AnyType::Float64(val) => val
//         }
//     }
// }

// enum AllowedTypes {bool, str, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64};

// impl From<AnyType> for AllowedTypes {
//     fn from(any_type: AnyType) -> AllowedTypes {
//         match any_type {
//             AnyType::Null => 0u8,
//             AnyType::Boolean(val) => val,
//             AnyType::Utf8(val) => val,
//             AnyType::UInt8(val) => val,
//             AnyType::UInt16(val) => val,
//             AnyType::UInt32(val) => val,
//             AnyType::UInt64(val) => val,
//             AnyType::USize(val) => val,
//             AnyType::Int8(val) => val,
//             AnyType::Int16(val) => val,
//             AnyType::Int32(val) => val,
//             AnyType::Int64(val) => val,
//             AnyType::ISize(val) => val,
//             AnyType::Float32(val) => val,
//             AnyType::Float64(val) => val
//         }
//     }
// }

impl Hash for AnyType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use AnyType::*;
        match self {
            Null => state.write_u64(u64::MAX / 2 + 135123),
            Int8(v) => state.write_i8(*v),
            Int16(v) => state.write_i16(*v),
            Int32(v) => state.write_i32(*v),
            Int64(v) => state.write_i64(*v),
            ISize(v) => state.write_isize(*v),
            UInt8(v) => state.write_u8(*v),
            UInt16(v) => state.write_u16(*v),
            UInt32(v) => state.write_u32(*v),
            UInt64(v) => state.write_u64(*v),
            USize(v) => state.write_usize(*v),
            Utf8(s) => state.write(s.as_bytes()),
            Boolean(v) => state.write_u8(*v as u8),
            Float32(v) => state.write_i32(v.floor() as i32),
            Float64(v) => state.write_i64(v.floor() as i64),
        }
    }
}

impl Display for AnyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AnyType::Null => write!(f, "null"),
            AnyType::Boolean(val) => write!(f, "{}", val),
            AnyType::Utf8(val) => write!(f, "{}", val),
            AnyType::UInt8(val) => write!(f, "{}", val),
            AnyType::UInt16(val) => write!(f, "{}", val),
            AnyType::UInt32(val) => write!(f, "{}", val),
            AnyType::UInt64(val) => write!(f, "{}", val),
            AnyType::USize(val) => write!(f, "{}", val),
            AnyType::Int8(val) => write!(f, "{}", val),
            AnyType::Int16(val) => write!(f, "{}", val),
            AnyType::Int32(val) => write!(f, "{}", val),
            AnyType::Int64(val) => write!(f, "{}", val),
            AnyType::ISize(val) => write!(f, "{}", val),
            AnyType::Float32(val) => write!(f, "{}", val),
            AnyType::Float64(val) => write!(f, "{}", val)
        }
    }
}

impl Add for AnyType {
    type Output = AnyType;
    fn add(self, rhs: Self) -> Self::Output {
        use AnyType::*;
        match (self, rhs) {
            (UInt8(lv), UInt8(rv)) => UInt8(lv + rv),
            (UInt16(lv), UInt16(rv)) => UInt16(lv + rv),
            (UInt32(lv), UInt32(rv)) => UInt32(lv + rv),
            (UInt64(lv), UInt64(rv)) => UInt64(lv + rv),
            (USize(lv), USize(rv)) => USize(lv + rv),
            (Int8(lv), Int8(rv)) => Int8(lv + rv),
            (Int16(lv), Int16(rv)) => Int16(lv + rv),
            (Int32(lv), Int32(rv)) => Int32(lv + rv),
            (Int64(lv), Int64(rv)) => Int64(lv + rv),
            (ISize(lv), ISize(rv)) => ISize(lv + rv),
            (Float32(lv), Float32(rv)) => Float32(lv + rv),
            (Float64(lv), Float64(rv)) => Float64(lv + rv),
            (Null, UInt8(rv)) => UInt8(rv),
            (Null, UInt16(rv)) => UInt16(rv),
            (Null, UInt32(rv)) => UInt32(rv),
            (Null, UInt64(rv)) => UInt64(rv),
            (Null, USize(rv)) => USize(rv),
            (Null, Int8(rv)) => Int8(rv),
            (Null, Int16(rv)) => Int16(rv),
            (Null, Int32(rv)) => Int32(rv),
            (Null, Int64(rv)) => Int64(rv),
            (Null, ISize(rv)) => ISize(rv),
            (Null, Float32(rv)) => Float32(rv),
            (Null, Float64(rv)) => Float64(rv),
            (_, _) => Null 
        }     
    }
}

impl Div for AnyType {
    type Output = AnyType;
    fn div(self, rhs: Self) -> Self::Output {
        use AnyType::*;
        match (self, rhs) {
            (UInt8(lv), UInt8(rv)) => UInt8(lv / rv),
            (UInt16(lv), UInt16(rv)) => UInt16(lv / rv),
            (UInt32(lv), UInt32(rv)) => UInt32(lv / rv),
            (UInt64(lv), UInt64(rv)) => UInt64(lv / rv),
            (USize(lv), USize(rv)) => USize(lv / rv),
            (Int8(lv), Int8(rv)) => Int8(lv / rv),
            (Int16(lv), Int16(rv)) => Int16(lv / rv),
            (Int32(lv), Int32(rv)) => Int32(lv / rv),
            (Int64(lv), Int64(rv)) => Int64(lv / rv),
            (ISize(lv), ISize(rv)) => ISize(lv / rv),
            (Float32(lv), Float32(rv)) => Float32(lv / rv),
            (Float64(lv), Float64(rv)) => Float64(lv / rv),
            (UInt8(lv), USize(rv)) => UInt8(lv / rv as u8),
            (UInt16(lv), USize(rv)) => UInt16(lv / rv as u16),
            (UInt32(lv), USize(rv)) => UInt32(lv / rv as u32),
            (UInt64(lv), USize(rv)) => UInt64(lv / rv as u64),
            (Int8(lv), USize(rv)) => Int8(lv / rv as i8),
            (Int16(lv), USize(rv)) => Int16(lv / rv as i16),
            (Int32(lv), USize(rv)) => Int32(lv / rv as i32),
            (Int64(lv), USize(rv)) => Int64(lv / rv as i64),
            (Float32(lv), USize(rv)) => Float32(lv / rv as f32),
            (Float64(lv), USize(rv)) => Float64(lv / rv as f64),
            (Int8(lv), ISize(rv)) => Int8(lv / rv as i8),
            (Int16(lv), ISize(rv)) => Int16(lv / rv as i16),
            (Int32(lv), ISize(rv)) => Int32(lv / rv as i32),
            (Int64(lv), ISize(rv)) => Int64(lv / rv as i64),
            (Float32(lv), ISize(rv)) => Float32(lv / rv as f32),
            (Float64(lv), ISize(rv)) => Float64(lv / rv as f64),
            (_, _) => Null 
        }     
    }
}

impl Mul for AnyType {
    type Output = AnyType;
    fn mul(self, rhs: Self) -> Self::Output {
        use AnyType::*;
        match (self, rhs) {
            (UInt8(lv), UInt8(rv)) => USize(lv as usize * rv as usize),
            (UInt16(lv), UInt16(rv)) => USize(lv as usize * rv as usize),
            (UInt32(lv), UInt32(rv)) => USize(lv as usize * rv as usize),
            (UInt64(lv), UInt64(rv)) => USize(lv as usize * rv as usize),
            (USize(lv), USize(rv)) => USize(lv * rv),
            (Int8(lv), Int8(rv)) => ISize(lv as isize * rv as isize),
            (Int16(lv), Int16(rv)) => ISize(lv as isize * rv as isize),
            (Int32(lv), Int32(rv)) => ISize(lv as isize * rv as isize),
            (Int64(lv), Int64(rv)) => ISize(lv as isize * rv as isize),
            (ISize(lv), ISize(rv)) => ISize(lv * rv),
            (Float32(lv), Float32(rv)) => Float64(lv as f64 * rv as f64),
            (Float64(lv), Float64(rv)) => Float64(lv * rv),
            (UInt8(lv), USize(rv)) => USize(lv as usize * rv),
            (UInt16(lv), USize(rv)) => USize(lv as usize * rv),
            (UInt32(lv), USize(rv)) => USize(lv as usize * rv),
            (UInt64(lv), USize(rv)) => USize(lv as usize * rv),
            (Int8(lv), USize(rv)) => ISize(lv as isize * rv as isize),
            (Int16(lv), USize(rv)) => ISize(lv as isize * rv as isize),
            (Int32(lv), USize(rv)) => ISize(lv as isize * rv as isize),
            (Int64(lv), USize(rv)) => ISize(lv as isize * rv as isize),
            (Float32(lv), USize(rv)) => Float64(lv as f64 * rv as f64),
            (Float64(lv), USize(rv)) => Float64(lv * rv as f64),
            (Int8(lv), ISize(rv)) => ISize(lv as isize * rv as isize),
            (Int16(lv), ISize(rv)) => ISize(lv as isize * rv as isize),
            (Int32(lv), ISize(rv)) => ISize(lv as isize * rv as isize),
            (Int64(lv), ISize(rv)) => ISize(lv as isize * rv as isize),
            (Float32(lv), ISize(rv)) => Float64(lv as f64 * rv as f64),
            (Float64(lv), ISize(rv)) => Float64(lv * rv as f64),
            (_, _) => Null 
        }     
    }
}

impl Sub for AnyType {
    type Output = AnyType;
    fn sub(self, rhs: Self) -> Self::Output {
        use AnyType::*;
        match (self, rhs) {
            (UInt8(lv), UInt8(rv)) => ISize(lv as isize - rv as isize),
            (UInt16(lv), UInt16(rv)) => ISize(lv as isize - rv as isize),
            (UInt32(lv), UInt32(rv)) => ISize(lv as isize - rv as isize),
            (UInt64(lv), UInt64(rv)) => ISize(lv as isize - rv as isize),
            (USize(lv), USize(rv)) => ISize(lv as isize - rv as isize),
            (Int8(lv), Int8(rv)) => ISize(lv as isize - rv as isize),
            (Int16(lv), Int16(rv)) => ISize(lv as isize - rv as isize),
            (Int32(lv), Int32(rv)) => ISize(lv as isize - rv as isize),
            (Int64(lv), Int64(rv)) => ISize(lv as isize - rv as isize),
            (ISize(lv), ISize(rv)) => ISize(lv as isize - rv as isize),
            (Float32(lv), Float32(rv)) => Float32(lv - rv),
            (Float64(lv), Float64(rv)) => Float64(lv - rv),
            (_, _) => Null 
        }     
    }
}

// #[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum DataType {
    Null,
    Boolean,
    Utf8,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    USize,
    Int8,
    Int16,
    Int32,
    Int64,
    ISize,
    Float32,
    Float64
}

impl PartialEq for AnyType {
    fn eq(&self, other: &Self) -> bool {
        use AnyType::*;
        match (self, other) {
            (Null, Null) => true,
            (Boolean(val), Boolean(rhs)) => val == rhs,
            (Utf8(val), Utf8(rhs)) => val == rhs,
            (UInt8(val), UInt8(rhs)) => val == rhs,
            (UInt16(val), UInt16(rhs)) => val == rhs,
            (UInt32(val), UInt32(rhs)) => val == rhs,
            (UInt64(val), UInt64(rhs)) => val == rhs,
            (USize(val), USize(rhs)) => val == rhs,
            (Int8(val), Int8(rhs)) => val == rhs,
            (Int16(val), Int16(rhs)) => val == rhs,
            (Int32(val), Int32(rhs)) => val == rhs,
            (Int64(val), Int64(rhs)) => val == rhs,
            (ISize(val), ISize(rhs)) => val == rhs,
            (Float32(val), Float32(rhs)) => val == rhs,
            (Float64(val), Float64(rhs)) => val == rhs,
            (_, _) => false
        }
    }
}

impl Eq for AnyType {}

// #[derive(Debug, Clone)]
// pub struct Date {}
// #[derive(Debug, Clone)]
// pub struct Datetime {}
// #[derive(Debug, Clone)]
// pub struct Duration {}
// #[derive(Debug, Clone)]
// pub struct Time {}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            DataType::Null => "null",
            DataType::Boolean => "bool",
            DataType::UInt8 => "u8",
            DataType::UInt16 => "u16",
            DataType::UInt32 => "u32",
            DataType::UInt64 => "u64",
            DataType::USize => "usize",
            DataType::Int8 => "i8",
            DataType::Int16 => "i16",
            DataType::Int32 => "i32",
            DataType::Int64 => "i64",
            DataType::ISize => "isize",
            DataType::Float32 => "f32",
            DataType::Float64 => "f64",
            DataType::Utf8 => "&'static str",
        };
        f.write_str(s)
    }
}

pub trait DType {
    fn dtype() -> DataType where Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_none() {
        let none: Option<u8> = None;
        let value: AnyType = none.into();
        assert!(value == AnyType::Null);
    }
    #[test]
    fn input_u8() {
        let value: AnyType = 3u8.into();
        assert!(value == AnyType::UInt8(3));
    }
    #[test]
    fn u8_addition() {
        let initial: AnyType = 3u8.into();
        let secondary: AnyType = 2u8.into();

        let value = initial + secondary;
        assert!(value == AnyType::UInt8(5));
    }
    #[test]
    fn u8_subtraction() {
        let initial: AnyType = 3u8.into();
        let secondary: AnyType = 5u8.into();

        let value = initial - secondary;
        assert!(value == AnyType::ISize(-2isize));
    }
    #[test]
    fn u8_multiplcation() {
        let initial: AnyType = 3u8.into();
        let secondary: AnyType = 2u8.into();

        let value = initial * secondary;
        assert!(value == AnyType::USize(6usize));
    }
    #[test]
    fn u8_division() {
        let initial: AnyType = 6u8.into();
        let secondary: AnyType = 2u8.into();

        let value = initial / secondary;
        assert!(value == AnyType::UInt8(3u8));
    }
    #[test]
    fn input_u16() {
        let value: AnyType = 3u16.into();
        assert!(value == AnyType::UInt16(3u16));
    }
    #[test]
    fn u16_addition() {
        let initial: AnyType = 3u16.into();
        let secondary: AnyType = 2u16.into();

        let value = initial + secondary;
        assert!(value == AnyType::UInt16(5u16));
    }
    #[test]
    fn u16_subtraction() {
        let initial: AnyType = 3u16.into();
        let secondary: AnyType = 5u16.into();

        let value = initial - secondary;
        assert!(value == AnyType::ISize(-2isize));
    }
    #[test]
    fn u16_multiplcation() {
        let initial: AnyType = 3u16.into();
        let secondary: AnyType = 2u16.into();

        let value = initial * secondary;
        assert!(value == AnyType::USize(6usize));
    }
    #[test]
    fn u16_division() {
        let initial: AnyType = 6u16.into();
        let secondary: AnyType = 2u16.into();

        let value = initial / secondary;
        assert!(value == AnyType::UInt16(3u16));
    }
    #[test]
    fn input_f32() {
        let value: AnyType = 3f32.into();
        assert!(value == AnyType::Float32(3f32));
    }
    #[test]
    fn f32_addition() {
        let initial: AnyType = 3f32.into();
        let secondary: AnyType = 2f32.into();

        let value = initial + secondary;
        assert!(value == AnyType::Float32(5f32));
    }
    #[test]
    fn f32_subtraction() {
        let initial: AnyType = 3f32.into();
        let secondary: AnyType = 5f32.into();

        let value = initial - secondary;
        assert!(value == AnyType::Float32(-2f32));
    }
    #[test]
    fn f32_multiplcation() {
        let initial: AnyType = 3f32.into();
        let secondary: AnyType = 2f32.into();

        let value = initial * secondary;
        assert!(value == AnyType::Float64(6f64));
    }
    #[test]
    fn f32_division() {
        let initial: AnyType = 6f32.into();
        let secondary: AnyType = 2f32.into();

        let value = initial / secondary;
        assert!(value == AnyType::Float32(3f32));
    }

    #[test]
    fn from_anytype() {
        let initial: AnyType = 6f32.into();
        let into: Option<f32> = initial.into();

        assert_eq!(Some(6f32), into);
    }

    #[test]
    fn from_anytype_incorrect_type() {
        let initial: AnyType = 6f32.into();
        let into: Option<i8> = initial.into();

        assert_eq!(None, into);
    }
}