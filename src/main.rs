mod downloader;
mod data_transform;

use std::fs;

fn main() {
    let file_path = "data/test.data";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
