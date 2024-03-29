mod init_py;
mod types_py;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        init_py::load(),
        types_py::load(),
    )
}
