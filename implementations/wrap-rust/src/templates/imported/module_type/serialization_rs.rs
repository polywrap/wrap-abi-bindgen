lazy_static! {
  static ref NAME: String = "imported/module_type/serialization.rs".to_string();
  static ref SOURCE: String = r#"{{#if (array_has_length methods)}}
{{> serialization_imports}}
use serde::{Serialize, Deserialize};

{{#each methods}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Args{{to_upper name}} {
    {{#each arguments}}
    {{serdeKeyword (to_lower name)}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}

pub fn deserialize_{{to_lower name}}_args(args: &[u8]) -> Result<Args{{to_upper name}}, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: {{to_lower name}} Args".to_string();

    {{#if (array_has_length arguments)}}
    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    {{#each arguments}}
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
            {{#each arguments}}
            "{{name}}" => {
                reader.context().push(&field, "{{to_rust (to_graphql_type this)}}", "type found, reading argument");
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
                {{> deserialize_object_refmut}}
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
    {{#each arguments}}
    {{#if required}}
    if !_{{to_lower name}}_set {
        return Err(DecodeError::MissingField("{{name}}: {{type}}.".to_string()));
    }
    {{/if}}
    {{/each}}
    {{/if}}

    Ok(Args{{to_upper name}} {
        {{#each arguments}}
        {{detect_keyword (to_lower name)}}: _{{to_lower name}},
        {{/each}}
    })
}

pub fn serialize_{{to_lower name}}_args(args: &Args{{to_upper name}}) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: {{to_lower name}} Args".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_{{to_lower name}}_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_{{to_lower name}}_args<W: Write>(args: &Args{{to_upper name}}, writer: &mut W) -> Result<(), EncodeError> {
    {{#if (array_has_length arguments)}}
    writer.write_map_length(&{{arguments.length}})?;
    {{/if}}
    {{^arguments}}
    writer.write_map_length(&0)?;
    {{/each}}
    {{#each arguments}}
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
    {{#enum}}
    {{#if required}}
    writer.write_i32(&(args.{{detect_keyword (to_lower name)}} as i32))?;
    {{/if}}
    {{^required}}
    writer.write_optional_i32(&args.{{detect_keyword (to_lower name)}}.map(|f| f as i32))?;
    {{/if}}
    {{/enum}}
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
    writer.context().pop();
    {{/each}}
    Ok(())
}

pub fn serialize_{{to_lower name}}_result(result: {{#with return}}&{{to_rust (to_graphql_type this)}}{{/with}}) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: {{to_lower name}} Result".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_{{to_lower name}}_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_{{to_lower name}}_result<W: Write>(result: {{#with return}}&{{to_rust (to_graphql_type this)}}{{/with}}, writer: &mut W) -> Result<(), EncodeError> {
    {{#with return}}
    writer.context().push("{{name}}", "{{to_rust (to_graphql_type this)}}", "writing result");
    {{#scalar}}
    writer.write_{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}(result)?;
    {{/scalar}}
    {{#array}}
    writer.write_{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}(&result, |writer, item| {
        {{> serialize_array}}
    })?;
    {{/array}}
    {{#map}}
    writer.write_{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}(&result, |writer, key| {
        writer.write_{{#key}}{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}{{/key}}(key)
    }, |writer, value| {
        {{> serialize_map_value}}
    })?;
    {{/map}}
    {{#enum}}
    {{#if required}}
    writer.write_i32(&(*result as i32))?;
    {{/if}}
    {{^required}}
    writer.write_optional_i32(&result.map(|f| f as i32))?;
    {{/if}}
    {{/enum}}
    {{#object}}
    {{#if required}}
    {{detect_keyword (to_upper type)}}::write(&result, writer)?;
    {{/if}}
    {{^required}}
    if result.is_some() {
        {{detect_keyword (to_upper type)}}::write(result.as_ref().unwrap(), writer)?;
    } else {
        writer.write_nil()?;
    }
    {{/if}}
    {{/object}}
    writer.context().pop();
    {{/with}}
    Ok(())
}

pub fn deserialize_{{to_lower name}}_result(result: &[u8]) -> Result<{{#with return}}{{to_rust (to_graphql_type this)}}{{/with}}, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: {{to_lower name}} Result".to_string();
    let mut reader = ReadDecoder::new(result, context);

    {{#with return}}
    reader.context().push("{{name}}", "{{to_rust (to_graphql_type this)}}", "reading function output");
    {{#scalar}}
    let res = reader.read_{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}()?;
    {{/scalar}}
    {{#array}}
    let res = reader.read_{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}(|reader| {
        {{> deserialize_array_nobox}}
    })?;
    {{/array}}
    {{#map}}
    let res = reader.read_{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}(|reader| {
        reader.read_{{#key}}{{#toLower}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/toLower}}{{/key}}()
    }, |reader| {
        {{> deserialize_map_value_nobox}}
    })?;
    {{/map}}
    {{#enum}}
    {{> deserialize_enum}}
    let res = value;
    {{/enum}}
    {{#object}}
    {{> deserialize_object_refmut}}
    let res = object;
    {{/object}}
    {{/with}}
    reader.context().pop();
    Ok(res)
}
{{#if (is_not_last @index ../methods)}}

{{/if}}
{{/each}}
{{/if}}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
