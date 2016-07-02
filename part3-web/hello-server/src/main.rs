#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter};

// use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
// fn hello_world<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
//     res.send("Hello World!")
// }

fn main() {
    let mut app = Nickel::new();
    app.get("/", middleware!("Hello World!"));
    app.listen("0.0.0.0:3000");
}
