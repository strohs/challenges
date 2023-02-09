/// # Problem 139 - Word Break
/// [Description](https://leetcode.com/problems/word-break/description/)
///


// uses dynamic programming to solve
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len()+1];
    dp[s.len()] = true;

    for i in (0..s.len()).rev() {
        for word in &word_dict {
            if (i + word.len()) <= s.len() && &s[i..(i + word.len())] == word {
                dp[i] = dp[i + word.len()];
            }
            // found a matching word, we can break early
            if dp[i] {
                break
            }
        }
    }
    dp[0]
}

fn main() {

}

#[cfg(test)]
mod tests {
    use crate::word_break;

    #[test]
    fn example1() {
        let s = "leetcode".to_string();
        let words = vec!["leet".to_string(), "code".to_string()];
        assert_eq!(word_break(s, words), true);
    }
}