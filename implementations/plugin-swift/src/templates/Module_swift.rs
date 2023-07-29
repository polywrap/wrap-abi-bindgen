lazy_static! {
  static ref NAME: String = "Module.swift".to_string();
  static ref SOURCE: String = r#"// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.

import PolywrapClient
import Foundation

{{#with moduleType}}
{{#each methods}}
public struct Args{{to_upper name}}: Codable {
    {{#each arguments}}
    var {{ name }}: {{to_swift (to_graphql_type this)}}
    {{/each}}
}

{{/each}}
{{/with}}

public protocol Plugin: PluginModule {
    {{#with moduleType}}
    {{#each methods}}
    func {{ name }}(_ args: Args{{to_upper name}}, _ env: {{#if env}}{{#with env}}Env{{#if required}}{{else}}?{{/if}}{{/with}}{{else}}VoidCodable?{{/if}}, _ invoker: Invoker) throws -> {{#with return}}{{to_swift (to_graphql_type this)}}{{/with}}
    {{#if (is_not_last @index ../methods)}}

    {{/if}}
    {{/each}}
    {{/with}}
}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE,
    }
}
