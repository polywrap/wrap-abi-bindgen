use handlebars::handlebars_helper;
use serde_json::Value;

handlebars_helper!(to_abstract_type: |val: Value| {
    let type_val = val.as_str().unwrap();
    _to_abstract_type(type_val)
});

pub fn _to_abstract_type(type_val: &str) -> String {
    if type_val.ends_with("_Module") {
        let new_type_val = type_val.strip_suffix("_Module").unwrap();
        return format!("Base{}", new_type_val);
    }
    return format!("{}", type_val);
}