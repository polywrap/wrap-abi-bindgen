// NOTE: This is an auto-generated file.
// All modifications will be overwritten.

import PolywrapClient
import Foundation

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

    func getDefaultEnv() -> TestImportEnv? {
        return nil
    }

    func importedMethod(
        args: TestImportArgsImportedMethod,
        client: PolywrapClient? = nil,
        env: TestImportEnv? = nil,
        uri: Uri? = nil
    ) throws -> TestImportObject? {
        let _client = client ?? self.client ?? getDefaultClient()
        let _uri = uri ?? self.uri ?? getDefaultUri()
        let _env = env ?? self.env ?? getDefaultEnv()
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
        let _env = env ?? self.env ?? getDefaultEnv()
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
        let _env = env ?? self.env ?? getDefaultEnv()
        return try _client.invoke(
            uri: _uri,
            method: "returnsArrayOfEnums",
            args: args,
            env: _env
        )
    }
}

// Imported Modules END //
