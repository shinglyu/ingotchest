ingotchest
-----

ingotchest is a plain-text-file-backed key-value store, with a REST API. You can write local applications with a client-server architecture, but at the same time being able to manually edit the data in the database.

## Usage
* Run: `cargo run`, the server will start on port 3000
* Get value by key `foo`: `curl -X GET http://localhost:3000/foo`
* Update value for key `foo`: `curl -X PUT http://localhost:3000/foo -d '{"Hello": "World"}'`
* Delete key `foo`: `curl -X DELETE http://localhost:3000/foo`

* Run with logging: `RUST_LOG=ingotchest=debug cargo run`
