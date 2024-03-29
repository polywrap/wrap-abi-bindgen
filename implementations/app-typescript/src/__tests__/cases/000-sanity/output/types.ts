// @ts-ignore
import * as Types from "./";

// @ts-ignore
import {
  CoreClient,
  InvokeResult,
  Uri,
} from "@polywrap/core-js";
import { PolywrapClient } from "@polywrap/client-js";

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
  en: Types.CustomEnum;
  optEnum?: Types.CustomEnum | null;
  enumArray: Array<Types.CustomEnum>;
  optEnumArray?: Array<Types.CustomEnum | null> | null;
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

export enum CustomEnumEnum {
  STRING,
  BYTES,
}

export type CustomEnumString =
  | "STRING"
  | "BYTES"

export type CustomEnum = CustomEnumEnum | CustomEnumString;

export enum whileEnum {
  for,
  in,
}

export type whileString =
  | "for"
  | "in"

export type _while = whileEnum | whileString;

/// Imported Objects START ///

/* URI: "testimport.uri.eth" */
export interface TestImport_Object {
  object: Types.TestImport_AnotherObject;
  optObject?: Types.TestImport_AnotherObject | null;
  objectArray: Array<Types.TestImport_AnotherObject>;
  optObjectArray?: Array<Types.TestImport_AnotherObject | null> | null;
  en: Types.TestImport_Enum;
  optEnum?: Types.TestImport_Enum | null;
  enumArray: Array<Types.TestImport_Enum>;
  optEnumArray?: Array<Types.TestImport_Enum | null> | null;
}

/* URI: "testimport.uri.eth" */
export interface TestImport_AnotherObject {
  prop: Types.String;
}

/// Imported Objects END ///

/// Imported Enums START ///

/* URI: "testimport.uri.eth" */
export enum TestImport_EnumEnum {
  STRING,
  BYTES,
}

export type TestImport_EnumString =
  | "STRING"
  | "BYTES"

export type TestImport_Enum = TestImport_EnumEnum | TestImport_EnumString;

/* URI: "testimport.uri.eth" */
export enum TestImport_Enum_ReturnEnum {
  STRING,
  BYTES,
}

export type TestImport_Enum_ReturnString =
  | "STRING"
  | "BYTES"

export type TestImport_Enum_Return = TestImport_Enum_ReturnEnum | TestImport_Enum_ReturnString;

/// Imported Enums END ///

/// Imported Modules START ///

/* URI: "testimport.uri.eth" */
export interface TestImport_Module_Args_importedMethod {
  str: Types.String;
  optStr?: Types.String | null;
  u: Types.UInt;
  optU?: Types.UInt | null;
  uArrayArray: Array<Array<Types.UInt | null> | null>;
  object: Types.TestImport_Object;
  optObject?: Types.TestImport_Object | null;
  objectArray: Array<Types.TestImport_Object>;
  optObjectArray?: Array<Types.TestImport_Object | null> | null;
  en: Types.TestImport_Enum;
  optEnum?: Types.TestImport_Enum | null;
  enumArray: Array<Types.TestImport_Enum>;
  optEnumArray?: Array<Types.TestImport_Enum | null> | null;
}

/* URI: "testimport.uri.eth" */
export interface TestImport_Module_Args_anotherMethod {
  arg: Array<Types.String>;
}

/* URI: "testimport.uri.eth" */
export interface TestImport_Module_Args_returnsArrayOfEnums {
  arg: Types.String;
}

/* URI: "testimport.uri.eth" */
export class TestImport {
  protected _defaultClient: CoreClient;
  protected _defaultUri: string;
  protected _defaultEnv?: Record<string, unknown>;

  constructor(
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ) {
    this._defaultClient = this._getClient(client);
    this._defaultEnv = this._getEnv(env);
    this._defaultUri = this._getUri(uri);
  }

  public get client(): CoreClient {
    return this._defaultClient;
  }

  public get uri(): string {
    return this._defaultUri;
  }

  public get env(): Record<string, unknown> | undefined {
    return this._defaultEnv;
  }

  private _getClient(client?: CoreClient): CoreClient {
    return client || this._defaultClient || this._getDefaultClient();
  }

  private _getUri(uri?: string): string {
    return uri || this._defaultUri || this._getDefaultUri();
  }

  private _getEnv(env?: Record<string, unknown>): Record<string, unknown> | undefined {
    return env || this._defaultEnv || this._getDefaultEnv();
  }

  protected _getDefaultClient(): CoreClient {
    return new PolywrapClient();
  }
  protected _getDefaultUri(): string {
    return "testimport.uri.eth";
  }
  protected _getDefaultEnv(): Record<string, unknown> | undefined {
    return undefined;
  }

  async importedMethod(
    args: TestImport_Module_Args_importedMethod,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Types.TestImport_Object | null>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Types.TestImport_Object | null>({
      uri: Uri.from(_uri),
      method: "importedMethod",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async anotherMethod(
    args: TestImport_Module_Args_anotherMethod,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Types.Int32>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Types.Int32>({
      uri: Uri.from(_uri),
      method: "anotherMethod",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };

  async returnsArrayOfEnums(
    args: TestImport_Module_Args_returnsArrayOfEnums,
    client?: CoreClient,
    env?: Record<string, unknown>,
    uri?: string,
  ): Promise<InvokeResult<Array<Types.TestImport_Enum_Return | null>>> {
    const _client = this._getClient(client);
    const _env = this._getEnv(env);
    const _uri = this._getUri(uri);

    return _client.invoke<Array<Types.TestImport_Enum_Return | null>>({
      uri: Uri.from(_uri),
      method: "returnsArrayOfEnums",
      args: (args as unknown) as Record<string, unknown>,
      env: _env,
    });
  };
};

/// Imported Modules END ///
