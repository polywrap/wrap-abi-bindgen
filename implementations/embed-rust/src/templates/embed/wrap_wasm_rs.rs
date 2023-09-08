lazy_static! {
  static ref NAME: String = "embed/wrap_wasm.rs".to_string();
  static ref SOURCE: String = r#"pub const WRAP_WASM: [u8; {{array_length wrapWasm}}] = [{{to_bytes wrapWasm}}];"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
