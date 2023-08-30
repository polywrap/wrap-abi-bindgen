#[macro_use]
extern crate lazy_static;

pub mod wrap;
pub use wrap::*;
use polywrap_wasm_rs::JSON;
use serde::Serialize;

pub mod templates;
pub mod helpers;
mod renderer;
use renderer::Renderer;

#[derive(Serialize)]
struct RenderData {
    version: String,
    name: String,
    #[serde(rename = "type")]
    _type: String,
    abi: JSON::Value
}

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
        let data = RenderData {
            version: wrap_info.version,
            name: wrap_info.name,
            _type: wrap_info._type,
            abi: wrap_info.abi.to_json()
        };

        let renderer = Renderer::new();
        let mut output = Output::new();

        output.files.push(File {
            name: "__init__.py".to_string(),
            data: renderer.render(
                "__init__.py",
                &None::<JSON::Value>
            )
        });

        output.files.push(File {
            name: "module.py".to_string(),
            data: renderer.render(
                "module.py",
                &data.abi
            )
        });

        output.files.push(File {
            name: "types.py".to_string(),
            data: renderer.render(
                "types.py",
                &data.abi
            )
        });

        output.files.push(File {
            name: "wrap_info.py".to_string(),
            data: renderer.render(
                "wrap_info.py",
                &data
            )
        });

        Ok(output)
    }
}
