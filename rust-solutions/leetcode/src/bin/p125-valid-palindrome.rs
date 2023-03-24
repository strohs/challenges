/// # Problem 125 Valid Palindrome
/// https://leetcode.com/problems/valid-palindrome/

pub fn is_palindrome(s: String) -> bool {
    let clean: Vec<char> = s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let mid = clean.len() / 2;
    let (left, right) = if clean.len() % 2 == 0 {
        clean.split_at(mid)
    } else {
        (&clean[0..=mid], &clean[mid..])
    };

    left.iter().eq(right.iter().rev())
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;

    #[test]
    fn test1() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert!(is_palindrome(s));
    }

    #[test]
    fn test2() {
        let s = String::from("");
        assert!(is_palindrome(s));
    }

    #[test]
    fn test3() {
        let s = String::from(" ");
        assert!(is_palindrome(s));
    }

    #[test]
    fn test4() {
        let s = String::from("race a car");
        assert_eq!(is_palindrome(s), false);
    }

    #[test]
    fn test5() {
        let s = String::from("0P");
        assert_eq!(is_palindrome(s), false);
    }
}