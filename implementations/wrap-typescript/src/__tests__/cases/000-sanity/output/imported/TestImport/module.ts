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
  ): Result<Types.TestImport_Object | null> {
    return __wrap_subinvoke(this.uri, "importedMethod", args);
  }

  public anotherMethod(args: Args_anotherMethod): Result<number> {
    return __wrap_subinvoke(this.uri, "anotherMethod", args);
  }

  public returnsArrayOfEnums(
    args: Args_returnsArrayOfEnums
  ): Result<Array<Types.TestImport_Enum_Return | null>> {
    return __wrap_subinvoke(this.uri, "returnsArrayOfEnums", args);
  }
}
