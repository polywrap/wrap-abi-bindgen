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
    deserialize_directory,
    read_directory,
    serialize_directory,
    write_directory
};

use crate::File;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Directory {
    pub name: String,
    pub files: Vec<File>,
    pub dirs: Vec<Directory>,
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            name: String::new(),
            files: vec![],
            dirs: vec![],
        }
    }

    pub fn to_buffer(args: &Directory) -> Result<Vec<u8>, EncodeError> {
        serialize_directory(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<Directory, DecodeError> {
        deserialize_directory(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &Directory, writer: &mut W) -> Result<(), EncodeError> {
        write_directory(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Directory, DecodeError> {
        read_directory(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
