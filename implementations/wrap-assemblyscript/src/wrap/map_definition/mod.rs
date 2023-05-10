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
    deserialize_map_definition,
    read_map_definition,
    serialize_map_definition,
    write_map_definition
};

use crate::MapKeyDefinition;
use crate::GenericDefinition;
use crate::ArrayDefinition;
use crate::ScalarDefinition;
use crate::ObjectRef;
use crate::EnumRef;
use crate::UnresolvedObjectOrEnumRef;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapDefinition {
    pub key: Option<Box<MapKeyDefinition>>,
    pub value: Option<GenericDefinition>,
    pub array: Option<ArrayDefinition>,
    pub scalar: Option<ScalarDefinition>,
    pub map: Option<Box<MapDefinition>>,
    pub object: Option<ObjectRef>,
    #[serde(rename = "enum")]
    pub _enum: Option<EnumRef>,
    pub unresolved_object_or_enum: Option<UnresolvedObjectOrEnumRef>,
    pub comment: Option<String>,
}

impl MapDefinition {
    pub fn new() -> MapDefinition {
        MapDefinition {
            key: None,
            value: None,
            array: None,
            scalar: None,
            map: None,
            object: None,
            _enum: None,
            unresolved_object_or_enum: None,
            comment: None,
        }
    }

    pub fn to_buffer(args: &MapDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_map_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<MapDefinition, DecodeError> {
        deserialize_map_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &MapDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_map_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<MapDefinition, DecodeError> {
        read_map_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
