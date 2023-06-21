lazy_static! {
  static ref NAME: String = "wrap_info.py".to_string();
  static ref SOURCE: String = r#"# NOTE: This is an auto-generated file. All modifications will be overwritten.
# type: ignore
from __future__ import annotations

import json

from polywrap_manifest import WrapManifest

abi = json.loads("""
{{pretty abi}}
""")

manifest = WrapManifest.parse_obj({
    "name": "{{name}}",
    "type": "{{type}}",
    "version": "{{version}}",
    "abi": abi,
})
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
