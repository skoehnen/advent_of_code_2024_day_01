mod downloader;
mod data_transform;
mod calculation;

use std::fs;
use crate::calculation::calculate_distance;

fn main() {
    let file_path = "data/input.data";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    //let split_data = data_transform::transform(contents);

    let distance = calculate_distance(contents);

    println!("{}", distance);
}
