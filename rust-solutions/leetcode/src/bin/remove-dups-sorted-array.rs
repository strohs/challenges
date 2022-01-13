/// # 26. Remove duplicates from a Sorted Array
/// Given a sorted array nums, remove the duplicates in-place such that each element appears only
/// once and **returns the new length.**
/// Do not allocate extra space for another array, you must do this by modifying the input array
/// in-place with *O(1)* extra memory.
///
/// ## Example 1
/// ```
/// Given nums = [1,1,2],
/// Your function should return length = 2, with the first two elements of nums being 1 and 2
/// respectively. It doesn't matter what you leave beyond the returned length.
/// ```
///
/// ## Example 2
/// ```
/// Given nums = [0,0,1,1,1,2,2,3,3,4]
/// Your function should return length = 5, with the first five elements of nums being modified
/// to 0,1,2,3 and 4 respectively. It doesn't matter what values are set beyond the returned
/// length.
/// ```

/// returns the length of nums once all duplicates have been removed. Nums will be modified in-place
/// so that the non repeating elements appear at the beginning of the array, making sure to keep
/// their sorted order
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0_i32;
    }

    let mut count: usize = 1;

    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            nums[count] = nums[i];
            count += 1;
        }
    }
    count as i32
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::remove_duplicates;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 2];
        let length = remove_duplicates(&mut nums);
        assert_eq!(length, 2);
        assert_eq!(nums[0], 1);
        assert_eq!(nums[1], 2);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 4];
        let length = remove_duplicates(&mut nums);
        assert_eq!(length, 5);
        assert_eq!(nums[0], 0);
        assert_eq!(nums[1], 1);
        assert_eq!(nums[2], 2);
        assert_eq!(nums[3], 3);
        assert_eq!(nums[4], 4);
    }

    #[test]
    fn example_3() {
        let mut nums = vec![1, 1, 2, 2, 3, 3];
        let length = remove_duplicates(&mut nums);
        assert_eq!(length, 3);
        assert_eq!(nums[0], 1);
        assert_eq!(nums[1], 2);
        assert_eq!(nums[2], 3);
    }

    #[test]
    fn no_duplicates() {
        let mut nums = vec![6, 7, 8];
        let length = remove_duplicates(&mut nums);
        assert_eq!(length, 3);
        assert_eq!(nums[0], 6);
        assert_eq!(nums[1], 7);
        assert_eq!(nums[2], 8);
    }
}
