lazy_static! {
  static ref NAME: String = "types.ts".to_string();
  static ref SOURCE: String = r#"// @ts-ignore
import * as Types from "./";

// @ts-ignore
import {
  CoreClient,
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
{{#each objectTypes}}

export interface {{detect_keyword type}} {
  {{#each properties}}
  {{name}}{{#if required}}{{else}}?{{/if}}: {{to_typescript (to_graphql_type this)}};
  {{/each}}
}
{{/each}}
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
{{#if (array_has_length importedObjectTypes)}}

/// Imported Objects START ///
{{#each importedObjectTypes}}

/* URI: "{{uri}}" */
export interface {{detect_keyword type}} {
  {{#each properties}}
  {{name}}{{#if required}}{{else}}?{{/if}}: {{to_typescript (to_graphql_type this)}};
  {{/each}}
}
{{/each}}

/// Imported Objects END ///
{{/if}}
{{#if (array_has_length importedEnumTypes)}}

/// Imported Enums START ///
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

/// Imported Enums END ///
{{/if}}
{{#if (array_has_length importedModuleTypes)}}

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
export const {{type}} = {
  {{#each methods}}
  {{name}}: async (
    args: {{../type}}_Args_{{name}},
    client: CoreClient,
    uri: string = "{{../uri}}"
  ): Promise<InvokeResult<{{#with return}}{{to_typescript (to_graphql_type this)}}{{/with}}>> => {
    return client.invoke<{{#with return}}{{to_typescript (to_graphql_type this)}}{{/with}}>({
      uri: Uri.from(uri),
      method: "{{name}}",
      args: (args as unknown) as Record<string, unknown>,
    });
  }{{#if (is_not_last @index ../methods)}},

  {{/if}}
  {{/each}}

};
{{/each}}

/// Imported Modules END ///
{{/if}}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
