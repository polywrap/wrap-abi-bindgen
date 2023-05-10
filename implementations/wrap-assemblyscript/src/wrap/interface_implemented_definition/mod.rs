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
    deserialize_interface_implemented_definition,
    read_interface_implemented_definition,
    serialize_interface_implemented_definition,
    write_interface_implemented_definition
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InterfaceImplementedDefinition {
    #[serde(rename = "type")]
    pub _type: String,
    pub name: Option<String>,
    pub required: Option<bool>,
}

impl InterfaceImplementedDefinition {
    pub fn new() -> InterfaceImplementedDefinition {
        InterfaceImplementedDefinition {
            _type: String::new(),
            name: None,
            required: None,
        }
    }

    pub fn to_buffer(args: &InterfaceImplementedDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_interface_implemented_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<InterfaceImplementedDefinition, DecodeError> {
        deserialize_interface_implemented_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &InterfaceImplementedDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_interface_implemented_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<InterfaceImplementedDefinition, DecodeError> {
        read_interface_implemented_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
