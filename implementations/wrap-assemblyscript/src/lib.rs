#[macro_use]
extern crate lazy_static;

pub mod wrap;
pub use wrap::*;

pub mod partials;
pub mod templates;
pub mod helpers;
mod renderer;
use renderer::Renderer;

impl ModuleTrait for Module {
    fn generate_bindings(args: ArgsGenerateBindings) -> Result<Output, String> {
        let renderer = Renderer::new();
        let mut output = Output::new();

        output.files.push(File {
            name: "index.ts".to_string(),
            data: renderer.render(
                "index.ts",
                &args.wrap_abi
            )
        });

        output.files.push(File {
            name: "entry.ts".to_string(),
            data: renderer.render(
                "entry.ts",
                &args.wrap_abi
            )
        });

        Ok(output)
    }
}
