/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

pub mod test_import;
pub mod second_test_import;

use std::sync::Arc;
use polywrap::core::package::WrapPackage;
use polywrap::Uri;

pub fn packages() -> Vec<(Uri, Arc<dyn WrapPackage>)> {
    vec![
        (
            Uri::try_from("testimport.uri.eth").unwrap(),
            Arc::new(test_import::lazy_loaded_wasm_package())
        ),
        (
            Uri::try_from("secondtestimport.uri.eth").unwrap(),
            Arc::new(second_test_import::lazy_loaded_wasm_package())
        ),
    ]
}
