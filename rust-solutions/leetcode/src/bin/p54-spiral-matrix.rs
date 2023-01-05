/// Given an `m x n matrix`, return all elements of the matrix in spiral order.
/// # Example
/// Given the matrix
/// ```
/// 1 2 3
/// 4 5 6
/// 7 8 9
/// ```
/// the spiral order would be:
/// `1,2,3,6,9,8,7,4,5`
///
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let row_len = matrix.len();
    let col_len = matrix[0].len();

    let mut spiral = vec![];
    let (mut rb, mut re, mut cb, mut ce) = (0_usize, row_len, 0_usize, col_len);

    for &i in [1, 2, 3, 4].iter().cycle() {
        // iterate columns
        if i % 4 == 1 {
            for c in cb..ce {
                spiral.push(matrix[rb][c]);
            }
            rb += 1;
        }
        // iterate rows
        if i % 4 == 2 {
            for r in rb..re {
                spiral.push(matrix[r][ce - 1]);
            }
            ce -= 1;
        }
        // iterate columnms in reverse
        if i % 4 == 3 {
            for c in (cb..ce).rev() {
                spiral.push(matrix[re - 1][c]);
            }
            re -= 1;
        }
        // iterate rows in reverse
        if i % 4 == 0 {
            for r in (rb..re).rev() {
                spiral.push(matrix[r][cb]);
            }
            cb += 1;
        }

        if rb >= re || cb >= ce {
            break;
        }
    }
    spiral
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::spiral_order;

    #[test]
    fn example1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let spiral = spiral_order(matrix);
        assert_eq!(spiral, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn example2() {
        let matrix = vec![vec![1, 2, 3]];
        let spiral = spiral_order(matrix);
        assert_eq!(spiral, vec![1, 2, 3]);
    }

    #[test]
    fn example3() {
        let matrix = vec![vec![1]];
        let spiral = spiral_order(matrix);
        assert_eq!(spiral, vec![1]);
    }

    #[test]
    fn example4() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let spiral = spiral_order(matrix);
        assert_eq!(spiral, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }
}
