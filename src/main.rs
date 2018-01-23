extern crate iron;
extern crate iron_test;
extern crate router;
#[macro_use] extern crate log;
extern crate env_logger;

use iron::prelude::*;
use iron::{Headers, status};
use iron_test::{request};
use router::Router;
use std::error::Error;
use std::fs::{File, remove_file};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

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
    debug!("GET {:?}", key_path);
    if !key_path.exists() {
        // TODO: Check what CouchDB return if database don't exist
        return Ok(Response::with((status::NotFound, format!("File not found: {:?}", key_path))))
    }
    // FIXME: Read file everytime, try caching
    // Check https://github.com/iron/staticfile for caching
    Ok(Response::with((status::Ok, key_path)))
}

fn put(req: &mut Request) -> IronResult<Response> {
    // TODO: Extract the key getting logic
    let route_info = req.extensions.get::<Router>().unwrap();
    let ref key = route_info.find("key").unwrap_or("");
    // TODO: Do not allow file not in the current dir
    let key_path = get_path(key);
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).expect("Fail to read request body");
    // TODO: validate JSON
    debug!("{:?}", payload);

    let mut file = match File::create(&key_path) {
        // TODO: Return 500 Server Error
        Err(why) => panic!("couldn't create {}: {}",
                           key_path.display(),
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all(payload.as_bytes()) {
        Err(why) => {
            // TODO: Return 500 Server Error
            panic!("couldn't write to {}: {}", key_path.display(),
            why.description())
        },
        Ok(_) => debug!("successfully wrote to {}", key_path.display()),
    }

    // TODO: Return code?
    Ok(Response::with((status::Ok, "Ok")))
}

fn delete(req: &mut Request) -> IronResult<Response> {
    // TODO: Extract the key getting logic
    let route_info = req.extensions.get::<Router>().unwrap();
    let ref key = route_info.find("key").unwrap_or("");
    // TODO: Do not allow file not in the current dir
    let key_path = get_path(key);
    match remove_file(key_path) {
        // TODO: Return 500 Server Error
        Err(_) => Ok(Response::with((status::Ok, "Not Ok"))),
        Ok(_) => Ok(Response::with((status::Ok, "Ok")))
    }
}

fn main() {
    env_logger::init();

    let mut router = Router::new();
    // TODO: Do we really need database?
    // router.get("/:database/:key", get_by_key, "get_by_key");
    router.get("/:key", get, "get");
    router.put("/:key", put, "put");
    router.delete("/:key", delete, "delete");


    info!("Starting server on http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}

#[test]
fn test_put_invalid_json() {
    let response = request::put("http://localhost:300/foo",
                                Headers::new(), 
                                "123 malformed json",
                                &put
                               ).unwrap();
    //assert_eq!(response.status.unwrap(), status::BadRequest);
    assert_eq!(response.status.unwrap(), status::Ok);
}
