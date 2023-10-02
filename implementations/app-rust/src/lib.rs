#[macro_use]
extern crate lazy_static;

pub mod wrap;

use polywrap_wasm_rs::JSON;
use serde_json::Value;
pub use wrap::*;

pub mod templates;
pub mod helpers;
mod renderer;
use renderer::Renderer;
use crate::helpers::to_lower::_to_lower;

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
        let mut has_embeds = Value::Null;

        if let Some(context) = args.context {
            let context_json = context.to_json();
            if let Some(embeds_json) = context_json.get("embeds") {
                let embeds = embeds_json.as_array().ok_or("embeds must be an array".to_string())?;

                let mut embeds_dir = Directory {
                    name: "embeds".to_string(),
                    files: vec!(),
                    dirs: vec!()
                };

                embeds_dir.files.push(File {
                    name: "mod.rs".to_string(),
                    data: renderer.render(
                        "embeds/mod.rs",
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
                                data: renderer.render("embeds/embed/mod.rs", embed)
                            },
                            File {
                                name: "wrap_info.rs".to_string(),
                                data: renderer.render("embeds/embed/wrap_info.rs", embed)
                            },
                            File {
                                name: "wrap_wasm.rs".to_string(),
                                data: renderer.render("embeds/embed/wrap_wasm.rs", embed)
                            },
                        ),
                        dirs: vec!()
                    };

                    embeds_dir.dirs.push(dir);
                });

                output.dirs.push(embeds_dir);
                has_embeds = JSON::from_str::<Value>("{ \"embeds\": true }").unwrap();
            }
        }

        output.files.push(File {
            name: "mod.rs".to_string(),
            data: renderer.render(
                "mod.rs",
                &has_embeds
            )
        });

        output.files.push(File {
            name: "types.rs".to_string(),
            data: renderer.render(
                "types.rs",
                &wrap_info.abi.to_json()
            )
        });

        Ok(output)
    }
}
