use serde::{Serialize, Deserialize};
pub mod serialization;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    DecodeError,
    EncodeError,
    Read,
    Write,
    JSON,
};
pub use serialization::{
    deserialize_method_definition,
    read_method_definition,
    serialize_method_definition,
    write_method_definition
};

use crate::PropertyDefinition;
use crate::EnvRequired;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MethodDefinition {
    pub arguments: Option<Vec<PropertyDefinition>>,
    pub env: Option<EnvRequired>,
    #[serde(rename = "return")]
    pub _return: Option<PropertyDefinition>,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: Option<String>,
    pub required: Option<bool>,
    pub comment: Option<String>,
}

impl MethodDefinition {
    pub fn new() -> MethodDefinition {
        MethodDefinition {
            arguments: None,
            env: None,
            _return: None,
            _type: String::new(),
            name: None,
            required: None,
            comment: None,
        }
    }

    pub fn to_buffer(args: &MethodDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_method_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<MethodDefinition, DecodeError> {
        deserialize_method_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &MethodDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_method_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<MethodDefinition, DecodeError> {
        read_method_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
