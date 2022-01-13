/// Given an array of n elements, where each element is at most k away from its
/// target position, devise an algorithm that sorts in O(n log k) time. For example,
/// let us consider k is 2, an element at index 7 in the sorted array, can be at
/// indexes 5, 6, 7, 8, 9 in the given array
/// # Example
/// ```
/// Input : arr[] = {6, 5, 3, 2, 8, 10, 9}
///             k = 3
/// Output : arr[] = {2, 3, 5, 6, 8, 9, 10}
/// ```

use std::collections::BinaryHeap;
use std::cmp::Reverse;


fn sort_nearly_sorted(arr: &mut [i32], k: usize) {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k + 1);

    for i in 0..arr.len() {
        heap.push(Reverse(arr[i])); // O(1)
        if i >= k + 1 {
            // O(log k)
            let min = heap.pop().expect("min-heap will always have at least one element").0;
            arr[i - (k + 1)] = min;
        }
    }
    let mut i = arr.len();
    // pop remaining items from the heap into the end of the array
    // O(k * log(k))
    while let Some(Reverse(min)) = heap.pop() {
        arr[i - (k + 1)] = min;
        i += 1;
    }

}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::sort_nearly_sorted;

    #[test]
    fn example1() {
        let mut arr = [6, 5, 3, 2, 8, 8, 10, 9];
        let k = 3_usize;
        sort_nearly_sorted(&mut arr, k);
        dbg!(&arr);
        assert_eq!(arr, [2,3,5,6,8,8,9,10]);
    }

    #[test]
    fn example2() {
        let mut arr = [10, 9, 8, 7, 4, 70, 60, 50];
        let k = 4_usize;
        sort_nearly_sorted(&mut arr, k);
        dbg!(&arr);
        assert_eq!(arr, [4, 7, 8, 9, 10, 50, 60, 70]);
    }
}