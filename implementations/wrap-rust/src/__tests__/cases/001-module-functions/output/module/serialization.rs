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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsFunction1 {
    pub arg1: u32,
    pub arg2: bool,
}

pub fn deserialize_function1_args(args: &[u8]) -> Result<ArgsFunction1, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: function1 Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _arg1: u32 = 0;
    let mut _arg1_set = false;
    let mut _arg2: bool = false;
    let mut _arg2_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "arg1" => {
                reader.context().push(&field, "u32", "type found, reading argument");
                _arg1 = reader.read_u32()?;
                _arg1_set = true;
                reader.context().pop();
            }
            "arg2" => {
                reader.context().push(&field, "bool", "type found, reading argument");
                _arg2 = reader.read_bool()?;
                _arg2_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_arg1_set {
        return Err(DecodeError::MissingField("arg1: UInt32.".to_string()));
    }
    if !_arg2_set {
        return Err(DecodeError::MissingField("arg2: Boolean.".to_string()));
    }

    Ok(ArgsFunction1 {
        arg1: _arg1,
        arg2: _arg2,
    })
}

pub fn serialize_function1_args(args: &ArgsFunction1) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: function1 Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_function1_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_function1_args<W: Write>(args: &ArgsFunction1, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("arg1", "u32", "writing property");
    writer.write_string("arg1")?;
    writer.write_u32(&args.arg1)?;
    writer.context().pop();
    writer.context().push("arg2", "bool", "writing property");
    writer.write_string("arg2")?;
    writer.write_bool(&args.arg2)?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_function1_result(result: &String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: function1 Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_function1_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_function1_result<W: Write>(result: &String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("function1", "String", "writing result");
    writer.write_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_function1_result(result: &[u8]) -> Result<String, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: function1 Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("function1", "String", "reading function output");
    let res = reader.read_string()?;
    reader.context().pop();
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsFunction2 {
    pub arg1: Option<i32>,
    pub arg2: Option<Vec<u8>>,
}

pub fn deserialize_function2_args(args: &[u8]) -> Result<ArgsFunction2, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: function2 Args".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _arg1: Option<i32> = None;
    let mut _arg2: Option<Vec<u8>> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "arg1" => {
                reader.context().push(&field, "Option<i32>", "type found, reading argument");
                _arg1 = reader.read_optional_i32()?;
                reader.context().pop();
            }
            "arg2" => {
                reader.context().push(&field, "Option<Vec<u8>>", "type found, reading argument");
                _arg2 = reader.read_optional_bytes()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(ArgsFunction2 {
        arg1: _arg1,
        arg2: _arg2,
    })
}

pub fn serialize_function2_args(args: &ArgsFunction2) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: function2 Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_function2_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_function2_args<W: Write>(args: &ArgsFunction2, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("arg1", "Option<i32>", "writing property");
    writer.write_string("arg1")?;
    writer.write_optional_i32(&args.arg1)?;
    writer.context().pop();
    writer.context().push("arg2", "Option<Vec<u8>>", "writing property");
    writer.write_string("arg2")?;
    writer.write_optional_bytes(&args.arg2)?;
    writer.context().pop();
    Ok(())
}

pub fn serialize_function2_result(result: &Option<String>) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: function2 Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_function2_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_function2_result<W: Write>(result: &Option<String>, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("function2", "Option<String>", "writing result");
    writer.write_optional_string(result)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_function2_result(result: &[u8]) -> Result<Option<String>, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: function2 Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("function2", "Option<String>", "reading function output");
    let res = reader.read_optional_string()?;
    reader.context().pop();
    Ok(res)
}
