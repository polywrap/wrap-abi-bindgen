lazy_static! {
  static ref NAME: String = "interface_type/mod.rs".to_string();
  static ref SOURCE: String = r#"{{#with capabilities}}
{{#with getImplementations}}
{{#if enabled}}
use polywrap_wasm_rs::wrap_get_implementations;
{{/if}}
{{/with}}
{{/with}}

pub struct {{detect_keyword (to_upper namespace)}} {}

impl {{detect_keyword (to_upper namespace)}} {
  const uri: &'static str = "{{uri}}";

  {{#with capabilities}}
  {{#with getImplementations}}
  {{#if enabled}}
  pub fn get_implementations() -> Vec<String> {
    wrap_get_implementations(Self::uri)
  }
  {{/if}}
  {{/with}}
  {{/with}}
}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
