pub mod entry;
pub mod env_object;
pub use env_object::EnvObject;
pub mod env_enum;
pub use env_enum::{
    get_env_enum_key,
    get_env_enum_value,
    sanitize_env_enum_value,
    EnvEnum
};
pub mod env;
pub use env::Env;

pub mod module;
pub use module::{
    Module,
    ModuleTrait,
    method_no_env_wrapped,
    ArgsMethodNoEnv,
    method_require_env_wrapped,
    ArgsMethodRequireEnv,
    method_optional_env_wrapped,
    ArgsMethodOptionalEnv
};

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
