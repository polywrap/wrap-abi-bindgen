lazy_static! {
  static ref NAME: String = "types.py".to_string();
  static ref SOURCE: String = r#"# NOTE: This is an auto-generated file. All modifications will be overwritten.
# type: ignore
from __future__ import annotations

from typing import TypedDict, Optional
from enum import IntEnum

from polywrap_core import InvokerClient, Uri, UriPackageOrWrapper
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
    {{#each members}}
    {{detect_keyword name}} = {{value}}, "{{value}}", "{{name}}"
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
    {{#each members}}
    {{detect_keyword name}} = {{value}}, "{{value}}", "{{name}}"
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
{{#if isInterface}}
class {{detect_keyword (to_upper type)}}:
    INTERFACE_URI: Uri = Uri.from_str("{{uri}}")
    uri: Uri

    def __init__(self, uri: Uri):
        self.uri = uri

    {{#each methods}}
    async def {{detect_keyword (to_lower name)}}(
        self,
        args: {{to_upper ../type}}Args{{to_upper name}},
        client: InvokerClient[UriPackageOrWrapper]
    ) -> {{#with return}}{{to_python (to_graphql_type this)}}{{/with}}:
        return client.invoke(
            InvokeOptions(
                uri=self.uri,
                method="{{name}}",
                args=args,
            )
        )
    {{#if (is_not_last @index ../methods)}}

    {{/if}}
    {{/each}}

{{else}}
class {{detect_keyword (to_upper type)}}:
    {{#each methods}}
    @staticmethod
    async def {{detect_keyword (to_lower name)}}(
        args: {{to_upper ../type}}Args{{to_upper name}},
        client: InvokerClient[UriPackageOrWrapper]
    ) -> {{#with return}}{{to_python (to_graphql_type this)}}{{/with}}:
        return client.invoke(
            InvokeOptions(
                uri=Uri.from_str("{{../uri}}"),
                method="{{name}}",
                args=args,
            )
        )
    {{#if (is_not_last @index ../methods)}}

    {{/if}}
    {{/each}}

{{/if}}
{{/each}}
### Imported Modules END ###

### Interface START ###

{{#each interfaceTypes}}

class {{detect_keyword (to_upper namespace)}}:
    URI: Uri = Uri.from_str("{{uri}}")

    {{#with capabilities}}
    {{#with getImplementations}}
    {{#if enabled}}
    def get_implementations(
        client: InvokerClient[UriPackageOrWrapper]
    ) -> list[str]:
        impls = client.getImplementations(self.uri)
        return [impl.uri for impl in impls]
    {{/if}}
    {{/with}}
    {{/with}}
{{/each}}

### Interface END ###
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
