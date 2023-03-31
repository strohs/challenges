/// # Problem 120 - Triangle
/// [Triangle](https://leetcode.com/problems/triangle/)
/// given a triangular array, find the minimum path sum
///
/// uses a recursive top down approach with memoization

pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {

    fn dfs(level: usize, i: usize, triangle: &Vec<Vec<i32>>, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if let Some(m) = memo[level][i] {
            return m;
        }
        let mut path = triangle[level][i];
        if level < triangle.len() - 1 {
            path += core::cmp::min(
                dfs(level + 1, i, &triangle, memo),
                dfs(level + 1, i + 1, &triangle, memo));
        }
        memo[level][i] = Some(path);
        path
    }

    // initialize a len x len memoization matrix
    let mut memo: Vec<Vec<Option<i32>>> = Vec::with_capacity(triangle.len());
    for _ in 0..triangle.len() {
        memo.push(vec![None; triangle.len()]);
    }
    dfs(0, 0, &triangle, &mut memo)
}



fn main() {}

#[cfg(test)]
mod tests {
    use crate::minimum_total;

    #[test]
    fn test1() {
        let triangle = vec![vec![2], vec![3,4], vec![6,5,7], vec![4,1,8,3]];
        let sum = minimum_total(triangle);
        assert_eq!(sum, 11);
    }

    #[test]
    fn test2() {
        let triangle = vec![vec![2]];
        let sum = minimum_total(triangle);
        assert_eq!(sum, 2);
    }

    #[test]
    fn test3() {
        let triangle = vec![vec![2], vec![3,4], vec![6,5,9], vec![4,4,8,0]];
        let sum = minimum_total(triangle);
        assert_eq!(sum, 14);
    }

    #[test]
    fn test4() {
        let triangle = vec![vec![-1], vec![3,2], vec![-3,-1,-2]];
        let sum = minimum_total(triangle);
        assert_eq!(sum, -1);
    }
}