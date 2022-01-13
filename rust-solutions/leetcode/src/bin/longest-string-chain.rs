/// Given a list of words, each word consists of English lowercase letters.
/// Let's say `word1` is a predecessor of `word2` if and only if we can add exactly one letter
/// anywhere in `word1` to make it equal to `word2`.  For example, **"abc"** is a predecessor of
/// **"abac"**
/// A word chain is a sequence of words `[word_1, word_2, ..., word_k]` with `k >= 1`, where
/// `word_1` is a predecessor of `word_2`, `word_2` is a predecessor of `word_3`, and so on.
/// Return the longest possible length of a word chain with words chosen from the given list
/// of words
///
/// # Example 1
/// ```
/// Input: ["a","b","ba","bca","bda","bdca"]
/// Output: 4
/// Explanation: one of the longest word chain is "a","ba","bda","bdca"
/// ```
///
/// ## Notes:
/// - the order of characters in a word matters! bdac is not a predecessor of bdcacc BUT
///   bdac to bdacc would be
/// - ` 1 <= words.length <= 1000`
/// - `1 <= words[i].length <= 16`
/// - `words[i]` only consists of English lowercase letters.

pub fn longest_str_chain(words: Vec<String>) -> i32 {
    // is word1 a predecessor of word2
    fn is_predecessor(w1: &str, w2: &str) -> bool {
        if w1.len() < w2.len() {
            for ch in w2.chars() {
                for i in 0..=w1.len() {
                    let splits = w1.split_at(i);
                    let joined = format!("{}{}{}", splits.0, ch, splits.1);
                    if joined == w2 {
                        return true;
                    }
                }
            }
        }
        false
    }

    // sort words by word length
    let mut sorted_words = words.to_vec();
    sorted_words.sort_by_key(|word| word.len());
    let mut longest_chain: Vec<&String> = vec![];

    for (i, word) in sorted_words.iter().enumerate() {
        let mut cur_chain = vec![word];

        let mut j = i;
        loop {
            let cur_word = *cur_chain.last().unwrap();
            let next_len = cur_word.len() + 1;

            // advance j until words of next_len are found
            while j < sorted_words.len() && sorted_words[j].len() != next_len {
                j += 1;
            }

            if j >= sorted_words.len() {
                // no words of appropriate length found, exit inner loop
                break;
            }
            // otherwise find first word of matching len() that is also a predecessor
            if let Some(predecessor) = sorted_words[j..]
                .iter()
                .find(|&w| w.len() == next_len && is_predecessor(cur_word, w))
            {
                cur_chain.push(predecessor);
            } else {
                // no predecessors found for the chunk, exit the inner loop
                break;
            }
        }
        if cur_chain.len() > longest_chain.len() {
            longest_chain = cur_chain.to_vec();
        }
    }
    dbg!(&longest_chain);
    longest_chain.len() as i32
}

fn main() {
    let words: Vec<String> = vec![
        String::from("b"),
        String::from("ba"),
        String::from("bca"),
        String::from("bda"),
        String::from("bdca"),
        String::from("a"),
    ];

    let longest_chain = longest_str_chain(words);
    dbg!(longest_chain);
}
