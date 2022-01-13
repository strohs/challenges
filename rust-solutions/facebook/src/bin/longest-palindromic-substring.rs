/// given a string **s**, find the longest palindromic substring in **s**. Assume that
/// the maximum length of **s** will be <= 1000
///
/// ## Example 1:
/// - input: "babad"
/// - output: "bab"
/// - Note: "aba" is also a valid answer
///
/// ## Example 2:
/// - input: "cbbd"
/// - output: "bb"

/// assuming s is an ASCII string of lowercase letters
fn is_palindrome(s: &str) -> bool {
    // assume that strings of length 1 are not palindromes
    if s.len() <= 1 {
        return false;
    }
    let mut is_palindrome = true;
    let mut fi = 0;
    let mut ti = s.len() - 1;

    while fi < ti && is_palindrome {
        if s[fi..=fi] != s[ti..=ti] {
            is_palindrome = false;
        }
        fi += 1;
        ti -= 1;
    }
    is_palindrome
}


fn longest_palindrome(s: &str) -> &str {
    let mut fi = 0;
    let mut curr_longest = "";
    while fi < s.len() {
        let fc = s[fi..].chars().next().unwrap();
        let mut opt_idx = s.rfind(fc);
        while let Some(li) = opt_idx {
            if li > fi {
                if is_palindrome(&s[fi..=li]) && s[fi..=li].len() > curr_longest.len() {
                    curr_longest = &s[fi..=li];
                }
                opt_idx = s[fi..li].rfind(fc);
            } else {
                opt_idx = None;
            }
        }
        fi += 1;
    }
    curr_longest
}

fn main() {
    println!("abcdaefggfxyz = {}", longest_palindrome("abcdaefggfxyz"));
    println!("fggfabcde: {}", longest_palindrome("fggfabcde"));
    println!("fgggggggggggf: {}", longest_palindrome("fgggggggggggf"));
    println!("abcdefghijklnnoop: {}", longest_palindrome("abcdefghijklnnoop"));

//    println!("abvba: {}", is_palindrome("abvba"));
//    println!("z: {}", is_palindrome("z"));
//    println!("avbz: {}", longest_palindrome("avbz"));


}