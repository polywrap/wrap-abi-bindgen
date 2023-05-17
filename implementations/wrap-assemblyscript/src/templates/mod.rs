mod index_ts;
mod entry_ts;
mod module_type;
mod object_type;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        index_ts::load(),
        entry_ts::load(),
        module_type::index_ts::load(),
        module_type::module_ts::load(),
        module_type::serialization_ts::load(),
        module_type::wrapped_ts::load(),
        object_type::index_ts::load(),
        object_type::serialization_ts::load()
    )
}
