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
    BigIntWrapper
};
use crate::EnvEnum;
use crate::EnvObject;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Env {
    pub str: String,
    #[serde(rename = "optStr")]
    pub opt_str: Option<String>,
    #[serde(rename = "optFilledStr")]
    pub opt_filled_str: Option<String>,
    pub number: i8,
    #[serde(rename = "optNumber")]
    pub opt_number: Option<i8>,
    pub bool: bool,
    #[serde(rename = "optBool")]
    pub opt_bool: Option<bool>,
    pub en: EnvEnum,
    #[serde(rename = "optEnum")]
    pub opt_enum: Option<EnvEnum>,
    pub object: EnvObject,
    #[serde(rename = "optObject")]
    pub opt_object: Option<EnvObject>,
    pub array: Vec<u32>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            str: String::new(),
            opt_str: None,
            opt_filled_str: None,
            number: 0,
            opt_number: None,
            bool: false,
            opt_bool: None,
            en: EnvEnum::_MAX_,
            opt_enum: None,
            object: EnvObject::new(),
            opt_object: None,
            array: vec![],
        }
    }
}
