lazy_static! {
  static ref NAME: String = "serialize_array".to_string();
  static ref SOURCE: String = r#"{{#scalar}}
writer.write{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}(item);
{{/scalar}}
{{#array}}
writer.write{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}(item, (writer: Write, item: {{#item}}{{#toWasm}}{{toGraphQLType}}{{/toWasm}}{{/item}}): void => {
  {{> serialize_array}}
});
{{/array}}
{{#map}}
writer.write{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}(item, (writer: Write, key: {{#key}}{{#toWasm}}{{toGraphQLType}}{{/toWasm}}{{/key}}) => {
  writer.write{{#toMsgPack}}{{toGraphQLType}}{{/toMsgPack}}(key);
}, (writer: Write, value: {{#value}}{{#toWasm}}{{toGraphQLType}}{{/toWasm}}{{/value}}): void => {
  {{> serialize_map_value}}
});
{{/map}}
{{#enum}}
{{> serialize_enum}}
{{/enum}}
{{#object}}
{{> serialize_object}}
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
