lazy_static! {
  static ref NAME: String = "serialize_map_value".to_string();
  static ref SOURCE: String = r#"{{#with scalar}}
writer.write{{to_msgpack (to_graphql_type this)}}(value);
{{/with}}
{{#with array}}
writer.write{{to_msgpack (to_graphql_type this)}}(value, (writer: Write, item: {{#with item}}{{to_wasm (to_graphql_type this)}}{{/with}}): void => {
{{indent_partial "serialize_array" 2}}
});
{{/with}}
{{#with map}}
writer.write{{to_msgpack (to_graphql_type this)}}(value, (writer: Write, key: {{#with key}}{{to_wasm (to_graphql_type this)}}{{/with}}) => {
  writer.write{{#with key}}{{to_msgpack (to_graphql_type this)}}{{/with}}(key);
}, (writer: Write, value: {{#with value}}{{to_wasm (to_graphql_type this)}}{{/with}}): void => {
{{indent_partial "serialize_map_value" 2}}
});
{{/with}}
{{#with enum}}
{{#if required}}
writer.writeInt32(value);
{{else}}
writer.writeOptionalInt32(value);
{{/if}}
{{/with}}
{{#with object}}
{{#if required}}
Types.{{detect_keyword type}}.write(writer, value);
{{else}}
if (value) {
  Types.{{detect_keyword type}}.write(writer, value as Types.{{detect_keyword type}});
} else {
  writer.writeNil();
}
{{/if}}
{{/with}}
"#.to_string();
}

use super::Partial;

pub fn load() -> Partial {
    Partial {
        name: &*NAME,
        source: &*SOURCE
    }
}
