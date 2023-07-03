mod entry_rs;
mod mod_rs;
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
        index_rs::load(),
        entry_rs::load(),
        enum_type::index_rs::load(),
        env_type::index_rs::load(),
        env_type::serialization_rs::load(),
        interface_type::index_rs::load(),
        module_type::index_rs::load(),
        module_type::module_rs::load(),
        module_type::serialization_rs::load(),
        module_type::wrapped_rs::load(),
        object_type::index_rs::load(),
        object_type::serialization_rs::load(),
        imported::enum_type::index_rs::load(),
        imported::env_type::index_rs::load(),
        imported::env_type::serialization_rs::load(),
        imported::module_type::index_rs::load(),
        imported::module_type::serialization_rs::load(),
        imported::object_type::index_rs::load(),
        imported::object_type::serialization_rs::load(),
        imported::index_rs::load(),
    )
}
