use uuid::Uuid;
use rscam;
use std::fs;
use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

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

fn foo() -> Result<PhotoRep, rscam::Error> {
    let mut camera = try!(rscam::new("/dev/video0"));
    let id = id();
    let filename = filename(&id);

    let frame = camera.capture();            // frame: Result<rscam::Frame, io::Error>
    let file  = fs::File::create(&filename); // file: Result<fs::File, io::Error>

    file.and_then(|mut file| {
        frame.and_then(|frame| {
            file.write_all(&frame[..])
        })
    });
}

fn do_burst() -> Result<PhotoRep, rscam::Error> {
    let mut camera = try!(rscam::new("/dev/video0"));
    try!(camera.start(&config()));

    let mut vec = Vec::new();
    for _ in 1..6 {
        let id = id();
        let filename = filename(&id);

        let frame = try!(camera.capture());
        let file = fs::File::create(&filename);
        try!(file.and_then(|mut f| {
            f.write_all(&frame[..])
        }));
        vec.push(path(&id));
        thread::sleep(Duration::from_millis(65));
    }

    try!(camera.stop());
    Ok(PhotoRep{ photos: vec })
}
