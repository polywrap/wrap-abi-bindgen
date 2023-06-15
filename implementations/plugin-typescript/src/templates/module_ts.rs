lazy_static! {
  static ref NAME: String = "module.ts".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

// @ts-ignore
import * as Types from "./types";

// @ts-ignore
import { CoreClient, MaybeAsync } from "@polywrap/core-js";
import { PluginModule } from "@polywrap/plugin-js";
{{#with moduleType}}
{{#each methods}}

export interface Args_{{name}} {
  {{#each arguments}}
  {{name}}{{#if required}}{{else}}?{{/if}}: {{to_typescript (to_graphql_type this)}};
  {{/each}}
}
{{/each}}
{{/with}}

export abstract class Module<TConfig> extends PluginModule<TConfig{{#with envType}}, Types.Env{{/with}}> {
  {{#with moduleType}}
  {{#each methods}}
  abstract {{name}}(
    args: Args_{{name}},
    client: CoreClient,
    {{#if env}}{{#with env}}env{{#if required}}{{else}}?{{/if}}: Types.Env{{#if required}}{{else}} | null{{/if}}{{/with}}{{else}}env?: null{{/if}}
  ): MaybeAsync<{{#with return}}{{to_typescript (to_graphql_type this)}}{{/with}}>;
  {{#if (is_not_last @index ../methods)}}

  {{/if}}
  {{/each}}
  {{/with}}
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
