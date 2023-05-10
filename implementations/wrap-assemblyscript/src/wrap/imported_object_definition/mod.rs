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
    deserialize_imported_object_definition,
    read_imported_object_definition,
    serialize_imported_object_definition,
    write_imported_object_definition
};

use crate::PropertyDefinition;
use crate::InterfaceImplementedDefinition;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImportedObjectDefinition {
    pub properties: Option<Vec<PropertyDefinition>>,
    pub interfaces: Option<Vec<InterfaceImplementedDefinition>>,
    pub comment: Option<String>,
}

impl ImportedObjectDefinition {
    pub fn new() -> ImportedObjectDefinition {
        ImportedObjectDefinition {
            properties: None,
            interfaces: None,
            comment: None,
        }
    }

    pub fn to_buffer(args: &ImportedObjectDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_imported_object_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ImportedObjectDefinition, DecodeError> {
        deserialize_imported_object_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ImportedObjectDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_imported_object_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ImportedObjectDefinition, DecodeError> {
        read_imported_object_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
