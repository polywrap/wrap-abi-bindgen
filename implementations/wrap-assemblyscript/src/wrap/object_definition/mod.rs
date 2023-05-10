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
    deserialize_object_definition,
    read_object_definition,
    serialize_object_definition,
    write_object_definition
};

use crate::PropertyDefinition;
use crate::InterfaceImplementedDefinition;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectDefinition {
    pub properties: Option<Vec<PropertyDefinition>>,
    pub interfaces: Option<Vec<InterfaceImplementedDefinition>>,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: Option<String>,
    pub required: Option<bool>,
    pub comment: Option<String>,
}

impl ObjectDefinition {
    pub fn new() -> ObjectDefinition {
        ObjectDefinition {
            properties: None,
            interfaces: None,
            _type: String::new(),
            name: None,
            required: None,
            comment: None,
        }
    }

    pub fn to_buffer(args: &ObjectDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_object_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ObjectDefinition, DecodeError> {
        deserialize_object_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ObjectDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_object_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ObjectDefinition, DecodeError> {
        read_object_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
