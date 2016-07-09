use rustc_serialize::json::ToJson;
use std::path::Path;
use std::fmt::Debug;
use nickel::{Response, MiddlewareResult};
use handlebars::{Handlebars, HelperDef, Renderable, RenderError, RenderContext, Helper, Context, JsonRender};

fn filter_todo(c: &Context, h: &Helper, ha: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let active_filter = c.navigate(".", "visibility_filter").as_string().unwrap();
    let is_completed = c.navigate( rc.get_path(), "completed").as_boolean().unwrap();
    println!("active_filter {:?}", active_filter);
    let show_todo: bool = match active_filter {
        "ShowAll" => {
            true
        },
        "ShowCompleted" => {
            if is_completed {
                true
            } else {
                false
            }
        },
        "ShowActive" => {
            if is_completed {
                false
            } else {
                true
            }
        },
        _ => false,
    };

    if show_todo {
        h.template().unwrap().render(c, ha, rc)
    } else {
        Ok(())
    }
}

pub fn render<'mw, T:ToJson + Debug>(res: Response<'mw>, path: &str, data: &T) -> MiddlewareResult<'mw> {
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("filter_todo", Box::new(filter_todo));
    handlebars.register_template_file("todos", &Path::new(path)).ok().unwrap();
    let result = handlebars.render("todos", data).ok().unwrap();

    res.send(result)
}
