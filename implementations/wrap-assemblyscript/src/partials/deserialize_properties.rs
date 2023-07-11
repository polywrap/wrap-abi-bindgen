lazy_static! {
  static ref NAME: String = "deserialize_properties".to_string();
  static ref SOURCE: String = r#"let numFields = reader.readMapLength();

{{#each properties}}
{{#if object}}
{{#with object}}
{{#if required}}
let _{{name}}: {{to_wasm (to_graphql_type this)}} | null = null;
{{else}}
let _{{name}}: {{to_wasm (to_graphql_type this)}} = {{to_wasm_init (to_graphql_type this)}};
{{/if}}
{{/with}}
{{else}}
let _{{name}}: {{to_wasm (to_graphql_type this)}} = {{to_wasm_init (to_graphql_type this)}};
{{/if}}
{{#if required}}
let _{{name}}Set: bool = false;
{{/if}}
{{/each}}

while (numFields > 0) {
  numFields--;
  const field = reader.readString();

  reader.context().push(field, "unknown", "searching for property type");
  {{#each properties}}
  {{#if (is_not_first @index)}}else {{/if}}if (field == "{{name}}") {
    reader.context().push(field, "{{to_wasm (to_graphql_type this)}}", "type found, reading property");
    {{#with scalar}}
    _{{name}} = reader.read{{to_msgpack (to_graphql_type this)}}();
    {{/with}}
    {{#with array}}
    _{{name}} = reader.read{{to_msgpack (to_graphql_type this)}}((reader: Read): {{#with item}}{{to_wasm (to_graphql_type this)}}{{/with}} => {
{{indent_partial "deserialize_array" 6}}
    });
    {{/with}}
    {{#with map}}
    _{{name}} = reader.read{{to_msgpack (to_graphql_type this)}}((reader: Read): {{#with key}}{{to_wasm (to_graphql_type this)}}{{/with}} => {
      return reader.read{{#with key}}{{to_msgpack (to_graphql_type this)}}{{/with}}();
    }, (reader: Read): {{#with value}}{{to_wasm (to_graphql_type this)}}{{/with}} => {
{{indent_partial "deserialize_map_value" 6}}
    });
    {{/with}}
    {{#with enum}}
{{indent_partial "deserialize_enum" 4}}
    _{{name}} = value;
    {{/with}}
    {{#with object}}
{{indent_partial "deserialize_object" 4}}
    _{{name}} = object;
    {{/with}}
    {{#if required}}
    _{{name}}Set = true;
    {{/if}}
    reader.context().pop();
  }
  {{/each}}
  reader.context().pop();
}

{{#each properties}}
{{#if required}}
{{#if object}}
{{#with object}}
if (!_{{name}} || !_{{name}}Set) {
{{/with}}
{{else}}
if (!_{{name}}Set) {
{{/if}}
  throw new Error(reader.context().printWithContext("Missing required property: '{{name}}: {{type}}'"));
}
{{/if}}
{{/each}}

return {
  {{#each properties}}
  {{detect_keyword name}}: _{{name}}{{#if (is_not_last @index ../properties)}},{{/if}}
  {{/each}}
};
"#.to_string();
}

use super::Partial;

pub fn load() -> Partial {
    Partial {
        name: &*NAME,
        source: &*SOURCE
    }
}
