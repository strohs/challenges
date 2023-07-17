/// # Excercism - Armstrong Numbers
/// An Armstrong number is a number that is the sum of its own digits
/// each raised to the power of the number of digits.

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let arm = digits
        .iter()
        .fold(0_usize, |sum, d| sum + d.pow(digits.len() as u32) as usize );

    arm == num as usize
}

#[cfg(test)]
mod tests {
    use crate::armstrong::is_armstrong_number;

    #[test]
    fn test1() {
        assert_eq!(true, is_armstrong_number(9))
    }

    #[test]
    fn test2() {
        assert_eq!(false, is_armstrong_number(10))
    }

    #[test]
    fn test3() {
        assert_eq!(true, is_armstrong_number(153))
    }

    #[test]
    fn test_properly_handles_overflow() {
        assert!(!is_armstrong_number(4_106_098_957));
    }
}