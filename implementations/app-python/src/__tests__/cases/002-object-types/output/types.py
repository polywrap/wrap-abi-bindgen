# NOTE: This is an auto-generated file. All modifications will be overwritten.
# type: ignore
from __future__ import annotations

from typing import Any, TypedDict, Optional
from enum import IntEnum

from polywrap import (
    Uri,
    Client,
    GenericMap,
    PolywrapClient,
    PolywrapClientConfigBuilder,
    sys_bundle,
    web3_bundle
)


### Env START ###

### Env END ###

### Objects START ###

CustomType = TypedDict("CustomType", {
    "str": str,
    "optStr": Optional[str],
    "u": int,
    "optU": Optional[int],
    "u8": int,
    "u16": int,
    "u32": int,
    "i": int,
    "i8": int,
    "i16": int,
    "i32": int,
    "bigint": str,
    "optBigint": Optional[str],
    "bignumber": str,
    "optBignumber": Optional[str],
    "json": str,
    "optJson": Optional[str],
    "bytes": bytes,
    "optBytes": Optional[bytes],
    "boolean": bool,
    "optBoolean": Optional[bool],
    "u_array": list[int],
    "uOpt_array": Optional[list[int]],
    "_opt_uOptArray": Optional[list[Optional[int]]],
    "optStrOptArray": Optional[list[Optional[str]]],
    "uArrayArray": list[list[int]],
    "uOptArrayOptArray": list[Optional[list[Optional[int]]]],
    "uArrayOptArrayArray": list[Optional[list[list[int]]]],
    "crazyArray": Optional[list[Optional[list[list[Optional[list[int]]]]]]],
    "object": "AnotherType",
    "optObject": Optional["AnotherType"],
    "objectArray": list["AnotherType"],
    "optObjectArray": Optional[list[Optional["AnotherType"]]],
    "map": GenericMap[str, int],
    "mapOfArr": GenericMap[str, list[int]],
    "mapOfObj": GenericMap[str, "AnotherType"],
    "mapOfArrOfObj": GenericMap[str, list["AnotherType"]],
    "mapCustomValue": GenericMap[str, Optional["CustomMapValue"]],
}, total=False)

AnotherType = TypedDict("AnotherType", {
    "prop": Optional[str],
    "circular": Optional["CustomType"],
    "const": Optional[str],
}, total=False)

CustomMapValue = TypedDict("CustomMapValue", {
    "foo": str,
}, total=False)

Else = TypedDict("Else", {
    "else": str,
}, total=False)

### Objects END ###

### Enums START ###
### Enums END ###

### Imported Objects START ###

### Imported Objects END ###

### Imported Enums START ###


### Imported Enums END ###

### Imported Modules START ###

### Imported Modules END ###
