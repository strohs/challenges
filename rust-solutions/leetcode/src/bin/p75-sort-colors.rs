/// # 75. Sort Colors
/// Given an array `nums` with `n` objects colored red, white, or blue, sort them in-place so that
/// objects of the same color are adjacent, with the colors in the order red, white, and blue.
///
/// We will use the integers `0`, `1`, and `2` to represent the color red, white, and blue, respectively.
///
/// You must solve this problem without using the library's sort function.

struct Solution;

impl Solution {

    pub fn sort_colors(nums: &mut Vec<i32>) {
        // index to current head of nums
        let mut h = 0;
        // index to current tail of nums
        let mut t = nums.len() - 1;
        // current index in nums
        let mut i = 0;

        while h < t && i <= t {
            if nums[i] == 0 {
                nums.swap(h, i);
                if nums[h] == 0 {
                    h += 1;
                    i += 1;
                }
            }
            else if nums[i] == 2 {
                nums.swap(t, i);
                t -= 1;
            } else {
                i += 1;
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
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0,0,1,1,2,2]);
    }

    #[test]
    fn example3() {
        let mut nums = vec![0, 1, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0,1,2]);
    }

    #[test]
    fn example4() {
        let mut nums = vec![2, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0,1,2]);
    }

    #[test]
    fn example5() {
        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0,1,2]);
    }
}