use std::vec;
type Image = Vec<Vec<u32>>;

fn rotate_matrix_90_degrees(matrix: &Image) -> Image {
    let matrix_size: usize = matrix.len();
    let mut result: Image = Vec::new();
    for _ in 0..matrix_size {
        result.push(vec![0; matrix_size]);
    }

    //Swap vertically
    for (i, matrix_row) in matrix.iter().enumerate().take(matrix_size) {
        for j in 0..matrix_size {
            result[matrix_size - 1 - j][i] = matrix_row[j];
        }
    }

    result
}

fn rotate_matrix_90_degrees_inplace(matrix: &mut Image) {
    let matrix_size: usize = matrix.len();
    //Swap vertically
    for i in 0..matrix_size / 2 {
        for j in 0..matrix_size - 1 - i {
            let tmp = matrix[matrix_size - 1 - j][i];
            matrix[matrix_size - 1 - j][i] = matrix[i][j];
            matrix[i][j] = matrix[j][matrix_size - 1 - i];
            matrix[j][matrix_size - 1 - i] = matrix[matrix_size - 1 - i][matrix_size - 1 - j];
            matrix[matrix_size - 1 - i][matrix_size - 1 - j] = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{rotate_matrix_90_degrees, rotate_matrix_90_degrees_inplace, Image};

    use rstest::rstest;

    #[rstest]
    #[case(
        vec![
            vec![255, 0],
            vec![0, 255]
        ],
         vec![
            vec![0, 255],
            vec![255, 0]
        ]
    )]
    #[case(
        vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ],
         vec![
            vec![3, 6, 9],
            vec![2, 5, 8],
            vec![1, 4, 7]
        ]
    )]
    fn test_rotate_matrix_90_degrees(#[case] matrix: Image, #[case] expected: Image) {
        assert_eq!(rotate_matrix_90_degrees(&matrix), expected);
    }

    #[rstest]
    #[case(
        vec![
            vec![255, 0],
            vec![0, 255]
        ],
         vec![
            vec![0, 255],
            vec![255, 0]
        ]
    )]
    #[case(
        vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ],
         vec![
            vec![3, 6, 9],
            vec![2, 5, 8],
            vec![1, 4, 7]
        ]
    )]
    fn test_rotate_matrix_90_degrees_inplace(#[case] matrix: Image, #[case] expected: Image) {
        let mut m = matrix;
        rotate_matrix_90_degrees_inplace(&mut m);

        assert_eq!(m, expected);
    }
}

fn main() {
    println!(
        "{:?}",
        rotate_matrix_90_degrees(&vec![vec![1, 2, 3], vec![1, 2, 3], vec![3, 3, 3]]),
    );

    let mut example = vec![vec![1, 2, 3], vec![1, 2, 3], vec![3, 3, 3]];
    rotate_matrix_90_degrees_inplace(&mut example);
    print!("{:?}", example);
}
