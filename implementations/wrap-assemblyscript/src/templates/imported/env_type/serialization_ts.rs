lazy_static! {
  static ref NAME: String = "imported/env_type/serialization.ts".to_string();
  static ref SOURCE: String = r#"{{> serialization_imports}}
import { {{detect_keyword type}} } from "./";
import * as Types from "../..";

export function serialize{{type}}(type: {{detect_keyword type}}): ArrayBuffer {
  const sizerContext: Context = new Context("Serializing (sizing) imported env-type: {{type}}");
  const sizer = new WriteSizer(sizerContext);
  write{{type}}(sizer, type);
  const buffer = new ArrayBuffer(sizer.length);
  const encoderContext: Context = new Context("Serializing (encoding) imported env-type: {{type}}");
  const encoder = new WriteEncoder(buffer, sizer, encoderContext);
  write{{type}}(encoder, type);
  return buffer;
}

export function write{{type}}(writer: Write, type: {{detect_keyword type}}): void {
{{indent_partial "serialize_properties" 2}}
}

export function deserialize{{type}}(buffer: ArrayBuffer): {{detect_keyword type}} {
  const context: Context = new Context("Deserializing imported env-type {{type}}");
  const reader = new ReadDecoder(buffer, context);
  return read{{type}}(reader);
}

export function read{{type}}(reader: Read): {{detect_keyword type}} {
{{indent_partial "deserialize_properties" 2}}
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
