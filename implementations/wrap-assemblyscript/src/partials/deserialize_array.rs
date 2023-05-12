lazy_static! {
    static ref NAME: String = "deserialize_array".to_string();
    static ref SOURCE: String = r#"{{#scalar}}
return reader.read{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}();
{{/scalar}}
{{#array}}
return reader.read{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}((reader: Read): {{#item}}{{#toWasm}}{{toGraphQLType}}{{/toWasm}}{{/item}} => {
  {{> deserialize_array}}
});
{{/array}}
{{#map}}
return reader.read{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}((reader: Read): {{#key}}{{#toWasm}}{{toGraphQLType}}{{/toWasm}}{{/key}} => {
  return reader.read{{#key}}{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}{{/key}}();
}, (reader: Read): {{#value}}{{#toWasm}}{{toGraphQLType}}{{/toWasm}}{{/value}} => {
  {{> deserialize_map_value}}
});
{{/map}}
{{#enum}}
{{> deserialize_enum}}
return value;
{{/enum}}
{{#object}}
{{> deserialize_object}}
return object;
{{/object}}
"#.to_string();
}

use super::Partial;

pub fn load() -> Partial {
  Partial {
    name: &*NAME,
    source: &*SOURCE
  }
}
