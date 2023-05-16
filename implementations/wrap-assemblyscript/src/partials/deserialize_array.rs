lazy_static! {
    static ref NAME: String = "deserialize_array".to_string();
    static ref SOURCE: String = r#"{{#if scalar}}
return reader.read{{to_msgpack (to_graphql_type)}}();
{{/if}}
{{#if array}}
return reader.read{{to_msgpack (to_graphql_type)}}((reader: Read): {{#if item}}{{to_wasm (to_graphql_type)}}{{/if}} => {
  {{> deserialize_array}}
});
{{/if}}
{{#if map}}
return reader.read{{to_msgpack (to_graphql_type)}}((reader: Read): {{#if key}}{{to_wasm (to_graphql_type)}}{{/if}} => {
  return reader.read{{#if key}}{{to_msgpack (to_graphql_type)}}{{/if}}();
}, (reader: Read): {{#if value}}{{to_wasm (to_graphql_type)}}{{/if}} => {
  {{> deserialize_map_value}}
});
{{/if}}
{{#if enum}}
{{> deserialize_enum}}
return value;
{{/if}}
{{#if object}}
{{> deserialize_object}}
return object;
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
