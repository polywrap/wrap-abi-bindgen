lazy_static! {
  static ref NAME: String = "serialize_enum".to_string();
  static ref SOURCE: String = r#"{{#required}}
writer.writeInt32(item);
{{/required}}
{{^required}}
writer.writeOptionalInt32(item);
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
