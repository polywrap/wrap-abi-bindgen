lazy_static! {
  static ref NAME: String = "index.ts".to_string();
  static ref SOURCE: String = r#"export * from "./types";
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
