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
    deserialize_enum_ref,
    read_enum_ref,
    serialize_enum_ref,
    write_enum_ref
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnumRef {
    #[serde(rename = "type")]
    pub _type: String,
    pub name: Option<String>,
    pub required: Option<bool>,
}

impl EnumRef {
    pub fn new() -> EnumRef {
        EnumRef {
            _type: String::new(),
            name: None,
            required: None,
        }
    }

    pub fn to_buffer(args: &EnumRef) -> Result<Vec<u8>, EncodeError> {
        serialize_enum_ref(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<EnumRef, DecodeError> {
        deserialize_enum_ref(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &EnumRef, writer: &mut W) -> Result<(), EncodeError> {
        write_enum_ref(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<EnumRef, DecodeError> {
        read_enum_ref(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
