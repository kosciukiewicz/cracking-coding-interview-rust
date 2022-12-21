use std::vec;

type Matrix = Vec<Vec<u32>>;

fn zero_matrix(matrix: &Matrix) -> Matrix {
    let mut result = matrix.clone();

    let mut col_to_zero: Vec<bool> = vec![false; matrix[0].len()];
    let mut row_to_zero: Vec<bool> = vec![false; matrix.len()];

    for (i, row) in matrix.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            if *element == 0 {
                row_to_zero[i] = true;
                col_to_zero[j] = true;
            }
        }
    }

    for row in result.iter_mut() {
        for (j, element) in row.iter_mut().enumerate() {
            if col_to_zero[j] {
                *element = 0;
            }
        }
    }

    for (i, row) in result.iter_mut().enumerate() {
        if row_to_zero[i] {
            for element in row.iter_mut() {
                *element = 0;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{zero_matrix, Matrix};

    use rstest::rstest;

    #[rstest]
    #[case(
    vec ! [
    vec ! [255, 0],
    vec ! [12, 255]
    ],
    vec ! [
    vec ! [0, 0],
    vec ! [12, 0]
    ]
    )]
    #[case(
    vec ! [
    vec ! [1, 2, 3],
    vec ! [4, 0, 6],
    vec ! [7, 8, 0]
    ],
    vec ! [
    vec ! [1, 0, 0],
    vec ! [0, 0, 0],
    vec ! [0, 0, 0]
    ]
    )]
    fn test_zero_matrix(#[case] matrix: Matrix, #[case] expected: Matrix) {
        assert_eq!(zero_matrix(&matrix), expected);
    }
}

fn main() {
    println!(
        "{:?}",
        zero_matrix(&vec![vec![1, 2, 3], vec![1, 2, 3], vec![3, 3, 3]]),
    );
}
