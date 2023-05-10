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
use crate::CapabilityEnabled;

pub fn serialize_capability_enabled(args: &CapabilityEnabled) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: CapabilityEnabled".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_capability_enabled(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_capability_enabled<W: Write>(args: &CapabilityEnabled, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("enabled", "bool", "writing property");
    writer.write_string("enabled")?;
    writer.write_bool(&args.enabled)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_capability_enabled(args: &[u8]) -> Result<CapabilityEnabled, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: CapabilityEnabled".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_capability_enabled(&mut reader)
}

pub fn read_capability_enabled<R: Read>(reader: &mut R) -> Result<CapabilityEnabled, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _enabled: bool = false;
    let mut _enabled_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "enabled" => {
                reader.context().push(&field, "bool", "type found, reading property");
                _enabled = reader.read_bool()?;
                _enabled_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_enabled_set {
        return Err(DecodeError::MissingField("enabled: Boolean.".to_string()));
    }

    Ok(CapabilityEnabled {
        enabled: _enabled,
    })
}
