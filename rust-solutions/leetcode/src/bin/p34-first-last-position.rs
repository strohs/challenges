use std::cmp::Ordering;

/// # 34. Find First and Last Position of Element in Sorted Array
/// Given an array of integers `nums` sorted in ascending order, find the starting and ending
/// position of a given `target` value.
/// If target is not found in the array, return `[-1, -1]`.
/// You must write an algorithm with `O(log n)` runtime complexity.

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // returns rightmost index(inclusive) of some target element within nums, beginning at start inclusive,
    // it's assumed that nums[start] will initially contain the target element you are searching for.
    fn find_rightmost_index(nums: &[i32], start: usize) -> usize {
        let target = nums[start];
        let mut start = start;
        let mut end = nums.len();
        while start < end {
            if nums[end - 1] == target {
                return end - 1;
            }
            let mid = (start + end) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => start = mid,
                Ordering::Greater => end = mid,
                _ => (),
            }
        }
        start
    }

    // returns left-most index(inclusive) of some target element within nums, beginning at end inclusive,
    // it's assumed that nums[end] will initially contain the target element you are searching for.
    fn find_leftmost_index(nums: &[i32], end: usize) -> usize {
        let target = nums[end];
        let mut end = end;
        let mut start = 0;
        while start <= end {
            if nums[start] == target {
                return start;
            }
            let mid = (start + end) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => end = mid,
                Ordering::Less => start = mid + 1,
                _ => (),
            }
        }
        end
    }

    // indices into the current nums slice we are examining
    let mut start = 0;
    let mut end = nums.len();

    while start < end {
        // base case, array of one element
        match nums.len() {
            1 if nums[0] == target => return vec![0, 0],
            1 if nums[0] != target => return vec![-1, -1],
            _ => (), // continue
        }

        // check if either nums[first] or nums[last] is == target
        if nums[start] == target {
            let si = find_leftmost_index(&nums, start);
            let ei = find_rightmost_index(&nums, start);
            return vec![si as i32, ei as i32];
        }

        if nums[end - 1] == target {
            let si = find_leftmost_index(&nums, end - 1);
            let ei = find_rightmost_index(&nums, end - 1);
            return vec![si as i32, ei as i32];
        }

        // now check nums[mid] where mid is the middle index between start and end
        let mid = (start + end) / 2;
        if nums[mid] == target {
            let si = find_leftmost_index(&nums, mid);
            let ei = find_rightmost_index(&nums, mid);
            return vec![si as i32, ei as i32];
        }

        if target <= nums[mid] {
            // target is on left side
            end = mid;
        } else {
            // target is on right side
            start = mid + 1;
        }
    }

    vec![-1, -1]
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::search_range;

    #[test]
    fn example1() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        assert_eq!(search_range(nums, target), vec![3, 4])
    }

    #[test]
    fn example2() {
        let nums = vec![];
        let target = 0;
        assert_eq!(search_range(nums, target), vec![-1, -1])
    }

    #[test]
    fn example3() {
        let nums = vec![8, 8, 8, 8];
        let target = 8;
        assert_eq!(search_range(nums, target), vec![0, 3])
    }

    #[test]
    fn example4() {
        let nums = vec![8];
        let target = 8;
        assert_eq!(search_range(nums, target), vec![0, 0])
    }

    #[test]
    fn example4_1() {
        let nums = vec![8];
        let target = 6;
        assert_eq!(search_range(nums, target), vec![-1, -1]);
    }

    #[test]
    fn example4_2() {
        let nums = vec![0, 8];
        let target = 8;
        assert_eq!(search_range(nums, target), vec![1, 1]);
    }

    #[test]
    fn example4_3() {
        let nums = vec![7, 8];
        let target = 9;
        assert_eq!(search_range(nums, target), vec![-1, -1]);
    }

    #[test]
    fn example4_4() {
        let nums = vec![2, 2];
        let target = 1;
        assert_eq!(search_range(nums, target), vec![-1, -1]);
    }

    #[test]
    fn example4_5() {
        let nums = vec![2, 2];
        let target = 3;
        assert_eq!(search_range(nums, target), vec![-1, -1]);
    }

    #[test]
    fn example5() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let target = 8;
        assert_eq!(search_range(nums, target), vec![-1, -1])
    }

    #[test]
    fn example5_5() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        assert_eq!(search_range(nums, target), vec![-1, -1])
    }

    #[test]
    fn example6() {
        let nums = vec![5, 6, 7, 8, 9, 10, 11];
        let target = 8;
        assert_eq!(search_range(nums, target), vec![3, 3])
    }
}
