use crate::{
    function1_wrapped,
    function2_wrapped
};
use polywrap_wasm_rs::{
    abort,
    invoke,
    InvokeArgs,
};

#[no_mangle]
pub extern "C" fn _wrap_invoke(method_size: u32, args_size: u32, env_size: u32) -> bool {
    // Ensure the abort handler is properly setup
    abort::wrap_abort_setup();

    let args: InvokeArgs = invoke::wrap_invoke_args(method_size, args_size);
    let result: Vec<u8>;

    match args.method.as_str() {
        "function1" => {
            result = function1_wrapped(args.args.as_slice(), env_size);
        }
        "function2" => {
            result = function2_wrapped(args.args.as_slice(), env_size);
        }
        _ => {
            invoke::wrap_invoke_error(format!("Could not find invoke function {}", args.method));
            return false;
        }
    };
    invoke::wrap_invoke_result(result);
    return true;
}
