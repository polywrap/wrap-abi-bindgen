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
    deserialize_map_key_definition,
    read_map_key_definition,
    serialize_map_key_definition,
    write_map_key_definition
};

use crate::MapKeyType;
use crate::ArrayDefinition;
use crate::ScalarDefinition;
use crate::MapDefinition;
use crate::ObjectRef;
use crate::EnumRef;
use crate::UnresolvedObjectOrEnumRef;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapKeyDefinition {
    #[serde(rename = "type")]
    pub _type: Option<MapKeyType>,
    pub array: Option<ArrayDefinition>,
    pub scalar: Option<ScalarDefinition>,
    pub map: Option<MapDefinition>,
    pub object: Option<ObjectRef>,
    #[serde(rename = "enum")]
    pub _enum: Option<EnumRef>,
    pub unresolved_object_or_enum: Option<UnresolvedObjectOrEnumRef>,
}

impl MapKeyDefinition {
    pub fn new() -> MapKeyDefinition {
        MapKeyDefinition {
            _type: None,
            array: None,
            scalar: None,
            map: None,
            object: None,
            _enum: None,
            unresolved_object_or_enum: None,
        }
    }

    pub fn to_buffer(args: &MapKeyDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_map_key_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<MapKeyDefinition, DecodeError> {
        deserialize_map_key_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &MapKeyDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_map_key_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<MapKeyDefinition, DecodeError> {
        read_map_key_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
