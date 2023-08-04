lazy_static! {
  static ref NAME: String = "__init__.py".to_string();
  static ref SOURCE: String = r#"from .types import *
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
