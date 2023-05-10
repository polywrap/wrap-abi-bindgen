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
use crate::CapabilityDefinition;

use crate::CapabilityEnabled;

pub fn serialize_capability_definition(args: &CapabilityDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: CapabilityDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_capability_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_capability_definition<W: Write>(args: &CapabilityDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("getImplementations", "Option<CapabilityEnabled>", "writing property");
    writer.write_string("getImplementations")?;
    if args.get_implementations.is_some() {
        CapabilityEnabled::write(args.get_implementations.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    Ok(())
}

pub fn deserialize_capability_definition(args: &[u8]) -> Result<CapabilityDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: CapabilityDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_capability_definition(&mut reader)
}

pub fn read_capability_definition<R: Read>(reader: &mut R) -> Result<CapabilityDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _get_implementations: Option<CapabilityEnabled> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "getImplementations" => {
                reader.context().push(&field, "Option<CapabilityEnabled>", "type found, reading property");
                let mut object: Option<CapabilityEnabled> = None;
                if !reader.is_next_nil()? {
                    object = Some(CapabilityEnabled::read(reader)?);
                } else {
                    object = None;
                }
                _get_implementations = object;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(CapabilityDefinition {
        get_implementations: _get_implementations,
    })
}
