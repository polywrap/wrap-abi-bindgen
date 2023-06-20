lazy_static! {
  static ref NAME: String = "module.ts".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

package {{pkg}}

import io.polywrap.core.Invoker
import io.polywrap.core.msgpack.msgPackDecode
import io.polywrap.core.msgpack.msgPackEncode
import io.polywrap.core.msgpack.MsgPackMap
import io.polywrap.plugin.PluginMethod
import io.polywrap.plugin.PluginModule
import kotlinx.serialization.Serializable
import kotlinx.serialization.serializer
{{#with moduleType}}
{{#each methods}}

@Serializable
data class Args{{toClassName name}}(
    {{#each arguments}}
    val {{detectKeyword name}}: {{#nullableDefault}}{{toKotlin toGraphQLType}}{{/nullableDefault}},
    {{/each}}
)
{{/each}}
{{/with}}

@Suppress("UNUSED_PARAMETER", "FunctionName")
abstract class Module<TConfig>(config: TConfig) : PluginModule<TConfig>(config) {

  final override val methods: Map<String, PluginMethod> = mapOf(
  {{#mwith oduleType}}
  {{#each methods}}
      "{{name}}" to ::__{{name}},
  {{/each}}
  {{/moduleType}}
  )

  {{#moduleType}}
  {{#each methods}}
  abstract suspend fun {{detectKeyword name}}(
      args: Args{{toClassName name}},{{#if env}}{{#with env}}
      env: Env{{#if required}}{{else}}? = null{{/if}},{{/with}}{{/if}}
      invoker: Invoker
  ): {{#with return}}{{toKotlin (toGraphQLType this)}}{{/with}}

  {{/each}}
  {{/with moduleType}}
  {{#with moduleType}}
  {{#each methods}}
  private suspend fun __{{name}}(
      encodedArgs: ByteArray?,
      encodedEnv: ByteArray?,
      invoker: Invoker
    ): ByteArray {
        val args: Args{{toClassName name}} = encodedArgs?.let {
            msgPackDecode(Args{{toClassName name}}.serializer(), it).getOrNull()
                ?: throw Exception("Failed to decode args in invocation to plugin method '{{name}}'")
        } ?: throw Exception("Missing args in invocation to plugin method '{{name}}'")
        {{#if env}}
        val env: Env = encodedEnv?.let {
            msgPackDecode(Env.serializer(), it).getOrNull()
                ?: throw Exception("Failed to decode env in invocation to plugin method '{{name}}'")
        } ?: throw Exception("Missing env in invocation to plugin method '{{name}}'")
        {{/if}}
        val response = {{detectKeyword name}}(args, {{#if env}}env, {{/if}}invoker)
        return msgPackEncode(serializer(), response)
  }
  {{#if (is_not_last @index ../methods)}}

  {{/if}}
  {{/each}}
  {{/with}}
}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
