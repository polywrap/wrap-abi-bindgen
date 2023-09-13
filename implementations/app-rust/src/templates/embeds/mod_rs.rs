lazy_static! {
  static ref NAME: String = "embeds/mod.rs".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

{{#each this}}
pub mod {{to_lower namespace}};
{{/each}}

use std::sync::Arc;
use polywrap::core::package::WrapPackage;
use polywrap::Uri;

pub fn packages() -> Vec<(Uri, Arc<dyn WrapPackage>)> {
    vec![
        {{#each this}}
        (
            Uri::try_from("{{uri}}").unwrap(),
            Arc::new({{to_lower namespace}}::lazy_loaded_wasm_package())
        ),
        {{/each}}
    ]
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
