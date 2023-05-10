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
    deserialize_property_definition,
    read_property_definition,
    serialize_property_definition,
    write_property_definition
};

use crate::ArrayDefinition;
use crate::ScalarDefinition;
use crate::MapDefinition;
use crate::ObjectRef;
use crate::EnumRef;
use crate::UnresolvedObjectOrEnumRef;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PropertyDefinition {
    pub comment: Option<String>,
    pub array: Option<ArrayDefinition>,
    pub scalar: Option<ScalarDefinition>,
    pub map: Option<MapDefinition>,
    pub object: Option<ObjectRef>,
    #[serde(rename = "enum")]
    pub _enum: Option<EnumRef>,
    pub unresolved_object_or_enum: Option<UnresolvedObjectOrEnumRef>,
}

impl PropertyDefinition {
    pub fn new() -> PropertyDefinition {
        PropertyDefinition {
            comment: None,
            array: None,
            scalar: None,
            map: None,
            object: None,
            _enum: None,
            unresolved_object_or_enum: None,
        }
    }

    pub fn to_buffer(args: &PropertyDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_property_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<PropertyDefinition, DecodeError> {
        deserialize_property_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &PropertyDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_property_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<PropertyDefinition, DecodeError> {
        read_property_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
