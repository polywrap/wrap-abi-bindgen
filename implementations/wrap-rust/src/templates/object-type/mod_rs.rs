lazy_static! {
  static ref NAME: String = "entry.ts".to_string();
  static ref SOURCE: String = r#"import {
  wrap_invoke_args,
  wrap_invoke_result,
  wrap_invoke_error,
  wrap_abort,
  InvokeArgs
} from "@polywrap/wasm-as";

{{#with moduleType}}
{{#if (array_has_length methods)}}
import {
  {{#each methods}}
  {{name}}Wrapped{{#if (is_not_last @index ../methods)}},{{/if}}
  {{/each}}
} from "./{{type}}/wrapped";
{{/if}}
{{/with}}

import { Module } from "../index";

export function _wrap_invoke(method_size: u32, args_size: u32, env_size: u32): bool {
  const module = new Module();
  const args: InvokeArgs = wrap_invoke_args(
    method_size,
    args_size
  );
  let result: ArrayBuffer;
  {{#with moduleType}}
  {{#each methods}}
  {{#if (is_not_first @index)}}else {{/if}}if (args.method == "{{name}}") {
    result = {{name}}Wrapped(module, args.args, env_size);
  }
  {{/each}}
  {{/with}}
  else {
    wrap_invoke_error(
      `Could not find invoke function "${args.method}"`
    );
    return false;
  }
  wrap_invoke_result(result);
  return true;
}

export function wrapAbort(
  msg: string | null,
  file: string | null,
  line: u32,
  column: u32
): void {
  wrap_abort(
    msg ? msg : "",
    file ? file : "",
    line,
    column
  );
}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
