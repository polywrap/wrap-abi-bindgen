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
use crate::WithKind;

pub fn serialize_with_kind(args: &WithKind) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: WithKind".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_with_kind(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_with_kind<W: Write>(args: &WithKind, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("kind", "i32", "writing property");
    writer.write_string("kind")?;
    writer.write_i32(&args.kind)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_with_kind(args: &[u8]) -> Result<WithKind, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: WithKind".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_with_kind(&mut reader)
}

pub fn read_with_kind<R: Read>(reader: &mut R) -> Result<WithKind, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _kind: i32 = 0;
    let mut _kind_set = false;

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
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_kind_set {
        return Err(DecodeError::MissingField("kind: Int.".to_string()));
    }

    Ok(WithKind {
        kind: _kind,
    })
}
