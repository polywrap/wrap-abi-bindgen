mod index_ts;
mod entry_ts;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        index_ts::load(),
        entry_ts::load()
    )
}
