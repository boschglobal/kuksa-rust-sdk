/*
 * *******************************************************************************
 *  Copyright (c) 2025 Contributors to the Eclipse Foundation
 *
 *  See the NOTICE file(s) distributed with this work for additional
 *  information regarding copyright ownership.
 *
 *  This program and the accompanying materials are made available under the
 *  terms of the Apache License 2.0 which is available at
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 *  SPDX-License-Identifier: Apache-2.0
 * ******************************************************************************
 */

pub use crate::proto::kuksa::val::v1 as v1_proto;
pub use crate::proto::kuksa::val::v2 as v2_proto;
pub use crate::proto::sdv::databroker::v1 as sdv_proto;

pub mod kuksa {
    pub mod common;
    pub mod val {
        pub mod v1;

        pub mod v2;
    }
}

pub mod sdv {
    pub mod databroker {
        pub mod v1;
    }
}

pub mod proto {
    pub mod kuksa {
        pub mod val {
            pub mod v1 {
                pub const FILE_DESCRIPTOR_SET: &[u8] =
                    tonic::include_file_descriptor_set!("kuksa.val.v1_descriptor");
                tonic::include_proto!("kuksa.val.v1");

                use datapoint::Value;
                use std::{any::Any, fmt::Display, str::FromStr};

                #[derive(Debug)]
                pub struct ParsingError {
                    message: String,
                }

                impl ParsingError {
                    pub fn new<T: Into<String>>(message: T) -> Self {
                        ParsingError {
                            message: message.into(),
                        }
                    }
                }

                impl Display for ParsingError {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.message.fmt(f)
                    }
                }

                impl std::error::Error for ParsingError {}

                impl FromStr for DataType {
                    type Err = ParsingError;
                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        match s.to_lowercase().as_str() {
                            "string" => Ok(DataType::String),
                            "string[]" => Ok(DataType::StringArray),
                            "bool" => Ok(DataType::Boolean),
                            "bool[]" => Ok(DataType::BooleanArray),
                            "int8" => Ok(DataType::Int8),
                            "int8[]" => Ok(DataType::Int8Array),
                            "int16" => Ok(DataType::Int16),
                            "int16[]" => Ok(DataType::Int16Array),
                            "int32" => Ok(DataType::Int32),
                            "int32[]" => Ok(DataType::Int32Array),
                            "int64" => Ok(DataType::Int64),
                            "int64[]" => Ok(DataType::Int64Array),
                            "uint8" => Ok(DataType::Uint8),
                            "uint8[]" => Ok(DataType::Uint8Array),
                            "uint16" => Ok(DataType::Uint16),
                            "uint16[]" => Ok(DataType::Uint16Array),
                            "uint32" => Ok(DataType::Uint32),
                            "uint32[]" => Ok(DataType::Uint32Array),
                            "uint64" => Ok(DataType::Uint64),
                            "uint64[]" => Ok(DataType::Uint64Array),
                            "float" => Ok(DataType::Float),
                            "float[]" => Ok(DataType::FloatArray),
                            "double" => Ok(DataType::Double),
                            "double[]" => Ok(DataType::DoubleArray),
                            _ => Err(ParsingError::new(format!("unsupported data type '{s}'"))),
                        }
                    }
                }

                impl Value {
                    pub fn new<T: Into<DataType>>(
                        vss_type: T,
                        value: &str,
                    ) -> Result<Self, ParsingError> {
                        let dt: DataType = vss_type.into();
                        match dt {
                            DataType::String => Ok(Value::String(value.to_string())),
                            DataType::Boolean => value
                                .parse::<bool>()
                                .map(Value::Bool)
                                .map_err(|e| ParsingError::new(e.to_string())),
                            DataType::Int8 => value
                                .parse::<i8>()
                                .map(|v| Value::Int32(v as i32))
                                .map_err(|e| ParsingError::new(e.to_string())),
                            DataType::Int16 => value
                                .parse::<i16>()
                                .map(|v| Value::Int32(v as i32))
                                .map_err(|e| ParsingError::new(e.to_string())),
                            DataType::Int32 => value
                                .parse::<i32>()
                                .map(Value::Int32)
                                .map_err(|e| ParsingError::new(e.to_string())),
                            DataType::Int64 => value
                                .parse::<i64>()
                                .map(Value::Int64)
                                .map_err(|e| ParsingError::new(e.to_string())),
                            DataType::Uint8 => value
                                .parse::<u8>()
                                .map(|v| Value::Uint32(v as u32))
                                .map_err(|e| ParsingError::new(e.to_string())),
                            DataType::Uint16 => value
                                .parse::<u16>()
                                .map(|v| Value::Uint32(v as u32))
                                .map_err(|e| ParsingError::new(e.to_string())),
                            DataType::Uint32 => value
                                .parse::<u32>()
                                .map(Value::Uint32)
                                .map_err(|e| ParsingError::new(e.to_string())),
                            DataType::Uint64 => value
                                .parse::<u64>()
                                .map(Value::Uint64)
                                .map_err(|e| ParsingError::new(e.to_string())),
                            DataType::Float => value
                                .parse::<f32>()
                                .map(Value::Float)
                                .map_err(|e| ParsingError::new(e.to_string())),
                            DataType::Double => value
                                .parse::<f64>()
                                .map(Value::Double)
                                .map_err(|e| ParsingError::new(e.to_string())),
                            _ => Err(ParsingError::new(format!(
                                "data type '{:?}' not supported for parsing string into typed value",
                                dt.type_id()
                            ))),
                        }
                    }
                }
            }
            pub mod v2 {
                use value::TypedValue;

                tonic::include_proto!("kuksa.val.v2");

                pub const FILE_DESCRIPTOR_SET: &[u8] =
                    tonic::include_file_descriptor_set!("kuksa.val.v2_descriptor");

                /// Indicates that a [`TypedValue`] cannot be converted to the
                /// desired type because it its value has an incompatible type.
                #[derive(Debug)]
                pub struct IncompatibleValueTypeError {}

                impl TryFrom<TypedValue> for u32 {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for u32 {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Uint32(v) => Ok(*v),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for Vec<u32> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for Vec<u32> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Uint32Array(v) => Ok(v.values.clone()),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for u64 {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for u64 {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Uint32(v) => Ok(*v as u64),
                            TypedValue::Uint64(v) => Ok(*v),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for Vec<u64> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for Vec<u64> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Uint32Array(v) => {
                                Ok(v.values.iter().map(|v| *v as u64).collect())
                            }
                            TypedValue::Uint64Array(v) => Ok(v.values.clone()),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for i32 {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for i32 {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Uint32(v) => Ok(*v as i32),
                            TypedValue::Int32(v) => Ok(*v),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for Vec<i32> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for Vec<i32> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Uint32Array(v) => {
                                Ok(v.values.iter().map(|v| *v as i32).collect())
                            }
                            TypedValue::Int32Array(v) => Ok(v.values.clone()),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for i64 {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for i64 {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Uint32(v) => Ok(*v as i64),
                            TypedValue::Uint64(v) => Ok(*v as i64),
                            TypedValue::Int32(v) => Ok(*v as i64),
                            TypedValue::Int64(v) => Ok(*v),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for Vec<i64> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for Vec<i64> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Uint32Array(v) => {
                                Ok(v.values.iter().map(|v| *v as i64).collect())
                            }
                            TypedValue::Uint64Array(v) => {
                                Ok(v.values.iter().map(|v| *v as i64).collect())
                            }
                            TypedValue::Int32Array(v) => {
                                Ok(v.values.iter().map(|v| *v as i64).collect())
                            }
                            TypedValue::Int64Array(v) => Ok(v.values.clone()),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for f32 {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for f32 {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Uint32(v) => Ok(*v as f32),
                            TypedValue::Int32(v) => Ok(*v as f32),
                            TypedValue::Float(v) => Ok(*v),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for Vec<f32> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for Vec<f32> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Uint32Array(v) => {
                                Ok(v.values.iter().map(|v| *v as f32).collect())
                            }
                            TypedValue::Int32Array(v) => {
                                Ok(v.values.iter().map(|v| *v as f32).collect())
                            }
                            TypedValue::FloatArray(v) => Ok(v.values.clone()),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for f64 {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for f64 {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Uint32(v) => Ok(*v as f64),
                            TypedValue::Uint64(v) => Ok(*v as f64),
                            TypedValue::Int32(v) => Ok(*v as f64),
                            TypedValue::Int64(v) => Ok(*v as f64),
                            TypedValue::Float(v) => Ok(*v as f64),
                            TypedValue::Double(v) => Ok(*v),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for Vec<f64> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for Vec<f64> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Uint32Array(v) => {
                                Ok(v.values.iter().map(|v| *v as f64).collect())
                            }
                            TypedValue::Uint64Array(v) => {
                                Ok(v.values.iter().map(|v| *v as f64).collect())
                            }
                            TypedValue::Int32Array(v) => {
                                Ok(v.values.iter().map(|v| *v as f64).collect())
                            }
                            TypedValue::Int64Array(v) => {
                                Ok(v.values.iter().map(|v| *v as f64).collect())
                            }
                            TypedValue::FloatArray(v) => {
                                Ok(v.values.iter().map(|v| *v as f64).collect())
                            }
                            TypedValue::DoubleArray(v) => Ok(v.values.clone()),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for String {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for String {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::String(v) => Ok(v.to_string()),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for Vec<String> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for Vec<String> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::StringArray(v) => Ok(v.values.clone()),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for bool {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for bool {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::Bool(v) => Ok(*v),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }

                impl TryFrom<TypedValue> for Vec<bool> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: TypedValue) -> Result<Self, Self::Error> {
                        Self::try_from(&value)
                    }
                }

                impl TryFrom<&TypedValue> for Vec<bool> {
                    type Error = IncompatibleValueTypeError;
                    fn try_from(value: &TypedValue) -> Result<Self, Self::Error> {
                        match value {
                            TypedValue::BoolArray(v) => Ok(v.values.clone()),
                            _ => Err(IncompatibleValueTypeError {}),
                        }
                    }
                }
            }
        }
    }
    pub mod sdv {
        pub mod databroker {
            pub mod v1 {
                pub const FILE_DESCRIPTOR_SET: &[u8] =
                    tonic::include_file_descriptor_set!("sdv.databroker.v1_descriptor");
                tonic::include_proto!("sdv.databroker.v1");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_proto::value::TypedValue;
    use test_case::test_case;

    #[test_case(
        TypedValue::String("one".to_string()),
        "one".to_string();
        "for String")]
    #[test_case(
        TypedValue::StringArray(proto::kuksa::val::v2::StringArray { values: vec!["one".to_string(), "two".to_string()] }),
        vec!["one".to_string(), "two".to_string()];
        "for StringArray")]
    #[test_case(
        TypedValue::Bool(true),
        true;
        "for Bool")]
    #[test_case(
        TypedValue::BoolArray(proto::kuksa::val::v2::BoolArray { values: vec![true, false] }),
        vec![true, false];
        "for BoolArray")]
    #[test_case(
        TypedValue::Uint32(0x01234567_u32),
        0x01234567_u32;
        "for UInt32")]
    #[test_case(
        TypedValue::Uint32Array(proto::kuksa::val::v2::Uint32Array { values: vec![0x01234567_u32, 0x89abcdef_u32] }),
        vec![0x01234567_u32, 0x89abcdef_u32];
        "for UInt32Array")]
    #[test_case(
        TypedValue::Int32(0x01234567_i32),
        0x01234567_i32;
        "for Int32")]
    #[test_case(
        TypedValue::Int32Array(proto::kuksa::val::v2::Int32Array { values: vec![0x01234567_i32, 0x89abcdef_u32 as i32] }),
        vec![0x01234567_i32, 0x89abcdef_u32 as i32];
        "for Int32Array")]
    #[test_case(
        TypedValue::Uint32(0x01234567_u32),
        0x01234567_u32 as i32;
        "for UInt32 as Int32")]
    #[test_case(
        TypedValue::Uint32Array(proto::kuksa::val::v2::Uint32Array { values: vec![0x01234567_u32, 0x89abcdef_u32] }),
        vec![0x01234567_u32 as i32, 0x89abcdef_u32 as i32];
        "for UInt32Array as UInt32Array")]
    #[test_case(
        TypedValue::Uint64(0x0123456789abcdef_u64),
        0x0123456789abcdef_u64;
        "for UInt64")]
    #[test_case(
        TypedValue::Uint64Array(proto::kuksa::val::v2::Uint64Array { values: vec![0x0123456789abcdef_u64, 0x0123456789abcdef_u64] }),
        vec![0x0123456789abcdef_u64, 0x0123456789abcdef_u64];
        "for UInt64Array")]
    #[test_case(
        TypedValue::Uint32(0x01234567_u32),
        0x01234567_u32 as u64;
        "for UInt32 as UInt64")]
    #[test_case(
        TypedValue::Uint32Array(proto::kuksa::val::v2::Uint32Array { values: vec![0x01234567_u32, 0x89abcdef_u32] }),
        vec![0x01234567_u32 as u64, 0x89abcdef_u32 as u64];
        "for UInt32Array as UInt64Array")]
    #[test_case(
        TypedValue::Int64(0x0123456789abcdef_i64),
        0x0123456789abcdef_i64;
        "for Int64")]
    #[test_case(
        TypedValue::Int64Array(proto::kuksa::val::v2::Int64Array { values: vec![0x0123456789abcdef_i64, 0xfedcba9876543210_u64 as i64] }),
        vec![0x0123456789abcdef_i64, 0xfedcba9876543210_u64 as i64];
        "for Int64Array")]
    #[test_case(
        TypedValue::Uint32(0x01234567_u32),
        0x01234567_u32 as i64;
        "for UInt32 as Int64")]
    #[test_case(
        TypedValue::Uint32Array(proto::kuksa::val::v2::Uint32Array { values: vec![0x01234567_u32, 0x89abcdef_u32] }),
        vec![0x01234567_u32 as i64, 0x89abcdef_u32 as i64];
        "for UInt32Array as Int64Array")]
    #[test_case(
        TypedValue::Int32(0x01234567_i32),
        0x01234567_i32 as i64;
        "for Int32 as Int64")]
    #[test_case(
        TypedValue::Int32Array(proto::kuksa::val::v2::Int32Array { values: vec![0x01234567_i32, 0x89abcdef_u32 as i32] }),
        vec![0x01234567_i32 as i64, (0x89abcdef_u32 as i32) as i64];
        "for Int32Array as Int64Array")]
    #[test_case(
        TypedValue::Uint64(0xfedcba9876543210_u64),
        0xfedcba9876543210_u64 as i64;
        "for UInt64 as Int64")]
    #[test_case(
        TypedValue::Uint64Array(proto::kuksa::val::v2::Uint64Array { values: vec![0x0123456789abcdef_u64, 0xfedcba9876543210_u64] }),
        vec![0x0123456789abcdef_u64 as i64, 0xfedcba9876543210_u64 as i64];
        "for UInt64Array as Int64Array")]
    #[test_case(
        TypedValue::Float(34.345),
        34.345 as f32;
        "for Float")]
    #[test_case(
        TypedValue::FloatArray(proto::kuksa::val::v2::FloatArray { values: vec![34.345, -34.345] }),
        vec![34.345 as f32, -34.345 as f32];
        "for FloatArray")]
    #[test_case(
        TypedValue::Uint32(0x01234567_u32),
        0x01234567_u32 as f32;
        "for UInt32 as Float")]
    #[test_case(
        TypedValue::Uint32Array(proto::kuksa::val::v2::Uint32Array { values: vec![0x01234567_u32, 0x89abcdef_u32] }),
        vec![0x01234567_u32 as f32, 0x89abcdef_u32 as f32];
        "for UInt32Array as Float32Array")]
    #[test_case(
        TypedValue::Int32(0x01234567_i32),
        0x01234567_i32 as f32;
        "for Int32 as Float")]
    #[test_case(
        TypedValue::Int32Array(proto::kuksa::val::v2::Int32Array { values: vec![0x01234567_i32, 0x89abcdef_u32 as i32] }),
        vec![0x01234567_i32 as f32, (0x89abcdef_u32 as i32) as f32];
        "for Int32Array as FloatArray")]
    #[test_case(
        TypedValue::Double(34.345),
        34.345 as f64;
        "for Double")]
    #[test_case(
        TypedValue::DoubleArray(proto::kuksa::val::v2::DoubleArray { values: vec![34.345, -34.345] }),
        vec![34.345 as f64, -34.345 as f64];
        "for DoubleArray")]
    #[test_case(
        TypedValue::Float(34.345),
        34.345_f32 as f64;
        "for Float as Double")]
    #[test_case(
        TypedValue::FloatArray(proto::kuksa::val::v2::FloatArray { values: vec![34.345, -34.345] }),
        vec![34.345_f32 as f64, -34.345_f32 as f64];
        "for FloatArray as DoubleArray")]
    #[test_case(
        TypedValue::Uint32(0x89abcdef_u32),
        0x89abcdef_u32 as f64;
        "for UInt32 as Double")]
    #[test_case(
        TypedValue::Uint32Array(proto::kuksa::val::v2::Uint32Array { values: vec![0x01234567_u32, 0x89abcdef_u32] }),
        vec![0x01234567_u32 as f64, 0x89abcdef_u32 as f64];
        "for UInt32Array as DoubleArray")]
    #[test_case(
        TypedValue::Uint64(0xfedcba9876543210_u64),
        0xfedcba9876543210_u64 as f64;
        "for UInt64 as Double")]
    #[test_case(
        TypedValue::Uint64Array(proto::kuksa::val::v2::Uint64Array { values: vec![0x0123456789abcdef_u64, 0xfedcba9876543210_u64] }),
        vec![0x0123456789abcdef_u64 as f64, 0xfedcba9876543210_u64 as f64];
        "for UInt64Array as DoubleArray")]
    #[test_case(
        TypedValue::Int32(0x01234567_i32),
        0x01234567_i32 as f64;
        "for Int32 as Double")]
    #[test_case(
        TypedValue::Int32Array(proto::kuksa::val::v2::Int32Array { values: vec![0x01234567_i32, 0x89abcdef_u32 as i32] }),
        vec![0x01234567_i32 as f64, (0x89abcdef_u32 as i32) as f64];
        "for Int32Array as DoubleArray")]
    #[test_case(
        TypedValue::Int64(0x0123456789abcdef_i64),
        0x0123456789abcdef_i64 as f64;
        "for Int64 as Double")]
    #[test_case(
        TypedValue::Int64Array(proto::kuksa::val::v2::Int64Array { values: vec![0x0123456789abcdef_i64, 0xfedcba9876543210_u64 as i64] }),
        vec![0x0123456789abcdef_i64, 0xfedcba9876543210_u64 as i64];
        "for Int64Array as DoubleArray")]
    fn test_try_from_typedvalue<T> (
        data: TypedValue,
        expected_value: T,
    ) where 
        T: Sized
            + std::convert::TryFrom<proto::kuksa::val::v2::value::TypedValue>
            + std::cmp::PartialEq {
        assert!(T::try_from(data).is_ok_and(|v| v.eq(&expected_value)));
    }
}
