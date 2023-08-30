// NOTE: This is an auto-generated file.
// All modifications will be overwritten.

import PolywrapClient
import Foundation

// Objects START //

public struct CustomType: Codable {
    var str: String
    var optStr: String?
    var u: UInt32
    var optU: UInt32?
    var u8: UInt8
    var u16: UInt16
    var u32: UInt32
    var i: Int32
    var i8: Int8
    var i16: Int16
    var i32: Int32
    var bigint: String
    var optBigint: String?
    var bignumber: String
    var optBignumber: String?
    var json: String
    var optJson: String?
    var bytes: Data
    var optBytes: Data?
    var boolean: Bool
    var optBoolean: Bool?
    var u_array: Array<UInt32>
    var uOpt_array: Array<UInt32>?
    var _opt_uOptArray: Array<UInt32?>?
    var optStrOptArray: Array<String?>?
    var uArrayArray: Array<Array<UInt32>>
    var uOptArrayOptArray: Array<Array<UInt32?>?>
    var uArrayOptArrayArray: Array<Array<Array<UInt32>>?>
    var crazyArray: Array<Array<Array<Array<UInt32>?>>?>?
    var object: AnotherType
    var optObject: AnotherType?
    var objectArray: Array<AnotherType>
    var optObjectArray: Array<AnotherType?>?
    var en: CustomEnum
    var optEnum: CustomEnum?
    var enumArray: Array<CustomEnum>
    var optEnumArray: Array<CustomEnum?>?
    var map: [String: Int32]
    var mapOfArr: [String: Array<Int32>]
    var mapOfObj: [String: AnotherType]
    var mapOfArrOfObj: [String: Array<AnotherType>]
    var mapCustomValue: [String: CustomMapValue?]
}

public struct AnotherType: Codable {
    var prop: String?
    var circular: CustomType?
    var const: String?
}

public struct CustomMapValue: Codable {
    var foo: String
}

public struct Else: Codable {
    var else: String
}


// Objects END //

// Enums START //

public enum CustomEnum: Int, Codable {
    case STRING
    case BYTES
}

public enum While: Int, Codable {
    case _for
    case _in
}


// Enums END //

// Imported objects START //

public struct TestImportObject: Codable {
    var object: TestImportAnotherObject
    var optObject: TestImportAnotherObject?
    var objectArray: Array<TestImportAnotherObject>
    var optObjectArray: Array<TestImportAnotherObject?>?
    var en: TestImportEnum
    var optEnum: TestImportEnum?
    var enumArray: Array<TestImportEnum>
    var optEnumArray: Array<TestImportEnum?>?
}

public struct TestImportAnotherObject: Codable {
    var prop: String
}


// Imported objects END //

// Imported envs START //

public struct TestImportEnv: Codable {
    var object: TestImportAnotherObject
    var optObject: TestImportAnotherObject?
    var objectArray: Array<TestImportAnotherObject>
    var optObjectArray: Array<TestImportAnotherObject?>?
    var en: TestImportEnum
    var optEnum: TestImportEnum?
    var enumArray: Array<TestImportEnum>
    var optEnumArray: Array<TestImportEnum?>?
}

// Imported envs END //

// Imported enums START //

public enum TestImportEnum: Int, Codable {
    case STRING
    case BYTES
}

public enum TestImportEnumReturn: Int, Codable {
    case STRING
    case BYTES
}


// Imported enums END //

// Imported modules START //

// URI: "testimport.uri.eth" //
public struct TestImportArgsImportedMethod: Codable {
    var str: String
    var optStr: String?
    var u: UInt32
    var optU: UInt32?
    var uArrayArray: Array<Array<UInt32?>?>
    var object: TestImportObject
    var optObject: TestImportObject?
    var objectArray: Array<TestImportObject>
    var optObjectArray: Array<TestImportObject?>?
    var en: TestImportEnum
    var optEnum: TestImportEnum?
    var enumArray: Array<TestImportEnum>
    var optEnumArray: Array<TestImportEnum?>?
}

// URI: "testimport.uri.eth" //
public struct TestImportArgsAnotherMethod: Codable {
    var arg: Array<String>
}

// URI: "testimport.uri.eth" //
public struct TestImportArgsReturnsArrayOfEnums: Codable {
    var arg: String
}

/* URI: "testimport.uri.eth" */
class TestImport {
    static let uri: Uri = try! Uri("testimport.uri.eth")

    var client: PolywrapClient? = nil
    var env: TestImportEnv? = nil
    var uri: Uri? = nil

    init(client: PolywrapClient? = nil, env: TestImportEnv? = nil, uri: Uri? = nil) {
        self.client = client
        self.env = env
        self.uri = uri
    }

    func getDefaultClient() -> PolywrapClient {
        if let client = self.client {
            return client
        } else {
            let newClient = BuilderConfig().addSystemDefault().addWeb3Default().build()
            self.client = newClient
            return newClient
        }
    }

    func getDefaultUri() -> Uri {
        if let uri = self.uri {
            return uri
        } else {
            let newUri = TestImport.uri
            self.uri = newUri
            return newUri
        }
    }

    func importedMethod(
        args: TestImportArgsImportedMethod,
        client: PolywrapClient? = nil,
        env: TestImportEnv? = nil,
        uri: Uri? = nil
    ) throws -> TestImportObject? {
        let _client = client ?? self.client ?? getDefaultClient()
        let _uri = uri ?? self.uri ?? getDefaultUri()
        let _env = env ?? self.env
        return try _client.invoke(
            uri: _uri,
            method: "importedMethod",
            args: args,
            env: _env
        )
    }

    func anotherMethod(
        args: TestImportArgsAnotherMethod,
        client: PolywrapClient? = nil,
        env: TestImportEnv? = nil,
        uri: Uri? = nil
    ) throws -> Int32 {
        let _client = client ?? self.client ?? getDefaultClient()
        let _uri = uri ?? self.uri ?? getDefaultUri()
        let _env = env ?? self.env
        return try _client.invoke(
            uri: _uri,
            method: "anotherMethod",
            args: args,
            env: _env
        )
    }

    func returnsArrayOfEnums(
        args: TestImportArgsReturnsArrayOfEnums,
        client: PolywrapClient? = nil,
        env: TestImportEnv? = nil,
        uri: Uri? = nil
    ) throws -> Array<TestImportEnumReturn?> {
        let _client = client ?? self.client ?? getDefaultClient()
        let _uri = uri ?? self.uri ?? getDefaultUri()
        let _env = env ?? self.env
        return try _client.invoke(
            uri: _uri,
            method: "returnsArrayOfEnums",
            args: args,
            env: _env
        )
    }
}

// Imported Modules END //
