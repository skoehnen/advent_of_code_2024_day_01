pub fn transform(data: String) -> (Vec<i32>, Vec<i32>){
    let mut vec_left = Vec::new();
    let mut vec_right = Vec::new();

    for line in data.lines() {
        let mut split_data = line.split("   ");

        let left_str = split_data.next();
        let right_str = split_data.next();

        let left_number: i32 = left_str.unwrap().parse().unwrap();
        let right_number: i32 = right_str.unwrap().parse().unwrap();

        vec_left.push(left_number);
        vec_right.push(right_number);
    }

    let result_tuple = (vec_left, vec_right);

    return result_tuple;
}