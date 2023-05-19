mod deserialize_arguments;
mod deserialize_array;
mod deserialize_enum;
mod deserialize_map_value;
mod deserialize_object;
mod deserialize_properties;
mod deserialize_result;
mod serialization_imports;
mod serialize_arguments;
mod serialize_array;
mod serialize_enum;
mod serialize_map_value;
mod serialize_object;
mod serialize_properties;
mod serialize_result;

pub struct Partial {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_partials() -> Vec<Partial> {
    vec!(
        deserialize_arguments::load(),
        deserialize_array::load(),
        deserialize_enum::load(),
        deserialize_map_value::load(),
        deserialize_object::load(),
        deserialize_properties::load(),
        deserialize_result::load(),
        serialization_imports::load(),
        serialize_arguments::load(),
        serialize_array::load(),
        serialize_enum::load(),
        serialize_map_value::load(),
        serialize_object::load(),
        serialize_properties::load(),
        serialize_result::load(),
    )
}
