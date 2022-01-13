/// # Problem 45 - Jump Game II
/// Given an array of non-negative integers `nums`, you are initially positioned at the first
/// index of the array.
/// Each element in the array represents your maximum jump length at that position.
/// Your goal is to reach the last index in the minimum number of jumps.
/// **You can assume that you can always reach the last index.**
///
/// ## Example 1
/// `Input: nums = [2,3,1,1,4]`
/// `Output: 2`
///  Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from
/// index 0 to 1, then 3 steps to the last index.

pub fn jump(nums: Vec<i32>) -> i32 {
    // (index, num_jumps_occured)
    let mut dfs: Vec<(usize, i32)> = vec![(nums.len() - 1, 0)];

    while let Some((end_idx, jumps)) = dfs.pop() {
        // base case
        if end_idx == 0 {
            return jumps;
        }

        for cur_idx in (0..end_idx).rev() {
            let jump_amt = nums[cur_idx] as usize;
            if cur_idx + jump_amt >= end_idx {
                dfs.push((cur_idx, jumps + 1));
            }
        }
    }

    -1
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::jump;

    #[test]
    fn example1() {
        let nums = vec![2, 3, 1, 1, 4];
        let jumps = jump(nums);
        assert_eq!(jumps, 2);
    }

    #[test]
    fn list_of_len_1() {
        let nums = vec![2];
        let jumps = jump(nums);
        assert_eq!(jumps, 0);
    }

    #[test]
    fn list_of_len_2() {
        let nums = vec![2, 4];
        let jumps = jump(nums);
        assert_eq!(jumps, 1);
    }

    #[test]
    fn direct_jump_from_start() {
        let nums = vec![4, 1, 1, 1, 1];
        let jumps = jump(nums);
        assert_eq!(jumps, 1);
    }

    #[test]
    fn example2() {
        let nums = vec![2, 3, 0, 1, 4];
        let jumps = jump(nums);
        assert_eq!(jumps, 2);
    }

    #[test]
    fn example3() {
        let nums = vec![3, 2, 1];
        let jumps = jump(nums);
        assert_eq!(jumps, 1);
    }

    #[test]
    fn example4() {
        let nums = vec![4, 1, 1, 3, 1, 1, 1];
        let jumps = jump(nums);
        assert_eq!(jumps, 2);
    }
}
