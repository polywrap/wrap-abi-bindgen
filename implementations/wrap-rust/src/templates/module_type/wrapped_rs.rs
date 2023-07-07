lazy_static! {
  static ref NAME: String = "module_type/wrapped.rs".to_string();
  static ref SOURCE: String = r#"{{#with moduleType}}
{{#if (array_has_length methods)}}
use polywrap_client::msgpack::{from_slice, to_vec};
use serde::{Deserialize, Serialize};
use crate::module::{ModuleTrait, Module};
use polywrap_wasm_rs::{
    wrap_load_env,
    BigInt,
    BigNumber,
    Map,
    JSON,
};
{{#each (property_deps this)}}
use {{_crate}}::{{detect_keyword (to_upper _type)}};
{{/each}}
{{#with ../envType}}
use crate::Env;
{{/with}}
{{/if}}

{{#each methods}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Args{{to_upper name}} {
    {{#each arguments}}
    {{serde_keyword (to_lower name)}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}

pub fn {{to_lower name}}_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    {{#with env}}
    {{#if required}}
    if env_size == 0 {
        panic!("Environment is not set, and it is required by method '{{../name}}'");
    }

    let env_buf = wrap_load_env(env_size);
    let env = Env::from_buffer(&env_buf).unwrap();

    {{else}}
    let mut env: Option<Env> = None;
    if env_size > 0 {
      let env_buf = wrap_load_env(env_size);
      env = Some(Env::from_buffer(&env_buf).unwrap());
    }

    {{/if}}
    {{/with}}
    {{#if (array_has_length arguments)}}
    match from_slice::<Args{{to_upper name}}>(args) {
        Ok(args) => {
    {{/if}}
            let result = Module::{{detect_keyword (to_lower name)}}(Args{{to_upper name}} {
                {{#each arguments}}
                {{detect_keyword (to_lower name)}}: args.{{detect_keyword (to_lower name)}},
                {{/each}}
            }{{#with env}}, env{{/with}});
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
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
