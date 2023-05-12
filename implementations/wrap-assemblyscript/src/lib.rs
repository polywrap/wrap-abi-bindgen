#[macro_use]
extern crate lazy_static;

pub mod wrap;
pub use wrap::*;

pub mod partials;
pub mod templates;
pub mod helpers;
mod renderer;
use renderer::Renderer;

impl ModuleTrait for Module {
    fn generate_bindings(args: ArgsGenerateBindings) -> Result<Output, String> {
        let renderer = Renderer::new();


        Ok(Output::new())
    }
}

/*
for (template of templates.abi) {
    let res = render(template, abi, functions);
    output.files.insert(res);
}

for (object of abi.objects) {
    let dir = Directory { name: object.name };
    for (template of templates.object) {
        let res = render(template, object, functions);
        dir.files.insert(res);
    }
    output.dirs.insert(dir);
}
*/
