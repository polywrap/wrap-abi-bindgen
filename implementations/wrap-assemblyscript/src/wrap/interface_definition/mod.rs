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
    deserialize_interface_definition,
    read_interface_definition,
    serialize_interface_definition,
    write_interface_definition
};

use crate::CapabilityDefinition;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InterfaceDefinition {
    pub capabilities: Option<CapabilityDefinition>,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: Option<String>,
    pub required: Option<bool>,
}

impl InterfaceDefinition {
    pub fn new() -> InterfaceDefinition {
        InterfaceDefinition {
            capabilities: None,
            _type: String::new(),
            name: None,
            required: None,
        }
    }

    pub fn to_buffer(args: &InterfaceDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_interface_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<InterfaceDefinition, DecodeError> {
        deserialize_interface_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &InterfaceDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_interface_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<InterfaceDefinition, DecodeError> {
        read_interface_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
