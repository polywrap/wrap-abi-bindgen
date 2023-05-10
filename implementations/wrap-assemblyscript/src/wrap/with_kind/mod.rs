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
    deserialize_with_kind,
    read_with_kind,
    serialize_with_kind,
    write_with_kind
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WithKind {
    pub kind: i32,
}

impl WithKind {
    pub fn new() -> WithKind {
        WithKind {
            kind: 0,
        }
    }

    pub fn to_buffer(args: &WithKind) -> Result<Vec<u8>, EncodeError> {
        serialize_with_kind(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<WithKind, DecodeError> {
        deserialize_with_kind(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &WithKind, writer: &mut W) -> Result<(), EncodeError> {
        write_with_kind(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<WithKind, DecodeError> {
        read_with_kind(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
