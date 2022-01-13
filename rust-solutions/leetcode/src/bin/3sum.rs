use std::collections::{HashMap, HashSet};

/// Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0?
/// Find all unique triplets in the array which gives the sum of zero.
///
/// ## Note:
/// The solution set must not contain duplicate triplets.
///
/// ## Example
/// ```
/// Given array nums = [-1, 0, 1, 2, -1, -4],
///
/// A solution set is:
/// [
///   [-1, 0, 1],
///   [-1, -1, 2]
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
        let target = sum - v[idx];
        let res = two_sum(&v[idx + 1..], target);
        for mut res_vec in res {
            res_vec.push(v[idx]);
            res_vec.sort_unstable();
            triples.insert(res_vec);
        }
    }
    triples.into_iter().collect()
}

fn main() {
    let input1 = vec![-1, 0, 1, 2, -1, -4];
    let res_set = three_sum(&input1, 0);
    println!("input1 result set\n{:?}", res_set);

    let input2 = vec![1, 2, 1, 2, 1, 2, 1];
    let res_set2 = three_sum(&input2, 6);
    println!("input2 result set\n{:?}", res_set2);
}
