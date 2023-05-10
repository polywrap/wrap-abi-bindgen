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
    deserialize_file,
    read_file,
    serialize_file,
    write_file
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub name: String,
    pub data: String,
}

impl File {
    pub fn new() -> File {
        File {
            name: String::new(),
            data: String::new(),
        }
    }

    pub fn to_buffer(args: &File) -> Result<Vec<u8>, EncodeError> {
        serialize_file(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<File, DecodeError> {
        deserialize_file(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &File, writer: &mut W) -> Result<(), EncodeError> {
        write_file(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<File, DecodeError> {
        read_file(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
