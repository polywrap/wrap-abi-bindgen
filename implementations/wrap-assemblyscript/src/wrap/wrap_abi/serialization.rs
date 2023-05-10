use std::convert::TryFrom;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    Context,
    DecodeError,
    EncodeError,
    Read,
    ReadDecoder,
    Write,
    WriteEncoder,
    JSON,
};
use crate::WrapAbi;

use crate::ObjectDefinition;
use crate::ModuleDefinition;
use crate::EnumDefinition;
use crate::InterfaceDefinition;
use crate::ImportedObjectDefinition;
use crate::ImportedModuleDefinition;
use crate::ImportedEnumDefinition;
use crate::ImportedEnvDefinition;
use crate::EnvDefinition;

pub fn serialize_wrap_abi(args: &WrapAbi) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: WrapAbi".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_wrap_abi(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_wrap_abi<W: Write>(args: &WrapAbi, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&10)?;
    writer.context().push("version", "Option<String>", "writing property");
    writer.write_string("version")?;
    writer.write_optional_string(&args.version)?;
    writer.context().pop();
    writer.context().push("objectTypes", "Option<Vec<ObjectDefinition>>", "writing property");
    writer.write_string("objectTypes")?;
    writer.write_optional_array(&args.object_types, |writer, item| {
        ObjectDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("moduleType", "Option<ModuleDefinition>", "writing property");
    writer.write_string("moduleType")?;
    if args.module_type.is_some() {
        ModuleDefinition::write(args.module_type.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("enumTypes", "Option<Vec<EnumDefinition>>", "writing property");
    writer.write_string("enumTypes")?;
    writer.write_optional_array(&args.enum_types, |writer, item| {
        EnumDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("interfaceTypes", "Option<Vec<InterfaceDefinition>>", "writing property");
    writer.write_string("interfaceTypes")?;
    writer.write_optional_array(&args.interface_types, |writer, item| {
        InterfaceDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("importedObjectTypes", "Option<Vec<ImportedObjectDefinition>>", "writing property");
    writer.write_string("importedObjectTypes")?;
    writer.write_optional_array(&args.imported_object_types, |writer, item| {
        ImportedObjectDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("importedModuleTypes", "Option<Vec<ImportedModuleDefinition>>", "writing property");
    writer.write_string("importedModuleTypes")?;
    writer.write_optional_array(&args.imported_module_types, |writer, item| {
        ImportedModuleDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("importedEnumTypes", "Option<Vec<ImportedEnumDefinition>>", "writing property");
    writer.write_string("importedEnumTypes")?;
    writer.write_optional_array(&args.imported_enum_types, |writer, item| {
        ImportedEnumDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("importedEnvTypes", "Option<Vec<ImportedEnvDefinition>>", "writing property");
    writer.write_string("importedEnvTypes")?;
    writer.write_optional_array(&args.imported_env_types, |writer, item| {
        ImportedEnvDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("envType", "Option<EnvDefinition>", "writing property");
    writer.write_string("envType")?;
    if args.env_type.is_some() {
        EnvDefinition::write(args.env_type.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn deserialize_wrap_abi(args: &[u8]) -> Result<WrapAbi, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: WrapAbi".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_wrap_abi(&mut reader)
}

pub fn read_wrap_abi<R: Read>(reader: &mut R) -> Result<WrapAbi, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _version: Option<String> = None;
    let mut _object_types: Option<Vec<ObjectDefinition>> = None;
    let mut _module_type: Option<ModuleDefinition> = None;
    let mut _enum_types: Option<Vec<EnumDefinition>> = None;
    let mut _interface_types: Option<Vec<InterfaceDefinition>> = None;
    let mut _imported_object_types: Option<Vec<ImportedObjectDefinition>> = None;
    let mut _imported_module_types: Option<Vec<ImportedModuleDefinition>> = None;
    let mut _imported_enum_types: Option<Vec<ImportedEnumDefinition>> = None;
    let mut _imported_env_types: Option<Vec<ImportedEnvDefinition>> = None;
    let mut _env_type: Option<EnvDefinition> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "version" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _version = reader.read_optional_string()?;
                reader.context().pop();
            }
            "objectTypes" => {
                reader.context().push(&field, "Option<Vec<ObjectDefinition>>", "type found, reading property");
                _object_types = reader.read_optional_array(|reader| {
                    let object = ObjectDefinition::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            "moduleType" => {
                reader.context().push(&field, "Option<ModuleDefinition>", "type found, reading property");
                let mut object: Option<ModuleDefinition> = None;
                if !reader.is_next_nil()? {
                    object = Some(ModuleDefinition::read(reader)?);
                } else {
                    object = None;
                }
                _module_type = object;
                reader.context().pop();
            }
            "enumTypes" => {
                reader.context().push(&field, "Option<Vec<EnumDefinition>>", "type found, reading property");
                _enum_types = reader.read_optional_array(|reader| {
                    let object = EnumDefinition::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            "interfaceTypes" => {
                reader.context().push(&field, "Option<Vec<InterfaceDefinition>>", "type found, reading property");
                _interface_types = reader.read_optional_array(|reader| {
                    let object = InterfaceDefinition::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            "importedObjectTypes" => {
                reader.context().push(&field, "Option<Vec<ImportedObjectDefinition>>", "type found, reading property");
                _imported_object_types = reader.read_optional_array(|reader| {
                    let object = ImportedObjectDefinition::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            "importedModuleTypes" => {
                reader.context().push(&field, "Option<Vec<ImportedModuleDefinition>>", "type found, reading property");
                _imported_module_types = reader.read_optional_array(|reader| {
                    let object = ImportedModuleDefinition::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            "importedEnumTypes" => {
                reader.context().push(&field, "Option<Vec<ImportedEnumDefinition>>", "type found, reading property");
                _imported_enum_types = reader.read_optional_array(|reader| {
                    let object = ImportedEnumDefinition::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            "importedEnvTypes" => {
                reader.context().push(&field, "Option<Vec<ImportedEnvDefinition>>", "type found, reading property");
                _imported_env_types = reader.read_optional_array(|reader| {
                    let object = ImportedEnvDefinition::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            "envType" => {
                reader.context().push(&field, "Option<EnvDefinition>", "type found, reading property");
                let mut object: Option<EnvDefinition> = None;
                if !reader.is_next_nil()? {
                    object = Some(EnvDefinition::read(reader)?);
                } else {
                    object = None;
                }
                _env_type = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(WrapAbi {
        version: _version,
        object_types: _object_types,
        module_type: _module_type,
        enum_types: _enum_types,
        interface_types: _interface_types,
        imported_object_types: _imported_object_types,
        imported_module_types: _imported_module_types,
        imported_enum_types: _imported_enum_types,
        imported_env_types: _imported_env_types,
        env_type: _env_type,
    })
}
