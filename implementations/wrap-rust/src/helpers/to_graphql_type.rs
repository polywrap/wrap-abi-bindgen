use handlebars::handlebars_helper;
use serde_json::{Value, Map};
use crate::helpers::util::DefinitionKind;

fn apply_required(type_str: &str, required: bool) -> String {
    format!("{}{}", type_str, match required {
        true => "!",
        false => ""
    })
}

fn any_to_graphql(any: &Map<String, Value>, prefixed: bool) -> String {
    if let Some(object) = any.get("object") {
        return to_graphql_type_fn(object.as_object().unwrap(), prefixed);
    } else if let Some(array) = any.get("array") {
        return to_graphql_type_fn(array.as_object().unwrap(), prefixed);
    } else if let Some(scalar) = any.get("scalar") {
        return to_graphql_type_fn(scalar.as_object().unwrap(), prefixed);
    } else if let Some(enum_value) = any.get("enum") {
        return to_graphql_type_fn(enum_value.as_object().unwrap(), prefixed);
    } else if let Some(map) = any.get("map") {
        return to_graphql_type_fn(map.as_object().unwrap(), prefixed);
    } else {
        panic!("anyToGraphQL: Any type is invalid.");
    }
}

pub fn to_graphql_type_fn(obj: &Map<String, Value>, prefixed: bool) -> String {
    let type_str = obj.get("type").unwrap().as_str().unwrap();
    let required = match obj.get("required") {
        Some(x) => x.as_bool().unwrap(),
        None => false
    };
    let kind = DefinitionKind::from(obj.get("kind").unwrap().as_u64().unwrap() as u32);

    match kind {
        DefinitionKind::Object
        | DefinitionKind::ObjectRef
        | DefinitionKind::Scalar
        | DefinitionKind::ImportedObject => apply_required(type_str, required),
        DefinitionKind::Enum
        | DefinitionKind::EnumRef
        | DefinitionKind::ImportedEnum => {
            if prefixed {
                apply_required(format!("Enum_{}", type_str).as_str(), required)
            } else {
                apply_required(type_str, required)
            }
        }
        DefinitionKind::Any | DefinitionKind::Property => {
            any_to_graphql(obj, prefixed)
        }
        DefinitionKind::Array => {    
            if let Some(item) = obj.get("item") {
                apply_required(format!("[{}]", to_graphql_type_fn(item.as_object().unwrap(), prefixed)).as_str(), required)
            } else {
                panic!(
                    "toGraphQL: ArrayDefinition's item type is undefined.\n{:?}",
                    obj
                );
            }
        }
        DefinitionKind::Map => {
            if let Some(key) = obj.get("key") {
                if let Some(value) = obj.get("value") {
                    apply_required(
                        format!(
                            "Map<{}, {}>",
                            to_graphql_type_fn(key.as_object().unwrap(), prefixed),
                            to_graphql_type_fn(value.as_object().unwrap(), prefixed)
                        ).as_str(),
                        required,
                    )
                } else {
                    panic!(
                        "toGraphQL: MapDefinition's value type is undefined.\n{:?}",
                        obj
                    );
                }
            } else {
                panic!(
                    "toGraphQL: MapDefinition's key type is undefined.\n{:?}",
                    obj
                );
            }
        }
        DefinitionKind::Method => {
            if let Some(return_type) = obj.get("returnType") {
                let arguments = obj.get("arguments").unwrap();
                let result = format!(
                    "{}(\n  {}\n): {}",
                    obj.get("name").unwrap().as_str().unwrap(),
                    arguments
                        .as_array().unwrap()
                        .into_iter()
                        .map(|arg| {
                            let arg_obj = arg.as_object().unwrap();
                            format!("{}: {}", arg_obj.get("name").unwrap().as_str().unwrap(), to_graphql_type_fn(arg_obj, prefixed))
                        })
                        .collect::<Vec<String>>()
                        .join("\n    "),
                    to_graphql_type_fn(return_type.as_object().unwrap(), prefixed)
                );
                result
            } else {
                panic!(
                    "toGraphQL: MethodDefinition's return type is undefined.\n{:?}",
                    obj
                );
            }
        }
        DefinitionKind::Module => obj.get("type").unwrap().as_str().unwrap().to_string(),
        DefinitionKind::ImportedModule => obj.get("type").unwrap().as_str().unwrap().to_string(),
        _ => panic!(
            "toGraphQL: Unrecognized DefinitionKind.\n{:?}",
            obj
        ),
    }
}

handlebars_helper!(to_graphql_type: |value: Value| {
    let obj = value.as_object().unwrap();
    to_graphql_type_fn(&obj, true)
});
