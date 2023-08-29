#[macro_use]
extern crate lazy_static;

pub mod wrap;
pub use wrap::*;

pub mod templates;
pub mod helpers;
mod renderer;
use renderer::Renderer;
use crate::helpers::detect_keyword::_detect_keyword;
use crate::helpers::to_lower::_to_lower;
use polywrap_wasm_rs::{JSON};

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
            name: "mod.rs".to_string(),
            data: renderer.render(
                "mod.rs",
                &wrap_info.abi
            )
        });

        output.files.push(File {
            name: "entry.rs".to_string(),
            data: renderer.render(
                "entry.rs",
                &wrap_info.abi
            )
        });

        let abi_value = wrap_info.abi.to_json();
        let abi = abi_value.as_object().unwrap();

        let get_dir_name = |value: &JSON::Value| -> String {
            let dir_name = value.get("type").unwrap().as_str().unwrap().to_string();
            _to_lower(&dir_name)
        };

        if let Some(object_types) = abi.get("objectTypes") {
            let objects = object_types.as_array().unwrap();

            for object in objects.iter() {
                let dir = Directory {
                    name: _detect_keyword(&get_dir_name(object)),
                    files: vec!(
                        File {
                            name: "mod.rs".to_string(),
                            data: renderer.render("object_type/mod.rs", object)
                        }
                    ),
                    dirs: vec!()
                };
                output.dirs.push(dir);
            }
        }

        if let Some(module_type) = abi.get("moduleType") {
            let dir = Directory {
                name: "module".to_string(),
                files: vec!(
                    File {
                        name: "mod.rs".to_string(),
                        data: renderer.render("module_type/mod.rs", module_type)
                    },
                    File {
                        name: "module.rs".to_string(),
                        data: renderer.render("module_type/module.rs", abi)
                    },
                    File {
                        name: "wrapped.rs".to_string(),
                        data: renderer.render("module_type/wrapped.rs", abi)
                    },
                ),
                dirs: vec!()
            };
            output.dirs.push(dir);
        }

        if let Some(enum_types) = abi.get("enumTypes") {
            let enums = enum_types.as_array().unwrap();

            for it in enums.iter() {
                let dir = Directory {
                    name: _detect_keyword(&get_dir_name(it)),
                    files: vec!(
                        File {
                            name: "mod.rs".to_string(),
                            data: renderer.render("enum_type/mod.rs", it)
                        },
                    ),
                    dirs: vec!()
                };
                output.dirs.push(dir);
            }
        }

        if let Some(interface_types) = abi.get("interfaceTypes") {
            let interfaces = interface_types.as_array().unwrap();

            for it in interfaces.iter() {
                let dir = Directory {
                    name: get_dir_name(it),
                    files: vec!(
                        File {
                            name: "mod.rs".to_string(),
                            data: renderer.render("interface_type/mod.rs", it)
                        },
                    ),
                    dirs: vec!()
                };
                output.dirs.push(dir);
            }
        }

        // imported dirs go within subdirectory
        let mut imported = Directory {
            name: "imported".to_string(),
            files: vec![],
            dirs: vec![],
        };

        if let Some(imported_object_types) = abi.get("importedObjectTypes") {
            let objects = imported_object_types.as_array().unwrap();

            for object in objects.iter() {
                let dir = Directory {
                    name: _detect_keyword(&get_dir_name(object)),
                    files: vec!(
                        File {
                            name: "mod.rs".to_string(),
                            data: renderer.render("imported/object_type/mod.rs", object)
                        }
                    ),
                    dirs: vec!()
                };
                imported.dirs.push(dir);
            }
        }

        if let Some(imported_module_types) = abi.get("importedModuleTypes") {
            let modules = imported_module_types.as_array().unwrap();

            for it in modules.iter() {
                let dir = Directory {
                    name: get_dir_name(it),
                    files: vec!(
                        File {
                            name: "mod.rs".to_string(),
                            data: renderer.render("imported/module_type/mod.rs", it)
                        }
                    ),
                    dirs: vec!()
                };
                imported.dirs.push(dir);
            }
        }

        if let Some(imported_enum_types) = abi.get("importedEnumTypes") {
            let enums = imported_enum_types.as_array().unwrap();

            for it in enums.iter() {
                let dir = Directory {
                    name: _detect_keyword(&get_dir_name(it)),
                    files: vec!(
                        File {
                            name: "mod.rs".to_string(),
                            data: renderer.render("imported/enum_type/mod.rs", it)
                        },
                    ),
                    dirs: vec!()
                };
                imported.dirs.push(dir);
            }
        }

        if let Some(imported_env_types) = abi.get("importedEnvTypes") {
            let envs = imported_env_types.as_array().unwrap();

            for it in envs.iter() {
                let dir = Directory {
                    name: _detect_keyword(&get_dir_name(it)),
                    files: vec!(
                        File {
                            name: "mod.rs".to_string(),
                            data: renderer.render("imported/env_type/mod.rs", it)
                        }
                    ),
                    dirs: vec!()
                };
                imported.dirs.push(dir);
            }
        }

        // add imported dirs to output
        if abi.get("importedObjectTypes").is_some() ||
            abi.get("importedModuleTypes").is_some() ||
            abi.get("importedEnumTypes").is_some() ||
            abi.get("importedEnvTypes").is_some() {
            imported.files.push(File {
                name: "mod.rs".to_string(),
                data: renderer.render("imported/mod.rs", &wrap_info.abi)
            });
            output.dirs.push(imported);
        }

        if let Some(env_type) = abi.get("envType") {
            let dir = Directory {
                name: "env".to_string(),
                files: vec!(
                    File {
                        name: "mod.rs".to_string(),
                        data: renderer.render("env_type/mod.rs", env_type)
                    }
                ),
                dirs: vec!()
            };
            output.dirs.push(dir);
        }

        Ok(output)
    }
}
