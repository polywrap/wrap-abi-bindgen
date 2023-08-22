#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use polywrap::{
    from_slice,
    uri,
    to_vec,
    Uri,
    Invoker,
    PolywrapClient,
    PolywrapClientConfigBuilder,
    ClientConfigBuilder,
    PolywrapClientConfig,
    SystemClientConfig,
    Web3ClientConfig,
    polywrap_core::error::Error
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

pub type BigInt = String;

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

#[derive(Clone)]
pub struct TestImportModule {
    uri: Uri,
    invoker: Arc<dyn Invoker>,
    env: Option<Vec<u8>>
}

impl TestImportModule {
    pub fn new(uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> TestImportModule {
        let mut config = PolywrapClientConfig::new();
        config.add(SystemClientConfig::default().into());
        config.add(Web3ClientConfig::default().into());
        let client = PolywrapClient::new(config.build());

        let _uri = uri.unwrap_or(uri!("testimport.uri.eth"));
        let _invoker = invoker.unwrap_or(Arc::new(client));
        let _env = env;

        TestImportModule {
            uri: _uri,
            invoker: _invoker,
            env: _env,
        }
    }

    pub fn imported_method(&self, args: &TestImportModuleArgsImportedMethod, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<Option<TestImportObject>, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match env {
            Some(e) => Some(e.as_slice()),
            None => match self.env.clone() {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "importedMethod",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn another_method(&self, args: &TestImportModuleArgsAnotherMethod, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<i32, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match env {
            Some(e) => Some(e.as_slice()),
            None => match self.env.clone() {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "anotherMethod",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }

    pub fn returns_array_of_enums(&self, args: &TestImportModuleArgsReturnsArrayOfEnums, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<Vec<Option<TestImportEnumReturn>>, Error> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match env {
            Some(e) => Some(e.as_slice()),
            None => match self.env.clone() {
                Some(e) => Some(e.as_slice()),
                None => None,
            },
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "returnsArrayOfEnums",
            opt_args,
            _env,
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
    }
}
// Imported Modules END //
