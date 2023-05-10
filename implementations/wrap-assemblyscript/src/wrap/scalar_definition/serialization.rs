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
use crate::ScalarDefinition;

use crate::{
    ScalarType,
    get_scalar_type_value,
    sanitize_scalar_type_value
};

pub fn serialize_scalar_definition(args: &ScalarDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: ScalarDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_scalar_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_scalar_definition<W: Write>(args: &ScalarDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&4)?;
    writer.context().push("kind", "i32", "writing property");
    writer.write_string("kind")?;
    writer.write_i32(&args.kind)?;
    writer.context().pop();
    writer.context().push("type", "ScalarType", "writing property");
    writer.write_string("type")?;
    writer.write_i32(&(args._type as i32))?;
    writer.context().pop();
    writer.context().push("name", "Option<String>", "writing property");
    writer.write_string("name")?;
    writer.write_optional_string(&args.name)?;
    writer.context().pop();
    writer.context().push("required", "Option<bool>", "writing property");
    writer.write_string("required")?;
    writer.write_optional_bool(&args.required)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_scalar_definition(args: &[u8]) -> Result<ScalarDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: ScalarDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_scalar_definition(&mut reader)
}

pub fn read_scalar_definition<R: Read>(reader: &mut R) -> Result<ScalarDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _kind: i32 = 0;
    let mut _kind_set = false;
    let mut _type: ScalarType = ScalarType::_MAX_;
    let mut _type_set = false;
    let mut _name: Option<String> = None;
    let mut _required: Option<bool> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "kind" => {
                reader.context().push(&field, "i32", "type found, reading property");
                _kind = reader.read_i32()?;
                _kind_set = true;
                reader.context().pop();
            }
            "type" => {
                reader.context().push(&field, "ScalarType", "type found, reading property");
                let mut value: ScalarType = ScalarType::_MAX_;
                if reader.is_next_string()? {
                    value = get_scalar_type_value(&reader.read_string()?)?;
                } else {
                    value = ScalarType::try_from(reader.read_i32()?)?;
                    sanitize_scalar_type_value(value as i32)?;
                }
                _type = value;
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
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_kind_set {
        return Err(DecodeError::MissingField("kind: Int.".to_string()));
    }
    if !_type_set {
        return Err(DecodeError::MissingField("type: ScalarType.".to_string()));
    }

    Ok(ScalarDefinition {
        kind: _kind,
        _type: _type,
        name: _name,
        required: _required,
    })
}
