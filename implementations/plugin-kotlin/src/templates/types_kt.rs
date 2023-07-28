lazy_static! {
  static ref NAME: String = "types.kt".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

package {{to_package_id name}}

{{#with abi}}
import io.polywrap.core.Invoker
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import io.polywrap.core.msgpack.GenericMap
import kotlinx.serialization.Serializable

typealias BigInt = String
typealias BigNumber = String
typealias Json = String

/// Env START ///
{{#with envType}}
@Serializable
{{#if (array_has_length properties)}}data{{/if}} class Env(
    {{#each properties}}
    val {{detect_keyword name}}: {{nullable_default (to_kotlin (to_graphql_type this))}},
    {{/each}}
)
{{/with}}
/// Env END ///

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
{{#each importedEnumTypes}}
/* URI: "{{uri}}" */
@Serializable
enum class {{to_class_name type}} {
    {{#each constants}}
    {{detect_keyword_strict this}}{{#if (is_not_last @index ../constants)}},{{/if}}
    {{/each}}
}

{{/each}}
/// Imported Objects END ///

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
{{#if isInterface}}
class {{to_class_name type}}(uri: String) {
    companion object {
        val interfaceUri: String = "{{uri}}"
    }

    val uri: Uri = Uri(uri)

    {{#each methods}}
    suspend fun {{detect_keyword name}}(
        args: {{to_class_name ../type}}Args{{to_class_name name}},
        invoker: Invoker
    ): InvokeResult<{{#with return}}{{to_kotlin (to_graphql_type this)}}{{/with}}> {
        return invoker.invoke(
            uri = this.uri,
            method = "{{name}}",
            args = args
        );
    }
    {{#if (is_not_last @index ../methods)}}

    {{/if}}
    {{/each}}
}

{{else}}
object {{to_class_name type}} {
    {{#each methods}}
    suspend fun {{detect_keyword name}}(
        args: {{to_class_name ../type}}Args{{to_class_name name}},
        invoker: Invoker
    ): InvokeResult<{{#with return}}{{to_kotlin (to_graphql_type this)}}{{/with}}> {
        return invoker.invoke(
            uri = Uri("{{../uri}}"),
            method = "{{name}}",
            args = args
        );
    }{{#if (is_not_last @index ../methods)}},{{/if}}
    {{#if (is_not_last @index ../methods)}}

    {{/if}}
    {{/each}}
}

{{/if}}
{{/each}}
/// Imported Modules END ///
{{#each interfaceTypes}}

object {{to_class_name namespace}} {
    val uri: Uri = Uri("{{uri}}");

    {{#with capabilities}}
    {{#with getImplementations}}
    {{#if enabled}}
    suspend fun getImplementations(invoker: Invoker): Result<List<Uri>> {
        return invoker.getImplementations(this.uri)
    }
    {{/if}}
    {{/with}}
    {{/with}}
}
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
