extern crate conduit;
extern crate conduit_router;
extern crate civet;
extern crate wurk;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::io::{self, Cursor};
use std::result;
use std::sync::mpsc::channel;

use civet::{Config, Server};
use conduit::{Request, Response};
use conduit_router::{RouteBuilder, RequestParams};
use serde::Serialize;

use wurk::system;
use wurk::camera;

enum HttpError {
    BadRequest(String),
    InternalServerError(String)
}
// TODO conversions from various other errors involved.
impl HttpError {
    fn bad_request(s: &str) -> HttpError {
        HttpError::BadRequest(String::from(s))
    }

    fn internal_server_error(s: &str) -> HttpError {
        HttpError::InternalServerError(String::from(s))
    }
}

type HttpResult<T> = result::Result<T, HttpError>;


fn mk_resp(s: String) -> Response {
    let mut headers = HashMap::with_capacity(1);
    headers.insert(String::from("Content-Type"), vec!(String::from("text/plain")));
    civet::response(200, headers, Cursor::new(s.into_bytes()))
}

fn mk_json_rep<T: Serialize>(attempt: HttpResult<T>) -> io::Result<Response> {
    let mut headers = HashMap::with_capacity(1);
    headers.insert(String::from("Content-Type"), vec!(String::from("application/json")));

    let json = attempt.and_then(|t| {
        serde_json::to_string(&t).map_err(|_| {
            // TODO: log what kind of error we have from JSON ser
            HttpError::internal_server_error("unable to create JSON")
        })
    });

    Ok(match json {
        Ok(j) =>
            civet::response(200, headers, Cursor::new(j.into_bytes())),
        Err(HttpError::BadRequest(e)) =>
            civet::response(400, headers, Cursor::new(e.into_bytes())),
        Err(HttpError::InternalServerError(e)) =>
            civet::response(500, headers, Cursor::new(e.into_bytes()))
    })
}

fn hostname(_req: &mut Request) -> io::Result<Response> {
    system::hostname().map(mk_resp)
}

fn photos(_req: &mut Request) -> io::Result<Response> {
    let imgs = camera::burst().map_err(HttpError::InternalServerError);
    mk_json_rep(imgs)
}

fn validate_photo_uuid(s: Option<&str>) -> HttpResult<String> {
    s.map(String::from).ok_or(HttpError::bad_request("please provide photo name"))
}

fn photo(_req: &mut Request) -> io::Result<Response> {
    let filepath = validate_photo_uuid(_req.params().find("uuid"));
    let photo = filepath.and_then(|path| camera::file_from_id(path).map_err(|e| {
        HttpError::InternalServerError(e.to_string())
    }));

    let mut headers = HashMap::new();

    Ok(match photo {
        Ok(p) => {
            headers.insert(String::from("Content-Type"), vec!(String::from("image/jpeg")));
            // TODO: Content-Size
            civet::response(200, headers, Box::new(p))
        },
        Err(HttpError::BadRequest(m)) => {
            headers.insert(String::from("Content-Type"), vec!(String::from("text/plain")));
            civet::response(400, headers, Cursor::new(m.into_bytes()))
        },
        Err(HttpError::InternalServerError(m)) => {
            headers.insert(String::from("Content-Type"), vec!(String::from("text/plain")));
            civet::response(500, headers, Cursor::new(m.into_bytes()))
        }
    })
}

fn server_listen() -> () {
    let mut router = RouteBuilder::new();

    router.get("/host", hostname);
    router.get("/photos", photos);
    router.get("/photos/:uuid", photo);

    let mut cfg = Config::new();
    cfg.port(8000).threads(1);

    let _server = Server::start(cfg, router);

    let (_tx, rx) = channel::<()>();
    rx.recv().unwrap();
}

pub fn main() -> () {
    println!("Listening on :8000");
    server_listen()
}
