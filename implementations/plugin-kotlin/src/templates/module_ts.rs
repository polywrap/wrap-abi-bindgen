lazy_static! {
  static ref NAME: String = "module.ts".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

package {{to_package_id name}}

{{#with abi}}
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
data class Args{{to_class_name name}}(
    {{#each arguments}}
    val {{detect_keyword name}}: {{nullable_default (to_kotlin (to_graphql_type this))}},
    {{/each}}
)
{{/each}}
{{/with}}

@Suppress("UNUSED_PARAMETER", "FunctionName")
abstract class Module<TConfig>(config: TConfig) : PluginModule<TConfig>(config) {

  final override val methods: Map<String, PluginMethod> = mapOf(
  {{#with moduleType}}
  {{#each methods}}
      "{{name}}" to ::__{{name}},
  {{/each}}
  {{/with}}
  )

  {{#with moduleType}}
  {{#each methods}}
  abstract suspend fun {{detect_keyword name}}(
      args: Args{{to_class_name name}},{{#if env}}{{#with env}}
      env: Env{{#if required}}{{else}}? = null{{/if}},{{/with}}{{/if}}
      invoker: Invoker
  ): {{#with return}}{{to_kotlin (to_graphql_type this)}}{{/with}}

  {{/each}}
  {{/with}}
  {{#with moduleType}}
  {{#each methods}}
  private suspend fun __{{name}}(
      encodedArgs: ByteArray?,
      encodedEnv: ByteArray?,
      invoker: Invoker
    ): ByteArray {
        val args: Args{{to_class_name name}} = encodedArgs?.let {
            msgPackDecode(Args{{to_class_name name}}.serializer(), it).getOrNull()
                ?: throw Exception("Failed to decode args in invocation to plugin method '{{name}}'")
        } ?: throw Exception("Missing args in invocation to plugin method '{{name}}'")
        {{#if env}}
        val env: Env = encodedEnv?.let {
            msgPackDecode(Env.serializer(), it).getOrNull()
                ?: throw Exception("Failed to decode env in invocation to plugin method '{{name}}'")
        } ?: throw Exception("Missing env in invocation to plugin method '{{name}}'")
        {{/if}}
        val response = {{detect_keyword name}}(args, {{#if env}}env, {{/if}}invoker)
        return msgPackEncode(serializer(), response)
  }
  {{#if (is_not_last @index ../methods)}}

  {{/if}}
  {{/each}}
  {{/with}}
}
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
