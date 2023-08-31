lazy_static! {
  static ref NAME: String = "Modules.swift".to_string();
  static ref SOURCE: String = r#"// NOTE: This is an auto-generated file.
// All modifications will be overwritten.

import PolywrapClient
import Foundation

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
    static let uri: Uri = try! Uri("{{uri}}")

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
        if let uri = self.uri {
            return uri
        } else {
            let newUri = {{to_upper (remove_module_suffix type)}}.uri
            self.uri = newUri
            return newUri
        }
    }
    {{#if (import_has_env ../importedEnvTypes namespace)}}

    func getDefaultEnv() -> {{to_upper namespace}}Env? {
        return nil
    }
    {{/if}}
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
        let _env = env ?? self.env ?? getDefaultEnv()
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
