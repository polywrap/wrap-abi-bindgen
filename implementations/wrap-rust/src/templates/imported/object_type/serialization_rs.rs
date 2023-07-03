lazy_static! {
  static ref NAME: String = "object_type/serialization.rs".to_string();
  static ref SOURCE: String = r#"{{> serialization_imports}}
use std::convert::TryFrom;
use crate::{{detect_keyword (to_upper type)}};

pub fn serialize_{{to_lower type}}(args: &{{detect_keyword (to_upper type)}}) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: {{to_upper type}}".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_{{to_lower type}}(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_{{to_lower type}}<W: Write>(args: &{{detect_keyword (to_upper type)}}, writer: &mut W) -> Result<(), EncodeError> {
    {{#properties.length}}
    writer.write_map_length(&{{properties.length}})?;
    {{/properties.length}}
    {{^properties}}
    writer.write_map_length(&0)?;
    {{/each}}
    {{#each properties}}
    writer.context().push("{{name}}", "{{to_rust (to_graphql_type this)}}", "writing property");
    writer.write_string("{{name}}")?;
    {{#scalar}}
    writer.write_{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}(&args.{{detect_keyword (to_lower name)}})?;
    {{/scalar}}
    {{#array}}
    writer.write_{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}(&args.{{detect_keyword (to_lower name)}}, |writer, item| {
        {{> serialize_array}}
    })?;
    {{/array}}
    {{#map}}
    writer.write_{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}(&args.{{detect_keyword (to_lower name)}}, |writer, key| {
        writer.write_{{#key}}{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}{{/key}}(key)
    }, |writer, value| {
        {{> serialize_map_value}}
    })?;
    {{/map}}
    {{#object}}
    {{#if required}}
    {{detect_keyword (to_upper type)}}::write(&args.{{detect_keyword (to_lower name)}}, writer)?;
    {{/if}}
    {{^required}}
    if args.{{detect_keyword (to_lower name)}}.is_some() {
        {{detect_keyword (to_upper type)}}::write(args.{{detect_keyword (to_lower name)}}.as_ref().as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    {{/if}}
    {{/object}}
    {{#enum}}
    {{#if required}}
    writer.write_i32(&(args.{{detect_keyword (to_lower name)}} as i32))?;
    {{/if}}
    {{^required}}
    writer.write_optional_i32(&args.{{detect_keyword (to_lower name)}}.map(|f| f as i32))?;
    {{/if}}
    {{/enum}}
    writer.context().pop();
    {{/each}}
    Ok(())
}

pub fn deserialize_{{to_lower type}}(args: &[u8]) -> Result<{{detect_keyword (to_upper type)}}, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: {{to_upper type}}".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_{{to_lower type}}(&mut reader)
}

pub fn read_{{to_lower type}}<R: Read>(reader: &mut R) -> Result<{{detect_keyword (to_upper type)}}, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    {{#each properties}}
    {{^object}}
    let mut _{{to_lower name}}: {{to_rust (to_graphql_type this)}} = {{to_rust_init (to_graphql_type this)}};
    {{/object}}
    {{#object}}
    {{#if required}}
    let mut _{{to_lower name}}: {{to_rust (to_graphql_type this)}} = {{to_rust_init (to_graphql_type this)}};
    {{/if}}
    {{^required}}
    let mut _{{to_lower name}}: {{to_rust (to_graphql_type this)}} = None;
    {{/if}}
    {{/object}}
    {{#if required}}
    let mut _{{to_lower name}}_set = false;
    {{/if}}
    {{/each}}

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            {{#each properties}}
            "{{name}}" => {
                reader.context().push(&field, "{{to_rust (to_graphql_type this)}}", "type found, reading property");
                {{#scalar}}
                _{{to_lower name}} = reader.read_{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}()?;
                {{/scalar}}
                {{#array}}
                _{{to_lower name}} = reader.read_{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}(|reader| {
                    {{> deserialize_array_nobox}}
                })?;
                {{/array}}
                {{#map}}
                _{{to_lower name}} = reader.read_{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}(|reader| {
                    reader.read_{{#key}}{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}{{/key}}()
                }, |reader| {
                    {{> deserialize_map_value_nobox}}
                })?;
                {{/map}}
                {{#enum}}
                {{> deserialize_enum}}
                _{{to_lower name}} = value;
                {{/enum}}
                {{#object}}
                {{> deserialize_object_nobox}}
                _{{to_lower name}} = object;
                {{/object}}
                {{#if required}}
                _{{to_lower name}}_set = true;
                {{/if}}
                reader.context().pop();
            }
            {{/each}}
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    {{#each properties}}
    {{#if required}}
    if !_{{to_lower name}}_set {
        return Err(DecodeError::MissingField("{{name}}: {{type}}.".to_string()));
    }
    {{/if}}
    {{/each}}

    Ok({{detect_keyword (to_upper type)}} {
        {{#each properties}}
        {{detect_keyword (to_lower name)}}: _{{to_lower name}},
        {{/each}}
    })
}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
