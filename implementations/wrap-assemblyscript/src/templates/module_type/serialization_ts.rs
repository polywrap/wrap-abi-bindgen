lazy_static! {
  static ref NAME: String = "module_type/serialization.ts".to_string();
  static ref SOURCE: String = r#"{{> serialization_imports}}
import * as Types from "..";

{{#each methods}}
export class Args_{{detect_keyword name}} {
  {{#each arguments}}
  {{detect_keyword name}}: {{to_wasm (to_graphql_type this)}};
  {{/each}}
}

export function deserialize{{name}}Args(argsBuf: ArrayBuffer): Args_{{detect_keyword name}} {
  const context: Context = new Context("Deserializing module-type: {{name}} Args");
  {{#if (array_has_length arguments)}}
  const reader = new ReadDecoder(argsBuf, context);
  let numFields = reader.readMapLength();

  {{#each arguments}}
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
    {{#each arguments}}
    {{#if (is_not_first @index)}}else {{/if}}if (field == "{{name}}") {
      reader.context().push(field, "{{to_wasm (to_graphql_type this)}}", "type found, reading property");
      {{#with scalar}}
      _{{name}} = reader.read{{to_msgpack (to_graphql_type this)}}();
      {{/with }}
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
      {{> deserialize_object}}
      _{{name}} = object;
      {{/with}}
      {{#if required}}
      _{{name}}Set = true;
      {{/if}}
      reader.context().pop();
    }
    {{/each}}
    reader.context().pop();
  }{{#each arguments}}{{#if required}}
  {{#if object}}if (!_{{name}} || !_{{name}}Set) {
  {{else}}if (!_{{name}}Set) {
  {{/if}}
    throw new Error(reader.context().printWithContext("Missing required argument: '{{name}}: {{type}}'"));
  }{{/if}}{{/each}}{{/if}}
  return {
    {{#each arguments}}
    {{detect_keyword name}}: _{{name}}{{#if (is_not_last @index ../arguments)}},{{/if}}
    {{/each}}
  };
}

export function serialize{{name}}Args(args: Args_{{detect_keyword name}}): ArrayBuffer {
  const sizerContext: Context = new Context("Serializing (sizing) module-type: {{name}} Args");
  const sizer = new WriteSizer(sizerContext);
  write{{name}}Args(sizer, args);
  const buffer = new ArrayBuffer(sizer.length);
  const encoderContext: Context = new Context("Serializing (encoding) module-type: {{name}} Args");
  const encoder = new WriteEncoder(buffer, sizer, encoderContext);
  write{{name}}Args(encoder, args);
  return buffer;
}

export function write{{name}}Args(
  writer: Write,
  args: Args_{{detect_keyword name}}
): void {
  {{#if (array_has_length arguments)}}
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
}

export function serialize{{name}}Result(result: {{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}}): ArrayBuffer {
  const sizerContext: Context = new Context("Serializing (sizing) module-type: {{name}} Result");
  const sizer = new WriteSizer(sizerContext);
  write{{name}}Result(sizer, result);
  const buffer = new ArrayBuffer(sizer.length);
  const encoderContext: Context = new Context("Serializing (encoding) module-type: {{name}} Result");
  const encoder = new WriteEncoder(buffer, sizer, encoderContext);
  write{{name}}Result(encoder, result);
  return buffer;
}

export function write{{name}}Result(writer: Write, result: {{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}}): void {
  {{#with return}}
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
}

export function deserialize{{name}}Result(buffer: ArrayBuffer): {{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}} {
  const context: Context = new Context("Deserializing module-type: {{name}} Result");
  const reader = new ReadDecoder(buffer, context);

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
}
{{#if (is_not_last @index ../methods)}}

{{/if}}
{{/each}}
"#.to_string();
}

use super::super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
