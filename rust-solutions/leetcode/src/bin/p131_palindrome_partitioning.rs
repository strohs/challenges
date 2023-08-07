/// # Problem 131 - Palindrome Partitioning
/// https://leetcode.com/problems/palindrome-partitioning/
///

pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut partitions = vec![];

    // Each bit in cutpoints determines whether to cut between characters i and i+1
    'outer: for cutpoints in 0..(1 << s.len() - 1) {
        let mut result = vec![];
        let mut lastcut = 0;
        for i in 0..s.len() {
            if (1 << i) & cutpoints != 0 {
                if !is_palindrome(&s[lastcut..=i]) {
                    continue 'outer;
                }
                result.push(s[lastcut..=i].to_string());
                lastcut = i + 1;
            }
        }
        if !is_palindrome(&s[lastcut..]) {
            continue 'outer;
        }
        result.push(s[lastcut..].to_string());
        partitions.push(result);
    }
    partitions
}

fn is_palindrome(s: &str) -> bool {
    s.bytes().eq(s.bytes().rev())
}

// fn recurse(curr: &str, rest: &str, valids: Vec<Vec<String>>, memo: &mut HashMap<&str, Vec<Vec<String>>>) -> Option<Vec<Vec<String>>> {
//
//     if memo.contains_key(curr) {
//         return memo.get(curr).map(|vv| vv.clone());
//     }
//
//     if curr.is_empty() {
//         return Some(vec![]);
//     }
//
//     for i in 0..curr.len() {
//         let cur = &curr[0..=i];
//         let rest = &curr[i+1..];
//         if is_palindrome(cur) {
//             //valid.push(cur.to_string());
//             if let Some(valid_palindromes) = recurse(rest, valids.clone(), memo) {
//                 if valid_palindromes.is_empty() {
//                     //valids.push(valid.clone());
//                 }
//             }
//         }
//     }
//
//     if !valids.is_empty() {
//         memo.entry(curr)
//             .and_modify(|vv| vv.extend(valids.clone()))
//             .or_insert(valids.clone());
//     }
//
//     None
// }




/// exponential runtime of Omega(2^n).. i.e. at least 2^n where n = s.len()
// fn gen_all_partitions(s: &str) -> Vec<Vec<&str>> {
//     let mut partitions = vec![];
//
//     'outer: for cutpoint in 0..(1 << s.len() - 1) {
//         let mut result = vec![];
//         let mut lastcut = 0;
//         for i in 0..s.len() {
//             if (1 << i) & cutpoint != 0 {
//                 if !is_palindrome(&s[lastcut..=i]) {
//                     continue 'outer;
//                 }
//                 result.push(&s[lastcut..=i]);
//                 lastcut = i + 1;
//             }
//         }
//         if !is_palindrome(&s[lastcut..]) {
//             continue 'outer;
//         }
//         result.push(&s[lastcut..]);
//         partitions.push(result);
//     }
//     partitions
// }


fn main() {}

#[cfg(test)]
mod tests {
    use crate::{partition, is_palindrome};

    #[test]
    fn test_gen_perms() {
        let perms = partition("aabbaa".to_string());
        for p in perms {
            println!("{:?}", &p);
        }
    }


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