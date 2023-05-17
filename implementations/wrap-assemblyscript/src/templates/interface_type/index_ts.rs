lazy_static! {
  static ref NAME: String = "interface_type/index.ts".to_string();
  static ref SOURCE: String = r#"{{#each capabilities}}
{{#if getImplementations}}
{{#if enabled}}
import {
  wrap_getImplementations
} from "@polywrap/wasm-as";
{{/if}}
{{/if}}
{{/each}}

export class {{detect_keyword namespace}} {
  static uri: string = "{{uri}}"

  {{#each capabilities}}
  {{#if getImplementations}}
  {{#if enabled}}
  public static getImplementations(): string[] {
    return wrap_getImplementations(this.uri);
  }
  {{/if}}
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
