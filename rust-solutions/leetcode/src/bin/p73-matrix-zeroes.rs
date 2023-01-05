/// # Problem 73 - Set Matrix Zeroes
/// Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
///
/// You must do it in place.
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    // check if the 0th row and 0th column have any natural zeroes in them
    // if so, they will need to be set to all zeroes at the end of the algorithm
    let zero_row = matrix[0].iter().any(|v| *v == 0);
    let zero_col = matrix.iter().any(|row| row[0] == 0);

    // use the top row and leftmost column to mark which rows/cols have a zero in them
    for ri in 1..matrix.len() {
        for ci in 1..matrix[0].len() {
            if matrix[ri][ci] == 0 {
                matrix[0][ci] = 0;
                matrix[ri][0] = 0;
            }
        }
    }

    // now iterate the top row, any zeroes indicate a column that needs to be filled
    for ci in 1..matrix[0].len() {
        if matrix[0][ci] == 0 {
            for ri in 1..matrix.len() {
                matrix[ri][ci] = 0;
            }
        }
    }

    // iterate the left column, any zeroes indicate a row that needs to be filled with zeroes
    for ri in 1..matrix.len() {
        if matrix[ri][0] == 0 {
            for ci in 1..matrix[0].len() {
                matrix[ri][ci] = 0;
            }
        }
    }

    if zero_row {
        for i in 0..matrix[0].len() {
            matrix[0][i] = 0;
        }
    }
    if zero_col {
        for i in 0..matrix.len() {
            matrix[i][0] = 0;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::set_zeroes;

    #[test]
    fn example1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let res_matrix = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, res_matrix);
    }

    #[test]
    fn example2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let res = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, res);
    }

    #[test]
    fn example3() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![0, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];
        let res = vec![
            vec![0, 0, 3, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, res);
    }

    #[test]
    fn example1x1() {
        let mut matrix = vec![vec![1]];
        let res = vec![vec![1]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, res);
    }
}
