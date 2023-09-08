lazy_static! {
  static ref NAME: String = "embed/wrap_info.rs".to_string();
  static ref SOURCE: String = r#"pub const WRAP_INFO: [u8; {{array_length wrapInfo}}] = [{{to_bytes wrapInfo}}];"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
