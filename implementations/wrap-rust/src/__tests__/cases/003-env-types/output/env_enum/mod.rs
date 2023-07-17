use polywrap_wasm_rs::{EnumTypeError};
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum EnvEnum {
    #[serde(rename = "FIRST")]
    FIRST,
    #[serde(rename = "SECOND")]
    SECOND,
    _MAX_
}

pub fn sanitize_env_enum_value(value: i32) -> Result<(), EnumTypeError> {
    if value < 0 && value >= EnvEnum::_MAX_ as i32 {
        return Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'EnvEnum': {}", value.to_string())));
    }
    Ok(())
}

pub fn get_env_enum_value(key: &str) -> Result<EnvEnum, EnumTypeError> {
    match key {
        "FIRST" => Ok(EnvEnum::FIRST),
        "SECOND" => Ok(EnvEnum::SECOND),
        "_MAX_" => Ok(EnvEnum::_MAX_),
        err => Err(EnumTypeError::EnumProcessingError(format!("Invalid key for enum 'EnvEnum': {}", err)))
    }
}

pub fn get_env_enum_key(value: EnvEnum) -> Result<String, EnumTypeError> {
    if sanitize_env_enum_value(value as i32).is_ok() {
        match value {
            EnvEnum::FIRST => Ok("FIRST".to_string()),
            EnvEnum::SECOND => Ok("SECOND".to_string()),
            EnvEnum::_MAX_ => Ok("_MAX_".to_string()),
        }
    } else {
        Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'EnvEnum': {}", (value  as i32).to_string())))
    }
}

impl TryFrom<i32> for EnvEnum {
    type Error = EnumTypeError;

    fn try_from(v: i32) -> Result<EnvEnum, Self::Error> {
        match v {
            x if x == EnvEnum::FIRST as i32 => Ok(EnvEnum::FIRST),
            x if x == EnvEnum::SECOND as i32 => Ok(EnvEnum::SECOND),
            x if x == EnvEnum::_MAX_ as i32 => Ok(EnvEnum::_MAX_),
            _ => Err(EnumTypeError::ParseEnumError(format!("Invalid value for enum 'EnvEnum': {}", (v  as i32).to_string()))),
        }
    }
}
