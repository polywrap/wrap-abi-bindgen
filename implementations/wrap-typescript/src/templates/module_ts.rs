lazy_static! {
  static ref NAME: String = "module.ts".to_string();
  static ref SOURCE: String = r#"import { BigInt, BigNumber, JSONString, Bytes } from "./common";
import * as Types from "..";

{{#each methods}}
export class Args_{{detect_keyword name}} {
  {{#each arguments}}
  {{detect_keyword name}}: {{to_wasm (to_graphql_type this)}};
  {{/each}}
}
{{/each}}

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

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
