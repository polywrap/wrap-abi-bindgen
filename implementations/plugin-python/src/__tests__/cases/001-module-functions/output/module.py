# NOTE: This is an auto-generated file. All modifications will be overwritten.
# type: ignore
from __future__ import annotations

from abc import abstractmethod
from typing import TypeVar, Generic, TypedDict, Optional

from .types import *

from polywrap_core import InvokerClient
from polywrap_plugin import PluginModule
from polywrap_msgpack import GenericMap

TConfig = TypeVar("TConfig")


ArgsFunction1 = TypedDict("ArgsFunction1", {
    "arg1": int,
    "arg2": bool
})

ArgsFunction2 = TypedDict("ArgsFunction2", {
    "arg1": Optional[int],
    "arg2": Optional[bytes]
})


class Module(Generic[TConfig], PluginModule[TConfig]):
    def __new__(cls, *args, **kwargs):
        # NOTE: This is used to dynamically add WRAP ABI compatible methods to the class
        instance = super().__new__(cls)
        setattr(instance, "function1", instance.function1)
        setattr(instance, "function2", instance.function2)
        return instance

    @abstractmethod
    def function1(
        self,
        args: ArgsFunction1,
        client: InvokerClient,
        env: None
    ) -> str:
        pass

    @abstractmethod
    def function2(
        self,
        args: ArgsFunction2,
        client: InvokerClient,
        env: None
    ) -> Optional[str]:
        pass
