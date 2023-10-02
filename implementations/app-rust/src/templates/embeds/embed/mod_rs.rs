lazy_static! {
  static ref NAME: String = "embeds/embed/mod.rs".to_string();
  static ref SOURCE: String = r#"#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.

// URI: "{{uri}}" //

use polywrap::core::file_reader::SimpleFileReader;
use polywrap::wasm::{wasm_package::WasmPackage, wasm_wrapper::WasmWrapper};
use std::sync::Arc;

mod wrap_info;
mod wrap_wasm;
use wrap_info::WRAP_INFO;
use wrap_wasm::WRAP_WASM;

pub fn lazy_loaded_wasm_package() -> WasmPackage {
    WasmPackage::from_bytecode(
        WRAP_WASM.to_vec(),
        Arc::new(SimpleFileReader::new()),
        Some(WRAP_INFO.to_vec()),
    )
}

pub fn wasm_wrapper() -> WasmWrapper {
    WasmWrapper::try_from_bytecode(&WRAP_WASM, Arc::new(SimpleFileReader::new())).unwrap()
}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
