mod mod_rs;
mod types_rs;
mod embeds;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        mod_rs::load(),
        types_rs::load(),
        embeds::mod_rs::load(),
        embeds::embed::mod_rs::load(),
        embeds::embed::wrap_info_rs::load(),
        embeds::embed::wrap_wasm_rs::load(),
    )
}
