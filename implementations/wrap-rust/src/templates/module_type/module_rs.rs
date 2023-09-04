lazy_static! {
  static ref NAME: String = "module_type/module.rs".to_string();
  static ref SOURCE: String = r#"{{#with moduleType}}
  use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    wrap_load_env,
    to_vec,
    from_slice,
    JSONString,
    BigIntWrapper
};
use crate::{
    {{#each methods}}
    Args{{to_upper name}},
    {{/each}}
};
{{#each (property_deps this)}}
use {{_crate}}::{{detect_keyword (to_upper _type)}};
{{/each}}
{{#with ../envType}}
use crate::env::Env;
{{/with}}

pub struct Module;

pub trait ModuleTrait {
  {{#each methods}}
  fn {{detect_keyword (to_lower name)}}(args: Args{{to_upper name}}{{#with env}}, env: {{#if required}}Env{{else}}Option<Env>{{/if}}{{/with}}) -> Result<{{#with return}}{{to_rust (to_graphql_type this)}}{{/with}}, String>;
  {{#if (is_not_last @index ../methods)}}

  {{/if}}
  {{/each}}
}
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
