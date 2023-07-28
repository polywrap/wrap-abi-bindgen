#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use bytes::ByteBuf;
use polywrap_core::{invoker::Invoker, uri::Uri};
use polywrap_plugin::error::PluginError;
use polywrap_msgpack_serde::{
  to_vec,
  from_slice,
  JSON,
  JSONString
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

pub type BigInt = String;
pub type BigNumber = String;

// Env START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Env {
    pub prop: String,
    #[serde(rename = "optProp")]
    pub opt_prop: Option<String>,
    #[serde(rename = "optMap")]
    pub opt_map: Option<BTreeMap<String, Option<i32>>>,
}
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
    pub json: JSONString,
    #[serde(rename = "optJson")]
    pub opt_json: Option<JSONString>,
    pub bytes: ByteBuf,
    #[serde(rename = "optBytes")]
    pub opt_bytes: Option<ByteBuf>,
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
    pub en: CustomEnum,
    #[serde(rename = "optEnum")]
    pub opt_enum: Option<CustomEnum>,
    #[serde(rename = "enumArray")]
    pub enum_array: Vec<CustomEnum>,
    #[serde(rename = "optEnumArray")]
    pub opt_enum_array: Option<Vec<Option<CustomEnum>>>,
    pub map: BTreeMap<String, i32>,
    #[serde(rename = "mapOfArr")]
    pub map_of_arr: BTreeMap<String, Vec<i32>>,
    #[serde(rename = "mapOfObj")]
    pub map_of_obj: BTreeMap<String, AnotherType>,
    #[serde(rename = "mapOfArrOfObj")]
    pub map_of_arr_of_obj: BTreeMap<String, Vec<AnotherType>>,
    #[serde(rename = "mapCustomValue")]
    pub map_custom_value: BTreeMap<String, Option<CustomMapValue>>,
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

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum CustomEnum {
    STRING,
    BYTES,
    _MAX_
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum While {
    _for,
    _in,
    _MAX_
}
// Enums END //

// Imported objects START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestImportObject {
    pub object: TestImportAnotherObject,
    #[serde(rename = "optObject")]
    pub opt_object: Option<TestImportAnotherObject>,
    #[serde(rename = "objectArray")]
    pub object_array: Vec<TestImportAnotherObject>,
    #[serde(rename = "optObjectArray")]
    pub opt_object_array: Option<Vec<Option<TestImportAnotherObject>>>,
    pub en: TestImportEnum,
    #[serde(rename = "optEnum")]
    pub opt_enum: Option<TestImportEnum>,
    #[serde(rename = "enumArray")]
    pub enum_array: Vec<TestImportEnum>,
    #[serde(rename = "optEnumArray")]
    pub opt_enum_array: Option<Vec<Option<TestImportEnum>>>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestImportAnotherObject {
    pub prop: String,
}
// Imported objects END //

// Imported envs START //

// Imported envs END //

// Imported enums START //

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum TestImportEnum {
    STRING,
    BYTES,
    _MAX_
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum TestImportEnumReturn {
    STRING,
    BYTES,
    _MAX_
}
// Imported enums END //

// Imported Modules START //

// URI: "testimport.uri.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestImportModuleArgsImportedMethod {
    pub str: String,
    #[serde(rename = "optStr")]
    pub opt_str: Option<String>,
    pub u: u32,
    #[serde(rename = "optU")]
    pub opt_u: Option<u32>,
    #[serde(rename = "uArrayArray")]
    pub u_array_array: Vec<Option<Vec<Option<u32>>>>,
    pub object: TestImportObject,
    #[serde(rename = "optObject")]
    pub opt_object: Option<TestImportObject>,
    #[serde(rename = "objectArray")]
    pub object_array: Vec<TestImportObject>,
    #[serde(rename = "optObjectArray")]
    pub opt_object_array: Option<Vec<Option<TestImportObject>>>,
    pub en: TestImportEnum,
    #[serde(rename = "optEnum")]
    pub opt_enum: Option<TestImportEnum>,
    #[serde(rename = "enumArray")]
    pub enum_array: Vec<TestImportEnum>,
    #[serde(rename = "optEnumArray")]
    pub opt_enum_array: Option<Vec<Option<TestImportEnum>>>,
}

// URI: "testimport.uri.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestImportModuleArgsAnotherMethod {
    pub arg: Vec<String>,
}

// URI: "testimport.uri.eth" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestImportModuleArgsReturnsArrayOfEnums {
    pub arg: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestImportModule<'a> {
    uri: &'a str
}

impl<'a> TestImportModule<'a> {
    pub const INTERFACE_URI: &'static str = "testimport.uri.eth";

    pub fn new(uri: &'a str) -> TestImportModule<'a> {
        TestImportModule { uri: uri }
    }

    pub fn imported_method(&self, args: &TestImportModuleArgsImportedMethod) -> Result<Option<TestImportObject>, PluginError> {
        let uri = self.uri;
        let serialized_args = to_vec(args.clone()).unwrap();
        let result = invoker.invoke_raw(
            uri,
            "importedMethod",
            serialized_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "importedMethod".to_string(),
            args: JSON::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok(Some(from_slice(result.as_slice())?))
    }

    pub fn another_method(&self, args: &TestImportModuleArgsAnotherMethod) -> Result<i32, PluginError> {
        let uri = self.uri;
        let serialized_args = to_vec(args.clone()).unwrap();
        let result = invoker.invoke_raw(
            uri,
            "anotherMethod",
            serialized_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "anotherMethod".to_string(),
            args: JSON::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok(from_slice(result.as_slice())?)
    }

    pub fn returns_array_of_enums(&self, args: &TestImportModuleArgsReturnsArrayOfEnums) -> Result<Vec<Option<TestImportEnumReturn>>, PluginError> {
        let uri = self.uri;
        let serialized_args = to_vec(args.clone()).unwrap();
        let result = invoker.invoke_raw(
            uri,
            "returnsArrayOfEnums",
            serialized_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "returnsArrayOfEnums".to_string(),
            args: JSON::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok(from_slice(result.as_slice())?)
    }
}
// Imported Modules END //
