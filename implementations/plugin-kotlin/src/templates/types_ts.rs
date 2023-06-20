lazy_static! {
  static ref NAME: String = "types.ts".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

package {{pkg}}

import io.polywrap.core.Invoker
import io.polywrap.core.InvokeResult
import io.polywrap.core.resolution.Uri
import io.polywrap.core.msgpack.MsgPackMap
import kotlinx.serialization.Serializable

typealias BigInt = String
typealias BigNumber = String
typealias Json = String

/// Env START ///
{{#with envType}}
@Serializable
data class Env(
    {{#each properties}}
    val {{detectKeyword name}}: {{nullableDefault (toKotlin toGraphQLType)}},
    {{/each}}
)
{{/with}}
/// Env END ///

/// Objects START ///
{{#each objectTypes}}
@Serializable
data class {{toClassName type}}(
    {{#each properties}}
    val {{detectKeyword name}}: {{nullableDefault (toKotlin toGraphQLType)}},
    {{/each}}
)

{{/each}}
/// Objects END ///

/// Enums START ///
{{#each enumTypes}}
@Serializable
enum class {{toClassName type}} {
    {{#constants}}
    {{detectKeywordStrict this}}{{#if (is_not_last @index ../constants)}},{{/if}}
    {{/constants}}
}

{{/each}}
/// Enums END ///

/// Imported Objects START ///
{{#each importedObjectTypes}}
/* URI: "{{uri}}" */
@Serializable
data class {{toClassName type}}(
    {{#each properties}}
    val {{detectKeyword name}}: {{nullableDefault (toKotlin toGraphQLType)}},
    {{/each}}
)

{{/each}}
{{#each importedEnumTypes}}
/* URI: "{{uri}}" */
@Serializable
enum class {{toClassName type}} {
    {{#constants}}
    {{detectKeywordStrict this}}{{#if (is_not_last @index ../constants)}},{{/if}}
    {{/constants}}
}

{{/each}}
/// Imported Objects END ///

/// Imported Modules START ///
{{#each importedModuleTypes}}
{{#each methods}}
/* URI: "{{../uri}}" */
@Serializable
data class {{toClassName ../type}}Args{{toClassName name}}(
    {{#each arguments}}
    val {{detectKeyword name}}: {{nullableDefault (toKotlin toGraphQLType)}},
    {{/each}}
)

{{/each}}
/* URI: "{{uri}}" */
{{#if isInterface}}
class {{toClassName type}}(uri: String) {
    companion object {
        val interfaceUri: String = "{{uri}}"
    }

    val uri: Uri = Uri.fromString(uri)

    {{#each methods}}
    suspend fun {{detectKeyword name}}(
        args: {{toClassName ../type}}Args{{toClassName name}},
        invoker: Invoker
    ): InvokeResult<{{#with return}}{{toKotlin toGraphQLType}}{{/with}}> {
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
object {{toClassName type}} {
    {{#each methods}}
    suspend fun {{detectKeyword name}}(
        args: {{toClassName ../type}}Args{{toClassName name}},
        invoker: Invoker
    ): InvokeResult<{{#with return}}{{toKotlin toGraphQLType}}{{/with}}> {
        return invoker.invoke(
            uri = Uri.fromString("{{../uri}}"),
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

object {{toClassName namespace}} {
    val uri: Uri = Uri.fromString("{{uri}}");

    {{#with capabilities}}
    {{#with getImplementations}}
    {{#if enabled}}
    suspend fun getImplementations(invoker: Invoker): List<String> {
        val implementations = invoker.getImplementations(this.uri)
        val uriStrings = implementations.map { it.toStringUri() }
        implementations.forEach { it.close() }
        return uriStrings
    }
    {{/if}}
    {{/with}}
    {{/with}}
}
{{/each}}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
