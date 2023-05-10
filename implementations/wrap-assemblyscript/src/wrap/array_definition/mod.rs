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
    deserialize_array_definition,
    read_array_definition,
    serialize_array_definition,
    write_array_definition
};

use crate::GenericDefinition;
use crate::ScalarDefinition;
use crate::MapDefinition;
use crate::ObjectRef;
use crate::EnumRef;
use crate::UnresolvedObjectOrEnumRef;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArrayDefinition {
    pub item: Option<GenericDefinition>,
    pub array: Option<Box<ArrayDefinition>>,
    pub scalar: Option<ScalarDefinition>,
    pub map: Option<Box<MapDefinition>>,
    pub object: Option<ObjectRef>,
    #[serde(rename = "enum")]
    pub _enum: Option<EnumRef>,
    pub unresolved_object_or_enum: Option<UnresolvedObjectOrEnumRef>,
}

impl ArrayDefinition {
    pub fn new() -> ArrayDefinition {
        ArrayDefinition {
            item: None,
            array: None,
            scalar: None,
            map: None,
            object: None,
            _enum: None,
            unresolved_object_or_enum: None,
        }
    }

    pub fn to_buffer(args: &ArrayDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_array_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ArrayDefinition, DecodeError> {
        deserialize_array_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ArrayDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_array_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ArrayDefinition, DecodeError> {
        read_array_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
