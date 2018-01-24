extern crate iron;
extern crate iron_test;
extern crate router;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate serde_value;

mod backend_yaml;

use iron::prelude::*;
use iron::{status};
use router::Router;
use std::error::Error;
use std::fs::{remove_file};
use std::io::{Read};
use std::path::{Path, PathBuf};

fn get_path(key: &str) -> PathBuf {
    return Path::new(key).with_extension("json");
}

fn get(req: &mut Request) -> IronResult<Response> {
    let route_info = req.extensions.get::<Router>().unwrap();
    let ref key = route_info.find("key").unwrap_or("");
    let key_path = get_path(key);
    debug!("GET {:?}", key_path);

    if !key_path.exists() {
        // TODO: Check what CouchDB return if database don't exist
        return Ok(Response::with((status::NotFound, format!("File not found: {:?}", key_path))))
    }

    // FIXME: Read file everytime, try caching
    // But the good point is we don't need to monitor if the file changes
    // Check https://github.com/iron/staticfile for caching
    Ok(Response::with((status::Ok, key_path)))
}

fn put(req: &mut Request) -> IronResult<Response> {
    // TODO: Extract the key getting logic
    let route_info = req.extensions.get::<Router>().unwrap();
    let ref key = route_info.find("key").unwrap_or("");
    // TODO: Do not allow file not in the current dir
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).expect("Fail to read request body");
    debug!("{:?}", payload);
    // Validate JSON format
    let value: serde_value::Value = match serde_json::from_str::<serde_value::Value>(&payload) {
        Ok(value) => value,
        Err(why) => {
            let error_msg = format!("{} at line {} column {}",
                                    why.description(),
                                    why.line(),
                                    why.column(),
                                   );
            return Ok(Response::with((status::BadRequest, error_msg)));
        }
    };

    match backend_yaml::put(key, value) {
        Ok(_) => Ok(Response::with((status::Ok, "Ok"))),
        Err(why) => Ok(Response::with((status::InternalServerError, &why[..]))),
    }
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

fn create_router() -> Router {
    let mut router = Router::new();
    router.get("/:key", get, "get");
    router.put("/:key", put, "put");
    router.delete("/:key", delete, "delete");

    router
}

fn main() {
    env_logger::init();

    info!("Starting server on http://localhost:3000");
    Iron::new(create_router()).http("localhost:3000").unwrap();
}


#[cfg(test)]
mod test {
    use iron::{Headers, status};
    use iron_test::{request, response};
    use super::{create_router};

    #[test]
    fn test_put_invalid_json() {
        let response = request::put("http://localhost:3000/foo",
                                    Headers::new(),
                                    "123 malformed json",
                                    &create_router()
                                   ).unwrap();
        assert_eq!(response.status.unwrap(), status::BadRequest);
        let msg = response::extract_body_to_string(response);
        assert_eq!(msg, "JSON error at line 1 column 5");
    }
}
