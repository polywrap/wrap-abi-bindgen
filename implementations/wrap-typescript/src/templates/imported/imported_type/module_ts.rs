lazy_static! {
  static ref NAME: String = "imported/module.ts".to_string();
  static ref SOURCE: String = r#"import * as Types from "../..";
{{#each methods}}
export class Args_{{detect_keyword name}} {
  {{#each arguments}}
  {{detect_keyword name}}: {{to_wasm (to_graphql_type this)}};
  {{/each}}
}
{{#if (is_not_last @index ../methods)}}
{{/if}}
{{/each}}
{{/if}}

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
  ): Result<{{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}}, string> {
    const argsBuf = serialize{{name}}Args(args);
    const result = wrap_subinvokeImplementation(
      "{{../uri}}",
      this.uri,
      "{{name}}",
      argsBuf
    );

    if (result.isErr) {
      return Result.Err<{{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}}, string>(
        result.unwrapErr()
      );
    }

    return Result.Ok<{{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}}, string>(
      deserialize{{name}}Result(result.unwrap())
    );
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
  ): Result<{{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}}, string> {
    const argsBuf = serialize{{name}}Args(args);
    const result = wrap_subinvoke(
      "{{../uri}}",
      "{{name}}",
      argsBuf
    );

    if (result.isErr) {
      return Result.Err<{{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}}, string>(
        result.unwrapErr()
      );
    }

    return Result.Ok<{{#with return}}{{to_wasm (to_graphql_type this)}}{{/with}}, string>(
      deserialize{{name}}Result(result.unwrap())
    );
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
