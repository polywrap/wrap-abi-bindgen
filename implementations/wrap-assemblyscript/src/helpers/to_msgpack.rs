use handlebars::handlebars_helper;

pub fn to_msgpack_fn(value: String) -> String {
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

handlebars_helper!(to_msgpack: |value: str| {
    to_msgpack_fn(value.to_string()).as_str()
});
