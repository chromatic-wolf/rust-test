use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
mod downloader;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() -> std::io::Result<()> {
    let point = Point { x: 1, y: 2, z: 12 };
    
    let now = Instant::now();
    downloader::download_file(
        String::from("test.zip"),
        String::from("http://ipv4.download.thinkbroadband.com:81/20MB.zip"),
    );
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    let mut file = File::create("foo.txt")?;
    file.write_all(serialized.as_bytes())?;

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
    Ok(())
}
