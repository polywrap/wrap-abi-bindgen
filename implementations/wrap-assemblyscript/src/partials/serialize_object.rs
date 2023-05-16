lazy_static! {
  static ref NAME: String = "serialize_object".to_string();
  static ref SOURCE: String = r#"{{#if required}}
Types.{{detect_keyword type}}.write(writer, item);
{{else}}
if (item) {
  Types.{{detect_keyword type}}.write(writer, item as Types.{{detect_keyword type}});
} else {
  writer.writeNil();
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
