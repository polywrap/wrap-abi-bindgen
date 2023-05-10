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
use crate::AnyDefinition;

use crate::ArrayDefinition;
use crate::ScalarDefinition;
use crate::MapDefinition;
use crate::ObjectRef;
use crate::EnumRef;
use crate::UnresolvedObjectOrEnumRef;

pub fn serialize_any_definition(args: &AnyDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: AnyDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_any_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_any_definition<W: Write>(args: &AnyDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&9)?;
    writer.context().push("array", "Option<ArrayDefinition>", "writing property");
    writer.write_string("array")?;
    if args.array.is_some() {
        ArrayDefinition::write(args.array.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("scalar", "Option<ScalarDefinition>", "writing property");
    writer.write_string("scalar")?;
    if args.scalar.is_some() {
        ScalarDefinition::write(args.scalar.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("map", "Option<MapDefinition>", "writing property");
    writer.write_string("map")?;
    if args.map.is_some() {
        MapDefinition::write(args.map.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("object", "Option<ObjectRef>", "writing property");
    writer.write_string("object")?;
    if args.object.is_some() {
        ObjectRef::write(args.object.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("enum", "Option<EnumRef>", "writing property");
    writer.write_string("enum")?;
    if args._enum.is_some() {
        EnumRef::write(args._enum.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("unresolvedObjectOrEnum", "Option<UnresolvedObjectOrEnumRef>", "writing property");
    writer.write_string("unresolvedObjectOrEnum")?;
    if args.unresolved_object_or_enum.is_some() {
        UnresolvedObjectOrEnumRef::write(args.unresolved_object_or_enum.as_ref().as_ref().unwrap(), writer)?;
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

pub fn deserialize_any_definition(args: &[u8]) -> Result<AnyDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: AnyDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_any_definition(&mut reader)
}

pub fn read_any_definition<R: Read>(reader: &mut R) -> Result<AnyDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _array: Option<ArrayDefinition> = None;
    let mut _scalar: Option<ScalarDefinition> = None;
    let mut _map: Option<MapDefinition> = None;
    let mut _object: Option<ObjectRef> = None;
    let mut _enum: Option<EnumRef> = None;
    let mut _unresolved_object_or_enum: Option<UnresolvedObjectOrEnumRef> = None;
    let mut _type: String = String::new();
    let mut _type_set = false;
    let mut _name: Option<String> = None;
    let mut _required: Option<bool> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "array" => {
                reader.context().push(&field, "Option<ArrayDefinition>", "type found, reading property");
                let mut object: Option<ArrayDefinition> = None;
                if !reader.is_next_nil()? {
                    object = Some(ArrayDefinition::read(reader)?);
                } else {
                    object = None;
                }
                _array = object;
                reader.context().pop();
            }
            "scalar" => {
                reader.context().push(&field, "Option<ScalarDefinition>", "type found, reading property");
                let mut object: Option<ScalarDefinition> = None;
                if !reader.is_next_nil()? {
                    object = Some(ScalarDefinition::read(reader)?);
                } else {
                    object = None;
                }
                _scalar = object;
                reader.context().pop();
            }
            "map" => {
                reader.context().push(&field, "Option<MapDefinition>", "type found, reading property");
                let mut object: Option<MapDefinition> = None;
                if !reader.is_next_nil()? {
                    object = Some(MapDefinition::read(reader)?);
                } else {
                    object = None;
                }
                _map = object;
                reader.context().pop();
            }
            "object" => {
                reader.context().push(&field, "Option<ObjectRef>", "type found, reading property");
                let mut object: Option<ObjectRef> = None;
                if !reader.is_next_nil()? {
                    object = Some(ObjectRef::read(reader)?);
                } else {
                    object = None;
                }
                _object = object;
                reader.context().pop();
            }
            "enum" => {
                reader.context().push(&field, "Option<EnumRef>", "type found, reading property");
                let mut object: Option<EnumRef> = None;
                if !reader.is_next_nil()? {
                    object = Some(EnumRef::read(reader)?);
                } else {
                    object = None;
                }
                _enum = object;
                reader.context().pop();
            }
            "unresolvedObjectOrEnum" => {
                reader.context().push(&field, "Option<UnresolvedObjectOrEnumRef>", "type found, reading property");
                let mut object: Option<UnresolvedObjectOrEnumRef> = None;
                if !reader.is_next_nil()? {
                    object = Some(UnresolvedObjectOrEnumRef::read(reader)?);
                } else {
                    object = None;
                }
                _unresolved_object_or_enum = object;
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

    Ok(AnyDefinition {
        array: _array,
        scalar: _scalar,
        map: _map,
        object: _object,
        _enum: _enum,
        unresolved_object_or_enum: _unresolved_object_or_enum,
        _type: _type,
        name: _name,
        required: _required,
    })
}
