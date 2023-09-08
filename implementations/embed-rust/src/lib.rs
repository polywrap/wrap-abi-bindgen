#[macro_use]
extern crate lazy_static;

pub mod wrap;
pub use wrap::*;

pub mod templates;
pub mod helpers;
mod renderer;
use renderer::Renderer;
use crate::helpers::to_lower::_to_lower;

impl ModuleTrait for Module {
    fn generate_bindings(args: ArgsGenerateBindings) -> Result<Output, String> {
        let renderer = Renderer::new();
        let mut output = Output::new();

        let context = match args.context {
            Some(context) => context.to_json(),
            None => return Err("Missing context".to_string())
        };

        let embeds_value = match context.get("embeds") {
            Some(embeds) => embeds,
            None => return Err("Missing 'embeds' in context".to_string())
        };

        let embeds = match embeds_value.as_array() {
            Some(embeds) => embeds,
            None => return Err("'embeds' must be an array".to_string())
        };

        output.files.push(File {
            name: "mod.rs".to_string(),
            data: renderer.render(
                "mod.rs",
                embeds
            )
        });

        embeds.iter().for_each(|embed| {
            let dirname = embed["namespace"].as_str().unwrap().to_string();
            let formatted_dirname = _to_lower(&dirname);

            let dir = Directory {
                name: formatted_dirname,
                files: vec!(
                    File {
                        name: "mod.rs".to_string(),
                        data: renderer.render("embed/mod.rs", embed)
                    },
                    File {
                        name: "wrap_info.rs".to_string(),
                        data: renderer.render("embed/wrap_info.rs", embed)
                    },
                    File {
                        name: "wrap_wasm.rs".to_string(),
                        data: renderer.render("embed/wrap_wasm.rs", embed)
                    },
                ),
                dirs: vec!()
            };

            output.dirs.push(dir);
        });

        Ok(output)
    }
}
