// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
import Foundation

// Env START //

struct Env: Codable {
    var prop: String
    var optProp: String? // Option in Rust, Optional in Swift
    var optMap: [String: Int?]? // BTreeMap in Rust, Dictionary in Swift
}

// Env END //

// Objects START //

struct CustomType: Codable {
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
    var uArray: [UInt32]
    var uOptArray: [UInt32]?
    var optUOptArray: [UInt32?]?
    var optStrOptArray: [String?]?
    var uArrayArray: [[UInt32]]
    var uOptArrayOptArray: [[UInt32]?]
    var uArrayOptArrayArray: [[[UInt32]]]?
    var crazyArray: [[[UInt32?]]]?
    var object: AnotherType
    var optObject: AnotherType?
    var objectArray: [AnotherType]
    var optObjectArray: [AnotherType?]?
    var en: CustomEnum
    var optEnum: CustomEnum?
    var enumArray: [CustomEnum]
    var optEnumArray: [CustomEnum?]?
    var map: [String: Int32]
    var mapOfArr: [String: [Int32]]
    var mapOfObj: [String: AnotherType]
    var mapOfArrOfObj: [String: [AnotherType]]
    var mapCustomValue: [String: CustomMapValue?]
}


struct AnotherType: Codable {
    var prop: String?
    var circular: CustomType?
    var const: String?
}

struct CustomMapValue {
    var foo: String
}

struct Else {
    var `else`: String
}

// Objects END //

// Enums START //

enum CustomEnum: String, Codable {
    case STRING
    case BYTES
    case _MAX_
}

enum While: String, Codable {
    case _for
    case _in
    case _MAX_
}

// Enums END //

// Imported objects START //

struct TestImportObject: Codable {
    var object: TestImportAnotherObject
    var optObject: TestImportAnotherObject?
    var objectArray: [TestImportAnotherObject]
    var optObjectArray: [TestImportAnotherObject?]?
    var en: TestImportEnum
    var optEnum: TestImportEnum?
    var enumArray: [TestImportEnum]
    var optEnumArray: [TestImportEnum?]?
}

struct TestImportAnotherObject: Codable {
    var prop: String
}

// Imported objects END //

// Imported envs START //

// Imported envs END //

// Imported enums START //

enum TestImportEnum: String, Codable {
    case string = "STRING"
    case bytes = "BYTES"
    case max = "_MAX_"
}

enum TestImportEnumReturn: String, Codable {
    case string = "STRING"
    case bytes = "BYTES"
    case max = "_MAX_"
}

// Imported enums END //

// Imported Modules START //

// URI: "testimport.uri.eth" //
struct TestImportModuleArgsImportedMethod: Codable {
    var str: String
    var optStr: String?
    var u: UInt32
    var optU: UInt32?
    var uArrayArray: [[UInt32?]?]?
    var object: TestImportObject
    var optObject: TestImportObject?
    var objectArray: [TestImportObject]
    var optObjectArray: [TestImportObject?]?
    var en: TestImportEnum
    var optEnum: TestImportEnum?
    var enumArray: [TestImportEnum]
    var optEnumArray: [TestImportEnum?]?
}


// URI: "testimport.uri.eth" //
struct TestImportModuleArgsAnotherMethod: Codable {
    var arg: [String]
}


// URI: "testimport.uri.eth" //
struct TestImportModuleArgsReturnsArrayOfEnums: Codable {
    var arg: String
}

class TestImportModule {
    static let INTERFACE_URI = "testimport.uri.eth"
    var uri: String
    
    init(uri: String) {
        self.uri = uri
    }
    
    func importedMethod(_ args: TestImportModuleArgsImportedMethod, _ env: VoidCodable?, _ invoker: Invoker) throws -> TestImportObject {
        let serializedArgs = try encode(value: args)
        let resultData = try invoker.invokeRaw(
            uri: self.uri,
            method: "importedMethod",
            args: serializedArgs,
            env: nil
        )
        
        return try decode(value: resultData)
    }
    
    func anotherMethod(_ args: TestImportModuleArgsAnotherMethod, _ env: VoidCodable?, _ invoker: Invoker) throws -> Int {
        let serializedArgs = try encode(value: args)
        let resultData = try invoker.invokeRaw(
            uri: self.uri,
            method: "anotherMethod",
            args: serializedArgs,
            env: nil
        )
        
        return try decode(value: resultData)
    }
    
    func returnsArrayOfEnums(_ args: TestImportModuleArgsReturnsArrayOfEnums,  _ env: VoidCodable?, _ invoker: Invoker) throws -> [TestImportEnumReturn?] {
        let serializedArgs = try encode(value: args)
        let resultData = try invoker.invokeRaw(
            uri: self.uri,
            method: "returnsArrayOfEnums",
            args: serializedArgs,
            env: nil,
         )

        return try decode(value: resultData)
    }
}

// Imported Modules END //
