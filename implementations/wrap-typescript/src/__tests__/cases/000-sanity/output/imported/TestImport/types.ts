import { BigInt, BigNumber, JSONString, Bytes } from "../../common"

export enum TestImport_Enum {
  STRING,
  BYTES,
}

export enum TestImport_Enum_Return {
  STRING,
  BYTES,
}

export class TestImport_AnotherObject {
  public static uri: string = "testimport.uri.eth";

  prop: string;
}

export class TestImport_Object {
  public static uri: string = "testimport.uri.eth";

  object: TestImport_AnotherObject;
  optObject: TestImport_AnotherObject | null;
  objectArray: Array<TestImport_AnotherObject>;
  optObjectArray: Array<TestImport_AnotherObject | null> | null;
  en: TestImport_Enum;
  optEnum: TestImport_Enum | null;
  enumArray: Array<TestImport_Enum>;
  optEnumArray: Array<TestImport_Enum | null> | null;
}

export class TestImport_Env {
  public static uri: string = "testimport.uri.eth";

  object: TestImport_AnotherObject;
  optObject: TestImport_AnotherObject | null;
  objectArray: Array<TestImport_AnotherObject>;
  optObjectArray: Array<TestImport_AnotherObject | null> | null;
  en: TestImport_Enum;
  optEnum: TestImport_Enum | null;
  enumArray: Array<TestImport_Enum>;
  optEnumArray: Array<TestImport_Enum | null> | null;
}
