mod downloader;
mod serialiser;
use std::time::Instant;
fn main() -> std::io::Result<()> {
    let now = Instant::now();
    downloader::download_file(
        String::from("test.zip"),
        String::from("http://speedcheck.cdn.on.net/10meg.test"),
    );
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
