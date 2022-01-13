use std::collections::HashMap;

/// write a program to determine if an array of integers contains exactly three occurrences of
/// an integer. return a vector containing the integers that occur exactly three times in the
/// input array

fn exactly3(arr: &[i32]) -> Vec<i32> {
    let mut hm:HashMap<i32,i32> = HashMap::new();
    for i in arr {
        let found_int = hm.entry(*i).or_insert(0);
        *found_int += 1;
    }
    hm.into_iter()
        .filter(|(_, v)| *v == 3)
        .map(|(k, _)| k)
        .collect::<Vec<i32>>()
}

fn main() {
    let v1 = [1,2,3,4,5,6,7,8,9,8,7,6,5,4,3,2,1,1,2,3,2,3];
    assert_eq!(exactly3(&v1), [1]);
}