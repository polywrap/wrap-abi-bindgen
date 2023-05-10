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
    deserialize_capability_enabled,
    read_capability_enabled,
    serialize_capability_enabled,
    write_capability_enabled
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CapabilityEnabled {
    pub enabled: bool,
}

impl CapabilityEnabled {
    pub fn new() -> CapabilityEnabled {
        CapabilityEnabled {
            enabled: false,
        }
    }

    pub fn to_buffer(args: &CapabilityEnabled) -> Result<Vec<u8>, EncodeError> {
        serialize_capability_enabled(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<CapabilityEnabled, DecodeError> {
        deserialize_capability_enabled(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &CapabilityEnabled, writer: &mut W) -> Result<(), EncodeError> {
        write_capability_enabled(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<CapabilityEnabled, DecodeError> {
        read_capability_enabled(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
