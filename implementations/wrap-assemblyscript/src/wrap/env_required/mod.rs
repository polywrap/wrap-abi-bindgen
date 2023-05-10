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
    deserialize_env_required,
    read_env_required,
    serialize_env_required,
    write_env_required
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnvRequired {
    pub required: Option<bool>,
}

impl EnvRequired {
    pub fn new() -> EnvRequired {
        EnvRequired {
            required: None,
        }
    }

    pub fn to_buffer(args: &EnvRequired) -> Result<Vec<u8>, EncodeError> {
        serialize_env_required(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<EnvRequired, DecodeError> {
        deserialize_env_required(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &EnvRequired, writer: &mut W) -> Result<(), EncodeError> {
        write_env_required(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<EnvRequired, DecodeError> {
        read_env_required(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
