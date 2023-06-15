use handlebars::handlebars_helper;
use serde_json::{Value, Map};

#[derive(Debug)]
pub enum DefinitionKind {
    Generic = 0,
    Object = 1 << 0,
    Any = 1 << 1,
    Scalar = 1 << 2,
    Enum = 1 << 3,
    Array = ((1 << 4) as u32 | DefinitionKind::Any as u32) as isize,
    Property = ((1 << 5) as u32 | DefinitionKind::Any as u32) as isize,
    Method = 1 << 6,
    Module = 1 << 7,
    ImportedModule = 1 << 8,
    ImportedEnum = ((1 << 9) as u32 | DefinitionKind::Enum as u32) as isize,
    ImportedObject = ((1 << 10) as u32 | DefinitionKind::Object as u32) as isize,
    InterfaceImplemented = 1 << 11,
    UnresolvedObjectOrEnum = 1 << 12,
    ObjectRef = 1 << 13,
    EnumRef = 1 << 14,
    Interface = 1 << 15,
    Env = 1 << 16,
    MapKey = 1 << 17,
    Map = ((1 << 18) as u32 | DefinitionKind::Any as u32) as isize,
    ImportedEnv = 1 << 19,
}

impl From<u32> for DefinitionKind {
    fn from(value: u32) -> Self {
        match value {
            v if v == Self::Generic as u32 => Self::Generic,
            v if v == Self::Object as u32 => Self::Object,
            v if v == Self::Any as u32 => Self::Any,
            v if v == Self::Scalar as u32 => Self::Scalar,
            v if v == Self::Enum as u32 => Self::Enum,
            v if v == Self::Array as u32 => Self::Array,
            v if v == Self::Property as u32 => Self::Property,
            v if v == Self::Method as u32 => Self::Method,
            v if v == Self::Module as u32 => Self::Module,
            v if v == Self::ImportedModule as u32 => Self::ImportedModule,
            v if v == Self::ImportedEnum as u32 => Self::ImportedEnum,
            v if v == Self::ImportedObject as u32 => Self::ImportedObject,
            v if v == Self::InterfaceImplemented as u32 => Self::InterfaceImplemented,
            v if v == Self::UnresolvedObjectOrEnum as u32 => Self::UnresolvedObjectOrEnum,
            v if v == Self::ObjectRef as u32 => Self::ObjectRef,
            v if v == Self::EnumRef as u32 => Self::EnumRef,
            v if v == Self::Interface as u32 => Self::Interface,
            v if v == Self::Env as u32 => Self::Env,
            v if v == Self::MapKey as u32 => Self::MapKey,
            v if v == Self::Map as u32 => Self::Map,
            v if v == Self::ImportedEnv as u32 => Self::ImportedEnv,
            _ => panic!("Invalid value for DefinitionKind"),
        }
    }
}

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
    to_graphql_type_fn(&obj, false)
});
