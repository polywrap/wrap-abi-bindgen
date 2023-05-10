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
use crate::WithComment;

pub fn serialize_with_comment(args: &WithComment) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: WithComment".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_with_comment(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_with_comment<W: Write>(args: &WithComment, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("comment", "Option<String>", "writing property");
    writer.write_string("comment")?;
    writer.write_optional_string(&args.comment)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_with_comment(args: &[u8]) -> Result<WithComment, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: WithComment".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_with_comment(&mut reader)
}

pub fn read_with_comment<R: Read>(reader: &mut R) -> Result<WithComment, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _comment: Option<String> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "comment" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _comment = reader.read_optional_string()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(WithComment {
        comment: _comment,
    })
}
