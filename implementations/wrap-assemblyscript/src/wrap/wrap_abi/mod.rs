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
    deserialize_wrap_abi,
    read_wrap_abi,
    serialize_wrap_abi,
    write_wrap_abi
};

use crate::ObjectDefinition;
use crate::ModuleDefinition;
use crate::EnumDefinition;
use crate::InterfaceDefinition;
use crate::ImportedObjectDefinition;
use crate::ImportedModuleDefinition;
use crate::ImportedEnumDefinition;
use crate::ImportedEnvDefinition;
use crate::EnvDefinition;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrapAbi {
    pub version: Option<String>,
    pub object_types: Option<Vec<ObjectDefinition>>,
    pub module_type: Option<ModuleDefinition>,
    pub enum_types: Option<Vec<EnumDefinition>>,
    pub interface_types: Option<Vec<InterfaceDefinition>>,
    pub imported_object_types: Option<Vec<ImportedObjectDefinition>>,
    pub imported_module_types: Option<Vec<ImportedModuleDefinition>>,
    pub imported_enum_types: Option<Vec<ImportedEnumDefinition>>,
    pub imported_env_types: Option<Vec<ImportedEnvDefinition>>,
    pub env_type: Option<EnvDefinition>,
}

impl WrapAbi {
    pub fn new() -> WrapAbi {
        WrapAbi {
            version: None,
            object_types: None,
            module_type: None,
            enum_types: None,
            interface_types: None,
            imported_object_types: None,
            imported_module_types: None,
            imported_enum_types: None,
            imported_env_types: None,
            env_type: None,
        }
    }

    pub fn to_buffer(args: &WrapAbi) -> Result<Vec<u8>, EncodeError> {
        serialize_wrap_abi(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<WrapAbi, DecodeError> {
        deserialize_wrap_abi(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &WrapAbi, writer: &mut W) -> Result<(), EncodeError> {
        write_wrap_abi(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<WrapAbi, DecodeError> {
        read_wrap_abi(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
