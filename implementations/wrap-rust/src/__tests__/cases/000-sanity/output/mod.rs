// Disable unused code warnings for this entire module
#![allow(unused)]

pub mod entry;
pub mod prelude;
pub mod custom_type;
pub mod another_type;
pub mod custom_map_value;
pub mod _else;
pub mod custom_enum;
pub mod _while;
pub mod env;
pub mod imported;
pub mod test_import;
pub mod module;

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
