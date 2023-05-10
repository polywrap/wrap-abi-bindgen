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
    deserialize_object_ref,
    read_object_ref,
    serialize_object_ref,
    write_object_ref
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectRef {
    #[serde(rename = "type")]
    pub _type: String,
    pub name: Option<String>,
    pub required: Option<bool>,
}

impl ObjectRef {
    pub fn new() -> ObjectRef {
        ObjectRef {
            _type: String::new(),
            name: None,
            required: None,
        }
    }

    pub fn to_buffer(args: &ObjectRef) -> Result<Vec<u8>, EncodeError> {
        serialize_object_ref(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ObjectRef, DecodeError> {
        deserialize_object_ref(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ObjectRef, writer: &mut W) -> Result<(), EncodeError> {
        write_object_ref(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ObjectRef, DecodeError> {
        read_object_ref(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
