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
    deserialize_with_comment,
    read_with_comment,
    serialize_with_comment,
    write_with_comment
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WithComment {
    pub comment: Option<String>,
}

impl WithComment {
    pub fn new() -> WithComment {
        WithComment {
            comment: None,
        }
    }

    pub fn to_buffer(args: &WithComment) -> Result<Vec<u8>, EncodeError> {
        serialize_with_comment(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<WithComment, DecodeError> {
        deserialize_with_comment(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &WithComment, writer: &mut W) -> Result<(), EncodeError> {
        write_with_comment(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<WithComment, DecodeError> {
        read_with_comment(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
