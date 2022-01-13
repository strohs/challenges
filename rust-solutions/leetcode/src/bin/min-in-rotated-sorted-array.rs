// find the minimum value in a rotated sorted array of ints
// assume no duplicate values
// ex 1:   [4,5,6,7,0,1,2] -> 0

fn find_min(nums: &[i32]) -> i32 {
    let mid_index = |si: usize, ei: usize| ((ei - si) / 2) + si;

    // current start index
    let mut si: usize = 0;
    // current end index
    let mut ei: usize = nums.len() - 1;

    loop {
        // if the start and end of the current slice is sorted, or the current slice length == 1,
        // return the first element of the slice, it is the lowest
        if nums[si] < nums[ei] || si == ei {
            return nums[si];
        }
        // OTHERWISE
        // find the middle index of the current slice and determine which half of the slice has
        // the lowest elem. The lowest element will be in the slice where the elem at start-idx is
        // > than the elem at end-index
        let mi = mid_index(si, ei);
        if nums[mi] > nums[ei] {
            si = mi + 1;
        } else {
            ei = mi;
        }
    }
}

fn main() {
    let nums1 = [1, 0];
    println!("{}", find_min(&nums1));

    let nums2 = [2, 0, 1];
    println!("{}", find_min(&nums2));

    let nums3 = [1, 2, 0];
    println!("{}", find_min(&nums3));

    let nums4 = [4, 5, 6, 7, 8, 1, 2];
    println!("{}", find_min(&nums4));

    let nums5 = [4, 5, 6, 7, 8, 9, 2];
    println!("{}", find_min(&nums5));
}

#[cfg(test)]
mod test {
    use crate::find_min;

    #[test]
    fn two_element_array() {
        let nums = [1, 0];

        let lowest = find_min(&nums);
        assert_eq!(lowest, 0);
    }

    #[test]
    fn three_element_array() {
        let nums = [1, 2, 0];

        let lowest = find_min(&nums);
        assert_eq!(lowest, 0);
    }

    #[test]
    fn three_element_array_with_lowest_in_middle() {
        let nums = [2, 0, 1];

        let lowest = find_min(&nums);
        assert_eq!(lowest, 0);
    }

    #[test]
    fn eight_element_array() {
        let nums = [9, 0, 1, 2, 3, 4, 5, 6, 7];

        let lowest = find_min(&nums);
        assert_eq!(lowest, 0);
    }
}
