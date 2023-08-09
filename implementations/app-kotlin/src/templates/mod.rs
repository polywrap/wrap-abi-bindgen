mod module_kt;
mod types_kt;
mod wrap_info_kt;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        module_kt::load(),
        types_kt::load(),
        wrap_info_kt::load(),
    )
}
