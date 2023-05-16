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

        let abi = args.wrap_abi.as_object().unwrap();
        let objects = abi.get("objectTypes").unwrap().as_array().unwrap();

        for object in objects.iter() {
            let dir = Directory {
                name: object.get("type").unwrap().as_str().unwrap().to_string(),
                files: vec!(
                    File {
                        name: "index.ts".to_string(),
                        data: renderer.render(
                            "object_type/index.ts",
                            object
                        )
                    },
                    File {
                        name: "serialization.ts".to_string(),
                        data: renderer.render(
                            "object_type/serialization.ts",
                            object
                        )
                    }
                ),
                dirs: vec!()
            };
            output.dirs.push(dir);
        }

        Ok(output)
    }
}
