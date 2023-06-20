#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use polywrap_core::{invoke::Invoker, uri::Uri};
use polywrap_msgpack::{decode, serialize};
use polywrap_plugin::{error::PluginError, BigInt, BigNumber, Map, JSON};
use serde::{Serialize, Deserialize};

// Env START //

// Env END //

// Objects START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomType {
    pub str: String,
    #[serde(rename = "optStr")]
    pub opt_str: Option<String>,
    pub u: u32,
    #[serde(rename = "optU")]
    pub opt_u: Option<u32>,
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub i: i32,
    pub i8: i8,
    pub i16: i16,
    pub i32: i32,
    pub bigint: BigInt,
    #[serde(rename = "optBigint")]
    pub opt_bigint: Option<BigInt>,
    pub bignumber: BigNumber,
    #[serde(rename = "optBignumber")]
    pub opt_bignumber: Option<BigNumber>,
    pub json: JSON::Value,
    #[serde(rename = "optJson")]
    pub opt_json: Option<JSON::Value>,
    #[serde(with = "serde_bytes")]
    pub bytes: Vec<u8>,
    #[serde(with = "serde_bytes")]
    #[serde(rename = "optBytes")]
    pub opt_bytes: Option<Vec<u8>>,
    pub boolean: bool,
    #[serde(rename = "optBoolean")]
    pub opt_boolean: Option<bool>,
    pub u_array: Vec<u32>,
    #[serde(rename = "uOpt_array")]
    pub u_opt_array: Option<Vec<u32>>,
    #[serde(rename = "_opt_uOptArray")]
    pub _opt_u_opt_array: Option<Vec<Option<u32>>>,
    #[serde(rename = "optStrOptArray")]
    pub opt_str_opt_array: Option<Vec<Option<String>>>,
    #[serde(rename = "uArrayArray")]
    pub u_array_array: Vec<Vec<u32>>,
    #[serde(rename = "uOptArrayOptArray")]
    pub u_opt_array_opt_array: Vec<Option<Vec<Option<u32>>>>,
    #[serde(rename = "uArrayOptArrayArray")]
    pub u_array_opt_array_array: Vec<Option<Vec<Vec<u32>>>>,
    #[serde(rename = "crazyArray")]
    pub crazy_array: Option<Vec<Option<Vec<Vec<Option<Vec<u32>>>>>>>,
    pub object: AnotherType,
    #[serde(rename = "optObject")]
    pub opt_object: Option<AnotherType>,
    #[serde(rename = "objectArray")]
    pub object_array: Vec<AnotherType>,
    #[serde(rename = "optObjectArray")]
    pub opt_object_array: Option<Vec<Option<AnotherType>>>,
    pub map: Map<String, i32>,
    #[serde(rename = "mapOfArr")]
    pub map_of_arr: Map<String, Vec<i32>>,
    #[serde(rename = "mapOfObj")]
    pub map_of_obj: Map<String, AnotherType>,
    #[serde(rename = "mapOfArrOfObj")]
    pub map_of_arr_of_obj: Map<String, Vec<AnotherType>>,
    #[serde(rename = "mapCustomValue")]
    pub map_custom_value: Map<String, Option<CustomMapValue>>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnotherType {
    pub prop: Option<String>,
    pub circular: Option<CustomType>,
    #[serde(rename = "const")]
    pub _const: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomMapValue {
    pub foo: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Else {
    #[serde(rename = "else")]
    pub _else: String,
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

// Imported Modules START //

// Imported Modules END //
