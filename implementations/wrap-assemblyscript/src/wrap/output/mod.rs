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
    deserialize_output,
    read_output,
    serialize_output,
    write_output
};

use crate::File;
use crate::Directory;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Output {
    pub files: Vec<File>,
    pub dirs: Vec<Directory>,
}

impl Output {
    pub fn new() -> Output {
        Output {
            files: vec![],
            dirs: vec![],
        }
    }

    pub fn to_buffer(args: &Output) -> Result<Vec<u8>, EncodeError> {
        serialize_output(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<Output, DecodeError> {
        deserialize_output(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &Output, writer: &mut W) -> Result<(), EncodeError> {
        write_output(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Output, DecodeError> {
        read_output(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
