// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.

import PolywrapClient
import Foundation

public struct ArgsFunction1: Codable {
    var arg1: UInt32
    var arg2: Bool
}

public struct ArgsFunction2: Codable {
    var arg1: Int32?
    var arg2: Data?
}


protocol Plugin: PluginModule {
    func function1(_ args: ArgsFunction1, _ env: VoidCodable?, _ invoker: Invoker) throws -> String

    func function2(_ args: ArgsFunction2, _ env: VoidCodable?, _ invoker: Invoker) throws -> String?
}
