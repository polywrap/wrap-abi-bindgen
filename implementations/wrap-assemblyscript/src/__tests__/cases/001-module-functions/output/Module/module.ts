import * as Types from "..";

import {
  BigInt,
  BigNumber,
  Box,
  JSON,
} from "@polywrap/wasm-as";

export abstract class ModuleBase {
  abstract function1(
    args: Types.Args_function1
  ): string;

  abstract function2(
    args: Types.Args_function2
  ): string | null;
}
