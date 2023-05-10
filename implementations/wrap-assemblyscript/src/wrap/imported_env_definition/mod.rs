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
    deserialize_imported_env_definition,
    read_imported_env_definition,
    serialize_imported_env_definition,
    write_imported_env_definition
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImportedEnvDefinition {
}

impl ImportedEnvDefinition {
    pub fn new() -> ImportedEnvDefinition {
        ImportedEnvDefinition {
        }
    }

    pub fn to_buffer(args: &ImportedEnvDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_imported_env_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ImportedEnvDefinition, DecodeError> {
        deserialize_imported_env_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ImportedEnvDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_imported_env_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ImportedEnvDefinition, DecodeError> {
        read_imported_env_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
