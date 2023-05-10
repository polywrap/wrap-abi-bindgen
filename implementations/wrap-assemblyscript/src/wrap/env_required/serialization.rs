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
use crate::EnvRequired;

pub fn serialize_env_required(args: &EnvRequired) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: EnvRequired".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_env_required(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_env_required<W: Write>(args: &EnvRequired, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("required", "Option<bool>", "writing property");
    writer.write_string("required")?;
    writer.write_optional_bool(&args.required)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_env_required(args: &[u8]) -> Result<EnvRequired, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: EnvRequired".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_env_required(&mut reader)
}

pub fn read_env_required<R: Read>(reader: &mut R) -> Result<EnvRequired, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _required: Option<bool> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "required" => {
                reader.context().push(&field, "Option<bool>", "type found, reading property");
                _required = reader.read_optional_bool()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(EnvRequired {
        required: _required,
    })
}
