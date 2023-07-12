pub mod entry;

pub mod module;
pub use module::{
    Module,
    ModuleTrait,
    function1_wrapped,
    ArgsFunction1,
    function2_wrapped,
    ArgsFunction2
};

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
