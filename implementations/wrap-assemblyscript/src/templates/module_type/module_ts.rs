lazy_static! {
  static ref NAME: String = "module_type/module.ts".to_string();
  static ref SOURCE: String = r#"import * as Types from "..";

import {
  BigInt,
  BigNumber,
  Box,
  JSON,
} from "@polywrap/wasm-as";

export abstract class ModuleBase {
  {{#each methods}}
  abstract {{detect_keyword name}}(
    args: Types.Args_{{detect_keyword name}}{{#with env}},
    env: {{#if required}}Types.Env{{else}}Types.Env | null{{/if}}{{/with}}
  ): {{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}};
  {{#if (is_not_last @index ../methods)}}

  {{/if}}
  {{/each}}
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
