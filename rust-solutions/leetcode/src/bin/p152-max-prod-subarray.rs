use std::cmp::max;

/// # Leetcode P151 - Maximum Product Subarray
/// https://leetcode.com/problems/maximum-product-subarray/
/// this solution uses kadane's algorithm with the major change being that if a zero is
/// encountered, then we set the max_ending_here variable to 1 instead of 0, since we are trying
/// to find the max subarray product (not the sum)

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {

        let max_lr = {
            // max so far
            let mut msf = nums[0];
            // max ending here
            let mut meh = 1;

            for n in nums.iter() {
                meh = meh * *n;
                if msf < meh {
                    msf = meh;
                }
                if meh == 0 {
                    meh = 1;
                }
            }
            msf
        };

        let max_rl = {
            // max so far
            let mut msf = nums[0];
            // max ending here
            let mut meh = 1;

            for n in nums.iter().rev() {
                meh = meh * *n;
                if msf < meh {
                    msf = meh;
                }
                if meh == 0 {
                    meh = 1;
                }
            }
            msf
        };
        max(max_lr, max_rl)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_product(vec![2,3,-2,4]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::max_product(vec![3, -1, 4]), 4);
    }
}