lazy_static! {
  static ref NAME: String = "module.py".to_string();
  static ref SOURCE: String = r#"# NOTE: This is an auto-generated file. All modifications will be overwritten.
# type: ignore
from __future__ import annotations

from abc import abstractmethod
from typing import TypeVar, Generic, TypedDict, Optional

from .types import *

from polywrap_core import InvokerClient
from polywrap_plugin import PluginModule
from polywrap_msgpack import GenericMap

TConfig = TypeVar("TConfig")

{{#with moduleType}}

{{#each methods}}
Args{{to_upper name}} = TypedDict("Args{{to_upper name}}", {
    {{#each arguments}}
    "{{name}}": {{to_python (to_graphql_type this)}}{{#if (is_not_last @index ../arguments)}},{{/if}}
    {{/each}}
})

{{/each}}

{{/with}}
class Module(Generic[TConfig], PluginModule[TConfig]):
    def __new__(cls, *args, **kwargs):
        # NOTE: This is used to dynamically add WRAP ABI compatible methods to the class
        instance = super().__new__(cls)
        {{#with moduleType}}
        {{#each methods}}
        setattr(instance, "{{name}}", instance.{{detect_keyword (to_lower name)}})
        {{/each}}
        {{/with}}
        return instance
    {{#with moduleType}}

    {{#each methods}}
    @abstractmethod
    def {{detect_keyword (to_lower name)}}(
        self,
        args: Args{{to_upper name}},
        client: InvokerClient,
        {{#if env}}{{#with env}}env: {{#if required}}{{else}}Optional[{{/if}}Env{{#if required}}{{else}}] = None{{/if}}{{/with}}{{else}}env: None{{/if}}
    ) -> {{#with return}}{{to_python (to_graphql_type this)}}{{/with}}:
        pass
    {{#if (is_not_last @index ../methods)}}

    {{/if}}
    {{/each}}
    {{/with}}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
