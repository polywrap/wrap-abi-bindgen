// NOTE: This is an auto-generated file.
// All modifications will be overwritten.

import PolywrapClient
import Foundation

// Env START //


// Env END //

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


// Enums END //

// Imported objects START //


// Imported objects END //

// Imported envs START //


// Imported envs END //

// Imported enums START //


// Imported enums END //

// Imported modules START //

// Imported Modules END //
