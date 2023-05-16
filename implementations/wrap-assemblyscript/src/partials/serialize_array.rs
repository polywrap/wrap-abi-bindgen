lazy_static! {
  static ref NAME: String = "serialize_array".to_string();
  static ref SOURCE: String = r#"{{#with scalar}}
writer.write{{to_msgpack (to_graphql_type this)}}(item);
{{/with}}
{{#with array}}
writer.write{{to_msgpack (to_graphql_type this)}}(item, (writer: Write, item: {{#with item}}{{to_wasm (to_graphql_type this)}}{{/with}}): void => {
  {{> serialize_array}}
});
{{/with}}
{{#with map}}
writer.write{{to_msgpack (to_graphql_type this)}}(item, (writer: Write, key: {{#with key}}{{to_wasm (to_graphql_type this)}}{{/with}}) => {
  writer.write{{to_msgpack (to_graphql_type this)}}(key);
}, (writer: Write, value: {{#with value}}{{to_wasm (to_graphql_type this)}}{{/with}}): void => {
  {{> serialize_map_value}}
});
{{/with}}
{{#with enum}}
{{> serialize_enum}}
{{/with}}
{{#with object}}
{{> serialize_object}}
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
