/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

// @ts-ignore
import * as Types from "./types";

// @ts-ignore
import { CoreClient, MaybeAsync } from "@polywrap/core-js";
import { PluginModule } from "@polywrap/plugin-js";

export interface Args_function1 {
  arg1: Types.UInt32;
  arg2: Types.Boolean;
}

export interface Args_function2 {
  arg1?: Types.Int32 | null;
  arg2?: Types.Bytes | null;
}

export abstract class Module<TConfig> extends PluginModule<TConfig> {
  abstract function1(
    args: Args_function1,
    client: CoreClient,
    env?: null
  ): MaybeAsync<Types.String>;

  abstract function2(
    args: Args_function2,
    client: CoreClient,
    env?: null
  ): MaybeAsync<Types.String | null>;
}
