lazy_static! {
  static ref NAME: String = "serialize_map_value".to_string();
  static ref SOURCE: String = r#"{{#if scalar}}
writer.write{{to_msgpack (to_graphql_type)}}(value);
{{/if}}
{{#if array}}
writer.write{{to_msgpack (to_graphql_type)}}(value, (writer: Write, item: {{#if item}}{{to_wasm (to_graphql_type)}}{{/if}}): void => {
  {{> serialize_array}}
});
{{/if}}
{{#if map}}
writer.write{{to_msgpack (to_graphql_type)}}(value, (writer: Write, key: {{#if key}}{{to_wasm (to_graphql_type)}}{{/if}}) => {
  writer.write{{#if key}}{{to_msgpack (to_graphql_type)}}{{/if}}(key);
}, (writer: Write, value: {{#if value}}{{to_wasm (to_graphql_type)}}{{/if}}): void => {
  {{> serialize_map_value}}
});
{{/if}}
{{#if enum}}
{{#if required}}
writer.writeInt32(value);
{{else}}
writer.writeOptionalInt32(value);
{{/if}}
{{/if}}
{{#if object}}
{{#if required}}
Types.{{detect_keyword type}}.write(writer, value);
{{else}}
if (value) {
  Types.{{detect_keyword type}}.write(writer, value as Types.{{detect_keyword type}});
} else {
  writer.writeNil();
}
{{/if}}
{{/if}}
"#.to_string();
}

use super::Partial;

pub fn load() -> Partial {
    Partial {
        name: &*NAME,
        source: &*SOURCE
    }
}
