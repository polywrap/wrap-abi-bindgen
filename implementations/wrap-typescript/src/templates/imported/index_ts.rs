lazy_static! {
  static ref NAME: String = "imported/index.ts".to_string();
  static ref SOURCE: String = r#"{{#each namespaces}}
export * from "./{{this}}";
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
