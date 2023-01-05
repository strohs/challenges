/// # Problem 63 - Unique Paths 2
/// You are given an m x n integer array grid. There is a robot initially located at
/// the top-left corner (i.e., `grid[0][0]`). The robot tries to move to the bottom-right
/// corner (i.e., `grid[m-1][n-1]`). The robot can only move either down or right at any
/// point in time.
/// An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot takes cannot include any square that is an obstacle.
///
/// Return the number of possible unique paths that the robot can take to reach the bottom-right corner.
///
/// The testcases are generated so that the answer will be less than or equal to 2 * 10^9
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let is_obstacle = |r: usize, c: usize| obstacle_grid[r][c] == 1;

    let row_len = obstacle_grid.len();
    let col_len = obstacle_grid[0].len();
    let mut paths: Vec<Vec<i32>> = vec![vec![0_i32; col_len]; row_len];

    for c in 0..col_len {
        paths[0][c] = if is_obstacle(0, c) {
            0
        } else {
            if c == 0 {
                1
            } else {
                paths[0][c - 1]
            }
        };
    }
    for r in 0..row_len {
        paths[r][0] = if is_obstacle(r, 0) {
            0
        } else {
            if r == 0 {
                1
            } else {
                paths[r - 1][0]
            }
        };
    }
    for r in 1..row_len {
        for c in 1..col_len {
            if is_obstacle(r, c) {
                paths[r][c] = 0;
            } else {
                paths[r][c] = paths[r - 1][c] + paths[r][c - 1];
            }
        }
    }

    paths[row_len - 1][col_len - 1]
}

fn main() {
    let grid = vec![vec![0_i32, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 0, 0]];

    let count = unique_paths_with_obstacles(grid);
    println!("{count}");
}

#[cfg(test)]
mod tests {
    use crate::unique_paths_with_obstacles;

    #[test]
    fn example1() {
        let grid = vec![vec![0_i32, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 0, 0]];
        assert_eq!(unique_paths_with_obstacles(grid), 4);
    }

    #[test]
    fn example_1x1_grid() {
        let grid = vec![vec![0_i32]];
        assert_eq!(unique_paths_with_obstacles(grid), 1);
    }

    #[test]
    fn example_1x1_obstacle() {
        let grid = vec![vec![1_i32]];
        assert_eq!(unique_paths_with_obstacles(grid), 0);
    }

    #[test]
    fn example_1x2() {
        let grid = vec![vec![1_i32, 0]];
        assert_eq!(unique_paths_with_obstacles(grid), 0);
    }
}
