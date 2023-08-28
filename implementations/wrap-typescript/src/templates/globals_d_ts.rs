lazy_static! {
  static ref NAME: String = "global.d.ts".to_string();
  static ref SOURCE: String = r#"declare const __wrap_args: any;
  declare const __wrap_method: string;
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
