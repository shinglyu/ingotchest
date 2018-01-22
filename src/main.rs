extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hi There!")))
    }

    println!("Starting server on http://localhost:3000");
    Iron::new(hello).http("localhost:3000").unwrap();
}
