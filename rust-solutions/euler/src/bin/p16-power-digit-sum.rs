extern crate num;

/// 215 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
/// What is the sum of the digits of the number 21000?

use num::bigint::BigUint;
use num::pow::pow;

/// this solution uses a BigUint implementation from the Rust num crate
fn power_digit_sum() -> u32 {
    // 2 ^ 1000
    pow(BigUint::from(2_u32), 1000)
        .to_radix_be(10)
        .into_iter()
        .map(|n| n as u32)
        .sum()
}

fn main() {
    let sum = power_digit_sum(); // 1366
    println!("power digit sum = {}", sum);

}