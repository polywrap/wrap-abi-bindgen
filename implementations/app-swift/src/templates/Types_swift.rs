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
public enum {{detect_keyword (to_upper type)}}: String, Codable {
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
public enum {{detect_keyword (to_upper type)}}: String, Codable {
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
public struct Args{{to_upper name}}: Codable {
    {{#each arguments}}
    var {{ name }}: {{to_swift (to_graphql_type this)}}
    {{/each}}
}

{{/each}}
/* URI: "{{uri}}" */
class {{to_upper (remove_module_suffix type)}} {
    var client: Invoker? = nil
    var env: {{to_upper namespace}}Env? = nil
    var uri: Uri? = nil

    init(client: Invoker? = nil, env: {{to_upper namespace}}Env? = nil, uri: Uri? = nil) {
        self.client = client
        self.env = env
        self.uri = uri
    }

    func getDefaultClient() -> Invoker {
        if (self.client == nil) {
            self.client = BuilderConfig().addSystemDefault().addWeb3Default().build()
        }
        return self.client!
    }

    func getDefaultUri() -> Uri {
        if (self.uri == nil) {
            self.uri = try Uri("{{uri}}")
        }
        return self.uri!
    }
    {{#each methods}}

    func {{detect_keyword name}}(
        args: {{to_upper ../type}}Args{{to_upper name}},
        client: Invoker? = nil,
        env: {{to_upper ../namespace}}Env? = nil,
        uri: Uri? = nil
    ) throws -> {{#with return}}{{to_swift (to_graphql_type this)}}{{/with}} {
        let _client = client ?? self.client ?? getDefaultClient()
        let _env = env ?? self.env
        let _uri = uri ?? self.uri ?? getDefaultUri()
        return try _client.invoke(
            uri: _uri,
            method: "{{name}}",
            args: args,
            env: _env
        )
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
  