use polywrap_wasm_rs::{EnumTypeError};
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ScalarType {
    UInt,
    UInt8,
    UInt16,
    UInt32,
    Int,
    Int8,
    Int16,
    Int32,
    String,
    Boolean,
    Bytes,
    BigInt,
    BigNumber,
    JSON,
    _MAX_
}

pub fn sanitize_scalar_type_value(value: i32) -> Result<(), EnumTypeError> {
    if value < 0 && value >= ScalarType::_MAX_ as i32 {
        return Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'ScalarType': {}", value.to_string())));
    }
    Ok(())
}

pub fn get_scalar_type_value(key: &str) -> Result<ScalarType, EnumTypeError> {
    match key {
        "UInt" => Ok(ScalarType::UInt),
        "UInt8" => Ok(ScalarType::UInt8),
        "UInt16" => Ok(ScalarType::UInt16),
        "UInt32" => Ok(ScalarType::UInt32),
        "Int" => Ok(ScalarType::Int),
        "Int8" => Ok(ScalarType::Int8),
        "Int16" => Ok(ScalarType::Int16),
        "Int32" => Ok(ScalarType::Int32),
        "String" => Ok(ScalarType::String),
        "Boolean" => Ok(ScalarType::Boolean),
        "Bytes" => Ok(ScalarType::Bytes),
        "BigInt" => Ok(ScalarType::BigInt),
        "BigNumber" => Ok(ScalarType::BigNumber),
        "JSON" => Ok(ScalarType::JSON),
        "_MAX_" => Ok(ScalarType::_MAX_),
        err => Err(EnumTypeError::EnumProcessingError(format!("Invalid key for enum 'ScalarType': {}", err)))
    }
}

pub fn get_scalar_type_key(value: ScalarType) -> Result<String, EnumTypeError> {
    if sanitize_scalar_type_value(value as i32).is_ok() {
        match value {
            ScalarType::UInt => Ok("UInt".to_string()),
            ScalarType::UInt8 => Ok("UInt8".to_string()),
            ScalarType::UInt16 => Ok("UInt16".to_string()),
            ScalarType::UInt32 => Ok("UInt32".to_string()),
            ScalarType::Int => Ok("Int".to_string()),
            ScalarType::Int8 => Ok("Int8".to_string()),
            ScalarType::Int16 => Ok("Int16".to_string()),
            ScalarType::Int32 => Ok("Int32".to_string()),
            ScalarType::String => Ok("String".to_string()),
            ScalarType::Boolean => Ok("Boolean".to_string()),
            ScalarType::Bytes => Ok("Bytes".to_string()),
            ScalarType::BigInt => Ok("BigInt".to_string()),
            ScalarType::BigNumber => Ok("BigNumber".to_string()),
            ScalarType::JSON => Ok("JSON".to_string()),
            ScalarType::_MAX_ => Ok("_MAX_".to_string()),
        }
    } else {
        Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'ScalarType': {}", (value  as i32).to_string())))
    }
}

impl TryFrom<i32> for ScalarType {
    type Error = EnumTypeError;

    fn try_from(v: i32) -> Result<ScalarType, Self::Error> {
        match v {
            x if x == ScalarType::UInt as i32 => Ok(ScalarType::UInt),
            x if x == ScalarType::UInt8 as i32 => Ok(ScalarType::UInt8),
            x if x == ScalarType::UInt16 as i32 => Ok(ScalarType::UInt16),
            x if x == ScalarType::UInt32 as i32 => Ok(ScalarType::UInt32),
            x if x == ScalarType::Int as i32 => Ok(ScalarType::Int),
            x if x == ScalarType::Int8 as i32 => Ok(ScalarType::Int8),
            x if x == ScalarType::Int16 as i32 => Ok(ScalarType::Int16),
            x if x == ScalarType::Int32 as i32 => Ok(ScalarType::Int32),
            x if x == ScalarType::String as i32 => Ok(ScalarType::String),
            x if x == ScalarType::Boolean as i32 => Ok(ScalarType::Boolean),
            x if x == ScalarType::Bytes as i32 => Ok(ScalarType::Bytes),
            x if x == ScalarType::BigInt as i32 => Ok(ScalarType::BigInt),
            x if x == ScalarType::BigNumber as i32 => Ok(ScalarType::BigNumber),
            x if x == ScalarType::JSON as i32 => Ok(ScalarType::JSON),
            x if x == ScalarType::_MAX_ as i32 => Ok(ScalarType::_MAX_),
            _ => Err(EnumTypeError::ParseEnumError(format!("Invalid value for enum 'ScalarType': {}", (v  as i32).to_string()))),
        }
    }
}
