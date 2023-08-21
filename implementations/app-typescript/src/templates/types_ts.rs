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
{{#with importedModuleTypes}}
import { PolywrapClient } from "@polywrap/client-js";
{{/with}}

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
export class {{remove_module_suffix (detect_keyword type)}} {
  protected _defaultClient: CoreClient;
  protected _defaultUri: string;
  protected _defaultEnv?: Record<string, unknown>;

  constructor(
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ) {
    this._defaultClient = this._getClient(client);
    this._defaultEnv = this._getEnv(env);
    this._defaultUri = this._getUri(uri);
  }

  public get client(): CoreClient {
    return this._defaultClient;
  }

  public get uri(): string {
    return this._defaultUri;
  }

  public get env(): Record<string, unknown> | undefined {
    return this._defaultEnv;
  }

  private _getClient(client?: CoreClient): CoreClient {
    return client || this._defaultClient || this._getDefaultClient();
  }

  private _getUri(uri?: string): string {
    return uri || this._defaultUri || this._getDefaultUri();
  }

  private _getEnv(env?: Record<string, unknown>): Record<string, unknown> | undefined {
    return env || this._defaultEnv || this._getDefaultEnv();
  }

  protected _getDefaultClient(): CoreClient {
    return new PolywrapClient();
  }
  protected _getDefaultUri(): string {
    return "{{{uri}}}";
  }
  protected _getDefaultEnv(): Record<string, unknown> | undefined {
    return undefined;
  }

  {{#each methods}}
  async {{name}}(
    args: {{../type}}_Args_{{name}},
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<{{#with return}}{{to_typescript (to_graphql_type this)}}{{/with}}>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<{{#with return}}{{to_typescript (to_graphql_type this)}}{{/with}}>({
      uri: Uri.from(_uri),
      method: "{{name}}",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };
  {{#if (is_not_last @index ../methods)}}

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
