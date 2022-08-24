/// # Problem 74 - Search a 2D Matrix
/// Write an efficient algorithm that searches for a value target in an m x n integer
/// matrix `matrix`. This matrix has the following properties:
///
/// - Integers in each row are sorted from left to right.
/// - The first integer of each row is greater than the last integer of the previous row.
///

struct Solution;

impl Solution {

    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // 1. binary search the first elements of each row to find the row target could be in
        // 2. binary search the target row for target

        return match matrix.binary_search_by_key(&target, |row| row[0]) {
            Ok(_) => true,
            Err(i) if i == 0 => matrix[i].binary_search(&target).map_or(false, |_| true),
            Err(i) => matrix[i-1].binary_search(&target).map_or(false, |_| true),
        };
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn target_in_matrix() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60],
            vec![62, 64, 66, 70],
            vec![80, 82, 84, 90],
        ];
        assert_eq!(Solution::search_matrix(matrix, 64), true);
    }

    #[test]
    fn target_less_than_first_row() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60],
            vec![62, 64, 66, 70],
            vec![80, 82, 84, 90],
        ];
        assert_eq!(Solution::search_matrix(matrix, 0), false);
    }

    #[test]
    fn target_greater_than_last_row() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60],
            vec![62, 64, 66, 70],
            vec![80, 82, 84, 90],
        ];
        assert_eq!(Solution::search_matrix(matrix, 111), false);
    }

    #[test]
    fn target_equal_last_item_in_row() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60],
            vec![62, 64, 66, 70],
            vec![80, 82, 84, 90],
        ];
        assert_eq!(Solution::search_matrix(matrix, 20), true);
    }

    #[test]
    fn target_in_1x1() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![7],
        ];
        assert_eq!(Solution::search_matrix(matrix, 7), true);
    }

    #[test]
    fn target_not_in_1x1() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![7],
        ];
        assert_eq!(Solution::search_matrix(matrix, 9), false);
    }
}