#[macro_use]
extern crate lazy_static;

pub mod wrap;

use polywrap_wasm_rs::JSON;
pub use wrap::*;

pub mod templates;
pub mod helpers;
mod renderer;
use renderer::Renderer;

impl ModuleTrait for Module {
    fn generate_bindings(args: ArgsGenerateBindings) -> Result<Output, String> {
        let version = &args.wrap_info.version;

        // First, ensure version is "0.1"
        if version != "0.1" {
            return Err(
                format!("Unsupported ABI Version - {}; Supported - 0.1", version)
            );
        }

        let wrap_info = args.wrap_info;
        let renderer = Renderer::new();
        let mut output = Output::new();

        output.files.push(File {
            name: "index.ts".to_string(),
            data: renderer.render(
                "index.ts",
                &None::<JSON::Value>
            )
        });

        output.files.push(File {
            name: "types.ts".to_string(),
            data: renderer.render(
                "types.ts",
                &wrap_info.abi.to_json()
            )
        });

        Ok(output)
    }
}
