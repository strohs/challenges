/// # 77 Combinations
/// Given two integers `n` and `k`, return all possible combinations of `k` numbers chosen
/// from the range `[1, n]`.
/// You may return the answer in any order.
/// Note that combinations are unordered, i.e., `[1,2]` and `[2,1]` are considered to be
/// the same combination.


struct Solution;

impl Solution {

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut tmp: Vec<i32> = Vec::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        Solution::combine_rec(1, n, k, &mut tmp, &mut ans);
        ans
    }

    fn combine_rec(left: i32, n: i32, k: i32, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if k == 0 {
            ans.push(tmp.clone());
        } else {
            for i in left..=n {
                tmp.push(i);
                println!("{:?}", &tmp);
                Solution::combine_rec(i+1, n, k-1, tmp, ans);
                tmp.pop();
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        let ans = Solution::combine(4, 2);
        assert_eq!(ans, vec![vec![1,2], vec![1,3], vec![1,4], vec![2, 3], vec![2, 4], vec![3,4]]);
    }

    #[test]
    fn example2() {
        let ans = Solution::combine(1,1);
        assert_eq!(ans, vec![vec![1]]);
    }

    #[test]
    fn example3() {
        let ans = Solution::combine(2,1);
        assert_eq!(ans, vec![vec![1], vec![2]]);
    }

    #[test]
    fn example4() {
        let ans = Solution::combine(4,3);
    }
}