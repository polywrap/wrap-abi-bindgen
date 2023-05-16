use handlebars::handlebars_helper;

pub fn to_msgpack_fn(value: &str) -> String {
    let mut value_str = String::from(value);
    let mut modifier = "";
    if value.ends_with('!') {
        value_str = value_str.trim_end_matches('!').to_string();
    } else {
        modifier = "Optional";
    }

    if value_str.starts_with('[') {
        return format!("{}Array", modifier);
    }

    if value_str.starts_with("Map<") {
        return format!("{}ExtGenericMap", modifier);
    }

    match value_str.as_ref() {
        "Int" => format!("{}Int32", modifier),
        "UInt" => format!("{}UInt32", modifier),
        "Boolean" => format!("{}Bool", modifier),
        _ => format!("{}{}", modifier, value_str),
    }
}

handlebars_helper!(to_msgpack: |value: str| {
    to_msgpack_fn(&value)
});
