/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use polywrap_plugin::JSON::{from_value, json};
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};

pub fn get_manifest() -> WrapManifest {
  WrapManifest {
    name: "001-module-functions".to_string(),
    type_: "plugin".to_string(),
    version: "0.1".to_string(),
    abi: from_value::<WrapManifestAbi>(json!({
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
})).unwrap()
  }
}
