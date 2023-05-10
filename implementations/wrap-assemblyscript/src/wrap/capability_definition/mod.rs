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
    deserialize_capability_definition,
    read_capability_definition,
    serialize_capability_definition,
    write_capability_definition
};

use crate::CapabilityEnabled;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CapabilityDefinition {
    pub get_implementations: Option<CapabilityEnabled>,
}

impl CapabilityDefinition {
    pub fn new() -> CapabilityDefinition {
        CapabilityDefinition {
            get_implementations: None,
        }
    }

    pub fn to_buffer(args: &CapabilityDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_capability_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<CapabilityDefinition, DecodeError> {
        deserialize_capability_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &CapabilityDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_capability_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<CapabilityDefinition, DecodeError> {
        read_capability_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
