use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(import_has_env: |importedEnvTypes: Value, namespace: Value| {
    let ns = namespace.as_str().expect("namespace must be a string");
    if let Some(importedEnvs) = importedEnvTypes.as_array() {
        importedEnvs.iter().any(|envValue| {
            let env = envValue.as_object().expect("expected env to be an object");
            let envNamespaceValue = env.get("namespace").expect("imported envs must have a namespace");
            let envNamespace = envNamespaceValue.as_str().expect("imported envs must have a namespace");
            envNamespace == ns
        })
    } else {
        false
    }
});