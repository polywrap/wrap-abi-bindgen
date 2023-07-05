lazy_static! {
  static ref NAME: String = "mod.rs".to_string();
  static ref SOURCE: String = r#"pub mod entry;
{{#each objectTypes}}
pub mod {{detect_keyword (to_lower type)}};
pub use {{detect_keyword (to_lower type)}}::{{detect_keyword (to_upper type)}};
{{/each}}
{{#each enumTypes}}
pub mod {{detect_keyword (to_lower type)}};
pub use {{detect_keyword (to_lower type)}}::{
    get_{{to_lower type}}_key,
    get_{{to_lower type}}_value,
    sanitize_{{to_lower type}}_value,
    {{detect_keyword (to_upper type)}}
};
{{/each}}
{{#with envType}}
pub mod {{detect_keyword (to_lower type)}};
pub use {{detect_keyword (to_lower type)}}::{{detect_keyword (to_upper type)}};
{{/with}}
{{#if hasImports}}
pub mod imported;
{{/if}}
{{#each importedObjectTypes}}
pub use imported::{{detect_keyword (to_lower type)}}::{{detect_keyword (to_upper type)}};
{{/each}}
{{#each importedEnumTypes}}
pub use imported::{{detect_keyword (to_lower type)}}::{
    get_{{to_lower type}}_key,
    get_{{to_lower type}}_value,
    sanitize_{{to_lower type}}_value,
    {{detect_keyword (to_upper type)}}
};
{{/each}}
{{#each importedEnvTypes}}
pub use imported::{{detect_keyword (to_lower type)}}::{{detect_keyword (to_upper type)}};
{{/each}}
{{#each importedModuleTypes}}
pub use imported::{{detect_keyword (to_lower type)}}::{{detect_keyword (to_upper type)}};
{{/each}}
{{#each interfaceTypes}}
pub mod {{detect_keyword (to_lower namespace)}};
pub use {{detect_keyword (to_lower namespace)}}::{{detect_keyword (to_upper namespace)}};
{{/each}}
{{#with moduleType}}
pub mod {{detect_keyword (to_lower type)}};
pub use {{detect_keyword (to_lower type)}}::{
    Module,
    ModuleTrait,
    {{#each methods}}
    {{to_lower name}}_wrapped,
    Args{{to_upper name}}{{#if (is_not_last @index ../methods)}},{{/if}}
    {{/each}}
};
{{/with}}

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
