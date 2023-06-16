lazy_static! {
  static ref NAME: String = "module_type/wrapped.ts".to_string();
  static ref SOURCE: String = r#"{{#if (array_has_length methods)}}
import { wrap_load_env } from "@polywrap/wasm-as";
import {
  {{#each methods}}
  deserialize{{name}}Args,
  serialize{{name}}Result{{#if (is_not_last @index ../methods)}},{{/if}}
  {{/each}}
} from "./serialization";
{{/if}}
import { ModuleBase } from "./module";
import * as Types from "..";

{{#each methods}}
export function {{name}}Wrapped(module: ModuleBase, argsBuf: ArrayBuffer, env_size: u32): ArrayBuffer {
  {{#with env}}
  {{#if required}}
  if (env_size == 0) {
    throw new Error("Environment is not set, and it is required by method '{{name}}'")
  }
  
  const envBuf = wrap_load_env(env_size);
  const env = Types.Env.fromBuffer(envBuf);
  {{else}}
  let env: Types.Env | null = null;
  if (env_size > 0) {
    const envBuf = wrap_load_env(env_size);
    env = Types.Env.fromBuffer(envBuf);
  }
  {{/if}}
  {{/with}}
  {{#if (array_has_length arguments)}}
  const args = deserialize{{name}}Args(argsBuf);
  {{/if}}

  const result = module.{{detect_keyword name}}({{#if (array_has_length arguments)}}
    {
      {{#each arguments}}
      {{detect_keyword name}}: args.{{detect_keyword name}}{{#if (is_not_last @index ../arguments)}},{{/if}}
      {{/each}}
    }{{else}}{}{{/if}}{{#with env}},
    env{{/with}}
  );
  return serialize{{name}}Result(result);
}
{{#if (is_not_last @index ../methods)}}

{{/if}}
{{/each}}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
