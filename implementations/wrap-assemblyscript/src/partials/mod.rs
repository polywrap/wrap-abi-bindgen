mod deserialize_array;
mod deserialize_enum;
mod deserialize_map_value;
mod deserialize_object;
mod serialization_imports;
mod serialize_array;
mod serialize_enum;
mod serialize_map_value;
mod serialize_object;

pub struct Partial {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_partials() -> Vec<Partial> {
    vec!(
        deserialize_array::load(),
        deserialize_enum::load(),
        deserialize_map_value::load(),
        deserialize_object::load(),
        serialization_imports::load(),
        serialize_array::load(),
        serialize_enum::load(),
        serialize_map_value::load(),
        serialize_object::load(),
    )
}
