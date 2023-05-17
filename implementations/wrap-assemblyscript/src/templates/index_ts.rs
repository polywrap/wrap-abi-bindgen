lazy_static! {
  static ref NAME: String = "index.ts".to_string();
  static ref SOURCE: String = r#"{{#with moduleType}}
{{#if (array_has_length methods)}}
import {
  {{#each methods}}
  Args_{{detect_keyword name}}{{#if (is_not_last @index ../methods)}},{{/if}}
  {{/each}}
} from "./{{type}}";

export {
  {{#each methods}}
  Args_{{detect_keyword name}}{{#if (is_not_last @index ../methods)}},{{/if}}
  {{/each}}
};

{{/if}}
export { ModuleBase } from "./Module";
{{/with}}
{{#each objectTypes}}
export { {{detect_keyword type}} } from "./{{type}}";
{{/each}}
{{#each enumTypes}}
export {
  {{detect_keyword type}},
  get{{type}}Key,
  get{{type}}Value,
  sanitize{{type}}Value
} from "./{{type}}";
{{/each}}
{{#each importedModuleTypes}}
export { {{detect_keyword type}} } from "./imported/{{type}}";
{{/each}}
{{#each importedObjectTypes}}
export { {{detect_keyword type}} } from "./imported/{{type}}";
{{/each}}
{{#each importedEnvTypes}}
export { {{detect_keyword type}} } from "./imported/{{type}}";
{{/each}}
{{#each importedEnumTypes}}
export {
  {{detect_keyword type}},
  get{{type}}Key,
  get{{type}}Value,
  sanitize{{type}}Value
} from "./imported/{{type}}";
{{/each}}
{{#each interfaceTypes}}
export { {{detect_keyword namespace}} } from "./{{namespace}}";
{{/each}}
{{#if envType}}
export { Env } from "./Env";
{{/if}}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
