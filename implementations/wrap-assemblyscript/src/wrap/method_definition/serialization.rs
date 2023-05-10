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
use crate::MethodDefinition;

use crate::PropertyDefinition;
use crate::EnvRequired;

pub fn serialize_method_definition(args: &MethodDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: MethodDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_method_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_method_definition<W: Write>(args: &MethodDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&7)?;
    writer.context().push("arguments", "Option<Vec<PropertyDefinition>>", "writing property");
    writer.write_string("arguments")?;
    writer.write_optional_array(&args.arguments, |writer, item| {
        PropertyDefinition::write(item, writer)
    })?;
    writer.context().pop();
    writer.context().push("env", "Option<EnvRequired>", "writing property");
    writer.write_string("env")?;
    if args.env.is_some() {
        EnvRequired::write(args.env.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("return", "Option<PropertyDefinition>", "writing property");
    writer.write_string("return")?;
    if args._return.is_some() {
        PropertyDefinition::write(args._return.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
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

pub fn deserialize_method_definition(args: &[u8]) -> Result<MethodDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: MethodDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_method_definition(&mut reader)
}

pub fn read_method_definition<R: Read>(reader: &mut R) -> Result<MethodDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _arguments: Option<Vec<PropertyDefinition>> = None;
    let mut _env: Option<EnvRequired> = None;
    let mut _return: Option<PropertyDefinition> = None;
    let mut _type: String = String::new();
    let mut _type_set = false;
    let mut _name: Option<String> = None;
    let mut _required: Option<bool> = None;
    let mut _comment: Option<String> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "arguments" => {
                reader.context().push(&field, "Option<Vec<PropertyDefinition>>", "type found, reading property");
                _arguments = reader.read_optional_array(|reader| {
                    let object = PropertyDefinition::read(reader)?;
                    Ok(object)
                })?;
                reader.context().pop();
            }
            "env" => {
                reader.context().push(&field, "Option<EnvRequired>", "type found, reading property");
                let mut object: Option<EnvRequired> = None;
                if !reader.is_next_nil()? {
                    object = Some(EnvRequired::read(reader)?);
                } else {
                    object = None;
                }
                _env = object;
                reader.context().pop();
            }
            "return" => {
                reader.context().push(&field, "Option<PropertyDefinition>", "type found, reading property");
                let mut object: Option<PropertyDefinition> = None;
                if !reader.is_next_nil()? {
                    object = Some(PropertyDefinition::read(reader)?);
                } else {
                    object = None;
                }
                _return = object;
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

    Ok(MethodDefinition {
        arguments: _arguments,
        env: _env,
        _return: _return,
        _type: _type,
        name: _name,
        required: _required,
        comment: _comment,
    })
}
