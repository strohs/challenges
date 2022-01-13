/// # 29. Divide Two Integers
///
/// Given two integers `dividend` and `divisor`, divide two integers without using multiplication,
/// division and mod operator.
/// Return the `quotient` after dividing `dividend` by `divisor`.
/// The integer division should truncate toward zero, which means losing its fractional part.
/// For example, `truncate(8.345) = 8` and `truncate(-2.7335) = -2`.
///
/// ## Note
/// - Both dividend and divisor will be 32-bit signed integers.
/// - The divisor will never be 0.
/// - Assume we are dealing with an environment which could only store integers within the 32-bit
///   signed integer range: `[−2^31,  2^31 − 1]`. For the purpose of this problem, assume that your
///   function returns `2^31 − 1` when the division result overflows.
///
/// ## Example 1
/// Input: dividend = 10, divisor = 3
/// Output: 3
/// Explanation: 10/3 = truncate(3.33333..) = 3.

/// Returns the `quotient` after dividing `dividend` by `divisor`
pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == 0 {
        return 0;
    }

    // compute the final sign
    let sign = if (divisor < 0 && dividend < 0) || (divisor > 0 && dividend > 0) {
        1
    } else {
        -1
    };

    let mut dividend = dividend.abs();
    let divisor = divisor.abs();

    // compute the quotient by continually subtracting the divisor from the dividend, stop
    // when the dividend is >= divisor
    let mut quotient = 0;
    while dividend >= divisor {
        quotient += 1;
        dividend -= divisor;
    }

    quotient * sign
}

fn main() {
    let mut n = 7;

    for _ in 0..10 {
        n <<= 3;
        println!("{}", &n);
    }
}

#[cfg(test)]
mod tests {
    use crate::divide;

    #[test]
    fn dividend_gt_divisor() {
        assert_eq!(divide(10, 3), 3);
    }

    #[test]
    fn neg_dividend_gt_divisor() {
        assert_eq!(divide(-10, 3), -3);
    }

    #[test]
    fn dividend_gt_neg_divisor() {
        assert_eq!(divide(22, -2), -11);
    }

    #[test]
    fn both_dividend_divisor_neg() {
        assert_eq!(divide(-10, -3), 3);
    }

    #[test]
    fn dividend_lt_divisor() {
        assert_eq!(divide(4, 7), 0);
    }

    #[test]
    fn dividend_eq_0() {
        assert_eq!(divide(0, 7), 0);
    }

    #[test]
    fn ex1() {
        assert_eq!(divide(100, 1), 100);
    }
}
