use std::collections::HashMap;

/// # Problem 131 - Palindrome Partitioning
/// https://leetcode.com/problems/palindrome-partitioning/
///

pub fn partition(s: String) -> Vec<Vec<String>> {
    //let mut palis: Vec<Vec<String>> = vec![];
    let mut valids: Vec<Vec<String>> = vec![];
    let mut pmap: HashMap<&str, Vec<Vec<String>>> = HashMap::new();
    recurse(&s, valids, &mut pmap);
    vec![]
}

fn is_palindrome(s: &str) -> bool {
    s.bytes().eq(s.bytes().rev())
}

fn recurse<'a>(s: &'a str, mut valids: Vec<Vec<String>>, pmap: &mut HashMap<&'a str, Vec<Vec<String>>>) -> Option<Vec<Vec<String>>> {

    if pmap.contains_key(s) {
        return pmap.get(s).map(|vv| vv.clone());
    }

    if s.is_empty() {
        return Some(vec![]);
    }

    for i in 0..s.len() {
        let cur = &s[0..=i];
        let rest = &s[i+1..];
        if is_palindrome(cur) {
            //valid.push(cur.to_string());
            if let Some(valid_palindromes) = recurse(rest, valids.clone(), pmap) {
                if valid_palindromes.is_empty() {
                    //valids.push(valid.clone());
                }
            }
        }
    }

    if !valids.is_empty() {
        pmap.entry(s)
            .and_modify(|vv| vv.extend(valids.clone()))
            .or_insert(valids.clone());
    }

    None
}


fn main() {}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(is_palindrome("aaa"));
        assert!(!is_palindrome("aab"));
    }

    #[test]
    fn test_vec_extends() {
        let mut v = vec!["a", "b", "c"];
        let nv: Vec<&str> = vec![];
        v.extend(nv);
        dbg!(&v);
    }
}