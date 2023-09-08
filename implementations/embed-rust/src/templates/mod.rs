mod mod_rs;
mod embed;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        mod_rs::load(),
        embed::mod_rs::load(),
        embed::wrap_info::load(),
        embed::wrap_wasm::load(),
    )
}
