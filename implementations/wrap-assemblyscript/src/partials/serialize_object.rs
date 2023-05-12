lazy_static! {
  static ref NAME: String = "serialize_object".to_string();
  static ref SOURCE: String = r#"{{#required}}
Types.{{#detectKeyword}}{{type}}{{/detectKeyword}}.write(writer, item);
{{/required}}
{{^required}}
if (item) {
  Types.{{#detectKeyword}}{{type}}{{/detectKeyword}}.write(writer, item as Types.{{#detectKeyword}}{{type}}{{/detectKeyword}});
} else {
  writer.writeNil();
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
