/// # 31 Next Permutation
/// Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.
/// If such an arrangement is not possible, it must rearrange it as the lowest possible order (i.e., sorted in ascending order).
/// The replacement must be in place and use only constant extra memory.
///
/// ## Example 1
///
/// ```
/// Input: nums = [1,2,3]
/// Output: [1,3,2]
/// ```
///
/// ## Example 2
///
/// ```
/// Input: nums = [3,2,1]
/// Output: [1,2,3]
/// ```
///
/// ## Example 3
///
/// ```
/// Input: nums = [1]
/// Output: [1]
/// ```

pub fn next_permutation(nums: &mut Vec<i32>) {
    // returns the index of the first element in `nums` that is lexicographically < then the element
    // after it, going in reverse order. If no such element is found, then `None` is returned and
    // you can assume the entire slice is in descending order
    fn r_find_first_non_asc_index(nums: &[i32]) -> Option<usize> {
        nums.windows(2).enumerate().rev().find_map(
            |(i, chunk)| {
                if chunk[0] < chunk[1] {
                    Some(i)
                } else {
                    None
                }
            },
        )
    }

    // start at back of nums and find first non ascending index, if any
    if let Some(idx) = r_find_first_non_asc_index(nums) {
        // find first element in nums that is greater than nums[idx] going in reverse order
        if let Some(gt_idx) = nums.iter().rposition(|n| *n > nums[idx]) {
            nums.swap(idx, gt_idx);
            // sort the remaining numbers to the right of idx in ascending order
            nums[idx + 1..].sort_unstable();
        }
    } else {
        nums.sort_unstable();
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::next_permutation;

    #[test]
    fn example1() {
        let mut nums = vec![1, 2, 3];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn example2() {
        let mut nums = vec![3, 2, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn example3() {
        let mut nums = vec![1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn example4() {
        let mut nums = vec![1, 2];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1]);
    }

    #[test]
    fn example4_1() {
        let mut nums = vec![2, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2]);
    }

    #[test]
    fn example4_2() {
        let mut nums = vec![1, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 1]);
    }

    #[test]
    fn example5() {
        let mut nums = vec![1, 3, 2];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1, 3]);
    }

    #[test]
    fn example6() {
        let mut nums = vec![2, 3, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![3, 1, 2]);
    }

    #[test]
    fn example7() {
        let mut nums = vec![5, 4, 7, 5, 3, 2];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![5, 5, 2, 3, 4, 7]);
    }

    #[test]
    fn example8() {
        let mut nums = vec![4, 2, 0, 2, 3, 2, 0];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![4, 2, 0, 3, 0, 2, 2]);
    }

    #[test]
    fn example9() {
        let mut nums = vec![2, 2, 0, 4, 3, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 2, 1, 0, 3, 4]);
    }

    #[test]
    fn example10() {
        let mut nums = vec![0, 0, 4, 2, 1, 0];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![0, 1, 0, 0, 2, 4]);
    }
}
