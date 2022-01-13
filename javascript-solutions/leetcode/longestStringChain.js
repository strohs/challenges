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


/**
 * `word1` is a predecessor of `word2` if and only if we can add exactly one letter
 * anywhere in `word1` to make it equal to `word2`.
 * This functions takes each character from w2 and interposes it into w1 until w1 === w2.
 * @param w1 {string} - word1
 * @param w2 {string} - word2
 * @returns {boolean} - true if w1 is a predecessor of w2
 */
function isPredecessor(w1, w2) {
    if (w1.length < w2.length) {
        const w2s = w2.split("");
        for (const ch of w2s) {
            for (let i=0; i <= w1.length; i++) {
                const first = w1.substring(0,i);
                const last = w1.substring(i);
                const joined = first.concat(ch, last);
                if (joined === w2) {
                    return true;
                }
            }
        }
    }
    return false;
}

function longestStringChain(words) {
    // sort words by length of each word
    const sortedWords = words.sort((w1, w2) => w1.length - w2.length);
    let longestChain = [];

    sortedWords.forEach((word, i) => {
        const curChain = [word];
        for (let j = i; j < sortedWords.length; j++) {
            const curWord = curChain[curChain.length - 1];
            const nextLen = curWord.length + 1;

            // advance j until words of nexLen are found
            while (j < sortedWords.length && sortedWords[j].length !== nextLen) {
                j++;
            }

            if (j >= sortedWords.length) break;

            while (sortedWords[j].length === nextLen) {
                if (isPredecessor(curWord, sortedWords[j])) {
                    curChain.push(sortedWords[j]);
                    break;
                } else {
                    j++;
                }
            }
        }
        if (curChain.length > longestChain.length) {
            longestChain = curChain.slice();
        }
    });
    return longestChain;
}

const words = ["ba", "b", "bca", "bda", "bdca", "a"];
//const words = ["bda", "bdca"];
console.log(longestStringChain(words));

