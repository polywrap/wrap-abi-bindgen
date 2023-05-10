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
use crate::ImportedEnumDefinition;

pub fn serialize_imported_enum_definition(args: &ImportedEnumDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: ImportedEnumDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_imported_enum_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_imported_enum_definition<W: Write>(args: &ImportedEnumDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("constants", "Option<Vec<String>>", "writing property");
    writer.write_string("constants")?;
    writer.write_optional_array(&args.constants, |writer, item| {
        writer.write_string(item)
    })?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_imported_enum_definition(args: &[u8]) -> Result<ImportedEnumDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: ImportedEnumDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_imported_enum_definition(&mut reader)
}

pub fn read_imported_enum_definition<R: Read>(reader: &mut R) -> Result<ImportedEnumDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _constants: Option<Vec<String>> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "constants" => {
                reader.context().push(&field, "Option<Vec<String>>", "type found, reading property");
                _constants = reader.read_optional_array(|reader| {
                    reader.read_string()
                })?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ImportedEnumDefinition {
        constants: _constants,
    })
}
