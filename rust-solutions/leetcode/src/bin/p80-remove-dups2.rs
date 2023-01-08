/// # 80 - Remove Duplicates from Sorted Array II
/// Given an integer array `nums` sorted in non-decreasing order, remove some duplicates in-place
/// such that each unique element appears at most twice. The relative order of the elements should be kept the same.
/// Since it is impossible to change the length of the array in some languages, you must instead have the result be
/// placed in the first part of the array `nums`. More formally, if there are `k` elements after removing the duplicates,
/// then the first `k` elements of `nums` should hold the final result. It does not matter what you leave beyond the
/// first elements.
///
/// Return `k` after placing the final result in the first `k` slots of `nums`.
///
/// **Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.**


pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

    let mut i = 0;
    for n in 0..nums.len() {
        if i < 2 || nums[n] > nums[i - 2] {
            nums[i] = nums[n];
            i += 1;
        }
    }
    i as i32
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::remove_duplicates;

    #[test]
    fn test1() {
        let mut nums = vec![1,1,1,2,2,3];
        assert_eq!(remove_duplicates(&mut nums), 5);
    }

    #[test]
    fn test2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(remove_duplicates(&mut nums), 7);
    }
}