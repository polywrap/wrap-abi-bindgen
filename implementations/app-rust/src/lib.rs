#[macro_use]
extern crate lazy_static;

pub mod wrap;

use polywrap_wasm_rs::JSON;
use serde_json::Value;
pub use wrap::*;

pub mod templates;
pub mod helpers;
mod renderer;
mod wasm_embeds;
use renderer::Renderer;
use crate::wasm_embeds::generate_embeds;

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
            if let Some(embeds_value) = context_json.get("embeds") {
                let embeds: Directory = generate_embeds(embeds_value.clone())?;
                output.dirs.push(embeds);
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
