/// # 28. Implement strStr()
/// Return the index of the first occurrence of `needle` in `haystack`, or -1 if `needle` is not
/// part of `haystack`
///
/// if `needle` is the empty string, return 0.
/// if `needle` is not in haystack, return -1.
///

pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return 0;
    }

    // assuming haystack and needle contain ASCII chars only
    for i in 0..haystack.len() {
        if i + needle.len() <= haystack.len() && haystack[i..(i + needle.len())] == needle[..] {
            return i as i32;
        }
    }

    -1
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::str_str;

    #[test]
    fn has_ll() {
        assert_eq!(str_str(String::from("hello"), String::from("ll")), 2);
    }

    #[test]
    fn has_lo_at_end() {
        assert_eq!(str_str(String::from("hello"), String::from("lo")), 3);
    }

    #[test]
    fn has_equal_strings() {
        assert_eq!(str_str(String::from("hello"), String::from("hello")), 0);
    }

    #[test]
    fn needle_is_empty() {
        assert_eq!(str_str(String::from("hello"), String::from("")), 0);
    }

    #[test]
    fn needle_not_in_haystack() {
        assert_eq!(str_str(String::from("hello"), String::from("zip")), -1);
    }
}
