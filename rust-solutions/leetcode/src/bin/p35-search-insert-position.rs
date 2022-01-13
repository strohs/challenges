/// # 35. Search Insert Position
/// Given a sorted array of distinct integers and a target value, return the index if the target is
/// found. If not, return the index where it would be if it were inserted in order.
/// You must write an algorithm with O(log n) runtime complexity.

// NOTE: rust has a binary search method that does exactly what we want. However; we'll implement
// our own search

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    // empty array return 0
    if nums.is_empty() {
        0
    } else {
        let mut slice = &nums[..];
        // si is the slice index, si points to where the slice begins within nums
        let mut si: usize = 0;

        while !slice.is_empty() {
            // base case slice = 1, which could be the element, or where it should be inserted
            if slice.len() == 1 {
                return match nums[si] {
                    n if target <= n => si as i32,
                    _ => (si + 1) as i32,
                };
            }

            // compute mid point of current slice, and determine which side of midpoint target is on
            let mid = slice.len() / 2;

            if target < slice[mid] {
                slice = &slice[..mid];
            } else {
                slice = &slice[mid..];
                si += mid;
            }
        }
        0
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::search_insert;

    #[test]
    fn example1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(search_insert(nums, target), 2);
    }

    #[test]
    fn example2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        assert_eq!(search_insert(nums, target), 1);
    }

    #[test]
    fn example3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        assert_eq!(search_insert(nums, target), 4);
    }

    #[test]
    fn example3_1() {
        let nums = vec![1, 3, 5, 7];
        let target = 6;
        assert_eq!(search_insert(nums, target), 3);
    }

    #[test]
    fn example4() {
        let nums = vec![1, 3, 5, 6];
        let target = 0;
        assert_eq!(search_insert(nums, target), 0);
    }

    #[test]
    fn example5() {
        let nums = vec![1];
        let target = 0;
        assert_eq!(search_insert(nums, target), 0);
    }

    #[test]
    fn example6() {
        let nums = vec![1];
        let target = 2;
        assert_eq!(search_insert(nums, target), 1);
    }
}
