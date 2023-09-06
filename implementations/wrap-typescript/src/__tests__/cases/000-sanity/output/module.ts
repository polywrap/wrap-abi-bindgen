import { BigInt, BigNumber, JSONString, Bytes } from "./common";
import * as Types from "./types";

export class Args_moduleMethod {
  str: string;
  optStr: string | null;
  en: Types.CustomEnum;
  optEnum: Types.CustomEnum | null;
  enumArray: Array<Types.CustomEnum>;
  optEnumArray: Array<Types.CustomEnum | null> | null;
  map: Map<string, number>;
  mapOfArr: Map<string, Array<number>>;
  mapOfMap: Map<string, Map<string, number>>;
  mapOfObj: Map<string, Types.AnotherType>;
  mapOfArrOfObj: Map<string, Array<Types.AnotherType>>;
}

export class Args_objectMethod {
  object: Types.AnotherType;
  optObject: Types.AnotherType | null;
  objectArray: Array<Types.AnotherType>;
  optObjectArray: Array<Types.AnotherType | null> | null;
}

export class Args_optionalEnvMethod {
  object: Types.AnotherType;
  optObject: Types.AnotherType | null;
  objectArray: Array<Types.AnotherType>;
  optObjectArray: Array<Types.AnotherType | null> | null;
}

export class Args__if {
  _if: Types._else;
}

export abstract class ModuleBase {
  abstract moduleMethod(
    args: Args_moduleMethod
  ): number;

  abstract objectMethod(
    args: Args_objectMethod,
    env: Types.Env
  ): Types.AnotherType | null;

  abstract optionalEnvMethod(
    args: Args_optionalEnvMethod,
    env: Types.Env | null
  ): Types.AnotherType | null;

  abstract _if(
    args: Args__if
  ): Types._else;
}
