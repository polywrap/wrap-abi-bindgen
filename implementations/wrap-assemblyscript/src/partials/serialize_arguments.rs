lazy_static! {
  static ref NAME: String = "serialize_arguments".to_string();
  static ref SOURCE: String = r#"{{#if (array_has_length arguments)}}
  writer.writeMapLength({{array_length arguments}});
  {{else}}
  writer.writeMapLength(0);
  {{/if}}
  {{#each arguments}}
  writer.context().push("{{name}}", "{{to_wasm (to_graphql_type this)}}", "writing property");
  writer.writeString("{{name}}");
  {{#with scalar}}
  writer.write{{to_msgpack (to_graphql_type this)}}(args.{{detect_keyword name}});
  {{/with}}
  {{#with array}}
  writer.write{{to_msgpack (to_graphql_type this)}}(args.{{detect_keyword name}}, (writer: Write, item: {{#with item}}{{to_wasm (to_graphql_type this)}}{{/with}}): void => {
    {{> serialize_array}}
  });
  {{/with}}
  {{#with map}}
  writer.write{{to_msgpack (to_graphql_type this)}}(args.{{detect_keyword name}}, (writer: Write, key: {{#with key}}{{to_wasm (to_graphql_type this)}}{{/with}}) => {
    writer.write{{#with key}}{{to_msgpack (to_graphql_type this)}}{{/with}}(key);
  }, (writer: Write, value: {{#with value}}{{to_wasm (to_graphql_type this)}}{{/with}}): void => {
    {{> serialize_map_value}}
  });
  {{/with}}
  {{#with enum}}
  {{#if required}}
  writer.writeInt32(args.{{detect_keyword name}});
  {{else}}
  writer.writeOptionalInt32(args.{{detect_keyword name}});
  {{/if}}
  {{/with}}
  {{#with object}}
  {{#if required}}
  Types.{{detect_keyword type}}.write(writer, args.{{detect_keyword name}});
  {{else}}
  if (args.{{detect_keyword name}}) {
    Types.{{detect_keyword type}}.write(writer, args.{{detect_keyword name}} as Types.{{detect_keyword type}});
  } else {
    writer.writeNil();
  }
  {{/if}}
  {{/with}}
  writer.context().pop();
  {{/each}}
"#.to_string();
}

use super::Partial;

pub fn load() -> Partial {
    Partial {
        name: &*NAME,
        source: &*SOURCE
    }
}
