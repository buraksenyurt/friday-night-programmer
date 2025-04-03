pub const MAX_SIZE: usize = 1024;

pub fn row_major_call(matrix: &[[u8; MAX_SIZE]; MAX_SIZE]) -> usize {
    let mut sum = 0;
    for row in 0..MAX_SIZE {
        for col in 0..MAX_SIZE {
            sum += matrix[row][col] as usize;
        }
    }
    sum
}

pub fn column_major_call(matrix: &[[u8; MAX_SIZE]; MAX_SIZE]) -> usize {
    let mut sum = 0;
    for col in 0..MAX_SIZE {
        for row in 0..MAX_SIZE {
            sum += matrix[row][col] as usize;
        }
    }
    sum
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
