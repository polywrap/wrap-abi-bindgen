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
use crate::ImportedObjectDefinition;

use crate::PropertyDefinition;
use crate::InterfaceImplementedDefinition;

pub fn serialize_imported_object_definition(args: &ImportedObjectDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: ImportedObjectDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_imported_object_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_imported_object_definition<W: Write>(args: &ImportedObjectDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("properties", "Option<Vec<PropertyDefinition>>", "writing property");
    writer.write_string("properties")?;
    writer.write_optional_array(&args.properties, |writer, item| {
        PropertyDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("interfaces", "Option<Vec<InterfaceImplementedDefinition>>", "writing property");
    writer.write_string("interfaces")?;
    writer.write_optional_array(&args.interfaces, |writer, item| {
        InterfaceImplementedDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("comment", "Option<String>", "writing property");
    writer.write_string("comment")?;
    writer.write_optional_string(&args.comment)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_imported_object_definition(args: &[u8]) -> Result<ImportedObjectDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: ImportedObjectDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_imported_object_definition(&mut reader)
}

pub fn read_imported_object_definition<R: Read>(reader: &mut R) -> Result<ImportedObjectDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _properties: Option<Vec<PropertyDefinition>> = None;
    let mut _interfaces: Option<Vec<InterfaceImplementedDefinition>> = None;
    let mut _comment: Option<String> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "properties" => {
                reader.context().push(&field, "Option<Vec<PropertyDefinition>>", "type found, reading property");
                _properties = reader.read_optional_array(|reader| {
                    let object = PropertyDefinition::read(reader)?;
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
            "comment" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _comment = reader.read_optional_string()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ImportedObjectDefinition {
        properties: _properties,
        interfaces: _interfaces,
        comment: _comment,
    })
}
