use handlebars::Handlebars;

mod apply_optional;
mod detect_keyword;
mod is_base_type;
mod is_keyword;
mod to_msgpack;
mod to_wasm_array;
mod to_wasm_init;
mod to_wasm_map;
mod to_wasm;

pub fn register(handlebars: &mut Handlebars) -> () {
    handlebars.register_helper(
        "apply_optional",
        Box::new(apply_optional::apply_optional)
    );
    handlebars.register_helper(
        "detect_keyword",
        Box::new(detect_keyword::detect_keyword)
    );
    handlebars.register_helper(
        "is_base_type",
        Box::new(is_base_type::is_base_type)
    );
    handlebars.register_helper(
        "is_keyword",
        Box::new(is_keyword::is_keyword)
    );
    handlebars.register_helper(
        "to_msgpack",
        Box::new(to_msgpack::to_msgpack)
    );
    handlebars.register_helper(
        "to_wasm_array",
        Box::new(to_wasm_array::to_wasm_array)
    );
    handlebars.register_helper(
        "to_wasm_init",
        Box::new(to_wasm_init::to_wasm_init)
    );
    handlebars.register_helper(
        "to_wasm_map",
        Box::new(to_wasm_map::to_wasm_map)
    );
    handlebars.register_helper(
        "to_wasm",
        Box::new(to_wasm::to_wasm)
    );
}
