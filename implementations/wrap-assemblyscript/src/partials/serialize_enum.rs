lazy_static! {
  static ref NAME: String = "serialize_enum".to_string();
  static ref SOURCE: String = r#"{{#if required}}
writer.writeInt32(item);
{{else}}
writer.writeOptionalInt32(item);
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
