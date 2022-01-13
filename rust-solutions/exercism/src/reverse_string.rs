pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::reverse;

    #[test]
    fn rev_string() {
        let rev_str = reverse("abc");
        assert_eq!(rev_str, "cba");
    }

    #[test]
    fn string_to_digit() {
        let digit: u32 = "9".parse::<u32>().expect("a valid digit string");
        assert_eq!(digit, 9);
    }
}
