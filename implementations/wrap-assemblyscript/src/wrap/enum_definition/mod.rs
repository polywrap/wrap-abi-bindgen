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
    deserialize_enum_definition,
    read_enum_definition,
    serialize_enum_definition,
    write_enum_definition
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnumDefinition {
    pub constants: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: Option<String>,
    pub required: Option<bool>,
    pub comment: Option<String>,
}

impl EnumDefinition {
    pub fn new() -> EnumDefinition {
        EnumDefinition {
            constants: None,
            _type: String::new(),
            name: None,
            required: None,
            comment: None,
        }
    }

    pub fn to_buffer(args: &EnumDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_enum_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<EnumDefinition, DecodeError> {
        deserialize_enum_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &EnumDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_enum_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<EnumDefinition, DecodeError> {
        read_enum_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
