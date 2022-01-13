use std::collections::HashMap;

// find the first character in a string that appears only one time.
// if no character appears one time, return '_'

/// Counts the number of times a character occurs in a string
///
/// # Returns
/// a HashMap of with a character as a key, and the number of times it occurs as the value
fn frequencies (s: &str) -> HashMap<char, u32> {
    let mut freqs: HashMap<char, u32> = HashMap::new();
    s.chars().for_each(|c| {
        let count = freqs.entry(c).or_insert(0);
        *count += 1;
    });
    freqs
}

fn fnrc(s: &str) -> char {
    // build freq map
    let fmap = frequencies(s);
    match s.chars().find(|c| fmap.get(c) == Some(&1)) {
        Some(key) => key,
        None => '-'
    }
}

fn main() {
    let str1 = "aaabcccdeeef";
    let str2 = "abcabcabc";
    let str3 = "aabbc";

    println!("{} = {}", str1, fnrc(str1));
    println!("{} = {}", str2, fnrc(str2));
    println!("{} = {}", str3, fnrc(str3));
}