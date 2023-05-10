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
use crate::MapDefinition;

use crate::MapKeyDefinition;
use crate::GenericDefinition;
use crate::ArrayDefinition;
use crate::ScalarDefinition;
use crate::ObjectRef;
use crate::EnumRef;
use crate::UnresolvedObjectOrEnumRef;

pub fn serialize_map_definition(args: &MapDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: MapDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_map_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_map_definition<W: Write>(args: &MapDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&9)?;
    writer.context().push("key", "Option<MapKeyDefinition>", "writing property");
    writer.write_string("key")?;
    if args.key.is_some() {
        MapKeyDefinition::write(args.key.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
    writer.context().push("value", "Option<GenericDefinition>", "writing property");
    writer.write_string("value")?;
    if args.value.is_some() {
        GenericDefinition::write(args.value.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    writer.context().pop();
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
    writer.context().push("comment", "Option<String>", "writing property");
    writer.write_string("comment")?;
    writer.write_optional_string(&args.comment)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_map_definition(args: &[u8]) -> Result<MapDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: MapDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_map_definition(&mut reader)
}

pub fn read_map_definition<R: Read>(reader: &mut R) -> Result<MapDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _key: Option<MapKeyDefinition> = None;
    let mut _value: Option<GenericDefinition> = None;
    let mut _array: Option<ArrayDefinition> = None;
    let mut _scalar: Option<ScalarDefinition> = None;
    let mut _map: Option<MapDefinition> = None;
    let mut _object: Option<ObjectRef> = None;
    let mut _enum: Option<EnumRef> = None;
    let mut _unresolved_object_or_enum: Option<UnresolvedObjectOrEnumRef> = None;
    let mut _comment: Option<String> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "key" => {
                reader.context().push(&field, "Option<MapKeyDefinition>", "type found, reading property");
                let mut object: Option<MapKeyDefinition> = None;
                if !reader.is_next_nil()? {
                    object = Some(MapKeyDefinition::read(reader)?);
                } else {
                    object = None;
                }
                _key = object;
                reader.context().pop();
            }
            "value" => {
                reader.context().push(&field, "Option<GenericDefinition>", "type found, reading property");
                let mut object: Option<GenericDefinition> = None;
                if !reader.is_next_nil()? {
                    object = Some(GenericDefinition::read(reader)?);
                } else {
                    object = None;
                }
                _value = object;
                reader.context().pop();
            }
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
            "comment" => {
                reader.context().push(&field, "Option<String>", "type found, reading property");
                _comment = reader.read_optional_string()?;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(MapDefinition {
        key: match _key {
            Some(x) => Some(Box::new(x)),
            _ => None
        },
        value: _value,
        array: _array,
        scalar: _scalar,
        map: match _map {
            Some(x) => Some(Box::new(x)),
            _ => None
        },
        object: _object,
        _enum: _enum,
        unresolved_object_or_enum: _unresolved_object_or_enum,
        comment: _comment,
    })
}
