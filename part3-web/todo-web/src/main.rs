#[macro_use] extern crate nickel;
extern crate nickel_mustache;
extern crate rustc_serialize;
mod store;
use store::{ Store, Todo, reducer };
use store::TodoAction::{ Add, Remove, Toggle };
use store::Action::{ Todos, Visibility };

use nickel_mustache::Render;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();
    let mut store = Store::create_store(reducer);
    &store.dispatch( Todos(Add("one thing".to_string())) );
    &store.dispatch( Todos(Add("another thing".to_string())) );
    &store.dispatch( Todos(Add("something completely different".to_string())) );


    server.get("/", middleware! { |_req, res|
        return Render::render(res, "src/todos", &store.get_state())
    });

    // server.get("/toggle/:todoid", middleware! { |_req, res|
    //     if let Ok(num) = _req.param("todoid").unwrap().parse::<i16>() {
    //         // &store.dispatch( Todos(Toggle(num)));
    //     }
    //     return Render::render(res, "src/todos", &store.get_state())
    // });

    server.listen("0.0.0.0:3000");
}
