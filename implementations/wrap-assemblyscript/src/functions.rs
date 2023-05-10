use crate::base_types::{is_base_type};
use crate::keywords::{is_keyword};

type MustacheFn = dyn FnMut(String) -> String + Send + 'static;

// check if any of the keywords match the property name;
// if there's a match, insert `_` at the beginning of the property name.
pub fn detect_keyword() -> MustacheFn {
    move |value: String| -> String {
        if is_keyword(&value) {
            format!("_{}", value)
        } else {
            value
        }
    }
}

pub fn to_msgpack() -> MustacheFn {
    move |value: String| -> String {
        let mut modifier = "";
        if value.ends_with('!') {
            value = value.trim_end_matches('!').to_string();
        } else {
            modifier = "Optional";
        }

        if value.starts_with('[') {
            return format!("{}Array", modifier);
        }

        if value.starts_with("Map<") {
            return format!("{}ExtGenericMap", modifier);
        }

        match value.as_ref() {
            "Int" => format!("{}Int32", modifier),
            "UInt" => format!("{}UInt32", modifier),
            "Boolean" => format!("{}Bool", modifier),
            _ => format!("{}{}", modifier, value),
        }
    }
}

pub fn to_wasm_init() -> MustacheFn {
    move |value: String| -> String {
        if value.chars().last() == Some('!') {
            value = value[0..value.len() - 1].to_string();
        } else {
            let null_type = to_wasm()(value.clone());
            let null_optional = "| null";

            if null_type.ends_with(null_optional) {
                return "null".to_string();
            }
        }

        if value.starts_with('[') {
            return "[]".to_string();
        }

        if value.starts_with("Map<") {
            let first_open_bracket_idx = value.find('<').unwrap_or_else(|| {
                panic!("Invalid Map: {}", value);
            });
            let last_close_bracket_idx = value
                .rfind('>')
                .unwrap_or_else(|| panic!("Invalid Map: {}", value));

            let key_val_types = value
                .get(first_open_bracket_idx + 1..last_close_bracket_idx)
                .unwrap_or_else(|| panic!("Invalid Map: {}", value));

            let first_comma_idx = key_val_types.find(',').unwrap_or_else(|| {
                panic!("Invalid Map: {}", value);
            });
            let key_type = key_val_types
                .get(0..first_comma_idx)
                .unwrap_or_else(|| panic!("Invalid Map: {}", value))
                .trim()
                .to_string();
            let val_type = key_val_types
                .get(first_comma_idx + 1..)
                .unwrap_or_else(|| panic!("Invalid Map: {}", value))
                .trim()
                .to_string();

            let wasm_key_type = to_wasm()(key_type.clone());
            let wasm_val_type = to_wasm()(val_type.clone());

            return format!("new Map<{}, {}>()", wasm_key_type, wasm_val_type);
        }

        match value.as_str() {
            "Int" | "Int8" | "Int16" | "Int32" | "UInt" | "UInt8" | "UInt16" | "UInt32" => {
                "0".to_string()
            }
            "String" => "\"\"".to_string(),
            "Boolean" => "false".to_string(),
            "Bytes" => "new ArrayBuffer(0)".to_string(),
            "BigInt" => "BigInt.fromUInt16(0)".to_string(),
            "BigNumber" => "new BigNumber(BigInt.fromUInt16(0), 0, 0)".to_string(),
            "JSON" => "JSON.Value.Null()".to_string(),
            _ => {
                if value.contains("Enum_") {
                    "0".to_string()
                } else {
                    let mut value = detect_keyword()(value);
                    format!("new Types.{}()", value)
                }
            }
        }
    }
}

fn to_wasm() -> MustacheFn {
    move |value: String| -> String {
        let mut is_enum = false;

        let mut optional = false;
        if value.ends_with('!') {
            value = value.trim_end_matches('!');
        } else {
            optional = true;
        }

        if value.starts_with('[') {
            return to_wasm_array(&value, optional);
        }

        if value.starts_with("Map<") {
            return to_wasm_map(&value, optional);
        }

        match value.as_str() {
            "Int" => value = "i32".to_owned(),
            "Int8" => value = "i8".to_owned(),
            "Int16" => value = "i16".to_owned(),
            "Int32" => value = "i32".to_owned(),
            "UInt" | "UInt32" => value = "u32".to_owned(),
            "UInt8" => value = "u8".to_owned(),
            "UInt16" => value = "u16".to_owned(),
            "String" => value = "string".to_owned(),
            "Boolean" => value = "bool".to_owned(),
            "Bytes" => value = "ArrayBuffer".to_owned(),
            "BigInt" => value = "BigInt".to_owned(),
            "BigNumber" => value = "BigNumber".to_owned(),
            "JSON" => value = "JSON.Value".to_owned(),
            _ => {
                if value.contains("Enum_") {
                    value = value.replacen("Enum_", "", 1);
                    is_enum = true;
                }
                value = detect_keyword()(&value);
                value = format!("Types.{}", value);
            }
        }

        apply_optional(&value, optional, is_enum)
    }
}

fn to_wasm_array(type_str: &str, optional: bool) -> Result<String, String> {
    let re = Regex::new(r"(\[)([[\]A-Za-z0-9_.!]+)(\])").unwrap();
    let captures = re.captures(type_str);

    let result = match captures {
        Some(captures) => captures,
        None => return Err(format!("Invalid Array: {}", type_str)),
    };

    let wasm_type = to_wasm()(result.get(1).unwrap().as_str());
    Ok(apply_optional(&format!("Array<{}>", wasm_type), optional, false))
}

fn to_wasm_map(type_str: &str, optional: bool) -> Result<String, String> {
    let first_open_bracket_idx = type_str.find('<').unwrap_or(0);
    let last_close_bracket_idx = type_str.rfind('>').unwrap_or(0);

    if first_open_bracket_idx == 0 || last_close_bracket_idx == 0 {
        return Err(format!("Invalid Map: {}", type_str));
    }

    let key_val_types = type_str[first_open_bracket_idx + 1..last_close_bracket_idx].trim();

    let first_comma_idx = key_val_types.find(',').unwrap_or(0);
    if first_comma_idx == 0 {
        return Err(format!("Invalid Map: {}", type_str));
    }

    let key_type = key_val_types[..first_comma_idx].trim();
    let val_type = key_val_types[first_comma_idx + 1..].trim();

    let wasm_key_type = to_wasm()(key_type);
    let wasm_val_type = to_wasm()(val_type);

    Ok(apply_optional(&format!("Map<{}, {}>", wasm_key_type, wasm_val_type), optional, false))
}

fn apply_optional(type_str: &str, optional: bool, is_enum: bool) -> String {
    if optional {
        if type_str.starts_with("Array") || type_str.starts_with("string") || (!is_enum && !is_base_type(type_str)) {
            return format!("{} | null", type_str);
        } else {
            return format!("Box<{}> | null", type_str);
        }
    } else {
        type_str.to_string()
    }
}
