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
use crate::ImportedEnvDefinition;

pub fn serialize_imported_env_definition(args: &ImportedEnvDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: ImportedEnvDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_imported_env_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_imported_env_definition<W: Write>(args: &ImportedEnvDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&0)?;
    Ok(())
}

pub fn deserialize_imported_env_definition(args: &[u8]) -> Result<ImportedEnvDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: ImportedEnvDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_imported_env_definition(&mut reader)
}

pub fn read_imported_env_definition<R: Read>(reader: &mut R) -> Result<ImportedEnvDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;


    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ImportedEnvDefinition {
    })
}
