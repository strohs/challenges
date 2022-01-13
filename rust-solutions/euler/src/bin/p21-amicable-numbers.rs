use std::collections::{HashMap, HashSet};

/// # Problem 21 Amicable Numbers
/// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide
/// evenly into n).
/// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b
/// are called amicable numbers.
///
/// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
/// therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
///
/// Evaluate the sum of all the amicable numbers under 10000.

fn divisors(n: i32, memo_divs: &mut HashMap<i32, Vec<i32>>) -> &Vec<i32> {
    let divs = memo_divs.entry(n).or_insert_with(|| {
        (1..n)
            .filter(|divisor| n % divisor == 0)
            .collect()
    });
    divs
}

/// is b an amicable pair of n
fn amicable_pair(n: i32, b: i32, memo_divs: &mut HashMap<i32, Vec<i32>>) -> bool {
    n != b && n == divisors(b, memo_divs).into_iter().sum::<i32>()
}

fn main() {
    let mut mem_divs: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut pairs: HashSet<i32> = HashSet::new();
    let n = 10000;
    for n in 1..n {
        let divs = divisors(n, &mut mem_divs);
        let divs_sum = divs.iter().sum();
        if amicable_pair(n, divs_sum, &mut mem_divs) {
            println!("found pair {} : {}", n, divs_sum);
            pairs.insert(n);
            pairs.insert(divs_sum);
        }
    }
    println!("length of hashmap {}", mem_divs.len());
    let ap_sum = pairs.iter().sum::<i32>();
    println!("sum of all amicable pairs is {}", ap_sum);
}