lazy_static! {
  static ref NAME: String = "imported/namespace/index.ts".to_string();
  static ref SOURCE: String = r#"export * from "./module";
export * from "./types";
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
