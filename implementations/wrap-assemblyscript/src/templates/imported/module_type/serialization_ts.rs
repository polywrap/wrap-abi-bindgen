lazy_static! {
  static ref NAME: String = "imported/module_type/serialization.ts".to_string();
  static ref SOURCE: String = r#"{{#if (array_has_length methods)}}
{{> serialization_imports}}
import * as Types from "../..";

{{#each methods}}
export class Args_{{detect_keyword name}} {
  {{#each arguments}}
  {{detect_keyword name}}: {{to_wasm (to_graphql_type this)}};
  {{/each}}
}

export function deserialize{{name}}Args(argsBuf: ArrayBuffer): Args_{{detect_keyword name}} {
  const context: Context = new Context("Deserializing imported module-type: {{name}} Args");
{{indent_partial "deserialize_arguments" 2}}
}

export function serialize{{name}}Args(args: Args_{{detect_keyword name}}): ArrayBuffer {
  const sizerContext: Context = new Context("Serializing (sizing) imported module-type: {{name}} Args");
  const sizer = new WriteSizer(sizerContext);
  write{{name}}Args(sizer, args);
  const buffer = new ArrayBuffer(sizer.length);
  const encoderContext: Context = new Context("Serializing (encoding) imported module-type: {{name}} Args");
  const encoder = new WriteEncoder(buffer, sizer, encoderContext);
  write{{name}}Args(encoder, args);
  return buffer;
}

export function write{{name}}Args(
  writer: Write,
  args: Args_{{detect_keyword name}}
): void {
{{indent_partial "serialize_arguments" 2}}
}

export function serialize{{name}}Result(result: {{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}}): ArrayBuffer {
  const sizerContext: Context = new Context("Serializing (sizing) imported module-type: {{name}} Result");
  const sizer = new WriteSizer(sizerContext);
  write{{name}}Result(sizer, result);
  const buffer = new ArrayBuffer(sizer.length);
  const encoderContext: Context = new Context("Serializing (encoding) imported module-type: {{name}} Result");
  const encoder = new WriteEncoder(buffer, sizer, encoderContext);
  write{{name}}Result(encoder, result);
  return buffer;
}

export function write{{name}}Result(writer: Write, result: {{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}}): void {
{{indent_partial "serialize_result" 2}}
}

export function deserialize{{name}}Result(buffer: ArrayBuffer): {{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}} {
  const context: Context = new Context("Deserializing imported module-type: {{name}} Result");
{{indent_partial "deserialize_result" 2}}
}
{{#if (is_not_last @index ../methods)}}

{{/if}}
{{/each}}
{{/if}}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
