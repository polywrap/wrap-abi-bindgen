// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.

import PolywrapClient
import Foundation

public struct ArgsModuleMethod: Codable {
    var str: String
    var optStr: String?
    var en: CustomEnum
    var optEnum: CustomEnum?
    var enumArray: Array<CustomEnum>
    var optEnumArray: Array<CustomEnum?>?
    var map: [String: Int32]
    var mapOfArr: [String: Array<Int32>]
    var mapOfMap: [String: [String: Int32]]
    var mapOfObj: [String: AnotherType]
    var mapOfArrOfObj: [String: Array<AnotherType>]
}

public struct ArgsObjectMethod: Codable {
    var object: AnotherType
    var optObject: AnotherType?
    var objectArray: Array<AnotherType>
    var optObjectArray: Array<AnotherType?>?
}

public struct ArgsOptionalEnvMethod: Codable {
    var object: AnotherType
    var optObject: AnotherType?
    var objectArray: Array<AnotherType>
    var optObjectArray: Array<AnotherType?>?
}

public struct ArgsIf: Codable {
    var if: Else
}


public protocol Plugin: PluginModule {
    func moduleMethod(_ args: ArgsModuleMethod, _ env: VoidCodable?, _ invoker: Invoker) throws -> Int32

    func objectMethod(_ args: ArgsObjectMethod, _ env: Env, _ invoker: Invoker) throws -> AnotherType?

    func optionalEnvMethod(_ args: ArgsOptionalEnvMethod, _ env: Env?, _ invoker: Invoker) throws -> AnotherType?

    func if(_ args: ArgsIf, _ env: VoidCodable?, _ invoker: Invoker) throws -> Else
}
