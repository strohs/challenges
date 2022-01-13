use std::collections::{HashMap, HashSet};

/// Given an array nums of n integers, are there elements a, b, c, d in nums such that a + b + c + d = target?
/// Find all unique quadruplets in the array which gives the sum of `target`.
///
/// ## Note:
/// The solution set must not contain duplicate quadruplets.
///
/// ## Example
/// ```
/// Given array nums = [-1, 0, 1, 0, -2, 2], and target = 0
///
/// A solution set is:
/// [
///   [-1, 0, 0, 1],
///   [-2, -1, 1, 2],
///   [-2, 0, 0, 2]
/// ]
/// ```

/// return all pairs of integers that sum up to **sum**.
/// v - the vector of integers
/// sum - the target sum
fn two_sum(v: &[i32], sum: i32) -> Vec<Vec<i32>> {
    let mut imap = HashMap::new();
    let mut res = Vec::new();
    for i in v {
        let target = sum - *i;
        if imap.contains_key(&target) {
            res.push(vec![*i, target]);
        }
        imap.entry(*i).or_insert(*i);
    }
    res
}

/// return all triplets of integers that add up to 'sum'. The returned triples must not
/// be repeated
fn three_sum(v: &[i32], sum: i32) -> Vec<Vec<i32>> {
    let mut triples = HashSet::new();
    for idx in 0..v.len() {
        let tsum = sum - v[idx];
        let res = two_sum(&v[idx + 1..], tsum);
        for mut res_vec in res {
            res_vec.push(v[idx]);
            res_vec.sort_unstable();
            triples.insert(res_vec);
        }
    }
    triples.into_iter().collect()
}

fn four_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut quads = HashSet::new();

    for idx in 0..nums.len() {
        let tsum = target - nums[idx];
        let res = three_sum(&nums[idx + 1..], tsum);
        for mut res_vec in res {
            res_vec.push(nums[idx]);
            res_vec.sort_unstable();
            quads.insert(res_vec);
        }
    }

    quads.into_iter().collect()
}

fn main() {
    let input1 = vec![-1, 0, -1, 0, -2, 2];
    let res_set = four_sum(&input1, 0);
    println!("input1 result set\n{:?}", res_set);

    let input2 = vec![1, 2, 1, 2, 1, 2, 1];
    let res_set2 = four_sum(&input2, 6);
    println!("input2 result set\n{:?}", res_set2);

    let res_set3 = four_sum(&input2, 4);
    println!("input3 result set\n{:?}", res_set3);
}
