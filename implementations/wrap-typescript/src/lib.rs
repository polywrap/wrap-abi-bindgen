#[macro_use]
extern crate lazy_static;

pub mod wrap;
use std::collections::HashMap;

use definition_kind::DefinitionKind;
use serde_json::Value;
pub use wrap::*;

pub mod templates;
pub mod helpers;
mod definition_kind;
mod renderer;
use renderer::Renderer;

fn group_by_namespace(json_array: &[Value]) -> HashMap<String, Vec<Value>> {
  let mut grouped: HashMap<String, Vec<Value>> = HashMap::new();

  for item in json_array.iter() {
      if let Some(namespace) = item.get("namespace").and_then(Value::as_str) {
          grouped.entry(namespace.to_string()).or_default().push(item.clone());
      }
  }

  grouped
}

fn group_by_kind(imports: &[Value]) -> HashMap<DefinitionKind, Vec<Value>> {
  let mut grouped: HashMap<DefinitionKind, Vec<Value>> = HashMap::new();

  for item in imports.iter() {
      let kind = DefinitionKind::from(item.get("kind").unwrap().as_u64().unwrap() as u32);
      grouped.entry(kind).or_default().push(item.clone());
  }

  grouped
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
        let mut abi_value = wrap_info.abi.to_json();
        let abi = abi_value.as_object_mut().unwrap();

        // imported dirs go within subdirectory
        let mut imported = Directory {
            name: "imported".to_string(),
            files: vec![],
            dirs: vec![],
        };

        let mut imported_objects = Value::Array(vec![]);
        let mut imported_enums = Value::Array(vec![]);
        let mut imported_envs = Value::Array(vec![]);
        let mut imported_modules = Value::Array(vec![]);

        if let Some(imported_object_types) = abi.get("importedObjectTypes") {
            imported_objects = imported_object_types.clone();
        }

        if let Some(imported_enum_types) = abi.get("importedEnumTypes") {
            imported_enums = imported_enum_types.clone();
        }

        if let Some(imported_env_types) = abi.get("importedEnvTypes") {
            imported_envs = imported_env_types.clone();
        }

        if let Some(imported_module_types) = abi.get("importedModuleTypes") {
            imported_modules = imported_module_types.clone();
        }

        let all_imports = [
            imported_objects.as_array().unwrap().clone(),
            imported_enums.as_array().unwrap().clone(),
            imported_envs.as_array().unwrap().clone(),
            imported_modules.as_array().unwrap().clone(),
        ].concat();

        let imports_by_namespace = group_by_namespace(
            &all_imports
        );

        let has_imports = all_imports.len() > 0;

        abi.insert(
            "hasImports".to_owned(),
            Value::Bool(has_imports)
        );

        let renderer = Renderer::new();
        let mut output = Output::new();

        output.files.push(File {
            name: "index.ts".to_string(),
            data: renderer.render(
                "index.ts",
                &abi
            )
        });

        output.files.push(File {
            name: "entry.ts".to_string(),
            data: renderer.render(
                "entry.ts",
                &abi
            )
        });

        output.files.push(File {
            name: "common.ts".to_string(),
            data: renderer.render(
                "common.ts",
                &abi
            )
        });

        output.files.push(File {
            name: "types.ts".to_string(),
            data: renderer.render(
                "types.ts",
                &abi
            )
        });

        if abi.get("moduleType").is_some() {
          output.files.push(File {
            name: "module.ts".to_string(),
            data: renderer.render(
                "module.ts",
                &abi
            )
          });
        }

        output.files.push(File {
          name: "globals.d.ts".to_string(),
          data: renderer.render(
              "globals.d.ts",
              &abi
          )
        });

        imports_by_namespace.iter().for_each(|(namespace, imports)| {
            let imports_by_kind = group_by_kind(&imports);

            let abi_clone = serde_json::from_str::<Value>(&abi_value.to_string()).unwrap();
            let mut abi_clone = abi_clone.as_object().unwrap().clone();

            abi_clone.insert("importedObjectTypes".to_string(), Value::Array(
                if let Some(objs) = imports_by_kind.get(&DefinitionKind::ImportedObject) {
                    objs.clone()
                } else {
                    vec![]
                }
            ));

            abi_clone.insert("importedEnvTypes".to_string(), Value::Array(
                if let Some(envs) = imports_by_kind.get(&DefinitionKind::ImportedEnv) {
                    envs.clone()
                } else {
                    vec![]
                }
            ));

            abi_clone.insert("importedEnumTypes".to_string(), Value::Array(
                if let Some(enums) = imports_by_kind.get(&DefinitionKind::ImportedEnum) {
                    enums.clone()
                } else {
                    vec![]
                }
            ));

            let mut files = vec!(
              File {
                  name: "index.ts".to_string(),
                  data: renderer.render("imported/namespace/index.ts", &abi_clone)
              },
              File {
                  name: "types.ts".to_string(),
                  data: renderer.render("imported/namespace/types.ts", &abi_clone)
              }
            );

            if let Some(imported_module) = imports_by_kind.get(&DefinitionKind::ImportedModule).unwrap().clone().get(0) {
                files.push(File {
                    name: "module.ts".to_string(),
                    data: renderer.render("imported/namespace/module.ts", &imported_module)
                });
            };

            let dir = Directory {
                name: namespace.to_string(),
                files,
                dirs: vec!()
            };
            imported.dirs.push(dir);
        });

        if imports_by_namespace.keys().len() > 0 {
            let namespaces: Vec<String> =
                imports_by_namespace.keys().into_iter().map(
                    |k| k.to_string()
                ).collect();

            let mut namespaces_obj = HashMap::new();
            namespaces_obj.insert("namespaces".to_string(), namespaces);

            imported.files.push(File {
                name: "index.ts".to_string(),
                data: renderer.render("imported/index.ts", &namespaces_obj)
            });
            output.dirs.push(imported);
        }

        Ok(output)
    }
}
