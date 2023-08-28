lazy_static! {
  static ref NAME: String = "types.kt".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

package wrap

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.Invoker
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import io.polywrap.core.msgpack.GenericMapExtensionSerializer
import kotlinx.serialization.Serializable

typealias GenericMap<K, V> = @Serializable(with = GenericMapExtensionSerializer::class) io.polywrap.core.msgpack.GenericMap<K, V>

typealias BigInt = String
typealias BigNumber = String
typealias Json = String

/// Objects START ///
{{#each objectTypes}}
@Serializable
{{#if (array_has_length properties)}}data{{/if}} class {{to_class_name type}}(
    {{#each properties}}
    val {{detect_keyword name}}: {{nullable_default (to_kotlin (to_graphql_type this))}},
    {{/each}}
)

{{/each}}
/// Objects END ///

/// Enums START ///
{{#each enumTypes}}
@Serializable
enum class {{to_class_name type}} {
    {{#each constants}}
    {{detect_keyword_strict this}}{{#if (is_not_last @index ../constants)}},{{/if}}
    {{/each}}
}

{{/each}}
/// Enums END ///

/// Imported Objects START ///
{{#each importedObjectTypes}}
/* URI: "{{uri}}" */
@Serializable
{{#if (array_has_length properties)}}data{{/if}} class {{to_class_name type}}(
    {{#each properties}}
    val {{detect_keyword name}}: {{nullable_default (to_kotlin (to_graphql_type this))}},
    {{/each}}
)

{{/each}}
/// Imported Objects END ///

/// Imported Enums START ///
{{#each importedEnumTypes}}
/* URI: "{{uri}}" */
@Serializable
enum class {{to_class_name type}} {
    {{#each constants}}
    {{detect_keyword_strict this}}{{#if (is_not_last @index ../constants)}},{{/if}}
    {{/each}}
}

{{/each}}
/// Imported Enums END ///

/// Imported Envs START ///
{{#each importedEnvTypes}}
@Serializable
{{#if (array_has_length properties)}}data{{/if}} class {{to_class_name namespace}}Env(
    {{#each properties}}
    val {{detect_keyword name}}: {{nullable_default (to_kotlin (to_graphql_type this))}},
    {{/each}}
)

{{/each}}
/// Imported Envs END ///

/// Imported Modules START ///
{{#each importedModuleTypes}}
{{#each methods}}
/* URI: "{{../uri}}" */
@Serializable
{{#if (array_has_length arguments)}}data{{/if}} class {{to_class_name (remove_module_suffix ../type)}}Args{{to_class_name name}}(
    {{#each arguments}}
    val {{detect_keyword name}}: {{nullable_default (to_kotlin (to_graphql_type this))}},
    {{/each}}
)

{{/each}}
/* URI: "{{uri}}" */
open class {{to_class_name (remove_module_suffix type)}}(
    client: Invoker? = null,
    {{#if (import_has_env ../importedEnvTypes namespace)}}
    env: {{to_class_name namespace}}Env? = null,
    {{/if}}
    uri: Uri? = null
) {
    protected val defaultClient: Invoker by lazy {
        polywrapClient { addDefaults() }
    }
    protected val defaultUri: Uri by lazy {
        Uri("{{uri}}")
    }
    {{#if (import_has_env ../importedEnvTypes namespace)}}
    protected val defaultEnv: {{to_class_name namespace}}Env?
    {{/if}}

    val client: Invoker = client ?: defaultClient
    val uri: Uri = uri ?: defaultUri
    {{#if (import_has_env ../importedEnvTypes namespace)}}
    val env: {{to_class_name namespace}}Env? = env ?: defaultEnv
    {{/if}}
    {{#each methods}}

    fun {{detect_keyword name}}(
        args: {{to_class_name (remove_module_suffix ../type)}}Args{{to_class_name name}},
        client: Invoker? = null,
        {{#if (import_has_env ../../importedEnvTypes ../namespace)}}
        env: {{to_class_name ../namespace}}Env? = null,
        {{/if}}
        uri: Uri? = null
    ): InvokeResult<{{#with return}}{{to_kotlin (to_graphql_type this)}}{{/with}}> {
        val _client = client ?: this.client
        val _uri = uri ?: this.uri
        {{#if (import_has_env ../../importedEnvTypes ../namespace)}}
        val _env = env ?: this.env
        return _client.invoke(_uri, "{{name}}", args, _env)
        {{else}}
        return _client.invoke(_uri, "{{name}}", args)
        {{/if}}
    }
    {{/each}}
}
{{/each}}
/// Imported Modules END ///
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}