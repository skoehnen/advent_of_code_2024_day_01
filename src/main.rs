mod downloader;

fn main() {
    println!("Hello, world!");
    //let body = reqwest::blocking::get("https://www.rust-lang.org")?
    //    .text()?;

    let body = match reqwest::blocking::get("https://www.rust-lang.org") {
        Ok(body) => body,
        Err(error) => panic!("There was a problem with the get: {:?}", error),
    };

    let body_text = match body.text() {
        Ok(body) => body,
        Err(error) => panic!("There was a problem with the request: {:?}", error),
    };

    println!("body_text = {body_text:?}");
}
