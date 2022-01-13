use std::collections::HashMap;

/// # Problem 49 - Group Anagrams
/// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically
/// using all the original letters exactly once.
/// ## Example 1
/// `Input: strs = ["eat","tea","tan","ate","nat","bat"]`
/// `Output: [["bat"],["nat","tan"],["ate","eat","tea"]]`

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::with_capacity(strs.len());

    for word in strs {
        let mut chars = word.chars().collect::<Vec<char>>();
        chars.sort_unstable();
        let sword: String = chars.into_iter().collect();

        let entry = groups.entry(sword).or_insert_with(Vec::new);
        entry.push(word);
    }
    groups.into_iter().map(|(_k, v)| v).collect()
}

fn main() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    //let strs = vec!["".to_string(), "".to_string()];
    let ags = group_anagrams(strs);
    dbg!(ags);
}
