import {
  wrap_invoke_args,
  wrap_invoke_result,
  wrap_invoke_error,
  wrap_abort,
  InvokeArgs
} from "@polywrap/wasm-as";


import { Module } from "../index";

export function _wrap_invoke(method_size: u32, args_size: u32, env_size: u32): bool {
  const module = new Module();
  const args: InvokeArgs = wrap_invoke_args(
    method_size,
    args_size
  );
  let result: ArrayBuffer;
  else {
    wrap_invoke_error(
      `Could not find invoke function "${args.method}"`
    );
    return false;
  }
  wrap_invoke_result(result);
  return true;
}

export function wrapAbort(
  msg: string | null,
  file: string | null,
  line: u32,
  column: u32
): void {
  wrap_abort(
    msg ? msg : "",
    file ? file : "",
    line,
    column
  );
}
