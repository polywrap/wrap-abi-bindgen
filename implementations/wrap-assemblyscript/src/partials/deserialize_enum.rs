lazy_static! {
  static ref NAME: String = "deserialize_enum".to_string();
  static ref SOURCE: String = r#"{{#required}}
let value: Types.{{#detectKeyword}}{{type}}{{/detectKeyword}};
if (reader.isNextString()) {
  value = Types.get{{type}}Value(reader.readString());
} else {
  value = reader.readInt32();
  Types.sanitize{{type}}Value(value);
}
{{/required}}
{{^required}}
let value: Box<Types.{{#detectKeyword}}{{type}}{{/detectKeyword}}> | null;
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
{{/required}}
"#.to_string();
}

use super::Partial;

pub fn load() -> Partial {
    Partial {
        name: &*NAME,
        source: &*SOURCE
    }
}
