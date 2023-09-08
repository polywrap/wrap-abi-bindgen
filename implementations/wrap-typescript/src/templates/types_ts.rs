lazy_static! {
  static ref NAME: String = "types.ts".to_string();
  static ref SOURCE: String = r#"import { BigInt, BigNumber, JSONString, Bytes } from "./common";
{{#if hasImports}}
import {
  {{#each importedObjectTypes}}
  {{detect_keyword type}},
  {{/each}}
  {{#each importedEnvTypes}}
  {{detect_keyword type}},
  {{/each}}
  {{#each importedEnumTypes}}
  {{detect_keyword type}},
  {{/each}}
} from "./imported";
{{/if}}
{{#each enumTypes}}

export enum {{detect_keyword type}} {
  {{#each constants}}
  {{detect_keyword this}},
  {{/each}}
}
{{/each}}
{{#each objectTypes}}

export class {{detect_keyword type}} {
  {{#each properties}}
  {{detect_keyword name}}: {{to_wasm (to_graphql_type this) true}};
  {{/each}}
}
{{/each}}
{{#with envType}}

export class {{detect_keyword type}} {
  {{#each properties}}
  {{detect_keyword name}}: {{to_wasm (to_graphql_type this) true}};
  {{/each}}
}
{{/with}}
{{#each interfaceTypes}}

export class {{detect_keyword namespace}} {
  static uri: string = "{{uri}}"

  {{#with capabilities}}
  {{#with getImplementations}}
  {{#if enabled}}
  public static getImplementations(): string[] {
    return __wrap_getImplementations(this.uri);
  }
  {{/if}}
  {{/with}}
  {{/with}}
}
{{/each}}

{{#if hasImports}}
export * from "./imported";
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
