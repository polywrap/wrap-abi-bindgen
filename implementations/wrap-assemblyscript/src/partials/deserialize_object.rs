lazy_static! {
  static ref NAME: String = "deserialize_object".to_string();
  static ref SOURCE: String = r#"{{#if required}}
const object = Types.{{detect_keyword type}}.read(reader);
{{else}}
let object: {{to_wasm (to_graphql_type)}} = null;
if (!reader.isNextNil()) {
  object = Types.{{detect_keyword type}}.read(reader);
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
