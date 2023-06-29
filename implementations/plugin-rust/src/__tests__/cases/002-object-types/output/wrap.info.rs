/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use polywrap_plugin::JSON::{from_value, json};
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};

pub fn get_manifest() -> WrapManifest {
  WrapManifest {
    name: "002-object-types".to_string(),
    type_: "plugin".to_string(),
    version: "0.1".to_string(),
    abi: from_value::<WrapManifestAbi>(json!({
  "objectTypes": [
    {
      "kind": 1,
      "properties": [
        {
          "kind": 34,
          "name": "str",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "str",
            "required": true,
            "type": "String"
          },
          "type": "String"
        },
        {
          "kind": 34,
          "name": "optStr",
          "scalar": {
            "kind": 4,
            "name": "optStr",
            "type": "String"
          },
          "type": "String"
        },
        {
          "kind": 34,
          "name": "u",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "u",
            "required": true,
            "type": "UInt"
          },
          "type": "UInt"
        },
        {
          "kind": 34,
          "name": "optU",
          "scalar": {
            "kind": 4,
            "name": "optU",
            "type": "UInt"
          },
          "type": "UInt"
        },
        {
          "kind": 34,
          "name": "u8",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "u8",
            "required": true,
            "type": "UInt8"
          },
          "type": "UInt8"
        },
        {
          "kind": 34,
          "name": "u16",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "u16",
            "required": true,
            "type": "UInt16"
          },
          "type": "UInt16"
        },
        {
          "kind": 34,
          "name": "u32",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "u32",
            "required": true,
            "type": "UInt32"
          },
          "type": "UInt32"
        },
        {
          "kind": 34,
          "name": "i",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "i",
            "required": true,
            "type": "Int"
          },
          "type": "Int"
        },
        {
          "kind": 34,
          "name": "i8",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "i8",
            "required": true,
            "type": "Int8"
          },
          "type": "Int8"
        },
        {
          "kind": 34,
          "name": "i16",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "i16",
            "required": true,
            "type": "Int16"
          },
          "type": "Int16"
        },
        {
          "kind": 34,
          "name": "i32",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "i32",
            "required": true,
            "type": "Int32"
          },
          "type": "Int32"
        },
        {
          "kind": 34,
          "name": "bigint",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "bigint",
            "required": true,
            "type": "BigInt"
          },
          "type": "BigInt"
        },
        {
          "kind": 34,
          "name": "optBigint",
          "scalar": {
            "kind": 4,
            "name": "optBigint",
            "type": "BigInt"
          },
          "type": "BigInt"
        },
        {
          "kind": 34,
          "name": "bignumber",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "bignumber",
            "required": true,
            "type": "BigNumber"
          },
          "type": "BigNumber"
        },
        {
          "kind": 34,
          "name": "optBignumber",
          "scalar": {
            "kind": 4,
            "name": "optBignumber",
            "type": "BigNumber"
          },
          "type": "BigNumber"
        },
        {
          "kind": 34,
          "name": "json",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "json",
            "required": true,
            "type": "JSON"
          },
          "type": "JSON"
        },
        {
          "kind": 34,
          "name": "optJson",
          "scalar": {
            "kind": 4,
            "name": "optJson",
            "type": "JSON"
          },
          "type": "JSON"
        },
        {
          "kind": 34,
          "name": "bytes",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "bytes",
            "required": true,
            "type": "Bytes"
          },
          "type": "Bytes"
        },
        {
          "kind": 34,
          "name": "optBytes",
          "scalar": {
            "kind": 4,
            "name": "optBytes",
            "type": "Bytes"
          },
          "type": "Bytes"
        },
        {
          "kind": 34,
          "name": "boolean",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "boolean",
            "required": true,
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        {
          "kind": 34,
          "name": "optBoolean",
          "scalar": {
            "kind": 4,
            "name": "optBoolean",
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        {
          "array": {
            "item": {
              "kind": 4,
              "name": "u_array",
              "required": true,
              "type": "UInt"
            },
            "kind": 18,
            "name": "u_array",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "u_array",
              "required": true,
              "type": "UInt"
            },
            "type": "[UInt]"
          },
          "kind": 34,
          "name": "u_array",
          "required": true,
          "type": "[UInt]"
        },
        {
          "array": {
            "item": {
              "kind": 4,
              "name": "uOpt_array",
              "required": true,
              "type": "UInt"
            },
            "kind": 18,
            "name": "uOpt_array",
            "scalar": {
              "kind": 4,
              "name": "uOpt_array",
              "required": true,
              "type": "UInt"
            },
            "type": "[UInt]"
          },
          "kind": 34,
          "name": "uOpt_array",
          "type": "[UInt]"
        },
        {
          "array": {
            "item": {
              "kind": 4,
              "name": "_opt_uOptArray",
              "type": "UInt"
            },
            "kind": 18,
            "name": "_opt_uOptArray",
            "scalar": {
              "kind": 4,
              "name": "_opt_uOptArray",
              "type": "UInt"
            },
            "type": "[UInt]"
          },
          "kind": 34,
          "name": "_opt_uOptArray",
          "type": "[UInt]"
        },
        {
          "array": {
            "item": {
              "kind": 4,
              "name": "optStrOptArray",
              "type": "String"
            },
            "kind": 18,
            "name": "optStrOptArray",
            "scalar": {
              "kind": 4,
              "name": "optStrOptArray",
              "type": "String"
            },
            "type": "[String]"
          },
          "kind": 34,
          "name": "optStrOptArray",
          "type": "[String]"
        },
        {
          "array": {
            "array": {
              "item": {
                "kind": 4,
                "name": "uArrayArray",
                "required": true,
                "type": "UInt"
              },
              "kind": 18,
              "name": "uArrayArray",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "uArrayArray",
                "required": true,
                "type": "UInt"
              },
              "type": "[UInt]"
            },
            "item": {
              "item": {
                "kind": 4,
                "name": "uArrayArray",
                "required": true,
                "type": "UInt"
              },
              "kind": 18,
              "name": "uArrayArray",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "uArrayArray",
                "required": true,
                "type": "UInt"
              },
              "type": "[UInt]"
            },
            "kind": 18,
            "name": "uArrayArray",
            "required": true,
            "type": "[[UInt]]"
          },
          "kind": 34,
          "name": "uArrayArray",
          "required": true,
          "type": "[[UInt]]"
        },
        {
          "array": {
            "array": {
              "item": {
                "kind": 4,
                "name": "uOptArrayOptArray",
                "type": "UInt32"
              },
              "kind": 18,
              "name": "uOptArrayOptArray",
              "scalar": {
                "kind": 4,
                "name": "uOptArrayOptArray",
                "type": "UInt32"
              },
              "type": "[UInt32]"
            },
            "item": {
              "item": {
                "kind": 4,
                "name": "uOptArrayOptArray",
                "type": "UInt32"
              },
              "kind": 18,
              "name": "uOptArrayOptArray",
              "scalar": {
                "kind": 4,
                "name": "uOptArrayOptArray",
                "type": "UInt32"
              },
              "type": "[UInt32]"
            },
            "kind": 18,
            "name": "uOptArrayOptArray",
            "required": true,
            "type": "[[UInt32]]"
          },
          "kind": 34,
          "name": "uOptArrayOptArray",
          "required": true,
          "type": "[[UInt32]]"
        },
        {
          "array": {
            "array": {
              "array": {
                "item": {
                  "kind": 4,
                  "name": "uArrayOptArrayArray",
                  "required": true,
                  "type": "UInt32"
                },
                "kind": 18,
                "name": "uArrayOptArrayArray",
                "required": true,
                "scalar": {
                  "kind": 4,
                  "name": "uArrayOptArrayArray",
                  "required": true,
                  "type": "UInt32"
                },
                "type": "[UInt32]"
              },
              "item": {
                "item": {
                  "kind": 4,
                  "name": "uArrayOptArrayArray",
                  "required": true,
                  "type": "UInt32"
                },
                "kind": 18,
                "name": "uArrayOptArrayArray",
                "required": true,
                "scalar": {
                  "kind": 4,
                  "name": "uArrayOptArrayArray",
                  "required": true,
                  "type": "UInt32"
                },
                "type": "[UInt32]"
              },
              "kind": 18,
              "name": "uArrayOptArrayArray",
              "type": "[[UInt32]]"
            },
            "item": {
              "array": {
                "item": {
                  "kind": 4,
                  "name": "uArrayOptArrayArray",
                  "required": true,
                  "type": "UInt32"
                },
                "kind": 18,
                "name": "uArrayOptArrayArray",
                "required": true,
                "scalar": {
                  "kind": 4,
                  "name": "uArrayOptArrayArray",
                  "required": true,
                  "type": "UInt32"
                },
                "type": "[UInt32]"
              },
              "item": {
                "item": {
                  "kind": 4,
                  "name": "uArrayOptArrayArray",
                  "required": true,
                  "type": "UInt32"
                },
                "kind": 18,
                "name": "uArrayOptArrayArray",
                "required": true,
                "scalar": {
                  "kind": 4,
                  "name": "uArrayOptArrayArray",
                  "required": true,
                  "type": "UInt32"
                },
                "type": "[UInt32]"
              },
              "kind": 18,
              "name": "uArrayOptArrayArray",
              "type": "[[UInt32]]"
            },
            "kind": 18,
            "name": "uArrayOptArrayArray",
            "required": true,
            "type": "[[[UInt32]]]"
          },
          "kind": 34,
          "name": "uArrayOptArrayArray",
          "required": true,
          "type": "[[[UInt32]]]"
        },
        {
          "array": {
            "array": {
              "array": {
                "array": {
                  "item": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "kind": 18,
                  "name": "crazyArray",
                  "scalar": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "type": "[UInt32]"
                },
                "item": {
                  "item": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "kind": 18,
                  "name": "crazyArray",
                  "scalar": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "type": "[UInt32]"
                },
                "kind": 18,
                "name": "crazyArray",
                "required": true,
                "type": "[[UInt32]]"
              },
              "item": {
                "array": {
                  "item": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "kind": 18,
                  "name": "crazyArray",
                  "scalar": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "type": "[UInt32]"
                },
                "item": {
                  "item": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "kind": 18,
                  "name": "crazyArray",
                  "scalar": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "type": "[UInt32]"
                },
                "kind": 18,
                "name": "crazyArray",
                "required": true,
                "type": "[[UInt32]]"
              },
              "kind": 18,
              "name": "crazyArray",
              "type": "[[[UInt32]]]"
            },
            "item": {
              "array": {
                "array": {
                  "item": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "kind": 18,
                  "name": "crazyArray",
                  "scalar": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "type": "[UInt32]"
                },
                "item": {
                  "item": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "kind": 18,
                  "name": "crazyArray",
                  "scalar": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "type": "[UInt32]"
                },
                "kind": 18,
                "name": "crazyArray",
                "required": true,
                "type": "[[UInt32]]"
              },
              "item": {
                "array": {
                  "item": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "kind": 18,
                  "name": "crazyArray",
                  "scalar": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "type": "[UInt32]"
                },
                "item": {
                  "item": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "kind": 18,
                  "name": "crazyArray",
                  "scalar": {
                    "kind": 4,
                    "name": "crazyArray",
                    "required": true,
                    "type": "UInt32"
                  },
                  "type": "[UInt32]"
                },
                "kind": 18,
                "name": "crazyArray",
                "required": true,
                "type": "[[UInt32]]"
              },
              "kind": 18,
              "name": "crazyArray",
              "type": "[[[UInt32]]]"
            },
            "kind": 18,
            "name": "crazyArray",
            "type": "[[[[UInt32]]]]"
          },
          "kind": 34,
          "name": "crazyArray",
          "type": "[[[[UInt32]]]]"
        },
        {
          "kind": 34,
          "name": "object",
          "object": {
            "kind": 8192,
            "name": "object",
            "required": true,
            "type": "AnotherType"
          },
          "required": true,
          "type": "AnotherType"
        },
        {
          "kind": 34,
          "name": "optObject",
          "object": {
            "kind": 8192,
            "name": "optObject",
            "type": "AnotherType"
          },
          "type": "AnotherType"
        },
        {
          "array": {
            "item": {
              "kind": 8192,
              "name": "objectArray",
              "required": true,
              "type": "AnotherType"
            },
            "kind": 18,
            "name": "objectArray",
            "object": {
              "kind": 8192,
              "name": "objectArray",
              "required": true,
              "type": "AnotherType"
            },
            "required": true,
            "type": "[AnotherType]"
          },
          "kind": 34,
          "name": "objectArray",
          "required": true,
          "type": "[AnotherType]"
        },
        {
          "array": {
            "item": {
              "kind": 8192,
              "name": "optObjectArray",
              "type": "AnotherType"
            },
            "kind": 18,
            "name": "optObjectArray",
            "object": {
              "kind": 8192,
              "name": "optObjectArray",
              "type": "AnotherType"
            },
            "type": "[AnotherType]"
          },
          "kind": 34,
          "name": "optObjectArray",
          "type": "[AnotherType]"
        },
        {
          "kind": 34,
          "map": {
            "key": {
              "kind": 4,
              "name": "map",
              "required": true,
              "type": "String"
            },
            "kind": 262146,
            "name": "map",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "map",
              "required": true,
              "type": "Int"
            },
            "type": "Map<String, Int>",
            "value": {
              "kind": 4,
              "name": "map",
              "required": true,
              "type": "Int"
            }
          },
          "name": "map",
          "required": true,
          "type": "Map<String, Int>"
        },
        {
          "kind": 34,
          "map": {
            "array": {
              "item": {
                "kind": 4,
                "name": "mapOfArr",
                "required": true,
                "type": "Int"
              },
              "kind": 18,
              "name": "mapOfArr",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "mapOfArr",
                "required": true,
                "type": "Int"
              },
              "type": "[Int]"
            },
            "key": {
              "kind": 4,
              "name": "mapOfArr",
              "required": true,
              "type": "String"
            },
            "kind": 262146,
            "name": "mapOfArr",
            "required": true,
            "type": "Map<String, [Int]>",
            "value": {
              "item": {
                "kind": 4,
                "name": "mapOfArr",
                "required": true,
                "type": "Int"
              },
              "kind": 18,
              "name": "mapOfArr",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "mapOfArr",
                "required": true,
                "type": "Int"
              },
              "type": "[Int]"
            }
          },
          "name": "mapOfArr",
          "required": true,
          "type": "Map<String, [Int]>"
        },
        {
          "kind": 34,
          "map": {
            "key": {
              "kind": 4,
              "name": "mapOfObj",
              "required": true,
              "type": "String"
            },
            "kind": 262146,
            "name": "mapOfObj",
            "object": {
              "kind": 8192,
              "name": "mapOfObj",
              "required": true,
              "type": "AnotherType"
            },
            "required": true,
            "type": "Map<String, AnotherType>",
            "value": {
              "kind": 8192,
              "name": "mapOfObj",
              "required": true,
              "type": "AnotherType"
            }
          },
          "name": "mapOfObj",
          "required": true,
          "type": "Map<String, AnotherType>"
        },
        {
          "kind": 34,
          "map": {
            "array": {
              "item": {
                "kind": 8192,
                "name": "mapOfArrOfObj",
                "required": true,
                "type": "AnotherType"
              },
              "kind": 18,
              "name": "mapOfArrOfObj",
              "object": {
                "kind": 8192,
                "name": "mapOfArrOfObj",
                "required": true,
                "type": "AnotherType"
              },
              "required": true,
              "type": "[AnotherType]"
            },
            "key": {
              "kind": 4,
              "name": "mapOfArrOfObj",
              "required": true,
              "type": "String"
            },
            "kind": 262146,
            "name": "mapOfArrOfObj",
            "required": true,
            "type": "Map<String, [AnotherType]>",
            "value": {
              "item": {
                "kind": 8192,
                "name": "mapOfArrOfObj",
                "required": true,
                "type": "AnotherType"
              },
              "kind": 18,
              "name": "mapOfArrOfObj",
              "object": {
                "kind": 8192,
                "name": "mapOfArrOfObj",
                "required": true,
                "type": "AnotherType"
              },
              "required": true,
              "type": "[AnotherType]"
            }
          },
          "name": "mapOfArrOfObj",
          "required": true,
          "type": "Map<String, [AnotherType]>"
        },
        {
          "kind": 34,
          "map": {
            "key": {
              "kind": 4,
              "name": "mapCustomValue",
              "required": true,
              "type": "String"
            },
            "kind": 262146,
            "name": "mapCustomValue",
            "object": {
              "kind": 8192,
              "name": "mapCustomValue",
              "type": "CustomMapValue"
            },
            "required": true,
            "type": "Map<String, CustomMapValue>",
            "value": {
              "kind": 8192,
              "name": "mapCustomValue",
              "type": "CustomMapValue"
            }
          },
          "name": "mapCustomValue",
          "required": true,
          "type": "Map<String, CustomMapValue>"
        }
      ],
      "type": "CustomType"
    },
    {
      "kind": 1,
      "properties": [
        {
          "kind": 34,
          "name": "prop",
          "scalar": {
            "kind": 4,
            "name": "prop",
            "type": "String"
          },
          "type": "String"
        },
        {
          "kind": 34,
          "name": "circular",
          "object": {
            "kind": 8192,
            "name": "circular",
            "type": "CustomType"
          },
          "type": "CustomType"
        },
        {
          "kind": 34,
          "name": "const",
          "scalar": {
            "kind": 4,
            "name": "const",
            "type": "String"
          },
          "type": "String"
        }
      ],
      "type": "AnotherType"
    },
    {
      "kind": 1,
      "properties": [
        {
          "kind": 34,
          "name": "foo",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "foo",
            "required": true,
            "type": "String"
          },
          "type": "String"
        }
      ],
      "type": "CustomMapValue"
    },
    {
      "kind": 1,
      "properties": [
        {
          "kind": 34,
          "name": "else",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "else",
            "required": true,
            "type": "String"
          },
          "type": "String"
        }
      ],
      "type": "else"
    }
  ],
  "version": "0.1"
})).unwrap()
  }
}
