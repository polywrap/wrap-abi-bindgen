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
    deserialize_env_definition,
    read_env_definition,
    serialize_env_definition,
    write_env_definition
};

use crate::PropertyDefinition;
use crate::InterfaceImplementedDefinition;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnvDefinition {
    pub properties: Option<Vec<PropertyDefinition>>,
    pub interfaces: Option<Vec<InterfaceImplementedDefinition>>,
}

impl EnvDefinition {
    pub fn new() -> EnvDefinition {
        EnvDefinition {
            properties: None,
            interfaces: None,
        }
    }

    pub fn to_buffer(args: &EnvDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_env_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<EnvDefinition, DecodeError> {
        deserialize_env_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &EnvDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_env_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<EnvDefinition, DecodeError> {
        read_env_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
