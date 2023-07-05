/// # Problem 151 - reverse words in a string of words
/// https://leetcode.com/problems/reverse-words-in-a-string/

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut reversed_words: Vec<&str> = s.split_whitespace()
            .filter(|&t| !t.is_empty())
            .rev()
            .collect();
        reversed_words[..].join(" ")
    }
}

fn main() {
    let s = String::from("  hello  world  ");
    let mut r: Vec<&str> = s.split_whitespace()
        .filter(|&t| !t.is_empty())
        .rev()
        .collect();
    let joined = r.into_iter().fold(String::new(), |mut s, c| {
        s.push_str(c);
        s
    });
    dbg!(&joined);
}