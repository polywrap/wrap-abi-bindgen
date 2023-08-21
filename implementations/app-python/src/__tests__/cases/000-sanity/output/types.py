# NOTE: This is an auto-generated file. All modifications will be overwritten.
# type: ignore
from __future__ import annotations

from abc import ABC, abstractmethod
from typing import TypedDict, Optional
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

Env = TypedDict("Env", {
    "prop": str,
    "optProp": Optional[str],
    "optMap": Optional[GenericMap[str, Optional[int]]],
})

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
    "en": "CustomEnum",
    "optEnum": Optional["CustomEnum"],
    "enumArray": list["CustomEnum"],
    "optEnumArray": Optional[list[Optional["CustomEnum"]]],
    "map": GenericMap[str, int],
    "mapOfArr": GenericMap[str, list[int]],
    "mapOfObj": GenericMap[str, "AnotherType"],
    "mapOfArrOfObj": GenericMap[str, list["AnotherType"]],
    "mapCustomValue": GenericMap[str, Optional["CustomMapValue"]],
})

AnotherType = TypedDict("AnotherType", {
    "prop": Optional[str],
    "circular": Optional["CustomType"],
    "const": Optional[str],
})

CustomMapValue = TypedDict("CustomMapValue", {
    "foo": str,
})

Else = TypedDict("Else", {
    "else": str,
})

### Objects END ###

### Enums START ###
class CustomEnum(IntEnum):
    STRING = 0, "0", "STRING"
    BYTES = 1, "1", "BYTES"

    def __new__(cls, value: int, *aliases: str):
        obj = int.__new__(cls)
        obj._value_ = value
        for alias in aliases:
            cls._value2member_map_[alias] = obj
        return obj

class While(IntEnum):
    r_for = 0, "0", "for"
    r_in = 1, "1", "in"

    def __new__(cls, value: int, *aliases: str):
        obj = int.__new__(cls)
        obj._value_ = value
        for alias in aliases:
            cls._value2member_map_[alias] = obj
        return obj

### Enums END ###

### Imported Objects START ###

# URI: "testimport.uri.eth" #
TestImportObject = TypedDict("TestImportObject", {
    "object": "TestImportAnotherObject",
    "optObject": Optional["TestImportAnotherObject"],
    "objectArray": list["TestImportAnotherObject"],
    "optObjectArray": Optional[list[Optional["TestImportAnotherObject"]]],
    "en": "TestImportEnum",
    "optEnum": Optional["TestImportEnum"],
    "enumArray": list["TestImportEnum"],
    "optEnumArray": Optional[list[Optional["TestImportEnum"]]],
})

# URI: "testimport.uri.eth" #
TestImportAnotherObject = TypedDict("TestImportAnotherObject", {
    "prop": str,
})

### Imported Objects END ###

### Imported Enums START ###

# URI: "testimport.uri.eth" #
class TestImportEnum(IntEnum):
    STRING = 0, "0", "STRING"
    BYTES = 1, "1", "BYTES"

    def __new__(cls, value: int, *aliases: str):
        obj = int.__new__(cls)
        obj._value_ = value
        for alias in aliases:
            cls._value2member_map_[alias] = obj
        return obj

# URI: "testimport.uri.eth" #
class TestImportEnumReturn(IntEnum):
    STRING = 0, "0", "STRING"
    BYTES = 1, "1", "BYTES"

    def __new__(cls, value: int, *aliases: str):
        obj = int.__new__(cls)
        obj._value_ = value
        for alias in aliases:
            cls._value2member_map_[alias] = obj
        return obj


### Imported Enums END ###

### Imported Modules START ###

# URI: "testimport.uri.eth" #
TestImportModuleArgsImportedMethod = TypedDict("TestImportModuleArgsImportedMethod", {
    "str": str,
    "optStr": Optional[str],
    "u": int,
    "optU": Optional[int],
    "uArrayArray": list[Optional[list[Optional[int]]]],
    "object": "TestImportObject",
    "optObject": Optional["TestImportObject"],
    "objectArray": list["TestImportObject"],
    "optObjectArray": Optional[list[Optional["TestImportObject"]]],
    "en": "TestImportEnum",
    "optEnum": Optional["TestImportEnum"],
    "enumArray": list["TestImportEnum"],
    "optEnumArray": Optional[list[Optional["TestImportEnum"]]],
})

# URI: "testimport.uri.eth" #
TestImportModuleArgsAnotherMethod = TypedDict("TestImportModuleArgsAnotherMethod", {
    "arg": list[str],
})

# URI: "testimport.uri.eth" #
TestImportModuleArgsReturnsArrayOfEnums = TypedDict("TestImportModuleArgsReturnsArrayOfEnums", {
    "arg": str,
})

# URI: "testimport.uri.eth" #
class BaseTestImport:
    _default_client: Client
    _default_uri: Uri
    _default_env: Optional[Any]

    def __init__(
        self,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ):
        self._default_client = self._get_client(client)
        self._default_uri = self._get_uri(uri)
        self._default_env = self._get_env(env)

    def _get_client(self, client: Optional[Client]) -> Client:
        return client or self._default_client or self._get_default_client()

    def _get_uri(self, uri: Optional[Uri]) -> Uri:
        return uri or self._default_uri or self._get_default_uri() or Uri.from_str("testimport.uri.eth")

    def _get_env(self, env: Optional[Any]) -> Any:
        return env or self._default_env or self._get_default_env()

    def _get_default_client(self) -> Client:
        config = (
            PolywrapClientConfigBuilder()
            .add_bundle(sys_bundle())
            .add_bundle(web3_bundle())
            .build()
        )
        return PolywrapClient(config)

    def _get_default_uri(self) -> Optional[Uri]:
        return Uri.from_str("{{uri}}")

    def _get_default_env(self) -> Any:
        return None

    def imported_method(
        self,
        args: TestImportModuleArgsImportedMethod,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> Optional["TestImportObject"]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="importedMethod",
            args=args,
            env=_env,
        )

    def another_method(
        self,
        args: TestImportModuleArgsAnotherMethod,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> int:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="anotherMethod",
            args=args,
            env=_env,
        )

    def returns_array_of_enums(
        self,
        args: TestImportModuleArgsReturnsArrayOfEnums,
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> list[Optional["TestImportEnumReturn"]]:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="returnsArrayOfEnums",
            args=args,
            env=_env,
        )

### Imported Modules END ###
