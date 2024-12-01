use crate::data_transform;

pub fn calculate_similarity_score(data: String) -> i32 {
    let similarity_vector = sort_vectors(
        data_transform::transform(data)
    );

    return 0;
}

pub fn calculate_distance(data: String) -> i32 {
    let difference_vector =
        calculate_difference_vector(
            sort_vectors(
                data_transform::transform(data)
            )
    );

    return difference_vector.iter().sum();;
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