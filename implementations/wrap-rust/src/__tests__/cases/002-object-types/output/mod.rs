pub mod entry;
pub mod custom_type;
pub use custom_type::CustomType;
pub mod another_type;
pub use another_type::AnotherType;
pub mod custom_map_value;
pub use custom_map_value::CustomMapValue;
pub mod _else;
pub use _else::Else;

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
