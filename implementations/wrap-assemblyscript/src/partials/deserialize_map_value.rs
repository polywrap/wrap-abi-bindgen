lazy_static! {
  static ref NAME: String = "deserialize_map_value".to_string();
  static ref SOURCE: String = r#"{{#with scalar}}
return reader.read{{to_msgpack (to_graphql_type this)}}();
{{/with}}
{{#with array}}
return reader.read{{to_msgpack (to_graphql_type this)}}((reader: Read): {{#with item}}{{to_wasm (to_graphql_type this)}}{{/with}} => {
{{indent_partial "deserialize_array" 2}}
});
{{/with}}
{{#with map}}
return reader.read{{to_msgpack (to_graphql_type this)}}((reader: Read): {{#with key}}{{to_wasm (to_graphql_type this)}}{{/with}} => {
  return reader.read{{#with key}}{{to_msgpack (to_graphql_type this)}}{{/with}}();
}, (reader: Read): {{#with value}}{{to_wasm (to_graphql_type this)}}{{/with}} => {
{{indent_partial "deserialize_map_value" 2}}
});
{{/with}}
{{#with enum}}
{{> deserialize_enum}}
return value;
{{/with}}
{{#with object}}
{{> deserialize_object}}
return object;
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
