lazy_static! {
  static ref NAME: String = "wrap.info.rs".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use polywrap_plugin::JSON::{from_value, json};
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};

pub fn get_manifest() -> WrapManifest {
  WrapManifest {
    name: "{{name}}".to_string(),
    type_: "{{type}}".to_string(),
    version: "{{version}}".to_string(),
    abi: from_value::<WrapManifestAbi>(json!({{abi}})).unwrap()
  }
}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
