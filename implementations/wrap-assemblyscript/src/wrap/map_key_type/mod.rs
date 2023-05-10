use polywrap_wasm_rs::{EnumTypeError};
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum MapKeyType {
    UInt,
    UInt8,
    UInt16,
    UInt32,
    Int,
    Int8,
    Int16,
    Int32,
    String,
    _MAX_
}

pub fn sanitize_map_key_type_value(value: i32) -> Result<(), EnumTypeError> {
    if value < 0 && value >= MapKeyType::_MAX_ as i32 {
        return Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'MapKeyType': {}", value.to_string())));
    }
    Ok(())
}

pub fn get_map_key_type_value(key: &str) -> Result<MapKeyType, EnumTypeError> {
    match key {
        "UInt" => Ok(MapKeyType::UInt),
        "UInt8" => Ok(MapKeyType::UInt8),
        "UInt16" => Ok(MapKeyType::UInt16),
        "UInt32" => Ok(MapKeyType::UInt32),
        "Int" => Ok(MapKeyType::Int),
        "Int8" => Ok(MapKeyType::Int8),
        "Int16" => Ok(MapKeyType::Int16),
        "Int32" => Ok(MapKeyType::Int32),
        "String" => Ok(MapKeyType::String),
        "_MAX_" => Ok(MapKeyType::_MAX_),
        err => Err(EnumTypeError::EnumProcessingError(format!("Invalid key for enum 'MapKeyType': {}", err)))
    }
}

pub fn get_map_key_type_key(value: MapKeyType) -> Result<String, EnumTypeError> {
    if sanitize_map_key_type_value(value as i32).is_ok() {
        match value {
            MapKeyType::UInt => Ok("UInt".to_string()),
            MapKeyType::UInt8 => Ok("UInt8".to_string()),
            MapKeyType::UInt16 => Ok("UInt16".to_string()),
            MapKeyType::UInt32 => Ok("UInt32".to_string()),
            MapKeyType::Int => Ok("Int".to_string()),
            MapKeyType::Int8 => Ok("Int8".to_string()),
            MapKeyType::Int16 => Ok("Int16".to_string()),
            MapKeyType::Int32 => Ok("Int32".to_string()),
            MapKeyType::String => Ok("String".to_string()),
            MapKeyType::_MAX_ => Ok("_MAX_".to_string()),
        }
    } else {
        Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'MapKeyType': {}", (value  as i32).to_string())))
    }
}

impl TryFrom<i32> for MapKeyType {
    type Error = EnumTypeError;

    fn try_from(v: i32) -> Result<MapKeyType, Self::Error> {
        match v {
            x if x == MapKeyType::UInt as i32 => Ok(MapKeyType::UInt),
            x if x == MapKeyType::UInt8 as i32 => Ok(MapKeyType::UInt8),
            x if x == MapKeyType::UInt16 as i32 => Ok(MapKeyType::UInt16),
            x if x == MapKeyType::UInt32 as i32 => Ok(MapKeyType::UInt32),
            x if x == MapKeyType::Int as i32 => Ok(MapKeyType::Int),
            x if x == MapKeyType::Int8 as i32 => Ok(MapKeyType::Int8),
            x if x == MapKeyType::Int16 as i32 => Ok(MapKeyType::Int16),
            x if x == MapKeyType::Int32 as i32 => Ok(MapKeyType::Int32),
            x if x == MapKeyType::String as i32 => Ok(MapKeyType::String),
            x if x == MapKeyType::_MAX_ as i32 => Ok(MapKeyType::_MAX_),
            _ => Err(EnumTypeError::ParseEnumError(format!("Invalid value for enum 'MapKeyType': {}", (v  as i32).to_string()))),
        }
    }
}
