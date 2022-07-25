/// # Problem 64 - Minimum Path Sum
/// Given a `m x n` grid filled with non-negative numbers, find a path from top left
/// to bottom right, which minimizes the sum of all numbers along its path.
///
/// Note: You can only move either down or right at any point in time.

// use a "dynamic programming solution"TM
// start at top left
//  prefill left column with the sum of each cell plus the cell above it
//  prefill top row with the sum of each cell plus the cell to its left
//
// now iterate rows and columns, starting from row_len + 1 and col_len + 1
// at each cell, sum its value with the cell to the left and sum its value with the cell above
//      take the minimum of those two values as the value of the current cell
// repeat until you reach the cell at index row_len - 1, col_len - 1; where it will have the minimum sum


pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let row_len = grid.len();
    let col_len = grid[0].len();

    // allocate a new matrix to hold the sums
    let mut sums = vec![vec![0_i32; col_len]; row_len];

    // prefill leftmost col
    let mut sum = 0;
    for r in 0..row_len {
        sum += grid[r][0];
        sums[r][0] = sum;
    }

    // prefill top row
    sum = 0;
    for c in 0..col_len {
        sum += grid[0][c];
        sums[0][c] = sum;
    }
    for r in 1..row_len {
        for c in 1..col_len {
            let left = grid[r][c] + sums[r][c-1];
            let top = grid[r][c] + sums[r-1][c];
            sums[r][c] = std::cmp::min(left, top);
        }
    }

    sums[row_len-1][col_len-1]
}


fn main() {}

#[cfg(test)]
mod tests {
    use crate::min_path_sum;

    #[test]
    fn example1() {
        let grid = vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1],
        ];
        assert_eq!(min_path_sum(grid), 7);
    }

    #[test]
    fn one_by_one_grid() {
        let grid = vec![
            vec![3],
        ];
        assert_eq!(min_path_sum(grid), 3);
    }
}