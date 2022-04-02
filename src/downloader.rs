use std::{fs::File, io};

pub fn download_file(path: String, url: String) {
    //print!("{}", value);
    let mut resp = reqwest::blocking::get(url).expect("request failed");
    let mut out = File::create(path.clone()).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}
