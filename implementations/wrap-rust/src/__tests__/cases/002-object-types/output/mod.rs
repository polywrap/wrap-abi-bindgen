pub mod entry;
pub mod prelude;
pub mod custom_type;
pub mod another_type;
pub mod custom_map_value;
pub mod _else;
pub mod arg;
pub mod nested;
pub mod output;
pub mod module;

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
