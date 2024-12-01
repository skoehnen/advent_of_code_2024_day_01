mod downloader;

fn main() {
    println!("Hello, world!");
    
    let body = match reqwest::blocking::get("https://adventofcode.com/2024/day/1/input") {
        Ok(body) => body,
        Err(error) => panic!("There was a problem with the get: {:?}", error),
    };

    let body_text = match body.text() {
        Ok(body) => body,
        Err(error) => panic!("There was a problem with the request: {:?}", error),
    };

    println!("body_text = {body_text:?}");
}
