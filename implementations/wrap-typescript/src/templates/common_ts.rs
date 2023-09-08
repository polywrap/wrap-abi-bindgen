lazy_static! {
  static ref NAME: String = "common.ts".to_string();
  static ref SOURCE: String = r#"export type Bytes = Uint8Array;
export type BigInt = string;
export type BigNumber = string;
export type JSONString = string;
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}