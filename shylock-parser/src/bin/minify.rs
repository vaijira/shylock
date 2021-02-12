use std::io::{Read, Write};

const MINIFIER_URL: &str = "https://javascript-minifier.com/raw";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("my_agent/0.1")
        .build()?;

    let src = "/tmp/test.js";
    let mut file = std::fs::File::open(src)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let params = [("input", data)];

    let body = client
        .post(MINIFIER_URL)
        .form(&params)
        .send()?
        .error_for_status()?
        .text()?;

    let dst = "/tmp/test.min.js";
    let mut file = std::fs::File::create(&dst)?;
    file.write_all(body.as_bytes())?;

    Ok(())
}
