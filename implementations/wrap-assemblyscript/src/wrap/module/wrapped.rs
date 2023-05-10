use polywrap_wasm_rs::{
  wrap_load_env
};

use crate::{
    ArgsGenerateBindings,
    deserialize_generate_bindings_args,
    serialize_generate_bindings_result
};

use crate::module::{ModuleTrait, Module};

pub fn generate_bindings_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_generate_bindings_args(args) {
        Ok(args) => {
            let result = Module::generate_bindings(ArgsGenerateBindings {
                wrap_abi: args.wrap_abi,
                project_name: args.project_name,
                context: args.context,
            });
            match result {
                Ok(res) => {
                    serialize_generate_bindings_result(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}
