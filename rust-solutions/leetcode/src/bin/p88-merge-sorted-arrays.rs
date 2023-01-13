/// # Problem 99 - Merge Sorted Arrays
/// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing
/// the number of elements in nums1 and nums2 respectively.
/// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
/// The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate
/// this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements
/// are set to 0 and should be ignored. nums2 has a length of n.

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

    if n == 0 {
        return
    }
    if m == 0 {
        for ix in 0..n as usize {
            nums1[ix] = nums2[ix];
        }
        return
    }

    // a indexes into nums1
    let mut a = m - 1;
    let mut b = n - 1;
    let mut i = m + n - 1;

    while i >= 0 && a >= 0 && b >= 0 {
        if nums1[a as usize] > nums2[b as usize] {
            nums1[i as usize] = nums1[a as usize];
            a -= 1;
            i -= 1;
        } else {
            nums1[i as usize] = nums2[b as usize];
            b -= 1;
            i -= 1;
        }
        if a < 0 || b < 0 {
            break;
        }
    }

    // there may still be some left over elements in either nums2 or nums1
    // a and b  will always be < i
    while a >= 0 {
        nums1[i as usize] = nums1[a as usize];
        a -= 1;
        i -= 1;
    }
    while b >= 0 {
        nums1[i as usize] = nums2[b as usize];
        b -= 1;
        i -= 1;
    }
}



fn main() {}

#[cfg(test)]
mod tests {
    use crate::{merge};

    #[test]
    fn test_both_empty() {
        let mut nums1 = vec![];
        let mut nums2 = vec![];
        let m = 0;
        let n = 0;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![]);
    }


    #[test]
    fn test_nums2_empty() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        let m = 1;
        let n = 0;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_nums1_empty_but_nums2_has_one_elem() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        let m = 0;
        let n = 1;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let m = 3;
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_2_arrays_equal() {
        let mut nums1 = vec![1, 1, 1, 0, 0, 0];
        let mut nums2 = vec![1, 1, 1];
        let m = 3;
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_3() {
        let mut nums1 = vec![7, 8, 9, 0, 0, 0];
        let mut nums2 = vec![1, 1, 1];
        let m = 3;
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 1, 1, 7, 8, 9]);
    }

    #[test]
    fn test_4() {
        let mut nums1 = vec![1, 1, 1, 0, 0, 0];
        let mut nums2 = vec![7, 8, 9];
        let m = 3;
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 1, 1, 7, 8, 9]);
    }
}