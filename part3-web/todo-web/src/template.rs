use rustc_serialize::json::ToJson;
use std::path::Path;
use std::fmt::Debug;
use std::io::Write;
use nickel::{Response, MiddlewareResult};
use handlebars::{Handlebars, Renderable, RenderError, RenderContext, Helper, Context, JsonRender};

fn filter_todo(c: &Context, h: &Helper, ha: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let active_filter = c.navigate(".", "visibility_filter").as_string().unwrap();
    let is_completed = c.navigate( rc.get_path(), "completed").as_boolean().unwrap();
    let show_todo: bool = match active_filter {
        "ShowAll" => true,
        "ShowCompleted" => if is_completed { true } else { false },
        "ShowActive" => if is_completed { false } else { true },
        _ => false,
    };

    if show_todo {
        h.template().unwrap().render(c, ha, rc)
    } else {
        Ok(())
    }
}

fn is_selected_filter(c: &Context, h: &Helper, ha: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value().as_string().unwrap();
    let active_filter = c.navigate(".", "visibility_filter").as_string().unwrap();
    let is_selected: bool = match active_filter {
        "ShowAll" => if param == "ShowAll" { true } else { false },
        "ShowCompleted" => if param == "ShowCompleted" { true } else { false },
        "ShowActive" => if param == "ShowActive" { true } else { false },
        _ => false,
    };
    if is_selected {
        h.template().unwrap().render(c, ha, rc)
    } else {
        Ok(())
    }
}

#[allow(unused_variables)]
fn active_count(c: &Context, h: &Helper, ha: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let todos = c.navigate(".", "todos").as_array().unwrap();
    let count = todos
        .into_iter()
        .filter(|todo| {
            !todo.find("completed").unwrap().as_boolean().unwrap() &&
            !todo.find("deleted").unwrap().as_boolean().unwrap()
        })
        .count();

    let mut output = count.to_string();
    if count == 1 {
        output.push_str(" item left");
    } else {
        output.push_str(" items left");
    }

    rc.writer.write(output.as_bytes()).unwrap();
    Ok(())
}

pub fn render<'mw, T:ToJson + Debug>(res: Response<'mw>, path: &str, data: &T) -> MiddlewareResult<'mw> {
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("filter_todo", Box::new(filter_todo));
    handlebars.register_helper("active_count", Box::new(active_count));
    handlebars.register_helper("is_selected_filter", Box::new(is_selected_filter));
    handlebars.register_template_file("template", &Path::new(path)).ok().unwrap();
    let result = handlebars.render("template", data).ok().unwrap();

    res.send(result)
}
