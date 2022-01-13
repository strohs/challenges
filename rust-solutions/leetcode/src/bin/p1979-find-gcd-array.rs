/// # Problem 1979 - Greatest Common Denominator of an Array
/// Given an integer array `nums`, return the greatest common divisor of the smallest
/// number and largest number in nums.
///
/// The greatest common divisor of two numbers is the largest positive integer that
/// evenly divides both numbers.

fn gcd(a: i32, b: i32) -> i32 {
    // a will hold the larger of the two numbers
    let (a, b) = if a > b { (a, b) } else { (b, a) };

    match (a, b) {
        (a, 0) => a,
        (0, b) => b,
        (a, b) => gcd(b, a % b),
    }
}

/// nums will have at least two numbers
pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let smallest = nums[0];
    let largest = *nums.last().unwrap();
    gcd(largest, smallest)
}

fn main() {
    let nums = vec![7, 5, 6, 8, 3];
    println!("{}", find_gcd(nums));
}
