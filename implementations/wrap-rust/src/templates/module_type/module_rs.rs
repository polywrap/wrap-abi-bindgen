lazy_static! {
  static ref NAME: String = "module_type/module.rs".to_string();
  static ref SOURCE: String = r#"{{#with moduleType}}
use crate::{
    {{#each methods}}
    Args{{to_upper name}},
    {{/each}}
};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
};
{{#if (array_has_length propertyDeps)}}

{{#each propertyDeps}}
{{#if isEnum}}
use crate::{
    {{detect_keyword (to_upper type)}},
};
{{else}}
use {{crate}}::{{detect_keyword (to_upper type)}};
{{/if}}
{{/each}}
{{/if}}

pub struct Module;

pub trait ModuleTrait {
  {{#each methods}}
  fn {{detectKeyword (to_lower name)}}(args: Args{{to_upper name}}{{#with env}}, env: {{#if required}}Env{{else}}Option<Env>{{/if}}{{/with}}) -> Result<{{#with return}}{{to_rust (to_graphql_type this)}}{{/with}}, String>;
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
