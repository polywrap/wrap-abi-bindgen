lazy_static! {
  static ref NAME: String = "deserialize_object".to_string();
  static ref SOURCE: String = r#"{{#required}}
const object = Types.{{#detectKeyword}}{{type}}{{/detectKeyword}}.read(reader);
{{/required}}
{{^required}}
let object: {{#toWasm}}{{toGraphQLType}}{{/toWasm}} = null;
if (!reader.isNextNil()) {
  object = Types.{{#detectKeyword}}{{type}}{{/detectKeyword}}.read(reader);
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
