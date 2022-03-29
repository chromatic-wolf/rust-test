use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}



fn main() -> std::io::Result<()> {
    let point = Point { x: 1, y: 2, z: 12 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    let mut file = File::create("foo.txt")?;
    file.write_all(serialized.as_bytes())?;
    

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
    Ok(())
}