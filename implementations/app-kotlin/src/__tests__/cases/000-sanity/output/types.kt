/// NOTE: This is an auto-generated file.
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
@Serializable
data class CustomType(
    val str: String,
    val optStr: String? = null,
    val u: UInt,
    val optU: UInt? = null,
    val u8: UByte,
    val u16: UShort,
    val u32: UInt,
    val i: Int,
    val i8: Byte,
    val i16: Short,
    val i32: Int,
    val bigint: BigInt,
    val optBigint: BigInt? = null,
    val bignumber: BigNumber,
    val optBignumber: BigNumber? = null,
    val json: Json,
    val optJson: Json? = null,
    val bytes: ByteArray,
    val optBytes: ByteArray? = null,
    val boolean: Boolean,
    val optBoolean: Boolean? = null,
    val u_array: List<UInt>,
    val uOpt_array: List<UInt>? = null,
    val _opt_uOptArray: List<UInt?>? = null,
    val optStrOptArray: List<String?>? = null,
    val uArrayArray: List<List<UInt>>,
    val uOptArrayOptArray: List<List<UInt?>?>,
    val uArrayOptArrayArray: List<List<List<UInt>>?>,
    val crazyArray: List<List<List<List<UInt>?>>?>? = null,
    val _object: AnotherType,
    val optObject: AnotherType? = null,
    val objectArray: List<AnotherType>,
    val optObjectArray: List<AnotherType?>? = null,
    val en: CustomEnum,
    val optEnum: CustomEnum? = null,
    val enumArray: List<CustomEnum>,
    val optEnumArray: List<CustomEnum?>? = null,
    val map: GenericMap<String, Int>,
    val mapOfArr: GenericMap<String, List<Int>>,
    val mapOfObj: GenericMap<String, AnotherType>,
    val mapOfArrOfObj: GenericMap<String, List<AnotherType>>,
    val mapCustomValue: GenericMap<String, CustomMapValue?>,
)

@Serializable
data class AnotherType(
    val prop: String? = null,
    val circular: CustomType? = null,
    val const: String? = null,
)

@Serializable
data class CustomMapValue(
    val foo: String,
)

@Serializable
data class Else(
    val _else: String,
)

/// Objects END ///

/// Enums START ///
@Serializable
enum class CustomEnum {
    STRING,
    BYTES
}

@Serializable
enum class While {
    _for,
    _in
}

/// Enums END ///

/// Imported Objects START ///
/* URI: "testimport.uri.eth" */
@Serializable
data class TestImportObject(
    val _object: TestImportAnotherObject,
    val optObject: TestImportAnotherObject? = null,
    val objectArray: List<TestImportAnotherObject>,
    val optObjectArray: List<TestImportAnotherObject?>? = null,
    val en: TestImportEnum,
    val optEnum: TestImportEnum? = null,
    val enumArray: List<TestImportEnum>,
    val optEnumArray: List<TestImportEnum?>? = null,
)

/* URI: "testimport.uri.eth" */
@Serializable
data class TestImportAnotherObject(
    val prop: String,
)

/// Imported Objects END ///

/// Imported Enums START ///
/* URI: "testimport.uri.eth" */
@Serializable
enum class TestImportEnum {
    STRING,
    BYTES
}

/* URI: "testimport.uri.eth" */
@Serializable
enum class TestImportEnumReturn {
    STRING,
    BYTES
}

/// Imported Enums END ///

/// Imported Envs START ///
@Serializable
data class TestImportEnv(
    val _object: TestImportAnotherObject,
    val optObject: TestImportAnotherObject? = null,
    val objectArray: List<TestImportAnotherObject>,
    val optObjectArray: List<TestImportAnotherObject?>? = null,
    val en: TestImportEnum,
    val optEnum: TestImportEnum? = null,
    val enumArray: List<TestImportEnum>,
    val optEnumArray: List<TestImportEnum?>? = null,
)

/// Imported Envs END ///

/// Imported Modules START ///
/* URI: "testimport.uri.eth" */
@Serializable
data class TestImportModuleArgsImportedMethod(
    val str: String,
    val optStr: String? = null,
    val u: UInt,
    val optU: UInt? = null,
    val uArrayArray: List<List<UInt?>?>,
    val _object: TestImportObject,
    val optObject: TestImportObject? = null,
    val objectArray: List<TestImportObject>,
    val optObjectArray: List<TestImportObject?>? = null,
    val en: TestImportEnum,
    val optEnum: TestImportEnum? = null,
    val enumArray: List<TestImportEnum>,
    val optEnumArray: List<TestImportEnum?>? = null,
)

/* URI: "testimport.uri.eth" */
@Serializable
data class TestImportModuleArgsAnotherMethod(
    val arg: List<String>,
)

/* URI: "testimport.uri.eth" */
@Serializable
data class TestImportModuleArgsReturnsArrayOfEnums(
    val arg: String,
)

/* URI: "testimport.uri.eth" */
abstract class BaseTestImportModule(
    client: Invoker? = null,
    env: TestImportEnv? = null,
    uri: Uri? = null
) {
    protected abstract val defaultClient: Invoker?
    protected abstract val defaultUri: Uri?
    protected abstract val defaultEnv: TestImportEnv?

    protected val client: Invoker = client ?: defaultClient ?: polywrapClient { addDefaults() }
    protected val uri: Uri = uri ?: defaultUri ?: Uri("testimport.uri.eth")
    protected val env: TestImportEnv? = env ?: defaultEnv

    fun importedMethod(
        args: TestImportModuleArgsImportedMethod,
        client: Invoker? = null,
        env: TestImportEnv? = null,
        uri: Uri? = null
    ): InvokeResult<TestImportObject?> {
        val _client = client ?: this.client
        val _env = env ?: this.env
        val _uri = uri ?: this.uri
        return _client.invoke(_uri, "importedMethod", args, _env)
    }

    fun anotherMethod(
        args: TestImportModuleArgsAnotherMethod,
        client: Invoker? = null,
        env: TestImportEnv? = null,
        uri: Uri? = null
    ): InvokeResult<Int> {
        val _client = client ?: this.client
        val _env = env ?: this.env
        val _uri = uri ?: this.uri
        return _client.invoke(_uri, "anotherMethod", args, _env)
    }

    fun returnsArrayOfEnums(
        args: TestImportModuleArgsReturnsArrayOfEnums,
        client: Invoker? = null,
        env: TestImportEnv? = null,
        uri: Uri? = null
    ): InvokeResult<List<TestImportEnumReturn?>> {
        val _client = client ?: this.client
        val _env = env ?: this.env
        val _uri = uri ?: this.uri
        return _client.invoke(_uri, "returnsArrayOfEnums", args, _env)
    }
}
/// Imported Modules END ///
