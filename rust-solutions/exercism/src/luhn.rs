
/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");

    if code.len() <= 1 {
        return false;
    }

    // string must contain only digits
    if code.parse::<usize>().is_err() {
        return false
    }
    let sum = code.chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, ch)| {
            if i % 2 != 0 {
                let mut dbl = ch.to_digit(10).unwrap() * 2;
                if dbl > 9 {
                    dbl = dbl - 9;
                }
                acc + dbl
            } else {
                acc + ch.to_digit(10).unwrap()
            }
        });
    sum % 10 == 0
}



#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn string_to_digit() {
        let digit: u32 = "9".parse::<u32>().expect("a valid digit string");
        assert_eq!(digit, 9);
    }

    #[test]
    fn char_to_digit() {
        let digit = '9'.to_digit(10).unwrap();
        assert_eq!(digit, 9);
    }

    #[test]
    fn test_str_with_spaces_is_valid() {
        assert!(is_valid("4539 3195 0343 6467"));
    }
}