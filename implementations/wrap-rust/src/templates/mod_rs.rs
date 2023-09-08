lazy_static! {
  static ref NAME: String = "mod.rs".to_string();
  static ref SOURCE: String = r#"pub mod entry;
pub mod prelude;
{{#each objectTypes}}
pub mod {{detect_keyword (to_lower type)}};
{{/each}}
{{#each enumTypes}}
pub mod {{detect_keyword (to_lower type)}};
{{/each}}
{{#with envType}}
pub mod {{detect_keyword (to_lower type)}};
{{/with}}
{{#if (array_has_length importedModuleTypes)}}
pub mod imported;
{{else}}{{#if (array_has_length importedObjectTypes)}}
pub mod imported;
{{else}}{{#if (array_has_length importedEnumTypes)}}
pub mod imported;
{{else}}{{#if (array_has_length importedEnvTypes)}}
pub mod imported;
{{/if}}{{/if}}{{/if}}{{/if}}
{{#each interfaceTypes}}
pub mod {{detect_keyword (to_lower namespace)}};
{{/each}}
{{#with moduleType}}
pub mod {{detect_keyword (to_lower type)}};
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
