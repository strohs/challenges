use num_bigint::BigInt;


/// returns the factorial of an integer, using BigIntegers
fn factorial(n: i32) -> BigInt {
    return (1..=n)
        .into_iter()
        .fold(BigInt::from(1), |fact, i| fact * BigInt::from(i));
}

fn main() {
    println!("{}", factorial(222));
}