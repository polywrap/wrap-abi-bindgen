lazy_static! {
  static ref NAME: String = "types.ts".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

// @ts-ignore
import * as Types from "./";

// @ts-ignore
import {
  CoreClient,{{#if (array_has_length interfaceTypes)}}
  Result,{{/if}}
  InvokeResult,
  Uri,
} from "@polywrap/core-js";

export type UInt = number;
export type UInt8 = number;
export type UInt16 = number;
export type UInt32 = number;
export type Int = number;
export type Int8 = number;
export type Int16 = number;
export type Int32 = number;
export type Bytes = Uint8Array;
export type BigInt = string;
export type BigNumber = string;
export type Json = string;
export type String = string;
export type Boolean = boolean;

/// Env START ///
{{#with envType}}
export interface {{detect_keyword type}} extends Record<string, unknown> {
  {{#each properties}}
  {{name}}{{#if required}}{{else}}?{{/if}}: {{to_typescript (to_graphql_type this)}};
  {{/each}}
}
{{/with}}
/// Env END ///

/// Objects START ///
{{#each objectTypes}}
export interface {{detect_keyword type}} {
  {{#each properties}}
  {{name}}{{#if required}}{{else}}?{{/if}}: {{to_typescript (to_graphql_type this)}};
  {{/each}}
}

{{/each}}
/// Objects END ///

/// Enums START ///
{{#each enumTypes}}
export enum {{type}}Enum {
  {{#each constants}}
  {{this}},
  {{/each}}
}

export type {{type}}String =
  {{#each constants}}
  | "{{this}}"
  {{/each}}

export type {{detect_keyword type}} = {{type}}Enum | {{type}}String;

{{/each}}
/// Enums END ///

/// Imported Objects START ///

{{#each importedObjectTypes}}
/* URI: "{{uri}}" */
export interface {{detect_keyword type}} {
  {{#each properties}}
  {{name}}{{#if required}}{{else}}?{{/if}}: {{to_typescript (to_graphql_type this)}};
  {{/each}}
}

{{/each}}
{{#each importedEnumTypes}}
/* URI: "{{uri}}" */
export enum {{type}}Enum {
  {{#each constants}}
  {{this}},
  {{/each}}
}

export type {{type}}String =
  {{#each constants}}
  | "{{this}}"
  {{/each}}

export type {{detect_keyword type}} = {{type}}Enum | {{type}}String;

{{/each}}
/// Imported Objects END ///

/// Imported Modules START ///

{{#each importedModuleTypes}}
{{#each methods}}
/* URI: "{{../uri}}" */
export interface {{../type}}_Args_{{name}} {
  {{#each arguments}}
  {{name}}{{#if required}}{{else}}?{{/if}}: {{to_typescript (to_graphql_type this)}};
  {{/each}}
}

{{/each}}
/* URI: "{{uri}}" */
{{#if isInterface}}
export class {{detect_keyword type}} {
  public static interfaceUri: string = "{{uri}}";
  public uri: Uri;

  constructor(uri: string) {
    this.uri = Uri.from(uri);
  }

  {{#each methods}}
  public async {{name}}(
    args: {{../type}}_Args_{{name}},
    client: CoreClient
  ): Promise<InvokeResult<{{#with return}}{{to_typescript (to_graphql_type this)}}{{/with}}>> {
    return client.invoke<{{#with return}}{{to_typescript (to_graphql_type this)}}{{/with}}>({
      uri: this.uri,
      method: "{{name}}",
      args: (args as unknown) as Record<string, unknown>,
    });
  }
  {{#if (is_not_last @index ../methods)}}

  {{/if}}
  {{/each}}
}

{{else}}
export const {{type}} = {
  {{#each methods}}
  {{name}}: async (
    args: {{../type}}_Args_{{name}},
    client: CoreClient
  ): Promise<InvokeResult<{{#with return}}{{to_typescript (to_graphql_type this)}}{{/with}}>> => {
    return client.invoke<{{#with return}}{{to_typescript (to_graphql_type this)}}{{/with}}>({
      uri: Uri.from("{{../uri}}"),
      method: "{{name}}",
      args: (args as unknown) as Record<string, unknown>,
    });
  }{{#if (is_not_last @index ../methods)}},

  {{/if}}
  {{/each}}

}

{{/if}}
{{/each}}
/// Imported Modules END ///
{{#each interfaceTypes}}

export class {{detect_keyword namespace}} {
  static uri: Uri = Uri.from("{{uri}}");

  {{#with capabilities}}
  {{#with getImplementations}}
  {{#if enabled}}
  public static async getImplementations(
    client: CoreClient
  ): Promise<Result<string[], Error>> {
    const impls = await client.getImplementations(this.uri, {});
    if (!impls.ok) {
      return { ok: false, error: impls.error};
    }

    return { ok: true, value: impls.value.map((impl) => (impl.uri))};
  }
  {{/if}}
  {{/with}}
  {{/with}}
}
{{/each}}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
