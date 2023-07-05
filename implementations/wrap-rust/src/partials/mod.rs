mod hello_world;

pub struct Partial {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_partials() -> Vec<Partial> {
    vec!(
        hello_world::load(),
    )
}
