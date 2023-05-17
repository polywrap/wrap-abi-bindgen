import { wrap_load_env } from "@polywrap/wasm-as";
import {
  deserializefunction1Args,
  serializefunction1Result,
  deserializefunction2Args,
  serializefunction2Result
} from "./serialization";
import { ModuleBase } from "./module";
import * as Types from "..";

export function function1Wrapped(module: ModuleBase, argsBuf: ArrayBuffer, env_size: u32): ArrayBuffer {
  const args = deserializefunction1Args(argsBuf);

  const result = module.function1(
    {
      arg1: args.arg1,
      arg2: args.arg2
    }
  );
  return serializefunction1Result(result);
}

export function function2Wrapped(module: ModuleBase, argsBuf: ArrayBuffer, env_size: u32): ArrayBuffer {
  const args = deserializefunction2Args(argsBuf);

  const result = module.function2(
    {
      arg1: args.arg1,
      arg2: args.arg2
    }
  );
  return serializefunction2Result(result);
}
