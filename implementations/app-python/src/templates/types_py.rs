lazy_static! {
  static ref NAME: String = "types.py".to_string();
  static ref SOURCE: String = r#"# NOTE: This is an auto-generated file. All modifications will be overwritten.
# type: ignore
from __future__ import annotations

from abc import ABC, abstractmethod
from typing import TypedDict, Optional
from enum import IntEnum

from polywrap_core import Uri, Client
from polywrap_msgpack import GenericMap


### Env START ###

{{#with envType}}
{{detect_keyword (to_upper type)}} = TypedDict("{{detect_keyword (to_upper type)}}", {
    {{#each properties}}
    "{{name}}": {{to_python (to_graphql_type this)}},
    {{/each}}
})

{{/with}}
### Env END ###

### Objects START ###

{{#each objectTypes}}
{{detect_keyword (to_upper type)}} = TypedDict("{{detect_keyword (to_upper type)}}", {
    {{#each properties}}
    "{{name}}": {{to_python (to_graphql_type this)}},
    {{/each}}
})

{{/each}}
### Objects END ###

### Enums START ###
{{#each enumTypes}}
class {{detect_keyword (to_upper type)}}(IntEnum):
    {{#each constants}}
    {{detect_keyword this}} = {{@index}}, "{{@index}}", "{{this}}"
    {{/each}}

    def __new__(cls, value: int, *aliases: str):
        obj = int.__new__(cls)
        obj._value_ = value
        for alias in aliases:
            cls._value2member_map_[alias] = obj
        return obj

{{/each}}
### Enums END ###

### Imported Objects START ###

{{#each importedObjectTypes}}
# URI: "{{uri}}" #
{{detect_keyword (to_upper type)}} = TypedDict("{{detect_keyword (to_upper type)}}", {
    {{#each properties}}
    "{{name}}": {{to_python (to_graphql_type this)}},
    {{/each}}
})

{{/each}}
### Imported Objects END ###

### Imported Enums START ###

{{#each importedEnumTypes}}
# URI: "{{uri}}" #
class {{detect_keyword (to_upper type)}}(IntEnum):
    {{#each constants}}
    {{detect_keyword this}} = {{@index}}, "{{@index}}", "{{this}}"
    {{/each}}

    def __new__(cls, value: int, *aliases: str):
        obj = int.__new__(cls)
        obj._value_ = value
        for alias in aliases:
            cls._value2member_map_[alias] = obj
        return obj

{{/each}}

### Imported Enums END ###

### Imported Modules START ###

{{#each importedModuleTypes}}
{{#each methods}}
# URI: "{{../uri}}" #
{{to_upper ../type}}Args{{to_upper name}} = TypedDict("{{to_upper ../type}}Args{{to_upper name}}", {
    {{#each arguments}}
    "{{name}}": {{to_python (to_graphql_type this)}},
    {{/each}}
})

{{/each}}
# URI: "{{uri}}" #
class {{to_abstract_type (detect_keyword type)}}(ABC):
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
        return uri or self._default_uri or self._get_default_uri() or Uri.from_str("{{uri}}")

    def _get_env(self, env: Optional[Any]) -> Any:
        return env or self._default_env or self._get_default_env()

    @abstractmethod
    def _get_default_client(self) -> Client:
        pass

    @abstractmethod
    def _get_default_uri(self) -> Optional[Uri]:
        pass

    @abstractmethod
    def _get_default_env(self) -> Any:
        pass

    {{#each methods}}
    def {{detect_keyword (to_lower name)}}(
        self,
        args: {{to_upper ../type}}Args{{to_upper name}},
        client: Optional[Client] = None,
        env: Optional[Any] = None,
        uri: Optional[Uri] = None,
    ) -> {{#with return}}{{to_python (to_graphql_type this)}}{{/with}}:
        _client = self._get_client(client)
        _env = self._get_env(env)
        _uri = self._get_uri(uri)

        return _client.invoke(
            uri=_uri,
            method="{{name}}",
            args=args,
            env=_env,
        )
    {{#if (is_not_last @index ../methods)}}

    {{/if}}
    {{/each}}

{{/each}}
### Imported Modules END ###
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
