use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(to_package_id: |value: Value| {
    let project_name = value.as_str().unwrap();
    _to_package_id(project_name)
});

// note: this transformation assumes the project name follows the constraints defined in polywrap manifest v0.4.0
pub fn _to_package_id(project_name: &str) -> String {
    let mut package_name = project_name
        .to_lowercase()
        .replace("-", "_")
        .trim_matches('_')
        .to_string();
    if package_name.starts_with(|c: char| c.is_numeric()) {
        package_name = format!("_{}", package_name);
    }
    format!("{}.wrap", package_name)
}

