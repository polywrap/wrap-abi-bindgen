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
use crate::MapKeyDefinition;

use crate::{
    MapKeyType,
    get_map_key_type_value,
    sanitize_map_key_type_value
};
use crate::ArrayDefinition;
use crate::ScalarDefinition;
use crate::MapDefinition;
use crate::ObjectRef;
use crate::EnumRef;
use crate::UnresolvedObjectOrEnumRef;

pub fn serialize_map_key_definition(args: &MapKeyDefinition) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) object-type: MapKeyDefinition".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_map_key_definition(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_map_key_definition<W: Write>(args: &MapKeyDefinition, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&7)?;
    writer.context().push("type", "Option<MapKeyType>", "writing property");
    writer.write_string("type")?;
    writer.write_optional_i32(&args._type.map(|f| f as i32))?;
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
    Ok(())
}

pub fn deserialize_map_key_definition(args: &[u8]) -> Result<MapKeyDefinition, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing object-type: MapKeyDefinition".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_map_key_definition(&mut reader)
}

pub fn read_map_key_definition<R: Read>(reader: &mut R) -> Result<MapKeyDefinition, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _type: Option<MapKeyType> = None;
    let mut _array: Option<ArrayDefinition> = None;
    let mut _scalar: Option<ScalarDefinition> = None;
    let mut _map: Option<MapDefinition> = None;
    let mut _object: Option<ObjectRef> = None;
    let mut _enum: Option<EnumRef> = None;
    let mut _unresolved_object_or_enum: Option<UnresolvedObjectOrEnumRef> = None;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "type" => {
                reader.context().push(&field, "Option<MapKeyType>", "type found, reading property");
                let mut value: Option<MapKeyType> = None;
                if !reader.is_next_nil()? {
                    if reader.is_next_string()? {
                        value = Some(get_map_key_type_value(&reader.read_string()?)?);
                    } else {
                        value = Some(MapKeyType::try_from(reader.read_i32()?)?);
                        sanitize_map_key_type_value(value.unwrap() as i32)?;
                    }
                } else {
                    value = None;
                }
                _type = value;
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
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }

    Ok(MapKeyDefinition {
        _type: _type,
        array: _array,
        scalar: _scalar,
        map: _map,
        object: _object,
        _enum: _enum,
        unresolved_object_or_enum: _unresolved_object_or_enum,
    })
}
