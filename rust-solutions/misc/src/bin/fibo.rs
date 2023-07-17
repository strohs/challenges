/// find the first fibonacci number over 1_000_000

/// returns the first fibonacci number >= `target`
fn fibo(target: u128) -> u128 {
    let mut m: u128 = 0;
    let mut n: u128 = 1;

    while n < target {
        let next = m + n;
        m = n;
        n = next;
    }
    n
}

fn main() {
    let target: u128 = 1_000_000;
    println!("target {} = {}", &target, fibo(target));

    let s = "12ab3c";

    let mut i = 0_usize;
    let mut count_str = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            count_str.push(ch);
        } else if ch.is_ascii_alphabetic() {

        }
    }
}