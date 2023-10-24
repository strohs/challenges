/// # 162 - Find Peak Element
/// https://leetcode.com/problems/find-peak-element/
///
/// Note some of the constraints for this problem:
/// - `nums[i] != nums[i + 1]` for all valid `i`. Therefore, there will be at least 1 peak element
/// - we can return any index if `nums` has multiple peaks
///
/// This solution will somewhat resemble a binary search. I will recursively find the
/// midpoint of the input nums vector and compare it with the element to its right and left.
/// If the midpoint is greater than both right and left then we can return the midpoint.
/// If the midpoint is not greater than both, then we select two new midpoint indices
/// from the left half and right half of the midpoint and repeat the process.
/// A stack is used to keep track of the indices we need to examine.

struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut stack = vec![(0_usize, nums.len())]; // holds (left, right) indices to check
        loop {
            let (li,ri) = stack.pop().expect("will always have at least one peak element");
            if li == ri { continue } // we've gone past either the leftmost or rightmost bound of nums

            let mi = ((ri - li) / 2) + li;
            let m = nums[mi];
            let l = if mi == 0 { i32::MIN } else { nums[mi - 1] };
            let r = if mi == nums.len() - 1 { i32::MIN } else { nums[mi + 1] };

            if m >= l && m >= r {
                return mi as i32;
            }
            stack.push((li, mi));
            stack.push((mi + 1, ri));
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_one_elem() {
        let nums = vec![5];
        assert_eq!(Solution::find_peak_element(nums), 0);
    }

    #[test]
    fn test_two_elem() {
        let nums = vec![2, 1];
        assert_eq!(Solution::find_peak_element(nums), 0);
    }

    #[test]
    fn test_two_elem_ascending() {
        let nums = vec![1, 2];
        assert_eq!(Solution::find_peak_element(nums), 1);
    }

    #[test]
    fn test_three_elem() {
        let nums = vec![3, 2, 1];
        assert_eq!(Solution::find_peak_element(nums), 0);
    }

    #[test]
    fn test_min_value() {
        let nums = vec![-2147483648];
        assert_eq!(Solution::find_peak_element(nums), 0);
    }

    #[test]
    fn test_max_value() {
        let nums = vec![2147483647];
        assert_eq!(Solution::find_peak_element(nums), 0);
    }

    #[test]
    fn test_example1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::find_peak_element(nums), 2);
    }

    #[test]
    fn test_example2() {
        let nums = vec![1, 2, 1, 3, 5, 6, 4];
        assert_eq!(Solution::find_peak_element(nums), 5);
    }

    #[test]
    fn test_example3() {
        let nums = vec![6, 5, 4, 3, 2, 3, 2];
        assert_eq!(Solution::find_peak_element(nums), 5);
    }
}