use num::BigUint;

/// n! means n × (n − 1) × ... × 3 × 2 × 1
/// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
/// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
/// Find the sum of the digits in the number 100!

fn fact(n: u32) -> BigUint {
    let mut m = BigUint::from(1_u32);
    for i in 1..=n {
        m *= i;
    }
    m
}

fn main() {
    let fact_100 = fact(100);
    let sum: u32 = fact_100
        .to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).expect("expecting a digit character"))
        .sum();
    println!("sum of 100! digit's is {}", sum);
}