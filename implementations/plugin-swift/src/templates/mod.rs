mod Module_swift;
mod Types_swift;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        Module_swift::load(),
        Types_swift::load(),
    )
}
