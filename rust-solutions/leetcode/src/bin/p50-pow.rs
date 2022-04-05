/// Implement `pow(x, n)`, which calculates `x` raised to the power `n` (i.e., xn).
/// x is a f64  and n is an i32
///
/// # Constraints
/// - -100.0 < x < 100.0
/// - -231 <= n <= 231-1
/// - -10^4 <= xn <= 10^4
///
/// optimizations found on the interwebs:
/// if `n` is even then `x^n = (x^2)^n/2`
///

fn main() {}


pub fn my_pow(x: f64, n: i32) -> f64 {

    let pos_n = if n >= 0 { true } else { false };
    let mut x = x;
    let mut n = if pos_n {
        n
    } else {
        if n == i32::MIN {
            i32::MAX - 1
        } else {
            -n
        }
    };

    let mut r = 1.0;

    while n > 0 {
        // if n is odd
        if n % 2 != 0 {
            r *= x;
        }
        // reduce exponent in half
        n /= 2;
        // square the base
        x *= x;
    }

    match pos_n {
        true => r,
        false => 1.0 / r,
    }
}

#[cfg(test)]
mod tests {
    use crate::my_pow;

    #[test]
    fn example1() {
        assert_eq!(my_pow(2.0, 10), 1024.0)
    }

    #[test]
    fn example2() {
        assert_eq!(format!("{:.4}",my_pow(2.1, 3)), "9.2610")
    }

    #[test]
    fn example3() {
        assert_eq!(format!("{:.4}",my_pow(2.0, -2)), "0.2500")
    }


    #[test]
    fn leet_test1() {
        dbg!(2.0f64.powi(-23));
        assert_eq!(format!("{:.4}",my_pow(2.0, -2147483648)), "0.0000")
    }

    #[test]
    fn leet_test2() {
        assert_eq!(format!("{:.4}",my_pow(-1.0, -2147483648)), "1.0000")
    }

    #[test]
    fn leet_test3() {
        assert_eq!(format!("{:.4}",my_pow(1.0, -2147483648)), "1.0000")
    }
}