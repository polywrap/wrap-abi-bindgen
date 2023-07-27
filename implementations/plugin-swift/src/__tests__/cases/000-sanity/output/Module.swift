/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

import PolywrapClient

struct ArgsModuleMethod: Codable {
    var str: String
    var optStr: String?
    var en: CustomEnum
    var optEnum: CustomEnum?
    var enumArray: [CustomEnum]
    var optEnumArray: [CustomEnum?]?
    var map: [String: Int]
    var mapOfArr: [String: [Int]]
    var mapOfMap: [String: [String: Int]]
    var mapOfObj: [String: AnotherType]
    var mapOfArrOfObj: [String: [AnotherType]]
}

struct ArgsObjectMethod: Codable {
    var object: AnotherType
    var optObject: AnotherType?
    var objectArray: [AnotherType]
    var optObjectArray: [AnotherType?]?
}

struct ArgsOptionalEnvMethod: Codable {
    var object: AnotherType
    var optObject: AnotherType?
    var objectArray: [AnotherType]
    var optObjectArray: [AnotherType?]?
}

struct ArgsIf: Codable {
    var `if`: Else
}

protocol Module: PluginModule {
  func moduleMethod(_ args: ArgsModuleMethod, _ env: VoidCodable?, _ invoker: Invoker) throws -> Int
  func objectMethod(_ args: ArgsObjectMethod, _ env: Env, _ invoker: Invoker) throws -> AnotherType? 
  func optionalEnvMethod(_ args: ArgsOptionalEnvMethod, _ env: Env?, _ invoker: Invoker) throws -> AnotherType?
  func _if(_ args: ArgsIf, _ env: VoidCodable?, _ invoker: Invoker) throws -> Else
}
