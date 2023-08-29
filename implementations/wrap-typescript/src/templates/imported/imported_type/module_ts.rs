lazy_static! {
  static ref NAME: String = "imported/namespace/module.ts".to_string();
  static ref SOURCE: String = r#"import * as Types from "./types";
{{#each methods}}

export class Args_{{detect_keyword name}} {
  {{#each arguments}}
  {{detect_keyword name}}: {{to_wasm (to_graphql_type this) false}};
  {{/each}}
}
{{/each}}
{{#if isInterface}}

export class {{detect_keyword type}} {
  public static interfaceUri: string = "{{uri}}";

  public uri: string;

  constructor(uri: string) {
    this.uri = uri;
  }

  {{#each methods}}
  public {{name}}(
    args: Args_{{detect_keyword name}}
  ): Result<{{#with return}}{{to_wasm (to_graphql_type this) false}}{{/with}}> {
    return __wrap_subinvoke(this.uri, "{{name}}", args);
  }
  {{#if (is_not_last @index ../methods)}}

  {{/if}}
  {{/each}}
}
{{else}}

export class {{detect_keyword type}} {
  public static uri: string = "{{uri}}";

  {{#each methods}}
  public static {{name}}(
    args: Args_{{detect_keyword name}}
  ): Result<{{#with return}}{{to_wasm (to_graphql_type this) false}}{{/with}}> {
    return wrap_subinvoke("{{../uri}}", "{{name}}", args);
  }
  {{#if (is_not_last @index ../methods)}}

  {{/if}}
  {{/each}}
}
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
