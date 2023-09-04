use serde::{Serialize, Deserialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    wrap_load_env,
    to_vec,
    from_slice,
    JSONString,
    BigIntWrapper,
    serde_bytes
};
use crate::TestImportAnotherObject;
use crate::TestImportEnum;

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

impl TestImportObject {
    pub const URI: &'static str = "testimport.uri.eth";

    pub fn new() -> TestImportObject {
        TestImportObject {
            object: TestImportAnotherObject::new(),
            opt_object: None,
            object_array: vec![],
            opt_object_array: None,
            en: TestImportEnum::_MAX_,
            opt_enum: None,
            enum_array: vec![],
            opt_enum_array: None,
        }
    }
}
