/// You have a string of lowercase English alphabetic letters. You can perform two types of operations on the string:
///
/// 1. Append a lowercase English alphabetic letter to the end of the string.
/// 2, Delete the last character in the string. Performing this operation on an empty string results in an empty string.
///
/// Given an integer, `k`, and two strings, `s` and `t`, determine whether or not you can
/// convert `s` to `t` by performing exactly `k` of the above operations on `s`. If it's possible,
/// print Yes. Otherwise, print No.

fn matching_count(s: &str, t: &str) -> usize {
    return s.chars().zip(t.chars())
        .take_while(|(sc, tc)| *sc == *tc)
        .count();
}

fn append_and_delete(s: &str, t: &str, k: usize) -> &'static str {

    if k >= s.len() + t.len() {
        "Yes"
    } else {
        // see if any characters match
        let count = matching_count(&s, &t);

        if (k - (s.len() - count + t.len() - count)) % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use crate::append_and_delete;

    #[test]
    fn test() {
        assert_eq!(append_and_delete("hackerhappy", "hackerrank", 9), "Yes");
    }
}