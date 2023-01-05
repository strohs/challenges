/// # 27 Remove Element
/// Given an array nums and a value val, remove all instances of that value in-place and return the
/// new length.
/// Do not allocate extra space for another array, you must do this by modifying the input array
/// in-place with O(1) extra memory.
/// The order of elements can be changed. It doesn't matter what you leave beyond the new length.
///
/// ## Example 1
/// Given nums = `[3,2,2,3], val = 3`,
/// our function should return length = 2, with the first two elements of nums being 2.
/// It doesn't matter what you leave beyond the returned length.
///
/// ## Example 2
/// Given nums = `[0,1,2,2,3,0,4,2], val = 2`,
/// Your function should return length = 5, with the first five elements of nums containing
/// 0, 1, 3, 0, and 4.
/// Note that the order of those five elements can be arbitrary.

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut length = 0;

    for i in 0..nums.len() {
        if nums[i] == val {
            // find index of first element != val and swap it with nums[i]
            if let Some(fi) = index_of_first_ne(&nums[i..], val) {
                nums.swap(i, i + fi);
            } else {
                // all remaining elements in nums are == val, so return
                return length;
            }
        }
        length += 1;
    }

    length
}

/// returns `Some(index)` of the first element, in `nums`, that is != to val
/// If no such element is found, None is returned
fn index_of_first_ne(nums: &[i32], val: i32) -> Option<usize> {
    nums.iter()
        .enumerate()
        .find(|&(_i, e)| *e != val)
        .map(|(i, _e)| i)
}

fn main() {
    let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let l = remove_element(&mut v, 2);
    dbg!(l, v);
}
