use handlebars::handlebars_helper;
use serde_json::Value;

handlebars_helper!(remove_module_suffix: |val: Value| {
    let type_val = val.as_str().unwrap();
    _remove_module_suffix(type_val)
});

pub fn _remove_module_suffix(type_val: &str) -> String {
    if type_val.ends_with("_Module") {
        let new_type_val = type_val.strip_suffix("_Module").unwrap();
        return format!("{}", new_type_val);
    }
    return format!("{}", type_val);
}