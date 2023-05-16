lazy_static! {
  static ref NAME: String = "object_type/serialization.ts".to_string();
  static ref SOURCE: String = r#"{{> serialization_imports}}
import { {{detect_keyword type}} } from "./";
import * as Types from "..";

export function serialize{{type}}(type: {{detect_keyword type}}): ArrayBuffer {
  const sizerContext: Context = new Context("Serializing (sizing) object-type: {{type}}");
  const sizer = new WriteSizer(sizerContext);
  write{{type}}(sizer, type);
  const buffer = new ArrayBuffer(sizer.length);
  const encoderContext: Context = new Context("Serializing (encoding) object-type: {{type}}");
  const encoder = new WriteEncoder(buffer, sizer, encoderContext);
  write{{type}}(encoder, type);
  return buffer;
}

export function write{{type}}(writer: Write, type: {{detect_keyword type}}): void {
  {{#if (array_has_length properties)}}
  writer.writeMapLength({{array_length properties}});
  {{else}}
  writer.writeMapLength(0);
  {{/if}}
  {{#each properties}}
  writer.context().push("{{name}}", "{{to_wasm (to_graphql_type this)}}", "writing property");
  writer.writeString("{{name}}");
  {{#with scalar}}
  writer.write{{to_msgpack (to_graphql_type this)}}(type.{{detect_keyword name}});
  {{/with}}
  {{#with array}}
  writer.write{{to_msgpack (to_graphql_type this)}}(type.{{detect_keyword name}}, (writer: Write, item: {{#with item}}{{to_wasm (to_graphql_type this)}}{{/with}}): void => {
    {{> serialize_array}}
  });
  {{/with}}
  {{#with map}}
  writer.write{{to_msgpack (to_graphql_type this)}}(type.{{detect_keyword name}}, (writer: Write, key: {{#with key}}{{to_wasm (to_graphql_type this)}}{{/with}}) => {
    writer.write{{#with key}}{{to_msgpack (to_graphql_type this)}}{{/with}}(key);
  }, (writer: Write, value: {{#with value}}{{to_wasm (to_graphql_type this)}}{{/with}}): void => {
    {{> serialize_map_value}}
  });
  {{/with}}
  {{#with object}}
  {{#if required}}
  Types.{{detect_keyword type}}.write(writer, type.{{detect_keyword name}});
  {{else}}
  if (type.{{detect_keyword name}}) {
    Types.{{detect_keyword type}}.write(writer, type.{{detect_keyword name}} as Types.{{detect_keyword type}});
  } else {
    writer.writeNil();
  }
  {{/if}}
  {{/with}}
  {{#with enum}}
  {{#if required}}
  writer.writeInt32(type.{{detect_keyword name}});
  {{else}}
  writer.writeOptionalInt32(type.{{detect_keyword name}});
  {{/if}}
  {{/with}}
  writer.context().pop();
  {{/each}}
}

export function deserialize{{type}}(buffer: ArrayBuffer): {{detect_keyword type}} {
  const context: Context = new Context("Deserializing object-type {{type}}");
  const reader = new ReadDecoder(buffer, context);
  return read{{type}}(reader);
}

export function read{{type}}(reader: Read): {{detect_keyword type}} {
  let numFields = reader.readMapLength();

  {{#each properties}}
  {{#if object}}{{#with object}}
  {{#if required}}
  let _{{name}}: {{to_wasm (to_graphql_type this)}} | null = null;
  {{else}}
  let _{{name}}: {{to_wasm (to_graphql_type this)}} = {{to_wasm_init (to_graphql_type this)}};
  {{/if}}
  {{/with}}{{else}}
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
        {{> deserialize_array}}
      });
      {{/with}}
      {{#with map}}
      _{{name}} = reader.read{{to_msgpack (to_graphql_type this)}}((reader: Read): {{#with key}}{{to_wasm (to_graphql_type this)}}{{/with}} => {
        return reader.read{{#with key}}{{to_msgpack (to_graphql_type this)}}{{/with}}();
      }, (reader: Read): {{#with value}}{{to_wasm (to_graphql_type this)}}{{/with}} => {
        {{> deserialize_map_value}}
      });
      {{/with}}
      {{#with enum}}
      {{> deserialize_enum}}
      _{{name}} = value;
      {{/with}}
      {{#with object}}
      {{> deserialize_object }}
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
}
"#.to_string();
}

use super::super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
