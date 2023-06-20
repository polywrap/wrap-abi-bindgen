#[macro_use]
extern crate lazy_static;

pub mod wrap;
pub use wrap::*;

// use polywrap_wasm_rs::JSON;

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
            name: "module.ts".to_string(),
            data: renderer.render(
                "module.ts",
                &wrap_info.abi
            )
        });

        output.files.push(File {
            name: "types.ts".to_string(),
            data: renderer.render(
                "types.ts",
                &wrap_info.abi
            )
        });

        output.files.push(File {
            name: "wrap.info.ts".to_string(),
            data: renderer.render(
                "wrap.info.ts",
                &wrap_info
            )
        });

        Ok(output)
    }
}
