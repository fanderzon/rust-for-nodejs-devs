use rustc_serialize::json::ToJson;
use std::path::Path;
use std::fmt::Debug;
use nickel::{Response, MiddlewareResult};
use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context, JsonRender};

pub fn render<'mw, T:ToJson + Debug>(res: Response<'mw>, path: &str, data: &T) -> MiddlewareResult<'mw> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("todos", &Path::new(path)).ok().unwrap();
    let result = handlebars.render("todos", data).ok().unwrap();

    res.send(result)
}
