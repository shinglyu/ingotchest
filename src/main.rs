extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn main() {
    let mut router = Router::new();
    // TODO: Do we really need database?
    // router.get("/:database/:key", get_by_key, "get_by_key");
    router.get("/:key", get, "get");
    router.put("/:key", put, "put");
    router.delete("/:key", delete, "delete");

    fn get_path(key: &str) -> PathBuf {
        return Path::new(key).with_extension("json");
    }

    fn get(req: &mut Request) -> IronResult<Response> {
        let route_info = req.extensions.get::<Router>().unwrap();
        //let ref database = route_info.find("database").unwrap_or("");
        let ref key = route_info.find("key").unwrap_or("");
        //let db_path = Path::new(database);
        //let key_path = db_path.join(key).with_extension("json");
        let key_path = get_path(key);
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
        let route_info = req.extensions.get::<Router>().unwrap();
        let ref key = route_info.find("key").unwrap_or("");
        let key_path = get_path(key);
        let mut payload = String::new();
        req.body.read_to_string(&mut payload).expect("Fail to read request body");
        // TODO: validate JSON
        println!("{:?}", payload);

        let mut file = match File::create(&key_path) {
            Err(why) => panic!("couldn't create {}: {}",
                               key_path.display(),
                               why.description()),
            Ok(file) => file,
        };

        match file.write_all(payload.as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", key_path.display(),
                                                   why.description())
            },
            // TODO: Proper logging
            Ok(_) => println!("successfully wrote to {}", key_path.display()),
        }

        // TODO: Return code?
        Ok(Response::with((status::Ok, "Ok")))
    }

    fn delete(req: &mut Request) -> IronResult<Response> {
        unimplemented!();
    }


    println!("Starting server on http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}
