/// Problem 214. Shortest Palindrome
/// You are given a string s. You can convert s to a palindrome by adding characters in front of it.
/// Return the shortest palindrome you can find by performing this transformation.

pub fn shortest_palindrome(s: String) -> String {
    // returns true if s is a palindrome
    fn is_palindrome(s: &str) -> bool {
        let mp = s.len() / 2;
        s[..=mp].chars().eq(s[mp..].chars().rev())
    }

    fn check_equal_slices(cur_sl: &str, st: &str) -> bool {
        let total_len = cur_sl.len() + st.len();
        // mid-point of st with len() of cur_sl len() subtracted
        let mp = total_len / 2 - cur_sl.len();
        let lhs = cur_sl.chars().rev().chain(st[..mp].chars());
        let mut rhs = st[mp..].chars().rev();
        let mut halfs_equal = true;
        for lc in lhs {
            if let Some(rc) = rhs.next() {
                if lc != rc {
                    halfs_equal = false;
                    break;
                } else {
                    continue;
                }
            }
        }
        halfs_equal
    }

    match s.len() {
        0 | 1 => s,
        _ if is_palindrome(&s) => s,
        _ => {
            for i in (0..s.len()).rev() {
                let rh_slice = &s[i..];
                if check_equal_slices(rh_slice, &s) {
                    return rh_slice.chars().rev().collect::<String>() + &s;
                }
            }
            // if we reach here, the shortest palindrome is the input string
            let mut palin_str: String = s[1..].chars().rev().collect();
            palin_str.push_str(&s);
            palin_str
        }
    }
}

fn main() {

    // let s1 = String::from("bc");
    // let s2 = String::from("abc");
}

#[cfg(test)]
mod tests {
    use crate::shortest_palindrome;

    // #[test]
    // fn a_is_palindrome() {
    //     let s1 = String::from("a");
    //     assert!(is_palindrome(&s1));
    // }
    //
    // #[test]
    // fn ab_not_palindrome() {
    //     let s1 = String::from("ab");
    //     assert_eq!(is_palindrome(&s1), false);
    // }

    // #[test]
    // fn aba_is_palindrome() {
    //     let s1 = String::from("aba");
    //     assert!(is_palindrome(&s1));
    // }

    #[test]
    fn input_str_is_empty() {
        let s = String::new();
        assert_eq!(shortest_palindrome(s), String::new());
    }

    #[test]
    fn shortest_palindrome_is_a() {
        let s = String::from("a");
        assert_eq!(shortest_palindrome(s), "a");
    }

    #[test]
    fn shortest_palindrome_is_dcbabcd() {
        let s = String::from("abcd");
        assert_eq!(shortest_palindrome(s), "dcbabcd");
    }

    #[test]
    fn shortest_palindrome_of_ab_is_bab() {
        let s = String::from("ab");
        assert_eq!(shortest_palindrome(s), "bab");
    }

    #[test]
    fn shortest_palindrome_of_aba_is_aba() {
        let s = String::from("aba");
        assert_eq!(shortest_palindrome(s), "aba");
    }

    #[test]
    fn shortest_palindrome_of_abc_is_cbabc() {
        let s = String::from("abc");
        assert_eq!(shortest_palindrome(s), "cbabc");
    }

    #[test]
    fn shortest_palindrome_of_aacecaaa_is_aaacecaaa() {
        let s = String::from("aacecaaa");
        assert_eq!(shortest_palindrome(s), "aaacecaaa");
    }

    #[test]
    fn shortest_palindrome_of_aaaa_is() {
        let s = String::from("aaa");
        assert_eq!(shortest_palindrome(s), "aaa");
    }

    #[test]
    fn shortest_palindrome_of_abbacd_is_dcabbacd() {
        let s = String::from("abbacd");
        assert_eq!(shortest_palindrome(s), "dcabbacd");
    }
}
