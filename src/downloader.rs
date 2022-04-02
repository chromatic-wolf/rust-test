use std::{fs::File, io, collections::HashMap};

pub fn download_file(path: String, url: String)
{
    
        let do_steps = || -> Result<(), Box<dyn std::error::Error>> {
            let resp = reqwest::blocking::get(url)?
                .json::<HashMap<String, String>>()?;

            //println!("{:#?}", resp);

            for (key, value) in &resp {
                if key == "message" {
                    //print!("{}", value);
                    let mut resp = reqwest::blocking::get(value).expect("request failed");
                    let mut out = File::create(path.clone()).expect("failed to create file");
                    io::copy(&mut resp, &mut out).expect("failed to copy content");
                }
            }

            Ok(())
        };
        
        if let Err(_err) = do_steps() {
            eprintln!("Application error: {}", _err);
        }

    }