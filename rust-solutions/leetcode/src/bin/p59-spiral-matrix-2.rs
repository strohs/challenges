/// # Problem 59 -  Spiral Matrix 2
/// Given a positive integer n, generate an n x n matrix
/// filled with elements from 1 to n2 in spiral order.
///
/// `1 <= n <= 20`
///
pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {

    let mut matrix: Vec<Vec<i32>> = vec![vec![0_i32; n as usize]; n as usize];
    let dim = n as usize; // dimension of the square matrix
    let (mut rb, mut re, mut cb, mut ce) = (0_usize, dim, 0_usize, dim);
    let mut cur = 1;

    for &i in [1,2,3,4].iter().cycle() {

        // iterate columns
        if i % 4 == 1 {
            for c in cb..ce {
                matrix[rb][c] = cur;
                cur += 1;
            }
            rb += 1;
        }
        // iterate rows
        if i % 4 == 2 {
            for r in rb..re {
                matrix[r][ce - 1] = cur;
                cur += 1;
            }
            ce -= 1;
        }
        // iterate columnms in reverse
        if i % 4 == 3 {
            for c in (cb..ce).rev() {
                matrix[re - 1][c] = cur;
                cur += 1;
            }
            re -= 1;
        }
        // iterate rows in reverse
        if i % 4 == 0 {
            for r in (rb..re).rev() {
                matrix[r][cb] = cur;
                cur += 1;
            }
            cb += 1;
        }

        if rb >= re || cb >= ce {
            break
        }

    }

    matrix
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::generate_matrix;

    #[test]
    fn example1() {
        let matrix = vec![vec![1,2,3], vec![8,9,4], vec![7,6,5]];
        let res = generate_matrix(3);
        assert_eq!(res, matrix);
    }

    #[test]
    fn example2() {
        let matrix = vec![vec![1]];
        let res = generate_matrix(1);
        assert_eq!(res, matrix);
    }

    #[test]
    fn example3() {
        let matrix = vec![vec![1,2], vec![4,3]];
        let res = generate_matrix(2);
        assert_eq!(res, matrix);
    }
}