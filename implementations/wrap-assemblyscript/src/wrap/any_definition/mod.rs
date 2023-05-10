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
    deserialize_any_definition,
    read_any_definition,
    serialize_any_definition,
    write_any_definition
};

use crate::ArrayDefinition;
use crate::ScalarDefinition;
use crate::MapDefinition;
use crate::ObjectRef;
use crate::EnumRef;
use crate::UnresolvedObjectOrEnumRef;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnyDefinition {
    pub array: Option<ArrayDefinition>,
    pub scalar: Option<ScalarDefinition>,
    pub map: Option<MapDefinition>,
    pub object: Option<ObjectRef>,
    #[serde(rename = "enum")]
    pub _enum: Option<EnumRef>,
    pub unresolved_object_or_enum: Option<UnresolvedObjectOrEnumRef>,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: Option<String>,
    pub required: Option<bool>,
}

impl AnyDefinition {
    pub fn new() -> AnyDefinition {
        AnyDefinition {
            array: None,
            scalar: None,
            map: None,
            object: None,
            _enum: None,
            unresolved_object_or_enum: None,
            _type: String::new(),
            name: None,
            required: None,
        }
    }

    pub fn to_buffer(args: &AnyDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_any_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<AnyDefinition, DecodeError> {
        deserialize_any_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &AnyDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_any_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<AnyDefinition, DecodeError> {
        read_any_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
