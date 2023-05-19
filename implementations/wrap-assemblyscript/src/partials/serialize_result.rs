lazy_static! {
  static ref NAME: String = "serialize_result".to_string();
  static ref SOURCE: String = r#"{{#with return}}
  writer.context().push("{{name}}", "{{to_wasm (to_graphql_type this)}}", "writing property");
  {{#with scalar}}
  writer.write{{to_msgpack (to_graphql_type this)}}(result);
  {{/with}}
  {{#with array}}
  writer.write{{to_msgpack (to_graphql_type this)}}(result, (writer: Write, item: {{#with item}}{{to_wasm (to_graphql_type this)}}{{/with}}): void => {
    {{> serialize_array}}
  });
  {{/with}}
  {{#with map}}
  writer.write{{to_msgpack (to_graphql_type this)}}(result, (writer: Write, key: {{#with key}}{{to_wasm (to_graphql_type this)}}{{/with}}) => {
    writer.write{{#with key}}{{to_msgpack (to_graphql_type this)}}{{/with}}(key);
  }, (writer: Write, value: {{#with value}}{{to_wasm (to_graphql_type this)}}{{/with}}): void => {
    {{> serialize_map_value}}
  });
  {{/with}}
  {{#with enum}}
  {{#if required}}
  writer.writeInt32(result);
  {{else}}
  writer.writeOptionalInt32(result);
  {{/if}}
  {{/with}}
  {{#with object}}
  {{#if required}}
  Types.{{detect_keyword type}}.write(writer, result);
  {{else}}
  if (result) {
    Types.{{detect_keyword type}}.write(writer, result as Types.{{detect_keyword type}});
  } else {
    writer.writeNil();
  }
  {{/if}}
  {{/with}}
  writer.context().pop();
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
