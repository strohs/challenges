/// Leetcode 33 - search rotated sorted array
///
/// There is an integer array `nums` sorted in ascending order (with distinct values).
/// Prior to being passed to your function, `nums` is rotated at an unknown pivot index
/// `k (0 <= k < nums.length)` such that the resulting array is
/// `[nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed)`.
/// For example, `[0,1,2,4,5,6,7]` might be rotated at pivot index 3 and become `[4,5,6,7,0,1,2]`.
///
/// Given the array `nums` after the rotation and an integer target, return the index of `target`
/// if it is in `nums`, or `-1` if it is not in `nums`.

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    fn is_sorted(slice: &[i32], start: usize, end: usize) -> bool {
        slice[start] <= slice[end]
    }

    fn is_within(slice: &[i32], start: usize, end: usize, target: i32) -> bool {
        target >= slice[start] && target <= slice[end]
    }

    // these initially point to the first and last elements of nums
    let mut start_idx = 0;
    let mut end_idx = nums.len() - 1;

    // we'll repeatedly reduce the slice that start/end index point to, by half,
    // as we figure out where target resides
    while !nums[start_idx..=end_idx].is_empty() {
        // if the "slice" we are looking at is sorted, then the target must be within that slice,
        // we can simply perform a binary search to find it
        if is_sorted(&nums, start_idx, end_idx) {
            return match nums[start_idx..=end_idx].binary_search(&target) {
                Ok(idx) => (start_idx + idx) as i32,
                _ => -1,
            };
        } else {
            // the slice we are looking at is not sorted, so compute the middle index of the slice
            // and determine if target is within the sorted slice, or within the un-sorted slice

            let mi = (end_idx + start_idx) / 2;

            // check if left slice is sorted and if target is within that sorted slice
            if is_sorted(&nums, start_idx, mi) {
                if is_within(&nums, start_idx, mi, target) {
                    end_idx = mi;
                } else {
                    start_idx = mi + 1;
                }
            } else {
                // else right side must be sorted, check if target is within right side slice
                if is_within(&nums, mi + 1, end_idx, target) {
                    start_idx = mi + 1;
                } else {
                    end_idx = mi;
                }
            }
        }
    }
    -1
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn example0() {
        let nums = vec![1];
        assert_eq!(search(nums, 1), 0);
    }

    #[test]
    fn example1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(search(nums, 0), 4);
    }

    #[test]
    fn example2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(search(nums, 3), -1);
    }

    #[test]
    fn example3() {
        let nums = vec![1];
        assert_eq!(search(nums, 0), -1);
    }

    #[test]
    fn example4() {
        let nums = vec![3, 4, 5, 6, 1, 2];
        assert_eq!(search(nums, 3), 0);
    }

    #[test]
    fn example5() {
        let nums = vec![3, 4, 5, 6, 1, 2];
        assert_eq!(search(nums, 2), 5);
    }

    #[test]
    fn example6() {
        let nums = vec![3, 4, 5, 6, 1, 2];
        assert_eq!(search(nums, 6), 3);
    }
}
