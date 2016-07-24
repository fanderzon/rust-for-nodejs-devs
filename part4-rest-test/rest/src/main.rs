#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate handlebars;
mod store;
mod template;
mod todo;
use template::render;
use store::{ Store, reducer };
use todo::TodoAction::{ Add, Remove, Toggle };
use store::Action::{ Todos, Visibility };
use store::VisibilityFilter::{ ShowAll, ShowActive, ShowCompleted };
use rustc_serialize::json::{ToJson};

use std::sync::{Arc, Mutex};

use nickel::{Nickel, HttpRouter, FormBody};

fn main() {
    let mut server = Nickel::new();

    // Create our todo list store
    let store = Store::create_store(reducer);

    // Put the store in a container that will let us
    // safely use it in multi-threaded environment
    let store_container = Arc::new( Mutex::new(store) );

    // Every clone() of our container is counted
    // so that when the last clone goes out of scope
    // the container can be deallocated
    let store = store_container.clone();

    // At the / path let's just render our current todo list
    server.get("/", middleware! { |_req, res|
        // We get our store from the container by locking it
        // from other threads.
        let store = store.lock().unwrap();

        // Render from nickel_mustache takes the
        // nickel Result struct, a path to a mustache
        // template, and the data to use
        return render(res, "./src/todos.tpl", store.get_state())
        // And here the lock is released..
    });

    // Let's clone it again for the next closure
    let store = store_container.clone();

    // This time we look for requests like /toggle/1
    server.get("/:action/:id", middleware! { |_req, res|
        // We will dispatch an action on our store so we
        // get a mutable reference
        let mut store = store.lock().unwrap();

        // We try to parse the id param to an int, this works for the
        // toggle and remove actions
        if let Ok(num) = _req.param("id").unwrap().parse::<i16>() {
            match _req.param("action").unwrap() {
                "toggle" => {
                    store.dispatch( Todos( Toggle(num) ) )
                },

                "remove" => store.dispatch( Todos( Remove(num) ) ),
                _ => (),
            }
        } else {
        // Otherwise look for a show action
            match _req.param("action").unwrap() {
                "show" => {
                    match _req.param("id").unwrap() {
                        "all" => store.dispatch( Visibility( ShowAll ) ),
                        "active" => store.dispatch( Visibility( ShowActive ) ),
                        "completed" => store.dispatch( Visibility( ShowCompleted ) ),
                        _ => (),
                    }
                },
                _ => (),
            }
        }
        // And render the now updated todo list
        return render(res, "./src/todos.tpl", store.get_state())
    });

    // Let's clone it again for the next closure
    let store = store_container.clone();

    server.post("/:action/:id", middleware! { |req, res|
        let mut store = store.lock().unwrap();
        let form_body = req.form_body().ok().unwrap();
        if let Some(new_todo) = form_body.get("todo") {
            if new_todo.len() > 0 {
                store.dispatch( Todos( Add(new_todo.to_string()) ) );
            }
        }

        return render(res, "./src/todos.tpl", store.get_state())
    });

    // Let's clone it again for the next closure
    let store = store_container.clone();

    server.post("/api*", middleware! { |req, res|
        let mut store = store.lock().unwrap();
        let form_body = req.form_body().ok().unwrap();
        if let Some(new_todo) = form_body.get("todo") {
            if new_todo.len() > 0 {
                store.dispatch( Todos( Add(new_todo.to_string()) ) );
            }
        }
        return res.send(store.get_state().to_json());
    });

    server.listen("0.0.0.0:3000");
}
