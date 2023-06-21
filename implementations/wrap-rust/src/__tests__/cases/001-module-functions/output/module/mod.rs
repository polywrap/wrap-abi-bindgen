pub mod wrapped;
pub use wrapped::{
    function1_wrapped,
    function2_wrapped
};
pub mod serialization;
pub use serialization::{
    deserialize_function1_args,
    serialize_function1_result,
    ArgsFunction1,
    deserialize_function2_args,
    serialize_function2_result,
    ArgsFunction2
};

pub mod module;
pub use module::*;
