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
    deserialize_unresolved_object_or_enum_ref,
    read_unresolved_object_or_enum_ref,
    serialize_unresolved_object_or_enum_ref,
    write_unresolved_object_or_enum_ref
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnresolvedObjectOrEnumRef {
    #[serde(rename = "type")]
    pub _type: String,
    pub name: Option<String>,
    pub required: Option<bool>,
}

impl UnresolvedObjectOrEnumRef {
    pub fn new() -> UnresolvedObjectOrEnumRef {
        UnresolvedObjectOrEnumRef {
            _type: String::new(),
            name: None,
            required: None,
        }
    }

    pub fn to_buffer(args: &UnresolvedObjectOrEnumRef) -> Result<Vec<u8>, EncodeError> {
        serialize_unresolved_object_or_enum_ref(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<UnresolvedObjectOrEnumRef, DecodeError> {
        deserialize_unresolved_object_or_enum_ref(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &UnresolvedObjectOrEnumRef, writer: &mut W) -> Result<(), EncodeError> {
        write_unresolved_object_or_enum_ref(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<UnresolvedObjectOrEnumRef, DecodeError> {
        read_unresolved_object_or_enum_ref(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
