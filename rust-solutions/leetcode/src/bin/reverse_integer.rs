/// # Problem 7 - Reverse Integer
/// Given a 32-bit signed integer, reverse digits of an integer.
///
/// Assume we are dealing with an environment which could only store integers within the
/// 32-bit signed integer range: [−231,  231 − 1]. For the purpose of this problem, assume that
/// your function returns 0 when the reversed integer overflows.

// fn reverse_as_str(n: i32) -> i32 {
//     let mut is_neg = false;
//     let mut n = n;
//     if n < 0 {
//         is_neg = true;
//         n = n.abs();
//     }
//     let mut s: String = n.to_string().chars().rev().collect();
//     if is_neg {
//         s = format!("-{}", s);
//     }
//
//     s.parse::<i32>().unwrap_or(0)
// }

fn reverse_as_digits(n: i32) -> i32 {
    fn multiplier(n: i32) -> i32 {
        if n > 999_999_999 {
            1_000_000_000
        } else if n > 99_999_999 {
            100_000_000
        } else if n > 9_999_999 {
            10_000_000
        } else if n > 999_999 {
            1_000_000
        } else if n > 99_999 {
            100_000
        } else if n > 9_999 {
            10_000
        } else if n > 999 {
            1000
        } else if n > 99 {
            100
        } else if n > 9 {
            10
        } else {
            1
        }
    }

    let is_neg = n < 0;
    let n = n.abs();
    let mut m = n;
    let mut mult = multiplier(m);
    let mut res = 0;
    while m > 0 {
        res += mult * (m % 10);
        m /= 10;
        mult /= 10;
    }
    if is_neg {
        -res
    } else {
        res
    }
}

fn main() {
    let res = reverse_as_digits(-987);
    dbg!(res);
}

#[cfg(test)]
mod tests {
    use crate::reverse_as_str;

    #[test]
    fn reverse_positive_i32() {
        assert_eq!(reverse_as_str(132), 231);
    }

    #[test]
    fn reverse_negative_i32() {
        assert_eq!(reverse_as_str(-132), -231);
    }

    #[test]
    fn reverse_trailing_zerp() {
        assert_eq!(reverse_as_str(120), 21);
    }
}
