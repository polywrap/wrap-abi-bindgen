mod init_py;
mod module_py;
mod types_py;
mod wrap_info_py;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        init_py::load(),
        module_py::load(),
        types_py::load(),
        wrap_info_py::load(),
    )
}
