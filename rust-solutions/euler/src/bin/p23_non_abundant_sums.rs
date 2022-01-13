use std::collections::{HashSet};

/// # Problem 23 Non-Abundant Sums
/// A perfect number is a number for which the sum of its proper divisors is exactly equal to
/// the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28,
/// which means that 28 is a perfect number.
///
/// A number n is called Deficient if the sum of its proper divisors is less than n and it is
/// called abundant if this sum exceeds n.
///
/// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can
/// be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown
/// that all integers greater than 28123 can be written as the sum of two abundant numbers. However,
/// this upper limit cannot be reduced any further by analysis even though it is known that the
/// greatest number that cannot be expressed as the sum of two abundant numbers is less than
/// this limit.
/// Find the sum of all the positive integers which cannot be written as the sum of two abundant
/// numbers.

const MAX_ABUNDANT: u64 = 28123;

#[derive(Debug,PartialEq)]
enum Divisors {
    Deficient,
    Perfect,
    Abundant
}

/// return a vector containing the proper divisors of n
fn proper_divisors(n: u64) -> Vec<u64> {
    let mut factors = vec![1_u64];
    let sqrt_n = (n as f64).sqrt() as usize;
    for i in 2..=sqrt_n as u64 {
        if n % i == 0 {
            factors.push(i);
            if n / i != i {
                factors.push(n / i);
            }
        }
    }
    factors
}

/// return if the divisors of a number `n` are **Deficient**, **Perfect**, or **Abundant**
fn divisor_type(n: u64) -> Divisors {
    let divs = proper_divisors(n);
    let divs_sum: u64 = divs.iter().sum();
    match divs_sum {
        sum if sum < n => Divisors::Deficient,
        sum if sum == n => Divisors::Perfect,
        _ => Divisors::Abundant
    }
}

/// compute all the abundant integers between 1 and MAX_ABUNDANT
fn abundant_numbers() -> HashSet<u64> {
    (1..=MAX_ABUNDANT)
        .filter(|&n| divisor_type(n) == Divisors::Abundant)
        .collect()
}

fn main() {
    let abundants: HashSet<u64> = abundant_numbers();
    let mut total_sum = 0;

    for n in 1..=MAX_ABUNDANT {
        let mut found_sum = false;
        for m in abundants.iter() {
            let dif = n as i64 - *m as i64;
            if dif > 0 && abundants.contains(&(dif as u64))  {
                // found a number that can be written as sum of two abundant numbers
                found_sum = true;
                break;
            }
        }
        if !found_sum {
            total_sum += n;
            //println!("{} has no abundant sum", n);
        }
    }

    println!("total sum = {}", total_sum);  // 4179871
}
