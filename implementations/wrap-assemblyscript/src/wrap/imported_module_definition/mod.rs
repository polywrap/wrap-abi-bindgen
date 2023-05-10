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
    deserialize_imported_module_definition,
    read_imported_module_definition,
    serialize_imported_module_definition,
    write_imported_module_definition
};

use crate::MethodDefinition;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImportedModuleDefinition {
    pub methods: Option<Vec<MethodDefinition>>,
    pub is_interface: Option<bool>,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: Option<String>,
    pub required: Option<bool>,
    pub comment: Option<String>,
}

impl ImportedModuleDefinition {
    pub fn new() -> ImportedModuleDefinition {
        ImportedModuleDefinition {
            methods: None,
            is_interface: None,
            _type: String::new(),
            name: None,
            required: None,
            comment: None,
        }
    }

    pub fn to_buffer(args: &ImportedModuleDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_imported_module_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ImportedModuleDefinition, DecodeError> {
        deserialize_imported_module_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ImportedModuleDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_imported_module_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ImportedModuleDefinition, DecodeError> {
        read_imported_module_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
