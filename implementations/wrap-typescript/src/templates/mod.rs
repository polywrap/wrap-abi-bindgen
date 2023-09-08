mod index_ts;
mod entry_ts;
mod common_ts;
mod globals_d_ts;
mod types_ts;
mod module_ts;
mod imported;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        index_ts::load(),
        entry_ts::load(),
        common_ts::load(),
        types_ts::load(),
        globals_d_ts::load(),
        module_ts::load(),
        imported::index_ts::load(),
        imported::imported_type::index_ts::load(),
        imported::imported_type::module_ts::load(),
        imported::imported_type::types_ts::load()
    )
}
