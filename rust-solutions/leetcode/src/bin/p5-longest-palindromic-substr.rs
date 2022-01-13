/// # Problem 5 - longest palindromic substring
/// Given a string `s`, return the longest palindromic substring in `s`
///
/// ## Example 1
/// ```
/// Input: s = "babad"
/// Output: "bab"
/// Note: "aba" is also a valid answer.
/// ```

/// returns true if `s` is a palindrome
fn is_palindrome(s: &[u8]) -> bool {
    if s.is_empty() || s.len() == 1 {
        return true;
    }

    s.iter()
        .zip(s.iter().rev())
        .all(|(left, right)| *left == *right)
}

pub fn longest_palindrome(s: String) -> String {
    let mut longest = String::new();

    if s.len() <= 1 {
        return s;
    }

    for win_size in (1..=s.len()).rev() {
        match s
            .as_bytes()
            .windows(win_size)
            .find(|&slice| is_palindrome(slice))
        {
            Some(palin_slice) if palin_slice.len() > longest.len() => {
                longest = String::from_utf8(palin_slice.to_vec()).unwrap()
            }
            _ => (),
        }
    }

    longest
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::longest_palindrome;

    #[test]
    fn ex1() {
        let s = String::from("babad");
        let longest = longest_palindrome(s);
        assert_eq!(longest, "bab");
    }

    #[test]
    fn ex2() {
        let s = String::from("cbbd");
        let longest = longest_palindrome(s);
        assert_eq!(longest, "bb");
    }

    #[test]
    fn ex3() {
        let s = String::from("a");
        let longest = longest_palindrome(s);
        assert_eq!(longest, "a");
    }

    #[test]
    fn ex4() {
        let s = String::from("ac");
        let longest = longest_palindrome(s);
        assert_eq!(longest, "a");
    }

    #[test]
    fn ex5() {
        let s = String::from("baabaaaaab");
        let longest = longest_palindrome(s);
        assert_eq!(longest, "baaaaab");
    }
}
