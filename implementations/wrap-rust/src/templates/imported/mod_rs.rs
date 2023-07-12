lazy_static! {
  static ref NAME: String = "imported/mod.rs".to_string();
  static ref SOURCE: String = r#"{{#each importedObjectTypes}}
pub mod {{to_lower type}};
pub use {{to_lower type}}::*;
{{/each}}
{{#each importedEnumTypes}}
pub mod {{to_lower type}};
pub use {{to_lower type}}::*;
{{/each}}
{{#each importedModuleTypes}}
pub mod {{to_lower type}};
pub use {{to_lower type}}::*;
{{/each}}
{{#each importedEnvTypes}}
pub mod {{to_lower type}};
pub use {{to_lower type}}::*;
{{/each}}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
