use std::collections::HashSet;

/// # Problem 128 Longest Conescutive Subsequence
/// https://leetcode.com/problems/longest-consecutive-sequence/
///

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let num_set: HashSet<i32> = nums.iter().cloned().collect();
    let mut lcs = 0_i32;

    for n in nums.iter() {
        let mut cur_lcs = 1_i32;

        if !num_set.contains(&(*n - 1)) {
            // at start of a possible sequence, check incremental numbers
            let mut t = *n + 1;
            while num_set.contains(&t) {
                cur_lcs += 1;
                t += 1;
            }
        }

        if cur_lcs > lcs {
            lcs = cur_lcs;
        }
    }
    lcs
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::longest_consecutive;

    #[test]
    fn test1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }

    #[test]
    fn test2() {
        let nums = vec![0,3,7,2,5,8,4,6,0,1];
        assert_eq!(longest_consecutive(nums), 9);
    }
}