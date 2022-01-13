/// # 20 Valid Parentheses
/// Given a string s containing just the characters `'(', ')', '{', '}', '[' and ']'`, determine
/// if the input string is valid.
///
/// An input string is valid if:
///
///     Open brackets must be closed by the same type of brackets.
///     Open brackets must be closed in the correct order.
///
/// ## Example 1
/// `Input: s = "()"`
/// `Output: true`
///
/// ## Example 2
/// `Input: s = "()[]{}"`
/// `Output: true`
///
/// ## Example 3
/// `Input: s = "(]"`
/// `Output: false`
///
/// ## Constraints
/// - 1 <= s.length <= 10^4
/// - s consists of parentheses characters only: `()[]{}`

// these are the 'closing' parenthesis characters
const CLOSE_PARENS: [char; 3] = [')', ']', '}'];

// returns the opening parentheses character that 'matches' the closing parentheses character
// could also put this into a HashMap
fn get_matching_opening_paren(closing_paren: &char) -> char {
    match closing_paren {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => panic!("invalid closing parentheses char {}", closing_paren),
    }
}

fn is_valid(s: &str) -> bool {
    let mut v: Vec<char> = vec![]; // vec acting as a stack of characters

    for ch in s.chars() {
        if !CLOSE_PARENS.contains(&ch) {
            // ch is a starting paren character, so push it on stack
            v.push(ch);
        } else {
            // ch is an ending paren char, check if last char on stack is a matching opening paren
            if let Some(prev) = v.pop() {
                if get_matching_opening_paren(&ch) != prev {
                    return false;
                }
            }
        }
    }

    true
}

fn main() {
    dbg!(is_valid("(]"));
}

#[cfg(test)]
mod tests {
    use crate::is_valid;

    #[test]
    fn example1() {
        assert!(is_valid("()"));
    }

    #[test]
    fn example2() {
        assert!(is_valid("()[]{}"));
    }

    #[test]
    fn example3() {
        assert_eq!(is_valid("(]"), false);
    }

    #[test]
    fn example4() {
        assert_eq!(is_valid("([{]})"), false);
    }

    #[test]
    fn example5() {
        assert!(is_valid("([{}])"));
    }
}
