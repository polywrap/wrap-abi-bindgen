lazy_static! {
  static ref NAME: String = "module_type/mod.rs".to_string();
  static ref SOURCE: String = r#"{{#if (array_has_length methods)}}
pub mod wrapped;
pub use wrapped::{
    {{#each methods}}
    {{to_lower name}}_wrapped{{#if (is_not_last @index ../methods)}},{{/if}}
    Args{{#toUpper}}{{name}}{{/toUpper}}{{#if (is_not_last @index ../methods)}},{{/if}}
    {{/each}}
};
{{/if}}

pub mod module;
pub use module::*;
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
