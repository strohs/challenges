/// # 162 - Find Peak Element
/// https://leetcode.com/problems/find-peak-element/
///
/// Note some of the constraints for this problem:
/// - `nums[i] != nums[i + 1]` for all valid `i`.
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
        let mut stack = vec![nums.len() / 2];
        loop {
            let midi = stack.pop().expect("will always have at least one element");
            let elem = nums[midi];
            let left_elem = if midi == 0 { i32::MIN } else { nums[midi - 1] };
            let right_elem = if midi == nums.len() - 1 { i32::MIN } else { nums[midi + 1] };
            if elem > left_elem && elem > right_elem {
                return midi as i32;
            } else {
                stack.push(nums[..midi].len() / 2); // push left side mid index
                stack.push(nums[midi..].len() / 2 + midi); // push right side mid index
            }
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
    fn test_example1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::find_peak_element(nums), 2);
    }

    #[test]
    fn test_example2() {
        let nums = vec![1, 2, 1, 3, 5, 6, 4];
        assert_eq!(Solution::find_peak_element(nums), 5);
    }
}