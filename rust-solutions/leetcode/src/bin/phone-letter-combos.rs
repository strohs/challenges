/// # Leetcode 17 Letter Combinations of a phone number
/// Given a string containing digits from `2-9` inclusive, return all possible letter combinations
/// that the number could represent.
/// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that
/// 1 does not map to any letters.
/// ```
/// 2 -> a,b
/// 3 -> d,e,f
/// 4 -> g,h,i
/// 5 -> j,k,l
/// 6 -> m,n,o
/// 7 -> p,q,r,s
/// 8 -> t,u,v
/// 9 -> w,x,y,z
/// ```
/// ## Example
/// input: "23"
/// output: `["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]`
///
/// ## Constraints
/// `0 <= digits.length <= 4`
/// `digits[i] is a digit in range ['2', '9']`
///
///
/// FYI permutation - you care about the order of the elements
///     n! / (n - k)!
/// combinations - you don't care about the order of the elements
///    combination formula, you have n objects and want to choose k    = n! / k!(n - k!)

pub fn letter_combinations(digits: String) -> Vec<String> {
    const LETTERS: [&str; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    fn get_letters(digit: &str) -> &[&str] {
        let ls = match digit {
            "2" => &LETTERS[0..3],
            "3" => &LETTERS[3..6],
            "4" => &LETTERS[6..9],
            "5" => &LETTERS[9..12],
            "6" => &LETTERS[12..15],
            "7" => &LETTERS[15..19],
            "8" => &LETTERS[19..22],
            "9" => &LETTERS[22..26],
            _ => panic!("invalid digit {}", digit),
        };
        ls
    }

    // combine all strings in l1 with all string in l2, storing the combined string in l1
    fn combine<T: AsRef<str>, U: AsRef<str>>(l1: &[T], l2: &[U]) -> Vec<String> {
        let mut combos = vec![];
        for l1s in l1 {
            for l2s in l2 {
                let mut combined_str = String::from(l1s.as_ref());
                combined_str.push_str(l2s.as_ref());
                combos.push(combined_str);
            }
        }
        combos
    }

    if digits.is_empty() {
        return vec![];
    }

    let mut combos = vec![String::from("")];
    for digit in digits.split("").filter(|s| !s.is_empty()) {
        let letters = get_letters(digit);
        combos = combine(&combos, letters);
    }

    combos
}

fn main() {
    let combos = letter_combinations(String::from("234"));
    dbg!(combos);
}
