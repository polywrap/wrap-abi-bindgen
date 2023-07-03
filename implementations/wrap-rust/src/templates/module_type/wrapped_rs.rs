lazy_static! {
  static ref NAME: String = "module_type/wrapped.rs".to_string();
  static ref SOURCE: String = r#"{{#with moduleType}}
{{#if (array_has_length methods)}}
use polywrap_wasm_rs::{
  wrap_load_env
};

use crate::{
    {{#each methods}}
    Args{{to_upper name}},
    deserialize_{{to_lower name}}_args,
    serialize_{{to_lower name}}_result{{#if (is_not_last @index ../methods)}},{{/if}}
    {{/each}}
};
{{/if}}

use crate::module::{ModuleTrait, Module};
{{#with envType}}
use crate::Env;
{{/with}}

{{#each methods}}
pub fn {{to_lower name}}_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    {{#with env}}
    {{#if required}}
    if env_size == 0 {
        panic!("Environment is not set, and it is required by method '{{name}}'");
    }

    let env_buf = wrap_load_env(env_size);
    let env = Env::from_buffer(&env_buf).unwrap();

    {{/else}}
    let mut env: Option<Env> = None;
    if env_size > 0 {
      let env_buf = wrap_load_env(env_size);
      env = Some(Env::from_buffer(&env_buf).unwrap());
    }

    {{/if}}
    {{/with}}
    {{#if (array_has_length arguments)}}
    match deserialize_{{to_lower name}}_args(args) {
        Ok(args) => {
    {{/if}}
            let result = Module::{{detect_keyword (to_lower name)}}(Args{{to_upper name}} {
                {{#each arguments}}
                {{detect_keyword (to_lower name)}}: args.{{detect_keyword (to_lower name)}},
                {{/each}}
            }{{#with env}}, env{{/with}});
            match result {
                Ok(res) => {
                    serialize_{{to_lower name}}_result({{#with return}}&{{/with}}res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
    {{#if (array_has_length arguments)}}
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
    {{/if}}
}
{{#if (is_not_last @index ../methods)}}

{{/if}}
{{/each}}
{{/with}}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
