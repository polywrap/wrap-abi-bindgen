lazy_static! {
  static ref NAME: String = "deserialize_result".to_string();
  static ref SOURCE: String = r#"const reader = new ReadDecoder(buffer, context);

  {{#with return}}
  reader.context().push("{{name}}", "{{to_wasm (to_graphql_type this)}}", "reading function output");
  {{#with scalar}}
  const res: {{to_wasm (to_graphql_type this)}} = reader.read{{to_msgpack (to_graphql_type this)}}();
  {{/with}}
  {{#with array}}
  const res: {{to_wasm (to_graphql_type this)}} = reader.read{{to_msgpack (to_graphql_type this)}}((reader: Read): {{#with item}}{{to_wasm (to_graphql_type this)}}{{/with}} => {
    {{> deserialize_array}}
  });
  {{/with}}
  {{#with map}}
  const res: {{to_wasm (to_graphql_type this)}} = reader.read{{to_msgpack (to_graphql_type this)}}((reader: Read): {{#with key}}{{to_wasm (to_graphql_type this)}}{{/with}} => {
    return reader.read{{#with key}}{{to_msgpack (to_graphql_type this)}}{{/with}}();
  }, (reader: Read): {{#with value}}{{to_wasm (to_graphql_type this)}}{{/with}} => {
    {{> deserialize_map_value}}
  });
  {{/with}}
  {{#with enum}}
  {{> deserialize_enum}}
  const res: {{to_wasm (to_graphql_type this)}} =  value;
  {{/with}}
  {{#with object}}
  {{> deserialize_object}}
  const res: {{to_wasm (to_graphql_type this)}} =  object;
  {{/with}}
  {{/with}}
  reader.context().pop();

  return res;
"#.to_string();
}

use super::Partial;

pub fn load() -> Partial {
    Partial {
        name: &*NAME,
        source: &*SOURCE
    }
}
