lazy_static! {
  static ref NAME: String = "module_type/index.ts".to_string();
  static ref SOURCE: String = r#"{{#if (array_has_length methods)}}
import {
  {{#each methods}}
  Args_{{detect_keyword name}}{{#if (is_not_last @index ../methods)}},{{/if}}
  {{/each}}
} from "./serialization";

export {
  {{#each methods}}
  Args_{{detect_keyword name}}{{#if (is_not_last @index ../methods)}},{{/if}}
  {{/each}}
};
export { ModuleBase } from "./module";
{{/if}}
"#.to_string();
}

use super::super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
