lazy_static! {
  static ref NAME: String = "imported/module_type/index.ts".to_string();
  static ref SOURCE: String = r#"import {
  wrap_subinvoke,
  wrap_subinvokeImplementation,
  Box,
  BigInt,
  BigNumber,
  JSON,
  Result
} from "@polywrap/wasm-as";
{{#if (array_has_length methods)}}
import {
  {{#each methods}}
  serialize{{name}}Args,
  deserialize{{name}}Result,
  Args_{{detect_keyword name}}{{#if (is_not_last @index ../methods)}},{{/if}}
  {{/each}}
} from "./serialization";
{{/if}}
import * as Types from "../..";

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
      "{{uri}}",
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
      "{{uri}}",
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
