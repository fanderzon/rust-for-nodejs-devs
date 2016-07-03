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
    store.dispatch( Todos(Add("one thing".to_string())) );
    store.dispatch( Todos(Add("another thing".to_string())) );
    store.dispatch( Todos(Add("something completely different".to_string())) );


    server.get("/*", middleware! { |_req, res|
        #[derive(RustcEncodable)]
        struct ViewData<'a> {
            name: &'a str,
            todos: &'a Vec<Todo>,
        }
        let mut data = ViewData {
            name: "Fredrik",
            todos: &store.get_state().todos,
        };

        return Render::render(res, "src/todos", &data)
    });

    server.listen("0.0.0.0:3000");
}
