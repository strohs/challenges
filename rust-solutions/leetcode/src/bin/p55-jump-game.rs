/// # Leetcode Problem 55 - Jump Game
/// You are given an integer array `nums`. You are initially positioned at the array's
/// first index, and each element in the array represents your maximum jump length at that position.
///
/// Return `true` if you can reach the last index, or `false` otherwise.
///
/// In this implementation, start at the end of the array and repeatedly find all indices that
/// can jump to the last position of the array
pub fn can_jump(nums: Vec<i32>) -> bool {
    // returns the indices of nums that can jump to (or past) the last element of `nums`
    // If no such indices where found, an empty vector is returned
    fn can_jump_to_last(nums: &[i32]) -> Vec<usize> {
        let mut idxs: Vec<usize> = vec![];
        if nums.len() == 1 && nums[0] >= 1 {
            idxs.push(0);
        } else {
            for i in 0..(nums.len() - 1) {
                if nums[i] as usize + i >= nums.len() - 1 {
                    idxs.push(i);
                }
            }
        }
        idxs
    }

    if nums.len() == 1 {
        return true;
    }
    // indices of nums that can 'jump' to the current last index
    let mut jump_indices = can_jump_to_last(&nums);
    while !jump_indices.is_empty() {
        // we can reach the first index of nums
        if *jump_indices.first().unwrap() == 0_usize {
            return true;
        }
        let li = jump_indices.pop().unwrap();
        jump_indices = can_jump_to_last(&nums[..=li]);
    }

    false
}

// /// recursive implementation
// pub fn can_jump(nums: Vec<i32>) -> bool {
//
//     fn can_jump_recursive(nums: &[i32]) -> bool {
//         match nums.len() {
//             // slice is empty
//             0 => false,
//             // slice only has one element, which should be the first
//             1 => true,
//             _ => {
//                 // find all indices within nums that can jump to the last element of nums
//                 // don't include the last element itself within the slice range
//                 let jump_indices = nums[..nums.len() - 1]
//                     .iter()
//                     .enumerate()
//                     .filter(|&(idx, amt)| idx + *amt as usize >= nums.len() - 1)
//                     .map(|(idx, _amt)| idx)
//                     .collect::<Vec<usize>>();
//
//                 for idx in jump_indices {
//                     if can_jump_recursive(&nums[..=idx]) {
//                         return true
//                     }
//                 }
//                 false
//             }
//         }
//     }
//
//     can_jump_recursive(&nums)
// }

fn main() {}

#[cfg(test)]
mod tests {
    use crate::can_jump;

    #[test]
    fn example1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(true, can_jump(nums));
    }

    #[test]
    fn example2() {
        let nums = vec![3, 2, 1, 0, 4];
        assert_eq!(false, can_jump(nums));
    }

    #[test]
    fn example3() {
        let nums = vec![1];
        assert_eq!(true, can_jump(nums));
    }

    #[test]
    fn example4() {
        let nums = vec![0];
        assert_eq!(true, can_jump(nums));
    }

    #[test]
    fn example5() {
        let nums = vec![0, 2, 3];
        assert_eq!(false, can_jump(nums));
    }

    #[test]
    fn example6() {
        let nums = vec![1, 2, 3];
        assert_eq!(true, can_jump(nums));
    }

    #[test]
    fn example7() {
        let nums = vec![1, 0, 1, 0];
        assert_eq!(false, can_jump(nums));
    }
}
