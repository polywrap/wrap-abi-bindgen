lazy_static! {
  static ref NAME: String = "index.ts".to_string();
  static ref SOURCE: String = r#"{{#if moduleType}}
export * from "./module";
{{/if}}
export * from "./types";
{{#if hasImports}}export * from "./imported";{{/if}}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
