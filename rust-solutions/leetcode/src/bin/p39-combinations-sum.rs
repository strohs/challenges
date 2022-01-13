/// # 39. Combination Sum
/// Given an array of distinct integers candidates and a target integer target, return a list of
/// all unique combinations of candidates where the chosen numbers sum to target. You may return
/// the combinations in any order.
///
/// The same number may be chosen from candidates an unlimited number of times. Two combinations
/// are unique if the frequency of at least one of the chosen numbers is different.
/// It is guaranteed that the number of unique combinations that sum up to target is less than
/// 150 combinations for the given input.
///
/// ## Example 1
/// `Input: candidates = [2,3,6,7], target = 7`
/// `Output: [[2,2,3],[7]]`
///
/// ## Note:
/// - candidates will always have at least one element
/// - all elements of candidates will be distinct

/// study this recursive solution
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn solve(candidates: &[i32], prev: Vec<i32>, target: i32, res: &mut Vec<Vec<i32>>) {
        println!("prev {:?}", &prev);
        println!("target {:?} ", &target);
        if target == 0 {
            res.push(prev);
            return;
        }
        let last = prev.last().unwrap_or(&i32::MIN);
        for &num in candidates {
            println!("  if num:{:?} >= last:{:?}", &num, &last);
            if num >= *last && target - num >= 0 {
                let mut nums = prev.clone();
                nums.push(num);
                println!("    nums is now {:?}", &nums);
                solve(candidates, nums, target - num, res);
            }
        }
    }
    let mut res = vec![];
    solve(&candidates, vec![], target, &mut res);
    res
}

// pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
//
//     /// returns the (idx, num) entry before `idx` in the sorted candidates slice
//     fn prev_entry(candidates: &[(usize, i32)], idx: usize) -> Option<(usize, i32)> {
//         if idx > 0 {
//             candidates.get(idx - 1).copied()
//         } else {
//             None
//         }
//     }
//
//     use std::collections::HashSet;
//     // combos stores the final candidates that sum to target
//     let mut combos: HashSet<Vec<i32>> = HashSet::new();
//
//     // filter out all numbers > target, store the index and number in a new vec
//     let mut candidates: Vec<i32> = candidates
//         .into_iter()
//         .filter(|n| *n <= target)
//         .collect();
//     candidates.sort();
//     let candidates: Vec<(usize, i32)> = candidates.into_iter().enumerate().collect();
//
//     if candidates.is_empty() { return combos.into_iter().collect(); }
//
//     // always at least one element
//     let mut cur_entry = candidates.last().unwrap();
//     let mut cur_sum = 0;
//     let mut stack: Vec<(usize, i32)> = vec![];
//
//     loop {
//         let (_idx, num) = *cur_entry;
//
//         if cur_sum == target {
//             let mut nums: Vec<i32> = stack.iter().map(|(_i, n)| *n).collect();
//             nums.sort_unstable();
//             combos.insert(nums);
//         }
//
//         if cur_sum < target {
//             cur_sum += num;
//             stack.push(cur_entry.clone());
//         } else {
//             // cur_entry >= target
//             if let Some((last_idx, last)) = stack.pop() {
//                 cur_sum -= last;
//                 if let Some((prev_idx, prev_num)) = prev_entry(&candidates, last_idx) {
//                     cur_sum += prev_num;
//                     stack.push((prev_idx, prev_num));
//                 } else {
//                     // no previous entry, begin loop that decrements any previous entries
//                     while let Some((idx, _e)) = stack.last() {
//                         if let Some(pentry) = prev_entry(&candidates, *idx) {
//                             // a prev entry exists, pop the old entry, and push the new one
//                             let last_entry = stack.pop().unwrap();
//                             cur_sum -= last_entry.1;
//                             stack.push(pentry);
//                             cur_sum += pentry.1;
//                             break;
//                         } else {
//                             // no previous entry, pop the stack
//                             let last_entry = stack.pop().unwrap();
//                             cur_sum -= last_entry.1;
//                         }
//                     }
//                 }
//             } else {
//                 // stack is empty???
//                 break;
//             }
//         }
//
//         if stack.is_empty() {
//             break
//         }
//     }
//
//     combos.into_iter().collect()
// }

fn main() {}

#[cfg(test)]
mod tests {
    use crate::combination_sum;

    #[test]
    fn ex0() {
        let candidates = vec![2];
        let target = 2;
        let combos = combination_sum(candidates, target);
        assert!(combos.contains(&vec![2]));
    }

    #[test]
    fn ex1_2() {
        let candidates = vec![3, 1, 2];
        let target = 6;
        let combos = combination_sum(candidates, target);
        assert_eq!(combos.len(), 7);
        assert!(combos.contains(&vec![3, 3]));
        assert!(combos.contains(&vec![1, 2, 3]));
        assert!(combos.contains(&vec![1, 1, 1, 3]));
        assert!(combos.contains(&vec![2, 2, 2]));
        assert!(combos.contains(&vec![1, 1, 2, 2]));
        assert!(combos.contains(&vec![1, 1, 1, 1, 2]));
        assert!(combos.contains(&vec![1, 1, 1, 1, 1, 1]));
    }

    #[test]
    fn ex1() {
        let candidates = vec![2, 3, 4];
        let target = 9;
        let combos = combination_sum(candidates, target);
        dbg!(&combos);
        assert_eq!(combos.len(), 3);
        assert!(combos.contains(&vec![2, 2, 2, 3]));
        assert!(combos.contains(&vec![2, 3, 4]));
        assert!(combos.contains(&vec![3, 3, 3]));
    }

    #[test]
    fn ex2() {
        let candidates = vec![1, 2, 3, 4];
        let target = 6;
        let combos = combination_sum(candidates, target);
        assert!(combos.contains(&vec![2, 4]));
        assert!(combos.contains(&vec![3, 3]));
        assert!(combos.contains(&vec![1, 2, 3]));
    }

    #[test]
    fn ex3() {
        let candidates = vec![2];
        let target = 1;
        let combos = combination_sum(candidates, target);
        assert!(combos.is_empty());
    }

    #[test]
    fn ex4() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let combos = combination_sum(candidates, target);
        assert_eq!(combos.len(), 2);
        assert!(combos.contains(&vec![2, 2, 3]));
        assert!(combos.contains(&vec![7]));
    }

    #[test]
    fn ex4_5() {
        let candidates = vec![2, 3];
        let target = 6;
        let combos = combination_sum(candidates, target);
        assert_eq!(combos.len(), 2);
        assert!(combos.contains(&vec![3, 3]));
        assert!(combos.contains(&vec![2, 2, 2]));
    }

    #[test]
    fn ex6() {
        let candidates = vec![2, 7, 6, 3, 5, 1];
        let target = 9;
        let combos = combination_sum(candidates, target);
        println!("len: {}", &combos.len());
        for comb in combos.iter() {
            println!("{:?}", comb);
        }
        //assert!(combos.contains(&vec![3, 2, 2]));
        //assert!(combos.contains(&vec![7]));
    }
}
