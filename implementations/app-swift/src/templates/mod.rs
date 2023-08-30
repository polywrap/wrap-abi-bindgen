mod Modules_swift;
mod Types_swift;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        Modules_swift::load(),
        Types_swift::load(),
    )
}
