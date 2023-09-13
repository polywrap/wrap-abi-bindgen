use serde::{Serialize, Deserialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    JSONString,
    BigIntWrapper,
    ByteBuf
};
use crate::TestImportAnotherObject;
use crate::TestImportEnum;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestImportEnv {
    pub object: TestImportAnotherObject,
    pub opt_object: Option<TestImportAnotherObject>,
    pub object_array: Vec<TestImportAnotherObject>,
    pub opt_object_array: Option<Vec<Option<TestImportAnotherObject>>>,
    pub en: TestImportEnum,
    pub opt_enum: Option<TestImportEnum>,
    pub enum_array: Vec<TestImportEnum>,
    pub opt_enum_array: Option<Vec<Option<TestImportEnum>>>,
}

impl TestImportEnv {
    pub const URI: &'static str = "testimport.uri.eth";

    pub fn new() -> TestImportEnv {
        TestImportEnv {
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
