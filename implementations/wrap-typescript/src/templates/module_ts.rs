lazy_static! {
  static ref NAME: String = "module.ts".to_string();
  static ref SOURCE: String = r#"import { BigInt, BigNumber, JSONString, Bytes } from "./common";
import * as Types from "./types";
{{#with moduleType}}
{{#each methods}}

export class Args_{{detect_keyword name}} {
  {{#each arguments}}
  {{detect_keyword name}}: {{to_wasm (to_graphql_type this) false}};
  {{/each}}
}
{{/each}}
{{/with}}
{{#with moduleType}}

export abstract class ModuleBase {
  {{#each methods}}
  abstract {{detect_keyword name}}(
    args: Args_{{detect_keyword name}}{{#with env}},
    env: {{#if required}}Types.Env{{else}}Types.Env | null{{/if}}{{/with}}
  ): {{#with return}}{{to_wasm (to_graphql_type this) false}}{{/with}};
  {{#if (is_not_last @index ../methods)}}

  {{/if}}
  {{/each}}
}
{{/with}}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
