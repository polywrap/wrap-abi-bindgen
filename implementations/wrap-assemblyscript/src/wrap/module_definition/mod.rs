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
    deserialize_module_definition,
    read_module_definition,
    serialize_module_definition,
    write_module_definition
};

use crate::MethodDefinition;
use crate::ImportedModuleRef;
use crate::InterfaceImplementedDefinition;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModuleDefinition {
    pub methods: Option<Vec<MethodDefinition>>,
    pub imports: Option<Vec<ImportedModuleRef>>,
    pub interfaces: Option<Vec<InterfaceImplementedDefinition>>,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: Option<String>,
    pub required: Option<bool>,
    pub comment: Option<String>,
}

impl ModuleDefinition {
    pub fn new() -> ModuleDefinition {
        ModuleDefinition {
            methods: None,
            imports: None,
            interfaces: None,
            _type: String::new(),
            name: None,
            required: None,
            comment: None,
        }
    }

    pub fn to_buffer(args: &ModuleDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_module_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ModuleDefinition, DecodeError> {
        deserialize_module_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ModuleDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_module_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ModuleDefinition, DecodeError> {
        read_module_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
