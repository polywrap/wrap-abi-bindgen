import * as Types from "./types";

export class Args_importedMethod {
  str: string;
  optStr: string | null;
  u: number;
  optU: number | null;
  uArrayArray: Array<Array<number | null> | null>;
  object: Types.TestImport_Object;
  optObject: Types.TestImport_Object | null;
  objectArray: Array<Types.TestImport_Object>;
  optObjectArray: Array<Types.TestImport_Object | null> | null;
  en: Types.TestImport_Enum;
  optEnum: Types.TestImport_Enum | null;
  enumArray: Array<Types.TestImport_Enum>;
  optEnumArray: Array<Types.TestImport_Enum | null> | null;
}

export class Args_anotherMethod {
  arg: Array<string>;
}

export class Args_returnsArrayOfEnums {
  arg: string;
}

export class TestImport_Module {
  public static interfaceUri: string = "testimport.uri.eth";

  public uri: string;

  constructor(uri: string) {
    this.uri = uri;
  }

  public importedMethod(
    args: Args_importedMethod
  ): Result<Types.TestImport_Object | null, string> {
    const argsBuf = serializeimportedMethodArgs(args);
    const result = wrap_subinvokeImplementation(
      "testimport.uri.eth",
      this.uri,
      "importedMethod",
      argsBuf
    );

    if (result.isErr) {
      return Result.Err<Types.TestImport_Object | null, string>(
        result.unwrapErr()
      );
    }

    return Result.Ok<Types.TestImport_Object | null, string>(
      deserializeimportedMethodResult(result.unwrap())
    );
  }

  public anotherMethod(args: Args_anotherMethod): Result<i32, string> {
    const argsBuf = serializeanotherMethodArgs(args);
    const result = wrap_subinvokeImplementation(
      "testimport.uri.eth",
      this.uri,
      "anotherMethod",
      argsBuf
    );

    if (result.isErr) {
      return Result.Err<i32, string>(result.unwrapErr());
    }

    return Result.Ok<i32, string>(
      deserializeanotherMethodResult(result.unwrap())
    );
  }

  public returnsArrayOfEnums(
    args: Args_returnsArrayOfEnums
  ): Result<Array<Box<Types.TestImport_Enum_Return> | null>, string> {
    const argsBuf = serializereturnsArrayOfEnumsArgs(args);
    const result = wrap_subinvokeImplementation(
      "testimport.uri.eth",
      this.uri,
      "returnsArrayOfEnums",
      argsBuf
    );

    if (result.isErr) {
      return Result.Err<
        Array<Box<Types.TestImport_Enum_Return> | null>,
        string
      >(result.unwrapErr());
    }

    return Result.Ok<Array<Box<Types.TestImport_Enum_Return> | null>, string>(
      deserializereturnsArrayOfEnumsResult(result.unwrap())
    );
  }
}
