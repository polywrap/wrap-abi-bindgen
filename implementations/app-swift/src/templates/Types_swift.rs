lazy_static! {
    static ref NAME: String = "Types.swift".to_string();
    static ref SOURCE: String = r#"// NOTE: This is an auto-generated file.
// All modifications will be overwritten.

import PolywrapClient
import Foundation

// Objects START //

{{#each objectTypes}}
public struct {{detect_keyword (to_upper type)}}: Codable {
    {{#each properties}}
    var {{ name }}: {{to_swift (to_graphql_type this)}}
    {{/each}}
}

{{/each}}

// Objects END //

// Enums START //

{{#each enumTypes}}
public enum {{detect_keyword (to_upper type)}}: Int, Codable {
    {{#each constants}}
    case {{detect_keyword this}}
    {{/each}}
}

{{/each}}

// Enums END //

// Imported objects START //

{{#each importedObjectTypes}}
public struct {{detect_keyword (to_upper type)}}: Codable {
    {{#each properties}}
    var {{ name }}: {{to_swift (to_graphql_type this)}}
    {{/each}}
}

{{/each}}

// Imported objects END //

// Imported envs START //

{{#each importedEnvTypes}}
public struct {{detect_keyword (to_upper type)}}: Codable {
    {{#each properties}}
    var {{ name }}: {{to_swift (to_graphql_type this)}}
    {{/each}}
}
{{/each}}

// Imported envs END //

// Imported enums START //

{{#each importedEnumTypes}}
public enum {{detect_keyword (to_upper type)}}: Int, Codable {
    {{#each constants}}
    case {{detect_keyword this}}
    {{/each}}
}

{{/each}}

// Imported enums END //

// Imported modules START //

{{#each importedModuleTypes}}
{{#each methods}}
// URI: "{{../uri}}" //
public struct {{to_upper (remove_module_suffix ../type)}}Args{{to_upper name}}: Codable {
    {{#each arguments}}
    var {{ name }}: {{to_swift (to_graphql_type this)}}
    {{/each}}
}

{{/each}}
/* URI: "{{uri}}" */
class {{to_upper (remove_module_suffix type)}} {
    var client: PolywrapClient? = nil
    {{#if (import_has_env ../importedEnvTypes namespace)}}
    var env: {{to_upper namespace}}Env? = nil
    {{/if}}
    var uri: Uri? = nil

    init(client: PolywrapClient? = nil{{#if (import_has_env ../importedEnvTypes namespace)}}, env: {{to_upper namespace}}Env? = nil{{/if}}, uri: Uri? = nil) {
        self.client = client
        {{#if (import_has_env ../importedEnvTypes namespace)}}
        self.env = env
        {{/if}}
        self.uri = uri
    }

    func getDefaultClient() -> PolywrapClient {
        if let client = self.client {
            return client
        } else {
            let newClient = BuilderConfig().addSystemDefault().addWeb3Default().build()
            self.client = newClient
            return newClient
        }
    }

    func getDefaultUri() -> Uri {
        if (self.uri == nil) {
            self.uri = try? Uri("{{uri}}")
        }
        return self.uri!
    }
    {{#each methods}}

    func {{detect_keyword name}}(
        args: {{to_upper (remove_module_suffix ../type)}}Args{{to_upper name}},
        client: PolywrapClient? = nil,
        {{#if (import_has_env ../../importedEnvTypes ../namespace)}}
        env: {{to_upper ../namespace}}Env? = nil,
        {{/if}}
        uri: Uri? = nil
    ) throws -> {{#with return}}{{to_swift (to_graphql_type this)}}{{/with}} {
        let _client = client ?? self.client ?? getDefaultClient()
        let _uri = uri ?? self.uri ?? getDefaultUri()
        {{#if (import_has_env ../../importedEnvTypes ../namespace)}}
        let _env = env ?? self.env
        return try _client.invoke(
            uri: _uri,
            method: "{{name}}",
            args: args,
            env: _env
        )
        {{else}}
        return try _client.invoke(
            uri: _uri,
            method: "{{name}}",
            args: args
        )
        {{/if}}
    }
    {{/each}}
}

{{/each}}
// Imported Modules END //
"#.to_string();
}
  
  use super::Template;
  
  pub fn load() -> Template {
      Template {
          name: &*NAME,
          source: &*SOURCE,
      }
  }
  