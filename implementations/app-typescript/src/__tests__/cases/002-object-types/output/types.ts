// @ts-ignore
import * as Types from "./";

// @ts-ignore
import {
  CoreClient,
  InvokeResult,
  Uri,
} from "@polywrap/core-js";

export type UInt = number;
export type UInt8 = number;
export type UInt16 = number;
export type UInt32 = number;
export type Int = number;
export type Int8 = number;
export type Int16 = number;
export type Int32 = number;
export type Bytes = Uint8Array;
export type BigInt = string;
export type BigNumber = string;
export type Json = string;
export type String = string;
export type Boolean = boolean;

export interface CustomType {
  str: Types.String;
  optStr?: Types.String | null;
  u: Types.UInt;
  optU?: Types.UInt | null;
  u8: Types.UInt8;
  u16: Types.UInt16;
  u32: Types.UInt32;
  i: Types.Int;
  i8: Types.Int8;
  i16: Types.Int16;
  i32: Types.Int32;
  bigint: Types.BigInt;
  optBigint?: Types.BigInt | null;
  bignumber: Types.BigNumber;
  optBignumber?: Types.BigNumber | null;
  json: Types.Json;
  optJson?: Types.Json | null;
  bytes: Types.Bytes;
  optBytes?: Types.Bytes | null;
  boolean: Types.Boolean;
  optBoolean?: Types.Boolean | null;
  u_array: Array<Types.UInt>;
  uOpt_array?: Array<Types.UInt> | null;
  _opt_uOptArray?: Array<Types.UInt | null> | null;
  optStrOptArray?: Array<Types.String | null> | null;
  uArrayArray: Array<Array<Types.UInt>>;
  uOptArrayOptArray: Array<Array<Types.UInt32 | null> | null>;
  uArrayOptArrayArray: Array<Array<Array<Types.UInt32>> | null>;
  crazyArray?: Array<Array<Array<Array<Types.UInt32> | null>> | null> | null;
  object: Types.AnotherType;
  optObject?: Types.AnotherType | null;
  objectArray: Array<Types.AnotherType>;
  optObjectArray?: Array<Types.AnotherType | null> | null;
  map: Map<Types.String, Types.Int>;
  mapOfArr: Map<Types.String, Array<Types.Int>>;
  mapOfObj: Map<Types.String, Types.AnotherType>;
  mapOfArrOfObj: Map<Types.String, Array<Types.AnotherType>>;
  mapCustomValue: Map<Types.String, Types.CustomMapValue | undefined>;
}

export interface AnotherType {
  prop?: Types.String | null;
  circular?: Types.CustomType | null;
  const?: Types.String | null;
}

export interface CustomMapValue {
  foo: Types.String;
}

export interface _else {
  else: Types.String;
}
