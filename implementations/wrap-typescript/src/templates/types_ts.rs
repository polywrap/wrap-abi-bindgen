lazy_static! {
  static ref NAME: String = "types.ts".to_string();
  static ref SOURCE: String = r#"import { BigInt, BigNumber, JSONString, Bytes } from "./common";
import * as Types from "..";

export enum {{detect_keyword type}} {
  {{#each constants}}
  {{detect_keyword this}},
  {{/each}}
  _MAX_
}

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

export class {{detect_keyword namespace}} {
  static uri: string = "{{uri}}"

  {{#with capabilities}}
  {{#with getImplementations}}
  {{#if enabled}}
  public static getImplementations(): string[] {
    return wrap_getImplementations(this.uri);
  }
  {{/if}}
  {{/with}}
  {{/with}}
}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
