extern crate conduit;
extern crate conduit_router;
extern crate civet;
extern crate wurk;

use std::collections::HashMap;
use std::io::{self, Cursor};
use std::sync::mpsc::channel;

use civet::{Config, Server};
use conduit::{Request, Response};
use conduit_router::RouteBuilder;

use wurk::system;
use wurk::camera;

fn mk_resp(s: String) -> Response {
    let mut headers = HashMap::with_capacity(1);
    headers.insert(String::from("Content-Type"), vec!(String::from("text/plain")));
    civet::response(200, headers, Cursor::new(s.into_bytes()))
}

fn hostname(_req: &mut Request) -> io::Result<Response> {
    system::hostname().map(mk_resp)
}


fn server_listen() -> () {
    let mut router = RouteBuilder::new();

    router.get("/", hostname);

    let mut cfg = Config::new();
    cfg.port(8000).threads(1);

    let _server = Server::start(cfg, router);

    let (_tx, rx) = channel::<()>();
    rx.recv().unwrap();
}

fn try_some_shit() -> () {
    let imgs = camera::burst();

    match imgs {
        Ok(is) =>
            for i in &is {
                println!("{}", i)
            },
        Err(err) =>
            panic!(err.to_string())
    }
}

pub fn main() -> () {
    // server_listen()
    try_some_shit()
}
