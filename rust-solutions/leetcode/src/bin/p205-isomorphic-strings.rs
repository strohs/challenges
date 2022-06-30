use std::collections::HashMap;
use std::collections::HashSet;

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut char_map = HashMap::new();
    let mut used_values = HashSet::new();

    // iter over bytes because strings are guaranteed to be ASCII
    for (s_char, used_char) in s.bytes().zip(t.bytes()) {

        match (char_map.get(&s_char), used_values.get(&used_char)) {
            (None, None) => {
                char_map.insert(s_char, used_char);
                used_values.insert(used_char);
            },
            (None, Some(_)) => {
                // no two characters may map to the same char
                return false;
            },
            (Some(s_value), _) => {
                // char_map must continue to map to the same character
                if s_value != &used_char {
                    return false;
                }
            },
        }
    }

    true
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::is_isomorphic;

    #[test]
    fn example1() {
        assert_eq!(is_isomorphic("foo".to_string(), "add".to_string()), true);
    }

    #[test]
    fn example2() {
        assert_eq!(is_isomorphic("foo".to_string(), "bar".to_string()), false);
    }

    #[test]
    fn example3() {
        assert_eq!(is_isomorphic("paper".to_string(), "title".to_string()), true);
    }

    #[test]
    fn example4() {
        assert_eq!(is_isomorphic("a".to_string(), "b".to_string()), true);
    }

    #[test]
    fn example5() {
        assert_eq!(is_isomorphic("a".to_string(), "a".to_string()), true);
    }

    #[test]
    fn example6() {
        assert_eq!(is_isomorphic("badc".to_string(), "baba".to_string()), false);
    }

    #[test]
    fn example7() {
        assert_eq!(is_isomorphic("egcd".to_string(), "adfd".to_string()), false);
    }
}