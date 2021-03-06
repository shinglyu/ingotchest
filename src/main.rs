extern crate iron;
extern crate iron_test;
extern crate iron_cors;
extern crate router;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate serde_value;

mod backend_yaml;

use iron::prelude::*;
use iron::{status};
use iron::headers::ContentType;
use iron_cors::CorsMiddleware;
use router::Router;
use std::error::Error;
use std::io::{Read};

fn get(req: &mut Request) -> IronResult<Response> {
    let route_info = req.extensions.get::<Router>().unwrap();
    let ref key = route_info.find("key").unwrap_or("");
    let value: serde_value::Value = match backend_yaml::get(key) {
        Ok(value) => value,
        Err(_)  => return Ok(Response::with((status::NotFound, format!("Key not found: {:?}", key))))
    };
    match serde_json::to_string(&value) {
        Ok(value) => {
            let mut resp = Response::with((status::Ok, value));
            resp.headers.set(ContentType::json());
            Ok(resp)
        }
        Err(_)  => Ok(Response::with((status::InternalServerError, "Can't serialize as JSON")))
    }
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

    if let Err(why) = backend_yaml::backup(key) {
        return Ok(Response::with((status::InternalServerError, &why[..])));
    }

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
    match backend_yaml::delete(key) {
        Ok(_) => {
            Ok(Response::with((status::Ok, "Ok")))
        },
        Err(why)  => Ok(Response::with((status::InternalServerError, why)))
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

    let cors_middleware = CorsMiddleware::with_allow_any();
    let mut chain = Chain::new(create_router());
    chain.link_around(cors_middleware);

    info!("Starting server on http://localhost:3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}


#[cfg(test)]
mod test {
    use iron::{Headers, status};
    use iron::headers::ContentType;
    use iron_test::{request, response};
    use serde_json;
    use serde_value;
    use std::fs::File;
    use std::io::Write;
    use super::{create_router};

    #[test]
    fn test_get_content_type() {
        // TODO: Use mock backend
        let value: serde_value::Value = serde_json::from_str("{\"hello\": \"world\"}").unwrap(); 

        let mut file = File::create("test_content_type.yml").expect("Fail to create test file");
        file.write_all(serde_json::to_string(&value).expect("Fail to serialize").as_bytes()).expect("Fail to write to file");


        let response = request::get("http://localhost:3000/test_content_type",
                                    Headers::new(),
                                    &create_router()
                                   ).unwrap();
        assert_eq!(response.status.unwrap(), status::Ok);
        assert!(response.headers.has::<ContentType>());
        assert_eq!(response.headers.get::<ContentType>().unwrap(), &ContentType::json());
        let msg = response::extract_body_to_string(response);
        assert_eq!(msg, serde_json::to_string(&value).expect("Fail to serialize"));
    }
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
