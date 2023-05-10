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
    deserialize_generic_definition,
    read_generic_definition,
    serialize_generic_definition,
    write_generic_definition
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GenericDefinition {
    #[serde(rename = "type")]
    pub _type: String,
    pub name: Option<String>,
    pub required: Option<bool>,
    pub kind: i32,
}

impl GenericDefinition {
    pub fn new() -> GenericDefinition {
        GenericDefinition {
            _type: String::new(),
            name: None,
            required: None,
            kind: 0,
        }
    }

    pub fn to_buffer(args: &GenericDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_generic_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<GenericDefinition, DecodeError> {
        deserialize_generic_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &GenericDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_generic_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<GenericDefinition, DecodeError> {
        read_generic_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
