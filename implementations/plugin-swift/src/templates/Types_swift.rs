lazy_static! {
    static ref NAME: String = "Types.swift".to_string();
    static ref SOURCE: String = r#"// NOTE: This is an auto-generated file.
// All modifications will be overwritten.

import PolywrapClient
import Foundation

// Env START //

{{#with envType}}
public struct {{detect_keyword (to_upper type)}}: Codable {
    {{#each properties}}
    var {{ name }}: {{to_swift (to_graphql_type this)}},
    {{/each}}
}
{{/with}}

// Env END //

// Objects START //

{{#each objectTypes}}
public struct {{detect_keyword (to_upper type)}}: Codable {
    {{#each properties}}
    var {{ name }}: {{to_swift (to_graphql_type this)}},
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
    var {{ name }}: {{to_swift (to_graphql_type this)}},
    {{/each}}
}

{{/each}}

// Imported objects END //

// Imported envs START //

{{#each importedEnvTypes}}
public struct {{detect_keyword (to_upper type)}}: Codable {
    {{#each properties}}
    var {{ name }}: {{to_swift (to_graphql_type this)}},
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
    var {{ name }}: {{to_swift (to_graphql_type this)}},
    {{/each}}
}

{{/each}}
{{#if isInterface}}
public class {{detect_keyword (to_upper type)}} {
    var uri: Uri

    public init(uri: Uri) {
        self.uri = uri
    }
    {{#each methods}}
    func {{name}}(
        _ args: {{to_upper ../type}}Args{{to_upper name}},
        _ invoker: Invoker
    ) throws -> {{#with return}}{{to_swift (to_graphql_type this)}}{{/with}} {
        let serializedArgs = try encode(value: args)
        return try invoker.invokeRaw(
            uri: try Uri("{{../uri}}"),
            method: "{{name}}",
            args: serializedArgs,
            env: nil,
        )
    }
    {{#if (is_not_last @index ../methods)}}

    {{/if}}
    {{/each}}

    
}
{{else}}
public class {{detect_keyword (to_upper type)}} {
    public init() {}
    {{#each methods}}

    func {{name}}(
        _ args: {{to_upper ../type}}Args{{to_upper name}},
        _ invoker: Invoker
    ) throws -> {{#with return}}{{to_swift (to_graphql_type this)}}{{/with}} {
        let serializedArgs = try encode(value: args)
        return try invoker.invokeRaw(
            uri: try Uri("{{../uri}}"),
            method: "{{name}}",
            args: serializedArgs,
            env: nil,
        )
    }
    {{#if (is_not_last @index ../methods)}}

    {{/if}}
    {{/each}}
}
{{/if}}
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
  