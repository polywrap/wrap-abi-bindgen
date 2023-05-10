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
    deserialize_scalar_definition,
    read_scalar_definition,
    serialize_scalar_definition,
    write_scalar_definition
};

use crate::ScalarType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalarDefinition {
    pub kind: i32,
    #[serde(rename = "type")]
    pub _type: ScalarType,
    pub name: Option<String>,
    pub required: Option<bool>,
}

impl ScalarDefinition {
    pub fn new() -> ScalarDefinition {
        ScalarDefinition {
            kind: 0,
            _type: ScalarType::_MAX_,
            name: None,
            required: None,
        }
    }

    pub fn to_buffer(args: &ScalarDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_scalar_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ScalarDefinition, DecodeError> {
        deserialize_scalar_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ScalarDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_scalar_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ScalarDefinition, DecodeError> {
        read_scalar_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
