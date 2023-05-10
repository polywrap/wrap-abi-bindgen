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
use crate::InterfaceDefinition;

use crate::CapabilityDefinition;

pub fn serialize_interface_definition(args: &InterfaceDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: InterfaceDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_interface_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_interface_definition<W: Write>(args: &InterfaceDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&4)?;
    writer.context().push("capabilities", "Option<CapabilityDefinition>", "writing property");
    writer.write_string("capabilities")?;
    if args.capabilities.is_some() {
        CapabilityDefinition::write(args.capabilities.as_ref().as_ref().unwrap(), writer)?;
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
    Ok(())
}

pub fn deserialize_interface_definition(args: &[u8]) -> Result<InterfaceDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: InterfaceDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_interface_definition(&mut reader)
}

pub fn read_interface_definition<R: Read>(reader: &mut R) -> Result<InterfaceDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _capabilities: Option<CapabilityDefinition> = None;
    let mut _type: String = String::new();
    let mut _type_set = false;
    let mut _name: Option<String> = None;
    let mut _required: Option<bool> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "capabilities" => {
                reader.context().push(&field, "Option<CapabilityDefinition>", "type found, reading property");
                let mut object: Option<CapabilityDefinition> = None;
                if !reader.is_next_nil()? {
                    object = Some(CapabilityDefinition::read(reader)?);
                } else {
                    object = None;
                }
                _capabilities = object;
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
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_type_set {
        return Err(DecodeError::MissingField("type: String.".to_string()));
    }

    Ok(InterfaceDefinition {
        capabilities: _capabilities,
        _type: _type,
        name: _name,
        required: _required,
    })
}
