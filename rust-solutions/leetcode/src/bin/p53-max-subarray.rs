use std::cmp;
use std::cmp::max;

/// Given an integer array `nums`, find the contiguous subarray (containing at least one number)
/// which has the largest sum and return its sum.
///
/// A subarray is a contiguous part of an array.
///
/// ## Example 1
/// ```
/// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
/// Output: 6
/// Explanation: [4,-1,2,1] has the largest sum = 6.
/// ```


pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = i32::MIN;
    let mut cur_sum = 0;

    for n in nums {
        cur_sum = cmp::max(n, cur_sum + n);
        max_sum = cmp::max(max_sum, cur_sum);
    }

    max_sum
}


fn main() {
    let v = vec![-2,1,-3,4,-1,2,1,-5,4];
    let mut maxsum = max_sub_array(v);
    println!("{}", maxsum);

    let v1 = vec![1];
    maxsum = max_sub_array(v1);
    println!("{}", maxsum);

    let v1 = vec![-2, -1, -3 , -1];
    maxsum = max_sub_array(v1);
    println!("{}", maxsum);

    let v1 = vec![5, -4, 3, -2, 98];
    maxsum = max_sub_array(v1);
    println!("{}", maxsum);
}