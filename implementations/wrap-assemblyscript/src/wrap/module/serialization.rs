use serde::{Serialize, Deserialize};
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

use crate::WrapAbi;
use crate::Output;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGenerateBindings {
    pub wrap_abi: WrapAbi,
    pub project_name: String,
    pub context: Option<JSON::Value>,
}

pub fn deserialize_generate_bindings_args(args: &[u8]) -> Result<ArgsGenerateBindings, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: generate_bindings Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _wrap_abi: WrapAbi = WrapAbi::new();
    let mut _wrap_abi_set = false;
    let mut _project_name: String = String::new();
    let mut _project_name_set = false;
    let mut _context: Option<JSON::Value> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "wrapAbi" => {
                reader.context().push(&field, "WrapAbi", "type found, reading argument");
                let object = WrapAbi::read(&mut reader)?;
                _wrap_abi = object;
                _wrap_abi_set = true;
                reader.context().pop();
            }
            "projectName" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _project_name = reader.read_string()?;
                _project_name_set = true;
                reader.context().pop();
            }
            "context" => {
                reader.context().push(&field, "Option<JSON::Value>", "type found, reading argument");
                _context = reader.read_optional_json()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_wrap_abi_set {
        return Err(DecodeError::MissingField("wrapAbi: WrapAbi.".to_string()));
    }
    if !_project_name_set {
        return Err(DecodeError::MissingField("projectName: String.".to_string()));
    }

    Ok(ArgsGenerateBindings {
        wrap_abi: _wrap_abi,
        project_name: _project_name,
        context: _context,
    })
}

pub fn serialize_generate_bindings_args(args: &ArgsGenerateBindings) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: generate_bindings Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_generate_bindings_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_generate_bindings_args<W: Write>(args: &ArgsGenerateBindings, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("wrapAbi", "WrapAbi", "writing property");
    writer.write_string("wrapAbi")?;
    WrapAbi::write(&args.wrap_abi, writer)?;
    writer.context().pop();
    writer.context().push("projectName", "String", "writing property");
    writer.write_string("projectName")?;
    writer.write_string(&args.project_name)?;
    writer.context().pop();
    writer.context().push("context", "Option<JSON::Value>", "writing property");
    writer.write_string("context")?;
    writer.write_optional_json(&args.context)?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_generate_bindings_result(result: &Output) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: generate_bindings Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_generate_bindings_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_generate_bindings_result<W: Write>(result: &Output, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("generateBindings", "Output", "writing result");
    Output::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_generate_bindings_result(result: &[u8]) -> Result<Output, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: generate_bindings Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("generateBindings", "Output", "reading function output");
    let object = Output::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}
