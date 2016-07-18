#![feature(custom_attribute, custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate rscam;
extern crate uuid;


pub mod system {
    use std::process::Command;
    use std::io::{Error, ErrorKind, Result};

    pub fn hostname() -> Result<String> {
        let output = Command::new("hostname").output();
        output.and_then(|o| {
            let res = String::from_utf8(o.stdout);
            res.map_err(|e| Error::new(ErrorKind::Other, e))
        })
    }
}

pub mod camera;
