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
use crate::ImportedModuleRef;

pub fn serialize_imported_module_ref(args: &ImportedModuleRef) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: ImportedModuleRef".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_imported_module_ref(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_imported_module_ref<W: Write>(args: &ImportedModuleRef, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&1)?;
    writer.context().push("type", "Option<String>", "writing property");
    writer.write_string("type")?;
    writer.write_optional_string(&args._type)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_imported_module_ref(args: &[u8]) -> Result<ImportedModuleRef, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: ImportedModuleRef".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_imported_module_ref(&mut reader)
}

pub fn read_imported_module_ref<R: Read>(reader: &mut R) -> Result<ImportedModuleRef, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _type: Option<String> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "type" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _type = reader.read_optional_string()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ImportedModuleRef {
        _type: _type,
    })
}
