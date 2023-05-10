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
use crate::File;

pub fn serialize_file(args: &File) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: File".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_file(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_file<W: Write>(args: &File, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("name", "String", "writing property");
    writer.write_string("name")?;
    writer.write_string(&args.name)?;
    writer.context().pop();
    writer.context().push("data", "String", "writing property");
    writer.write_string("data")?;
    writer.write_string(&args.data)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_file(args: &[u8]) -> Result<File, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: File".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_file(&mut reader)
}

pub fn read_file<R: Read>(reader: &mut R) -> Result<File, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _name: String = String::new();
    let mut _name_set = false;
    let mut _data: String = String::new();
    let mut _data_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "name" => {
                reader.context().push(&field, "String", "type found, reading property");
                _name = reader.read_string()?;
                _name_set = true;
                reader.context().pop();
            }
            "data" => {
                reader.context().push(&field, "String", "type found, reading property");
                _data = reader.read_string()?;
                _data_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_name_set {
        return Err(DecodeError::MissingField("name: String.".to_string()));
    }
    if !_data_set {
        return Err(DecodeError::MissingField("data: String.".to_string()));
    }

    Ok(File {
        name: _name,
        data: _data,
    })
}
