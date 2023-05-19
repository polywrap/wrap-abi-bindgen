lazy_static! {
  static ref NAME: String = "imported/index.ts".to_string();
  static ref SOURCE: String = r#"{{#each importedModuleTypes}}
export * from "./{{type}}";
{{/each}}
{{#each importedObjectTypes}}
export * from "./{{type}}";
{{/each}}
{{#each importedEnumTypes}}
export * from "./{{type}}";
{{/each}}
{{#each importedEnvTypes}}
export * from "./{{type}}";
{{/each}}
"#.to_string();
}

use super::super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
