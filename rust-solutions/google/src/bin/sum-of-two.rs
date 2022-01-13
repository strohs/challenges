use std::collections::BTreeSet;

/// given two arrays of integers, ns and ms, find one int from each array that sums
/// to the target value, v
/// Assume the arrays are of equal size, and contain random integers
/// if two such integers are found return a Vec of the two integers, else return None
fn sum_of_two(ns: &Vec<i32>, rs: &Vec<i32>, v: i32) -> Option<Vec<i32>> {

    // 1. collect one of the arrays into a tree set
    let ss = rs.iter().collect::<BTreeSet<&i32>>();

    // 2. iterate through the ns array, looking for the first element in ss such that
    // ns[i] - ms[i] = v
    for n in ns.iter() {
        let target = v - *n;
        if let Some(e) = ss.get(&target) {
            return Some(vec![*n, **e]);
        }
    }
    None
}

fn main() {
    let n1 = vec![5,8,2,6,9,4];
    let m1 = vec![9,2,5,7,3,8];
    println!("{:?}", sum_of_two(&n1, &m1, 17));

    println!("{:?}", sum_of_two(&n1, &m1, 2));

    println!("{:?}", sum_of_two(&n1, &m1, 12));

    println!("{:?}", sum_of_two(&n1, &m1, 11));
}