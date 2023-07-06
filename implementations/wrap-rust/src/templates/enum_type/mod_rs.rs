lazy_static! {
  static ref NAME: String = "enum_type/mod.rs".to_string();
  static ref SOURCE: String = r#"use polywrap_wasm_rs::{EnumTypeError};
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum {{detect_keyword (to_upper type)}} {
    {{#each constants}}
    {{serde_keyword (to_lower this)}}{{detect_keyword this}},
    {{/each}}
    _MAX_
}

pub fn sanitize_{{to_lower type}}_value(value: i32) -> Result<(), EnumTypeError> {
    if value < 0 && value >= {{detect_keyword (to_upper type)}}::_MAX_ as i32 {
        return Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum '{{detect_keyword (to_upper type)}}': {}", value.to_string())));
    }
    Ok(())
}

pub fn get_{{to_lower type}}_value(key: &str) -> Result<{{detect_keyword (to_upper type)}}, EnumTypeError> {
    match key {
        {{#each constants}}
        "{{detect_keyword this}}" => Ok({{detect_keyword (to_upper ../type)}}::{{detect_keyword this}}),
        {{/each}}
        "_MAX_" => Ok({{detect_keyword (to_upper type)}}::_MAX_),
        err => Err(EnumTypeError::EnumProcessingError(format!("Invalid key for enum '{{detect_keyword (to_upper type)}}': {}", err)))
    }
}

pub fn get_{{to_lower type}}_key(value: {{detect_keyword (to_upper type)}}) -> Result<String, EnumTypeError> {
    if sanitize_{{to_lower type}}_value(value as i32).is_ok() {
        match value {
            {{#each constants}}
            {{detect_keyword (to_upper ../type)}}::{{detect_keyword this}} => Ok("{{detect_keyword this}}".to_string()),
            {{/each}}
            {{detect_keyword (to_upper type)}}::_MAX_ => Ok("_MAX_".to_string()),
        }
    } else {
        Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum '{{detect_keyword (to_upper type)}}': {}", (value  as i32).to_string())))
    }
}

impl TryFrom<i32> for {{detect_keyword (to_upper type)}} {
    type Error = EnumTypeError;

    fn try_from(v: i32) -> Result<{{detect_keyword (to_upper type)}}, Self::Error> {
        match v {
            {{#each constants}}
            x if x == {{detect_keyword (to_upper ../type)}}::{{detect_keyword this}} as i32 => Ok({{detect_keyword (to_upper ../type)}}::{{detect_keyword this}}),
            {{/each}}
            x if x == {{detect_keyword (to_upper type)}}::_MAX_ as i32 => Ok({{detect_keyword (to_upper type)}}::_MAX_),
            _ => Err(EnumTypeError::ParseEnumError(format!("Invalid value for enum '{{detect_keyword (to_upper type)}}': {}", (v  as i32).to_string()))),
        }
    }
}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
