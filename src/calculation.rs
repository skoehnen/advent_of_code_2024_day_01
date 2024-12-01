use crate::data_transform;

pub fn calculate_similarity_score(data: String) -> i32 {
    let similarity_vector = calculate_similarity_vector(
        data_transform::transform(data)
    );

    return similarity_vector.iter().sum();
}

pub fn calculate_distance(data: String) -> i32 {
    let difference_vector =
        calculate_difference_vector(
            sort_vectors(
                data_transform::transform(data)
            )
    );

    return difference_vector.iter().sum();
}

fn calculate_similarity_vector(data: (Vec<i32>, Vec<i32>)) -> Vec<i32> {
    let mut similarity_vector: Vec<i32> = Vec::new();

    for element in data.0 {
        let count = data.1.iter().filter(|&n| *n == element).count();
        similarity_vector.push(element * i32::try_from(count).unwrap());
    }

    return similarity_vector;
}

fn sort_vectors(data: (Vec<i32>, Vec<i32>)) -> (Vec<i32>, Vec<i32>) {
    let vec_left = &mut data.0.clone();
    let vec_right = &mut data.1.clone();

    vec_left.sort();
    vec_right.sort();

    return ( vec_left.to_vec(), vec_right.to_vec() );
}

fn calculate_difference_vector(data: (Vec<i32>, Vec<i32>)) -> Vec<i32> {
    let mut vec_right_iter = data.1.iter();
    let mut vec_difference = Vec::new();

    for item in data.0 {
        let current_vec_right = vec_right_iter.next().unwrap();
        println!("{:?} - {:?}", item, current_vec_right);
        let difference = item - current_vec_right;
        vec_difference.push(difference.abs());
    }

    return vec_difference;
}