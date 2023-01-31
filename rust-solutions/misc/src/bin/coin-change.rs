use std::time::Instant;

/// Coin Change, brute force recursive algorithm
/// count the number of ways that the `target` amount can be formed from the
/// coin amounts in `coins`


fn brute_force(target: i64, coins: &[i64]) -> i64 {
    brute_force_helper(target, coins, 0)
}

fn brute_force_helper(target: i64, coins: &[i64], i: usize) -> i64 {
    match target {
        0 => return 1,
        t if t < 0 => return 0,
        _ => (),
    }

    let mut ways_count = 0_i64;

    for j in i..coins.len() {
        ways_count += brute_force_helper(target - coins[j], coins, j)
    }

    return ways_count;
}


fn main() {
    println!("{}", brute_force(5, &[1, 2, 5]));

    println!("{}", brute_force(1000, &[1, 2, 4, 6, 8]));

}