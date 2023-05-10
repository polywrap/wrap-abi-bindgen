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
use crate::GenericDefinition;

pub fn serialize_generic_definition(args: &GenericDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: GenericDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_generic_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_generic_definition<W: Write>(args: &GenericDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&4)?;
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
    writer.context().push("kind", "i32", "writing property");
    writer.write_string("kind")?;
    writer.write_i32(&args.kind)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_generic_definition(args: &[u8]) -> Result<GenericDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: GenericDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_generic_definition(&mut reader)
}

pub fn read_generic_definition<R: Read>(reader: &mut R) -> Result<GenericDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _type: String = String::new();
    let mut _type_set = false;
    let mut _name: Option<String> = None;
    let mut _required: Option<bool> = None;
    let mut _kind: i32 = 0;
    let mut _kind_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
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
            "kind" => {
                reader.context().push(&field, "i32", "type found, reading property");
                _kind = reader.read_i32()?;
                _kind_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_type_set {
        return Err(DecodeError::MissingField("type: String.".to_string()));
    }
    if !_kind_set {
        return Err(DecodeError::MissingField("kind: Int.".to_string()));
    }

    Ok(GenericDefinition {
        _type: _type,
        name: _name,
        required: _required,
        kind: _kind,
    })
}
