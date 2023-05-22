mod index_ts;
mod module_ts;
mod types_ts;
mod wrap_info_ts;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        index_ts::load(),
        module_ts::load(),
        types_ts::load(),
        wrap_info_ts::load(),
    )
}
