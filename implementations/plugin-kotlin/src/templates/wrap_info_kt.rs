lazy_static! {
  static ref NAME: String = "wrap.info.kt".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

package {{to_package_id name}}

import io.polywrap.core.wrap.WrapManifest
import io.polywrap.core.wrap.formats.wrap01.abi.Abi01
import io.polywrap.core.msgpack.msgPackDecode

val manifest = WrapManifest(
    name = "{{name}}",
    type = "{{type}}",
    version = "{{version}}",
    abi = msgPackDecode(Abi01.serializer(), {{to_kotlin_byte_array abi}}).getOrThrow()
    )
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
