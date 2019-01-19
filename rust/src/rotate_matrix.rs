pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let len = matrix.len();

    for index in 0..len {
        for in_index in index..len {
            let buf = matrix[index][in_index];
            matrix[index][in_index] = matrix[in_index][index];
            matrix[in_index][index] = buf;
        }
    }

    for index in 0..len {
        matrix[index].reverse();
    }
}
