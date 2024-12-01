mod downloader;
mod data_transform;
mod calculation;

use std::fs;
use crate::calculation::{calculate_distance, calculate_similarity_score};

fn main() {
    let file_path = "data/test.data";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    //let split_data = data_transform::transform(contents);

    //let distance = calculate_distance(contents);
    let similarity = calculate_similarity_score(contents);

    println!("{}", similarity);
}
