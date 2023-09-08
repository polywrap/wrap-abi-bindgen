lazy_static! {
  static ref NAME: String = "prelude.rs".to_string();
  static ref SOURCE: String = r#"{{#each objectTypes}}
pub use super::{{detect_keyword (to_lower type)}}::{{detect_keyword (to_upper type)}};
{{/each}}
{{#each enumTypes}}
pub use super::{{detect_keyword (to_lower type)}}::{
    get_{{to_lower type}}_key,
    get_{{to_lower type}}_value,
    sanitize_{{to_lower type}}_value,
    {{detect_keyword (to_upper type)}}
};
{{/each}}
{{#with envType}}
pub use super::{{detect_keyword (to_lower type)}}::{{detect_keyword (to_upper type)}};
{{/with}}
{{#each importedObjectTypes}}
pub use super::imported::{{detect_keyword (to_lower type)}}::{{detect_keyword (to_upper type)}};
{{/each}}
{{#each importedEnumTypes}}
pub use super::imported::{{detect_keyword (to_lower type)}}::{
    get_{{to_lower type}}_key,
    get_{{to_lower type}}_value,
    sanitize_{{to_lower type}}_value,
    {{detect_keyword (to_upper type)}}
};
{{/each}}
{{#each importedEnvTypes}}
pub use super::imported::{{detect_keyword (to_lower type)}}::{{detect_keyword (to_upper type)}};
{{/each}}
{{#each importedModuleTypes}}
pub use super::imported::{{detect_keyword (to_lower type)}}::{{detect_keyword (to_upper type)}};
{{/each}}
{{#each interfaceTypes}}
pub use super::{{detect_keyword (to_lower namespace)}}::{{detect_keyword (to_upper namespace)}};
{{/each}}
{{#with moduleType}}
pub use super::{{detect_keyword (to_lower type)}}::{
    Module,
    ModuleTrait,
    {{#each methods}}
    {{to_lower name}}_wrapped,
    Args{{to_upper name}}{{#if (is_not_last @index ../methods)}},{{/if}}
    {{/each}}
};
{{/with}}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
