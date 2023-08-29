lazy_static! {
  static ref NAME: String = "entry.ts".to_string();
  static ref SOURCE: String = r#"import { Module } from "../index";

new Module()[__wrap_method](__wrap_args);
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
