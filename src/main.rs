extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut router = Router::new();
    // TODO: Do we really need database?
    // router.get("/:database/:key", get_by_key, "get_by_key");
    router.get("/:key", get, "get");
    router.put("/:key", put, "put");
    router.delete("/:key", delete, "delete");

    fn get(req: &mut Request) -> IronResult<Response> {
        let route_info = req.extensions.get::<Router>().unwrap();
        //let ref database = route_info.find("database").unwrap_or("");
        let ref key = route_info.find("key").unwrap_or("");
        //let db_path = Path::new(database);
        //let key_path = db_path.join(key).with_extension("json");
        let key_path = Path::new(key).with_extension("json");
        // TODO: proper logging
        println!("GET {:?}", key_path);
        if !key_path.exists() {
            // TODO: Check what CouchDB return if database don't exist
            return Ok(Response::with((status::NotFound, format!("File not found: {:?}", key_path))))
        }
        // FIXME: Read file everytime, try caching
        // Check https://github.com/iron/staticfile for caching
        Ok(Response::with((status::Ok, key_path)))
    }

    fn put(req: &mut Request) -> IronResult<Response> {
        unimplemented!();
    }

    fn delete(req: &mut Request) -> IronResult<Response> {
        unimplemented!();
    }


    println!("Starting server on http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}
