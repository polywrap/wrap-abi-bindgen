lazy_static! {
  static ref NAME: String = "entry.rs".to_string();
  static ref SOURCE: String = r#"{{#with moduleType}}
{{#if (array_has_length methods)}}
use crate::{
    {{#each methods}}
    {{to_lower name}}_wrapped{{#if (is_not_last @index ../methods)}},{{/if}}
    {{/each}}
};
{{/if}}
{{/with}}
use polywrap_wasm_rs::{
    abort,
    invoke,
    InvokeArgs,
};

#[no_mangle]
pub extern "C" fn _wrap_invoke(method_size: u32, args_size: u32, env_size: u32) -> bool {
    // Ensure the abort handler is properly setup
    abort::wrap_abort_setup();

    let args: InvokeArgs = invoke::wrap_invoke_args(method_size, args_size);
    let result: Vec<u8>;

    match args.method.as_str() {
        {{#with moduleType}}
        {{#each methods}}
        "{{name}}" => {
            result = {{to_lower name}}_wrapped(args.args.as_slice(), env_size);
        }
        {{/each}}
        {{/with}}
        _ => {
            invoke::wrap_invoke_error(format!("Could not find invoke function {}", args.method));
            return false;
        }
    };
    invoke::wrap_invoke_result(result);
    return true;
}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
