lazy_static! {
  static ref NAME: String = "mod.rs".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

pub mod types;
{{#if embeds}}
pub mod embeds;
{{/if}}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
