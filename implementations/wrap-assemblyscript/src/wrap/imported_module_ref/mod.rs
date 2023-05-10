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
    deserialize_imported_module_ref,
    read_imported_module_ref,
    serialize_imported_module_ref,
    write_imported_module_ref
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImportedModuleRef {
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

impl ImportedModuleRef {
    pub fn new() -> ImportedModuleRef {
        ImportedModuleRef {
            _type: None,
        }
    }

    pub fn to_buffer(args: &ImportedModuleRef) -> Result<Vec<u8>, EncodeError> {
        serialize_imported_module_ref(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ImportedModuleRef, DecodeError> {
        deserialize_imported_module_ref(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ImportedModuleRef, writer: &mut W) -> Result<(), EncodeError> {
        write_imported_module_ref(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ImportedModuleRef, DecodeError> {
        read_imported_module_ref(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
