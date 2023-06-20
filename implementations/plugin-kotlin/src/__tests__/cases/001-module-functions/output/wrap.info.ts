/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
import { WrapManifest } from "@polywrap/wrap-manifest-types-js"

export const manifest: WrapManifest = {
  name: "001-module-functions",
  type: "plugin",
  version: "0.1",
  abi: {
  "moduleType": {
    "kind": 128,
    "methods": [
      {
        "arguments": [
          {
            "kind": 34,
            "name": "arg1",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "arg1",
              "required": true,
              "type": "UInt32"
            },
            "type": "UInt32"
          },
          {
            "kind": 34,
            "name": "arg2",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "arg2",
              "required": true,
              "type": "Boolean"
            },
            "type": "Boolean"
          }
        ],
        "kind": 64,
        "name": "function1",
        "required": true,
        "return": {
          "kind": 34,
          "name": "function1",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "function1",
            "required": true,
            "type": "String"
          },
          "type": "String"
        },
        "type": "Method"
      },
      {
        "arguments": [
          {
            "kind": 34,
            "name": "arg1",
            "scalar": {
              "kind": 4,
              "name": "arg1",
              "type": "Int32"
            },
            "type": "Int32"
          },
          {
            "kind": 34,
            "name": "arg2",
            "scalar": {
              "kind": 4,
              "name": "arg2",
              "type": "Bytes"
            },
            "type": "Bytes"
          }
        ],
        "kind": 64,
        "name": "function2",
        "required": true,
        "return": {
          "kind": 34,
          "name": "function2",
          "scalar": {
            "kind": 4,
            "name": "function2",
            "type": "String"
          },
          "type": "String"
        },
        "type": "Method"
      }
    ],
    "type": "Module"
  },
  "version": "0.1"
}
}
