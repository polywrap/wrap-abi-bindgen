lazy_static! {
  static ref NAME: String = "globals.d.ts".to_string();
  static ref SOURCE: String = r#"declare const __wrap_args: any;
type WrapMethod =
{{#with moduleType}}
{{#each methods}}
| "{{detect_keyword name}}"
{{/each}}
{{/with}}
declare const __wrap_method: WrapMethod;
declare interface Result<T = any> {
  ok: boolean;
  error: string | undefined;
  value: T | undefined;
}
declare const __wrap_subinvoke: (
  uri: string,
  name: string,
  args: any
) => Result;
declare const __wrap_abort: (args: any) => void;
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
