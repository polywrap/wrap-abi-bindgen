lazy_static! {
  static ref NAME: String = "types.kt".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

package wrap

import io.polywrap.configBuilder.polywrapClient
import io.polywrap.core.WrapEnv
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

/// Imported Modules START ///
{{#each importedModuleTypes}}
{{#each methods}}
/* URI: "{{../uri}}" */
@Serializable
{{#if (array_has_length arguments)}}data{{/if}} class {{to_class_name ../type}}Args{{to_class_name name}}(
    {{#each arguments}}
    val {{detect_keyword name}}: {{nullable_default (to_kotlin (to_graphql_type this))}},
    {{/each}}
)

{{/each}}
/* URI: "{{uri}}" */
abstract class Base{{to_class_name type}}(
    client: Invoker? = null,
    env: WrapEnv? = null,
    uri: Uri? = null
) {
    protected abstract val defaultClient: Invoker?
    protected abstract val defaultUri: Uri?
    protected abstract val defaultEnv: WrapEnv?

    protected val client: Invoker = client ?: defaultClient ?: polywrapClient { addDefaults() }
    protected val uri: Uri = uri ?: defaultUri ?: Uri("{{uri}}")
    protected val env: WrapEnv? = env ?: defaultEnv
    {{#each methods}}

    fun {{detect_keyword name}}(
        args: {{to_class_name ../type}}Args{{to_class_name name}},
        client: Invoker? = null,
        env: WrapEnv? = null,
        uri: Uri? = null
    ): InvokeResult<{{#with return}}{{to_kotlin (to_graphql_type this)}}{{/with}}> {
        val _client = client ?: this.client
        val _env = env ?: this.env
        val _uri = uri ?: this.uri
        return _client.invoke(_uri, "{{name}}", args, _env)
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