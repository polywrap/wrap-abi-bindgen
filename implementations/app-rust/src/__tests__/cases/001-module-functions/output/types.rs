#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use polywrap::*;
use std::result::Result;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

pub type BigInt = String;

#[derive(Clone)]
pub struct InvokeOptions {
    pub uri: Option<Uri>,
    pub client: Option<Arc<dyn Invoker>>,
    pub env: Option<Vec<u8>> 
}

fn get_default_client() -> Arc<PolywrapClient> {
    let mut config = PolywrapClientConfig::new();
    config.add(SystemClientConfig::default().into());
    config.add(Web3ClientConfig::default().into());
    let client = PolywrapClient::new(config.build());
    Arc::new(client)
}

// Env START //

// Env END //

// Objects START //

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
