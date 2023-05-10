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
use crate::ImportedModuleDefinition;

use crate::MethodDefinition;

pub fn serialize_imported_module_definition(args: &ImportedModuleDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: ImportedModuleDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_imported_module_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_imported_module_definition<W: Write>(args: &ImportedModuleDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&6)?;
    writer.context().push("methods", "Option<Vec<MethodDefinition>>", "writing property");
    writer.write_string("methods")?;
    writer.write_optional_array(&args.methods, |writer, item| {
        MethodDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("isInterface", "Option<bool>", "writing property");
    writer.write_string("isInterface")?;
    writer.write_optional_bool(&args.is_interface)?;
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

pub fn deserialize_imported_module_definition(args: &[u8]) -> Result<ImportedModuleDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: ImportedModuleDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_imported_module_definition(&mut reader)
}

pub fn read_imported_module_definition<R: Read>(reader: &mut R) -> Result<ImportedModuleDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _methods: Option<Vec<MethodDefinition>> = None;
    let mut _is_interface: Option<bool> = None;
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
            "isInterface" => {
                reader.context().push(&field, "Option<bool>", "type found, reading property");
                _is_interface = reader.read_optional_bool()?;
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

    Ok(ImportedModuleDefinition {
        methods: _methods,
        is_interface: _is_interface,
        _type: _type,
        name: _name,
        required: _required,
        comment: _comment,
    })
}
