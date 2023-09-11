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
    fn generate_embeds(args: ArgsGenerateEmbeds) -> Result<BindgenOutput, String> {
        let renderer = Renderer::new();
        let mut output = BindgenOutput::new();

        let embeds_json = serde_json::to_value(args.embeds).map_err(|e| e.to_string())?;
        let embeds = embeds_json.as_array().ok_or("embeds must be an array".to_string())?;

        output.files.push(BindgenFile {
            name: "mod.rs".to_string(),
            data: renderer.render(
                "mod.rs",
                embeds
            )
        });

        embeds.iter().for_each(|embed| {
            let dirname = embed["namespace"].as_str().unwrap().to_string();
            let formatted_dirname = _to_lower(&dirname);

            let dir = BindgenDirectory {
                name: formatted_dirname,
                files: vec!(
                    BindgenFile {
                        name: "mod.rs".to_string(),
                        data: renderer.render("embed/mod.rs", embed)
                    },
                    BindgenFile {
                        name: "wrap_info.rs".to_string(),
                        data: renderer.render("embed/wrap_info.rs", embed)
                    },
                    BindgenFile {
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
