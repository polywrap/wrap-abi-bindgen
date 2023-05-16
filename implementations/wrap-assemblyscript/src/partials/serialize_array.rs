lazy_static! {
  static ref NAME: String = "serialize_array".to_string();
  static ref SOURCE: String = r#"{{#if scalar}}
writer.write{{to_msg_pack (to_graphql_type)}}(item);
{{/if}}
{{#if array}}
writer.write{{to_msg_pack (to_graphql_type)}}(item, (writer: Write, item: {{#if item}}{{to_wasm (to_graphql_type)}}{{/if}}): void => {
  {{> serialize_array}}
});
{{/if}}
{{#if map}}
writer.write{{to_msg_pack to_graphql_type}}(item, (writer: Write, key: {{#if key}}{{to_wasm (to_graphql_type)}}{{/if}}) => {
  writer.write{{to_msg_pack (to_graphql_type)}}(key);
}, (writer: Write, value: {{#if value}}{{to_wasm (to_graphql_type)}}{{/if}}): void => {
  {{> serialize_map_value}}
});
{{/if}}
{{#if enum}}
{{> serialize_enum}}
{{/if}}
{{#if object}}
{{> serialize_object}}
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
