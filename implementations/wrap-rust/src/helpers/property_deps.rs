use handlebars::handlebars_helper;
use polywrap_wasm_rs::JSON;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};
use crate::helpers::util::DefinitionKind;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PropertyDep {
    _crate: String,
    _type: String,
    is_enum: bool,
}

handlebars_helper!(property_deps: |value: Value| {
    let obj = value.as_object().unwrap();
    let root_type = obj.get("type")
        .and_then(|t| t.as_str())
        .ok_or_else(|| format!("Invalid type for 'type' property of ABI definition. Expected string."))
        .unwrap();
    let mut deps: Vec<PropertyDep> = Vec::new();
    _property_deps(root_type, obj, &mut deps).unwrap();
    JSON::to_value(deps).unwrap()
});

fn _property_deps(root_type: &str, def: &Map<String, Value>, deps: &mut Vec<PropertyDep>) -> Result<(), String> {
    let kind: DefinitionKind = def.get("kind")
        .and_then(|k| k.as_u64())
        .map(|k| DefinitionKind::from(k as u32))
        .ok_or_else(|| "Invalid type for 'kind' property of ABI definition. Expected u32.".to_string())?;

    match kind {
        DefinitionKind::Env
        | DefinitionKind::ImportedEnv
        | DefinitionKind::Object
        | DefinitionKind::Module
        | DefinitionKind::ImportedModule => {
            for v in def.values() {
                match v.as_object() {
                    Some(obj) => _property_deps(root_type,obj, deps)?,
                    None => {}
                }
            }
            _append_property_dep(root_type, deps, &def)?
        }
        _ => {}
    }

    Ok(())
}

fn _append_property_dep(
    root_type: &str,
    array: &mut Vec<PropertyDep>,
    def: &Map<String, Value>
) -> Result<(), String> {
    let mut type_name = def.get("type")
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid type for 'type' property of ABI definition. Expected string.".to_string())?;

    if type_name.starts_with("[") {
        type_name = type_name.replace(&['[', ']', '!', '?'], "");
    }

    if type_name.starts_with("Map<") {
        // def.map?.object?.type ?? def.map?.enum?.type
        let value_name = def.get("map")
            .and_then(|m| m.get("object"))
            .and_then(|o| o.get("type"))
            .or_else(|| {
                def.get("map")
                    .and_then(|m| m.get("enum"))
                    .and_then(|e| e.get("type"))
            })
            .and_then(|t| t.as_str());


        if let Some(value_name) = value_name {
            if !is_known_type(value_name, root_type) {
                // def.map?.enum?.type
                let type_if_enum = def.get("map")
                    .and_then(|m| m.get("enum"))
                    .and_then(|e| e.get("type"))
                    .and_then(|e| e.as_str())
                    .unwrap_or("");
                append_unique(array, PropertyDep {
                    _crate: "crate".to_string(),
                    _type: value_name.to_string(),
                    is_enum: value_name == type_if_enum
                });
            }
        }
    } else if !is_known_type(&type_name, root_type) {
        append_unique(array, PropertyDep {
            _crate: "crate".to_string(),
            _type: type_name.clone(),
            // !!def.enum || !!def.array?.enum
            is_enum: def.contains_key("enum") || def.get("array").and_then(|a| a.get("enum")).is_some(),
        });
    }

    Ok(())
}


fn is_base_type(s: &str) -> bool {
    match s {
        "UInt" | "UInt8" | "UInt16" | "UInt32" | "UInt64"
        | "Int" | "Int8" | "Int16" | "Int32" | "Int64"
        | "String" | "Boolean" | "Bytes" => true,
        _ => false,
    }
}

fn is_builtin_type(s: &str) -> bool {
    match s {
        "BigInt" | "BigNumber" | "JSON" => true,
        _ => false,
    }
}

fn append_unique(array: &mut Vec<PropertyDep>, item: PropertyDep) {
    if array.iter().any(|i| i._crate == item._crate && i._type == item._type) {
        return;
    }
    array.push(item);
}

fn is_known_type(name: &str, root_type: &str) -> bool {
    is_base_type(name) || is_builtin_type(name) || name == root_type
}
