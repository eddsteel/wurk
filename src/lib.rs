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

pub mod camera {
    use uuid::Uuid;
    use rscam;
    use std::fs;
    use std::io::Write;

    fn rscamerr_to_string(err: rscam::Error) -> String {
        match err {
            _ => String::from("boo boo")
        }
    }

    fn filename() -> String {
        format!("/var/tmp/{}.jpeg", Uuid::new_v4().hyphenated().to_string())
    }

    fn config<'a>() -> rscam::Config<'a>{
        rscam::Config {
            interval: (1, 15),
            resolution: (1280, 720),
            format: b"MJPG",
            ..Default::default()
        }
    }

    pub fn burst() -> Result<Vec<String>, String> {
        do_burst().map_err(rscamerr_to_string)
    }

    fn do_burst() -> Result<Vec<String>, rscam::Error> {
        let mut camera = try!(rscam::new("/dev/video0").map_err(rscam::Error::Io));
        try!(camera.start(&config()));

        let mut vec = Vec::new();
        for _ in 1..3 {
            let filename = filename();
            let frame = camera.capture().map_err(rscam::Error::Io);
            let mut file = try!(fs::File::create(&filename));
            try!(frame.and_then(|f| file.write_all(&f[..]).map_err(rscam::Error::Io)));

            vec.push(filename);
        }

        try!(camera.stop().map_err(rscam::Error::Io));

        Ok(vec)
    }
}
