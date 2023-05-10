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
    deserialize_imported_enum_definition,
    read_imported_enum_definition,
    serialize_imported_enum_definition,
    write_imported_enum_definition
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImportedEnumDefinition {
    pub constants: Option<Vec<String>>,
}

impl ImportedEnumDefinition {
    pub fn new() -> ImportedEnumDefinition {
        ImportedEnumDefinition {
            constants: None,
        }
    }

    pub fn to_buffer(args: &ImportedEnumDefinition) -> Result<Vec<u8>, EncodeError> {
        serialize_imported_enum_definition(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<ImportedEnumDefinition, DecodeError> {
        deserialize_imported_enum_definition(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &ImportedEnumDefinition, writer: &mut W) -> Result<(), EncodeError> {
        write_imported_enum_definition(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ImportedEnumDefinition, DecodeError> {
        read_imported_enum_definition(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
