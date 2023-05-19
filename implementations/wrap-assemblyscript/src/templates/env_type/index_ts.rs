lazy_static! {
  static ref NAME: String = "env_type/index.ts".to_string();
  static ref SOURCE: String = r#"import {
  Read,
  Write,
  Box,
  BigInt,
  BigNumber,
  JSON
} from "@polywrap/wasm-as";
import {
  serialize{{type}},
  deserialize{{type}},
  write{{type}},
  read{{type}}
} from "./serialization";
import * as Types from "..";

export class {{detect_keyword type}} {
  {{#each properties}}
  {{detect_keyword name}}: {{to_wasm (to_graphql_type this)}};
  {{/each}}

  static toBuffer(type: {{detect_keyword type}}): ArrayBuffer {
    return serialize{{type}}(type);
  }

  static fromBuffer(buffer: ArrayBuffer): {{detect_keyword type}} {
    return deserialize{{type}}(buffer);
  }

  static write(writer: Write, type: {{detect_keyword type}}): void {
    write{{type}}(writer, type);
  }

  static read(reader: Read): {{detect_keyword type}} {
    return read{{type}}(reader);
  }
}
"#.to_string();
}

use super::super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
