lazy_static! {
  static ref NAME: String = "Module.swift".to_string();
  static ref SOURCE: String = r#"import PolywrapClient
  
{{#with moduleType}}
{{#each methods}}
pub struct Args{{to_upper name}} {
    {{#each arguments}}
    {{#with scalar}}{{serde_annotate_if_bytes type}}{{/with}}{{serde_rename_if_case_mismatch name}}var {{detect_keyword (to_lower name)}}: {{to_swift (to_graphql_type this)}},
    {{/each}}

    public init(
        {{#each arguments}}
        {{#with scalar}}{{serde_annotate_if_bytes type}}{{/with}}{{serde_rename_if_case_mismatch name}}{{detect_keyword (to_lower name)}}: {{to_swift (to_graphql_type this)}},
        {{/each}}
    ) {
        {{#each arguments}}
        self.{{detect_keyword (to_lower name)}} = {{detect_keyword (to_lower name)}}
        {{/each}}
    }
}

  {{/each}}

protocol Plugin: PluginModule {
    {{#with moduleType}}
    {{#each methods}}
    func {{detect_keyword (to_lower name)}}(_ args: Args{{to_upper name}}, _ env: {{#if env}}{{else}}VoidCodable?{{/if}}, _ invoker: Invoker) throws -> {{#with return}}{{to_swift (to_graphql_type this)}}{{/with}}
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
