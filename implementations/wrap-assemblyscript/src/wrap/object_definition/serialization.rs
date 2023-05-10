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
use crate::ObjectDefinition;

use crate::PropertyDefinition;
use crate::InterfaceImplementedDefinition;

pub fn serialize_object_definition(args: &ObjectDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: ObjectDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_object_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_object_definition<W: Write>(args: &ObjectDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&6)?;
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

pub fn deserialize_object_definition(args: &[u8]) -> Result<ObjectDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: ObjectDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_object_definition(&mut reader)
}

pub fn read_object_definition<R: Read>(reader: &mut R) -> Result<ObjectDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _properties: Option<Vec<PropertyDefinition>> = None;
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

    Ok(ObjectDefinition {
        properties: _properties,
        interfaces: _interfaces,
        _type: _type,
        name: _name,
        required: _required,
        comment: _comment,
    })
}
