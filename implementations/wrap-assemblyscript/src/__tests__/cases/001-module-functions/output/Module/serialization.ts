import {
  Read,
  ReadDecoder,
  Write,
  WriteSizer,
  WriteEncoder,
  Box,
  BigInt,
  BigNumber,
  JSON,
  Context
} from "@polywrap/wasm-as";
import * as Types from "..";

export class Args_function1 {
  arg1: u32;
  arg2: bool;
}

export function deserializefunction1Args(argsBuf: ArrayBuffer): Args_function1 {
  const context: Context = new Context("Deserializing module-type: function1 Args");
  const reader = new ReadDecoder(argsBuf, context);
  let numFields = reader.readMapLength();

  let _arg1: u32 = 0;
  let _arg1Set: bool = false;
  let _arg2: bool = false;
  let _arg2Set: bool = false;

  while (numFields > 0) {
    numFields--;
    const field = reader.readString();

    reader.context().push(field, "unknown", "searching for property type");
    if (field == "arg1") {
      reader.context().push(field, "u32", "type found, reading property");
      _arg1 = reader.readUInt32();
      _arg1Set = true;
      reader.context().pop();
    }
    else if (field == "arg2") {
      reader.context().push(field, "bool", "type found, reading property");
      _arg2 = reader.readBool();
      _arg2Set = true;
      reader.context().pop();
    }
    reader.context().pop();
  }

  if (!_arg1Set) {
    throw new Error(reader.context().printWithContext("Missing required argument: 'arg1: UInt32'"));
  }
  if (!_arg2Set) {
    throw new Error(reader.context().printWithContext("Missing required argument: 'arg2: Boolean'"));
  }

  return {
    arg1: _arg1,
    arg2: _arg2
  };
}

export function serializefunction1Args(args: Args_function1): ArrayBuffer {
  const sizerContext: Context = new Context("Serializing (sizing) module-type: function1 Args");
  const sizer = new WriteSizer(sizerContext);
  writefunction1Args(sizer, args);
  const buffer = new ArrayBuffer(sizer.length);
  const encoderContext: Context = new Context("Serializing (encoding) module-type: function1 Args");
  const encoder = new WriteEncoder(buffer, sizer, encoderContext);
  writefunction1Args(encoder, args);
  return buffer;
}

export function writefunction1Args(
  writer: Write,
  args: Args_function1
): void {
  writer.writeMapLength(2);
  writer.context().push("arg1", "u32", "writing property");
  writer.writeString("arg1");
  writer.writeUInt32(args.arg1);
  writer.context().pop();
  writer.context().push("arg2", "bool", "writing property");
  writer.writeString("arg2");
  writer.writeBool(args.arg2);
  writer.context().pop();
}

export function serializefunction1Result(result: string): ArrayBuffer {
  const sizerContext: Context = new Context("Serializing (sizing) module-type: function1 Result");
  const sizer = new WriteSizer(sizerContext);
  writefunction1Result(sizer, result);
  const buffer = new ArrayBuffer(sizer.length);
  const encoderContext: Context = new Context("Serializing (encoding) module-type: function1 Result");
  const encoder = new WriteEncoder(buffer, sizer, encoderContext);
  writefunction1Result(encoder, result);
  return buffer;
}

export function writefunction1Result(writer: Write, result: string): void {
  writer.context().push("function1", "string", "writing property");
  writer.writeString(result);
  writer.context().pop();
}

export function deserializefunction1Result(buffer: ArrayBuffer): string {
  const context: Context = new Context("Deserializing module-type: function1 Result");
  const reader = new ReadDecoder(buffer, context);

  reader.context().push("function1", "string", "reading function output");
  const res: string = reader.readString();
  reader.context().pop();

  return res;
}

export class Args_function2 {
  arg1: Box<i32> | null;
  arg2: ArrayBuffer | null;
}

export function deserializefunction2Args(argsBuf: ArrayBuffer): Args_function2 {
  const context: Context = new Context("Deserializing module-type: function2 Args");
  const reader = new ReadDecoder(argsBuf, context);
  let numFields = reader.readMapLength();

  let _arg1: Box<i32> | null = null;
  let _arg2: ArrayBuffer | null = null;

  while (numFields > 0) {
    numFields--;
    const field = reader.readString();

    reader.context().push(field, "unknown", "searching for property type");
    if (field == "arg1") {
      reader.context().push(field, "Box<i32> | null", "type found, reading property");
      _arg1 = reader.readOptionalInt32();
      reader.context().pop();
    }
    else if (field == "arg2") {
      reader.context().push(field, "ArrayBuffer | null", "type found, reading property");
      _arg2 = reader.readOptionalBytes();
      reader.context().pop();
    }
    reader.context().pop();
  }

  return {
    arg1: _arg1,
    arg2: _arg2
  };
}

export function serializefunction2Args(args: Args_function2): ArrayBuffer {
  const sizerContext: Context = new Context("Serializing (sizing) module-type: function2 Args");
  const sizer = new WriteSizer(sizerContext);
  writefunction2Args(sizer, args);
  const buffer = new ArrayBuffer(sizer.length);
  const encoderContext: Context = new Context("Serializing (encoding) module-type: function2 Args");
  const encoder = new WriteEncoder(buffer, sizer, encoderContext);
  writefunction2Args(encoder, args);
  return buffer;
}

export function writefunction2Args(
  writer: Write,
  args: Args_function2
): void {
  writer.writeMapLength(2);
  writer.context().push("arg1", "Box<i32> | null", "writing property");
  writer.writeString("arg1");
  writer.writeOptionalInt32(args.arg1);
  writer.context().pop();
  writer.context().push("arg2", "ArrayBuffer | null", "writing property");
  writer.writeString("arg2");
  writer.writeOptionalBytes(args.arg2);
  writer.context().pop();
}

export function serializefunction2Result(result: string | null): ArrayBuffer {
  const sizerContext: Context = new Context("Serializing (sizing) module-type: function2 Result");
  const sizer = new WriteSizer(sizerContext);
  writefunction2Result(sizer, result);
  const buffer = new ArrayBuffer(sizer.length);
  const encoderContext: Context = new Context("Serializing (encoding) module-type: function2 Result");
  const encoder = new WriteEncoder(buffer, sizer, encoderContext);
  writefunction2Result(encoder, result);
  return buffer;
}

export function writefunction2Result(writer: Write, result: string | null): void {
  writer.context().push("function2", "string | null", "writing property");
  writer.writeOptionalString(result);
  writer.context().pop();
}

export function deserializefunction2Result(buffer: ArrayBuffer): string | null {
  const context: Context = new Context("Deserializing module-type: function2 Result");
  const reader = new ReadDecoder(buffer, context);

  reader.context().push("function2", "string | null", "reading function output");
  const res: string | null = reader.readOptionalString();
  reader.context().pop();

  return res;
}
