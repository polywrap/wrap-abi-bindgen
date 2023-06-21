use handlebars::{
    Handlebars,
    no_escape
};
use serde::Serialize;

use crate::partials;
use crate::templates;
use crate::helpers;

pub struct Renderer<'reg> {
    instance: Handlebars<'reg> 
}

impl<'reg> Renderer<'reg> {
    pub fn new() -> Renderer<'reg> {
        let mut handlebars: Handlebars = Handlebars::new();

        // Remove the HTML escape function
        handlebars.register_escape_fn(no_escape);

        // Register all partials
        let partials = partials::load_partials();
        for partial in partials.iter() {
            handlebars.register_partial(
                &partial.name,
                &partial.source
            ).unwrap();
        }

        // Register all templates
        let templates = templates::load_templates();
        for template in templates.iter() {
            handlebars.register_template_string(
                &template.name,
                &template.source
            ).unwrap();
        }

        // Register all helpers
        helpers::register(&mut handlebars);

        Renderer {
            instance: handlebars
        }
    }

    pub fn render<T>(self: &Renderer<'reg>, name: &str, data: &T) -> String
    where
        T: Serialize,
    {
        self.instance.render(name, data).unwrap()
    }
}
