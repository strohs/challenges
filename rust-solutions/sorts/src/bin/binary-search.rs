use std::cmp::Ordering;

/// basic binary search on a sorted array

fn index_of(arr: &[i32], n: i32) -> Option<usize> {
    let mut l = 0;
    let mut r: usize = arr.len() - 1;

    while l <= r {
        let m = (l + r) / 2;
        match arr[m].cmp(&n) {
            Ordering::Less => l = m + 1,
            Ordering::Greater => r = m - 1,
            _ => return Some(m)
        }
    }
    None
}

fn main() {
    let arr = [-5,-4, -3, 1, 3, 4, 5];
    println!("index of 1 is {:?}", index_of(&arr, 1));
}