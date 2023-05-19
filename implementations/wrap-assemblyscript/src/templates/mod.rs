mod index_ts;
mod entry_ts;
mod enum_type;
mod env_type;
mod interface_type;
mod module_type;
mod object_type;
mod imported;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        index_ts::load(),
        entry_ts::load(),
        enum_type::index_ts::load(),
        env_type::index_ts::load(),
        env_type::serialization_ts::load(),
        interface_type::index_ts::load(),
        module_type::index_ts::load(),
        module_type::module_ts::load(),
        module_type::serialization_ts::load(),
        module_type::wrapped_ts::load(),
        object_type::index_ts::load(),
        object_type::serialization_ts::load(),
        imported::enum_type::index_ts::load(),
        imported::env_type::index_ts::load(),
        imported::env_type::serialization_ts::load(),
        imported::module_type::index_ts::load(),
        imported::module_type::serialization_ts::load(),
        imported::object_type::index_ts::load(),
        imported::object_type::serialization_ts::load(),
        imported::index_ts::load(),
    )
}
