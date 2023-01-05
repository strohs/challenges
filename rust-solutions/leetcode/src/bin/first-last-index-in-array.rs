// find the first and last position of an element in a sorted array
// the algorithm should run in O(log n) time
// Ex.
// given array: [5, 7, 7, 8, 8, 10] target = 8
//   return: [3, 4]
//
// Ex 2.
// given array: [5, 7, 7, 8, 8, 10] target = 6
//   return : [-1, -1]

fn find_first_last_index(nums: &[i32], target: i32) -> Vec<i32> {
    fn mid_index(si: usize, ei: usize) -> usize {
        ((ei - si) / 2) + si
    }

    // find the first index in nums that does not equal the number in nums[idx]
    let start_idx = |idx: usize| {
        return if let Some(i) = nums[..=idx].iter().rposition(|n| *n != nums[idx]) {
            i + 1
        } else {
            0
        };
    };

    let end_idx = |idx: usize| {
        return if let Some(i) = nums[idx..].iter().position(|n| *n != nums[idx]) {
            // position is relative to the nums slice
            (idx + i) - 1
        } else {
            nums.len() - 1
        };
    };

    let mut si: usize = 0;
    let mut ei = nums.len() - 1;
    let mut cmi = mid_index(si, ei);

    while nums[cmi] != target && si < ei {
        if target < nums[cmi] {
            ei = cmi - 1;
        } else {
            si = cmi + 1;
        }
        cmi = mid_index(si, ei);
    }

    if nums[cmi] == target {
        let spos = start_idx(cmi) as i32;
        let epos = end_idx(cmi) as i32;
        vec![spos, epos]
    } else {
        vec![-1, -1]
    }
}

fn main() {
    println!("[3, 4] target 3 = {:?}", find_first_last_index(&[3, 4], 3));
    println!("[3, 4] target 6 = {:?}", find_first_last_index(&[3, 4], 6));
    println!(
        "[5,7,7,8,8,10] target 8 = {:?}",
        find_first_last_index(&[5, 7, 7, 8, 8, 10], 8)
    );
    println!(
        "[5,7,7,8,8,10] target 9 = {:?}",
        find_first_last_index(&[5, 7, 7, 8, 8, 10], 9)
    );
    println!(
        "[5,7,7,8,8,9,10] target 9 = {:?}",
        find_first_last_index(&[5, 7, 7, 8, 8, 9, 10], 9)
    );
    println!(
        "[5,7,7,8,8,9,9] target 9 = {:?}",
        find_first_last_index(&[5, 7, 7, 8, 8, 9, 9], 9)
    );
}
