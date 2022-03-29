use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() -> std::io::Result<()> {
    let point = Point { x: 1, y: 2, z: 12 };
    let mut count: i64 = 1;
    while true {
        let do_steps = || -> Result<(), Box<dyn std::error::Error>> {
            let resp = reqwest::blocking::get("https://dog.ceo/api/breeds/image/random")?
                .json::<HashMap<String, String>>()?;

            //println!("{:#?}", resp);

            for (key, value) in &resp {
                if key == "message" {
                    //print!("{}", value);
                    let mut resp = reqwest::blocking::get(value).expect("request failed");
                    let mut out = File::create(String::from(&count.to_string()) + ".jpg").expect("failed to create file");
                    io::copy(&mut resp, &mut out).expect("failed to copy content");
                }
            }

            Ok(())
        };
        
        if let Err(_err) = do_steps() {
            eprintln!("Application error: {}", _err);
        }
        count = count + 1;

    }

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    let mut file = File::create("foo.txt")?;
    file.write_all(serialized.as_bytes())?;

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
    Ok(())
}
