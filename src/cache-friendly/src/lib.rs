pub fn row_major_call(matrix: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;
    for row in matrix {
        for &val in row {
            sum += val as usize;
        }
    }
    sum
}

pub fn column_major_call(matrix: &Vec<Vec<u8>>) -> usize {
    let size = matrix.len();
    let mut sum = 0;
    for col in 0..size {
        for row in 0..size {
            sum += matrix[row][col] as usize;
        }
    }
    sum
}