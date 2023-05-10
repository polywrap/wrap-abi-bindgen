pub mod wrap;
pub use wrap::*;

impl ModuleTrait for Module {
    fn generate_bindings(args: ArgsGenerateBindings) -> Result<Output, String> {
        Ok(Output::new())
    }
}
