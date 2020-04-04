#[macro_use]
extern crate http_guest;

use http_guest::{KVStore, Request, Response};

pub fn entrypoint(kvs: &mut KVStore, req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    let path = req.uri().to_string();

    if req.method().as_str() == "POST" && !path.is_empty() {
        kvs.insert(&path, req.body());
        Response::builder()
            .status(200)
            .body(b"".to_vec())
            .unwrap()
    } else if req.method().as_str() == "GET" && !path.is_empty() {
        Response::builder()
            .status(200)
            .body(kvs.get(&path).unwrap_or(b"".to_vec()))
            .unwrap()
    } else {
        Response::builder()
            .status(400)
            .body(b"".to_vec())
            .unwrap()
    }
}

guest_app_kvs!(entrypoint);
