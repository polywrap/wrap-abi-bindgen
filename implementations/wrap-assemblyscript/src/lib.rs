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
                &wrap_info.abi
            )
        });

        output.files.push(File {
            name: "entry.ts".to_string(),
            data: renderer.render(
                "entry.ts",
                &wrap_info.abi
            )
        });

        let abi = wrap_info.abi.as_object().unwrap();

        if let Some(object_types) = abi.get("objectTypes") {
            let objects = object_types.as_array().unwrap();

            for object in objects.iter() {
                let dir = Directory {
                    name: object.get("type").unwrap().as_str().unwrap().to_string(),
                    files: vec!(
                        File {
                            name: "index.ts".to_string(),
                            data: renderer.render("object_type/index.ts", object)
                        },
                        File {
                            name: "serialization.ts".to_string(),
                            data: renderer.render("object_type/serialization.ts", object)
                        }
                    ),
                    dirs: vec!()
                };
                output.dirs.push(dir);
            }
        }

        if let Some(module_type) = abi.get("moduleType") {
            let dir = Directory {
                name: "Module".to_string(),
                files: vec!(
                    File {
                        name: "index.ts".to_string(),
                        data: renderer.render("module_type/index.ts", module_type)
                    },
                    File {
                        name: "module.ts".to_string(),
                        data: renderer.render("module_type/module.ts", module_type)
                    },
                    File {
                        name: "serialization.ts".to_string(),
                        data: renderer.render("module_type/serialization.ts", module_type)
                    },
                    File {
                        name: "wrapped.ts".to_string(),
                        data: renderer.render("module_type/wrapped.ts", module_type)
                    },
                ),
                dirs: vec!()
            };
            output.dirs.push(dir);
        }

        Ok(output)
    }
}
