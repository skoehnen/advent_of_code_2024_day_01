mod downloader;
mod data_transform;
mod calculation;

use std::fs;

fn main() {
    let file_path = "data/test.data";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    let split_data = data_transform::transform(contents);

    let (vec_left, vec_right) = split_data;

    for item in vec_left.iter() {
        println!("{}", item)
    }

    println!(" ");

    for item in vec_right.iter() {
        println!("{}", item)
    }
}
