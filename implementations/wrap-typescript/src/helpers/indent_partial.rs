use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError};

pub struct IndentPartialDef;

impl HelperDef for IndentPartialDef {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let partial_name = h.param(0)
            .ok_or_else(|| RenderError::new("Param not found for template name"))?
            .value()
            .as_str()
            .ok_or_else(|| RenderError::new("Template name must be a string"))?;

        let indent_level = h.param(1)
            .ok_or_else(|| RenderError::new("Param not found for indent level"))?
            .value()
            .as_u64()
            .ok_or_else(|| RenderError::new("Indent level must be an integer"))? as usize;

        let indent = " ".repeat(indent_level);

        if let Some(tpl) = r.get_template(partial_name) {
            let data = rc.evaluate(ctx, "this")?;
            let rendered = r.render(&tpl.name.clone().unwrap(), &data.as_json())?;
            let indented = rendered.lines()
                .map(|line| format!("{}{}", indent, line))
                .map(|line| if line.trim().is_empty() { line.trim().to_string() } else { line })
                .collect::<Vec<String>>()
                .join("\n");

            out.write(&indented)?;
        }

        Ok(())
    }
}
