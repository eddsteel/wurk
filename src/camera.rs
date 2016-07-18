use uuid::Uuid;
use rscam;
use std::fs;
use std::io;
use std::io::Write;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PhotoRep {
    pub photos: Vec<String>
}

fn rscamerr_to_string(err: rscam::Error) -> String {
    match err {
        rscam::Error::Io(e) => e.to_string(),
        rscam::Error::BadInterval => String::from("bad interval"),
        rscam::Error::BadResolution => String::from("bad resolution"),
        rscam::Error::BadFormat => String::from("bad format"),
        rscam::Error::BadField => String::from("bad field")
    }
}

fn id() -> String {
    Uuid::new_v4().hyphenated().to_string()
}

fn filename(id: &String) -> String {
    format!("/var/tmp/{}.jpeg", id)
}

fn path(id: &String) -> String {
    format!("/photos/{}", id)
}

fn config<'a>() -> rscam::Config<'a>{
    rscam::Config {
        interval: (1, 15),
        resolution: (640, 480),
        format: b"MJPG",
        ..Default::default()
    }
}

pub fn file_from_id(id: String) -> io::Result<fs::File> {
    fs::File::open(&filename(&id))
}

pub fn burst() -> Result<PhotoRep, String> {
    do_burst().map_err(rscamerr_to_string)
}

fn do_burst() -> Result<PhotoRep, rscam::Error> {
    let mut camera = try!(rscam::new("/dev/video0").map_err(rscam::Error::Io));
    try!(camera.start(&config()));

    let mut vec = Vec::new();
    for _ in 1..3 {
        let id = id();
        let filename = filename(&id);
        let frame = camera.capture().map_err(rscam::Error::Io);
        let mut file = try!(fs::File::create(&filename));
        try!(frame.and_then(|f| file.write_all(&f[..]).map_err(rscam::Error::Io)));

        vec.push(path(&id));
    }

    try!(camera.stop().map_err(rscam::Error::Io));

    Ok(PhotoRep{ photos: vec })
}
