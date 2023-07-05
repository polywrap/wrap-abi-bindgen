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
        mod_rs::load(),
        entry_rs::load(),
        enum_type::mod_rs::load(),
        env_type::mod_rs::load(),
        interface_type::mod_rs::load(),
        module_type::mod_rs::load(),
        module_type::module_rs::load(),
        module_type::wrapped_rs::load(),
        object_type::mod_rs::load(),
        imported::enum_type::mod_rs::load(),
        imported::env_type::mod_rs::load(),
        imported::module_type::mod_rs::load(),
        imported::object_type::mod_rs::load(),
        imported::mod_rs::load(),
    )
}
