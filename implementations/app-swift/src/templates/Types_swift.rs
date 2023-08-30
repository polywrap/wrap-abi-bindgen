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
"#.to_string();
}
  
  use super::Template;
  
  pub fn load() -> Template {
      Template {
          name: &*NAME,
          source: &*SOURCE,
      }
  }
  