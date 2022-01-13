/// # Problem 25 - First Fibonacci number with 1000 digits
/// The Fibonacci sequence is defined by the recurrence relation:
///     `Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1`
/// Hence the first 12 terms will be:
///     F1 = 1
///     F2 = 1
///     F3 = 2
///     F4 = 3
///     F5 = 5
///     F6 = 8
///     F7 = 13
///     F8 = 21
///     F9 = 34
///     F10 = 55
///     F11 = 89
///     F12 = 144
///
/// The 12th term, F12, is the first term to contain three digits.
/// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

use num::BigUint;

#[derive(Debug)]
struct Fib {
    c: BigUint,     // current fib number
    n: BigUint,     // next fib number
}

impl Fib {
    fn new() -> Self {
        Self {
            c: BigUint::from(0_usize),
            n: BigUint::from(1_usize),
        }
    }
}

impl Iterator for Fib {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let n = &self.c + &self.n;
        std::mem::swap(&mut self.c, &mut self.n);
        self.n = n;

        Some(self.n.clone())
    }
}

fn length(n: &BigUint) -> usize {
    n.to_str_radix(10).len()
}

fn main() {

    let res = Fib::new().into_iter()
        .enumerate()
        .find(|(_i, f)| {
            length(f) == 1000
        } );

    if let Some((i, _f)) = res {
        // + 2 because Euler is using 1 based index
        println!("index of first fibonacci number with 1000 digits is {}", i + 2);
    }
}