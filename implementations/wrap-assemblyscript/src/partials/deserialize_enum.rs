lazy_static! {
  static ref NAME: String = "deserialize_enum".to_string();
  static ref SOURCE: String = r#"{{#if required}}
let value: Types.{{detect_keyword type}};
if (reader.isNextString()) {
  value = Types.get{{type}}Value(reader.readString());
} else {
  value = reader.readInt32();
  Types.sanitize{{type}}Value(value);
}
{{else}}
let value: Box<Types.{{detect_keyword type}}> | null;
if (!reader.isNextNil()) {
  if (reader.isNextString()) {
    value = Box.from(
      Types.get{{type}}Value(reader.readString())
    );
  } else {
    value = Box.from(
      reader.readInt32()
    );
    Types.sanitize{{type}}Value(value.unwrap());
  }
} else {
  value = null;
}
{{/if}}
"#.to_string();
}

use super::Partial;

pub fn load() -> Partial {
    Partial {
        name: &*NAME,
        source: &*SOURCE
    }
}
