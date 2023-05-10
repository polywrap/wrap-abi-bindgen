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
use crate::ModuleDefinition;

use crate::MethodDefinition;
use crate::ImportedModuleRef;
use crate::InterfaceImplementedDefinition;

pub fn serialize_module_definition(args: &ModuleDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: ModuleDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_module_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_module_definition<W: Write>(args: &ModuleDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&7)?;
    writer.context().push("methods", "Option<Vec<MethodDefinition>>", "writing property");
    writer.write_string("methods")?;
    writer.write_optional_array(&args.methods, |writer, item| {
        MethodDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("imports", "Option<Vec<ImportedModuleRef>>", "writing property");
    writer.write_string("imports")?;
    writer.write_optional_array(&args.imports, |writer, item| {
        ImportedModuleRef::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("interfaces", "Option<Vec<InterfaceImplementedDefinition>>", "writing property");
    writer.write_string("interfaces")?;
    writer.write_optional_array(&args.interfaces, |writer, item| {
        InterfaceImplementedDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("type", "String", "writing property");
    writer.write_string("type")?;
    writer.write_string(&args._type)?;
    writer.context().pop();
    writer.context().push("name", "Option<String>", "writing property");
    writer.write_string("name")?;
    writer.write_optional_string(&args.name)?;
    writer.context().pop();
    writer.context().push("required", "Option<bool>", "writing property");
    writer.write_string("required")?;
    writer.write_optional_bool(&args.required)?;
    writer.context().pop();
    writer.context().push("comment", "Option<String>", "writing property");
    writer.write_string("comment")?;
    writer.write_optional_string(&args.comment)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_module_definition(args: &[u8]) -> Result<ModuleDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: ModuleDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_module_definition(&mut reader)
}

pub fn read_module_definition<R: Read>(reader: &mut R) -> Result<ModuleDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _methods: Option<Vec<MethodDefinition>> = None;
    let mut _imports: Option<Vec<ImportedModuleRef>> = None;
    let mut _interfaces: Option<Vec<InterfaceImplementedDefinition>> = None;
    let mut _type: String = String::new();
    let mut _type_set = false;
    let mut _name: Option<String> = None;
    let mut _required: Option<bool> = None;
    let mut _comment: Option<String> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "methods" => {
                reader.context().push(&field, "Option<Vec<MethodDefinition>>", "type found, reading property");
                _methods = reader.read_optional_array(|reader| {
                    let object = MethodDefinition::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            "imports" => {
                reader.context().push(&field, "Option<Vec<ImportedModuleRef>>", "type found, reading property");
                _imports = reader.read_optional_array(|reader| {
                    let object = ImportedModuleRef::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            "interfaces" => {
                reader.context().push(&field, "Option<Vec<InterfaceImplementedDefinition>>", "type found, reading property");
                _interfaces = reader.read_optional_array(|reader| {
                    let object = InterfaceImplementedDefinition::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            "type" => {
                reader.context().push(&field, "String", "type found, reading property");
                _type = reader.read_string()?;
                _type_set = true;
                reader.context().pop();
            }
            "name" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _name = reader.read_optional_string()?;
                reader.context().pop();
            }
            "required" => {
                reader.context().push(&field, "Option<bool>", "type found, reading property");
                _required = reader.read_optional_bool()?;
                reader.context().pop();
            }
            "comment" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _comment = reader.read_optional_string()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_type_set {
        return Err(DecodeError::MissingField("type: String.".to_string()));
    }

    Ok(ModuleDefinition {
        methods: _methods,
        imports: _imports,
        interfaces: _interfaces,
        _type: _type,
        name: _name,
        required: _required,
        comment: _comment,
    })
}
