lazy_static! {
  static ref NAME: String = "imported/namespace/types.ts".to_string();
  static ref SOURCE: String = r#"import { BigInt, BigNumber, JSONString, Bytes } from "../../common"
{{#each importedEnumTypes}}
export enum {{detect_keyword type}} {
  {{#each constants}}
  {{detect_keyword this}},
  {{/each}}
}
{{/each}}
{{#each importedObjectTypes}}
export class {{detect_keyword type}} {

  public static uri: string = "{{uri}}";

  {{#each properties}}
  {{detect_keyword name}}: {{to_wasm (to_graphql_type this) true}};
  {{/each}}
}
{{/each}}
{{#each importedEnvTypes}}
export class {{detect_keyword type}} {

  public static uri: string = "{{uri}}";

  {{#each properties}}
  {{detect_keyword name}}: {{to_wasm (to_graphql_type this) true}};
  {{/each}}
}
{{/each}}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
