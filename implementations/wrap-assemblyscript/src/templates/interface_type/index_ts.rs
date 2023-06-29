lazy_static! {
  static ref NAME: String = "interface_type/index.ts".to_string();
  static ref SOURCE: String = r#"{{#with capabilities}}
{{#with getImplementations}}
{{#if enabled}}
import {
  wrap_getImplementations
} from "@polywrap/wasm-as";
{{/if}}
{{/with}}
{{/with}}

export class {{detect_keyword namespace}} {
  static uri: string = "{{uri}}"

  {{#with capabilities}}
  {{#with getImplementations}}
  {{#if enabled}}
  public static getImplementations(): string[] {
    return wrap_getImplementations(this.uri);
  }
  {{/if}}
  {{/with}}
  {{/with}}
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
