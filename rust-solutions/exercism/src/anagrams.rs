use std::collections::HashSet;

// Exercism's Anagrams challenge for rust

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // sorts the characters of a &str, returning a new, sorted Vec<char>
    fn sort_chars(word: &str) -> Vec<char> {
        let mut chars = word.chars().collect::<Vec<char>>();
        chars.sort_unstable();
        chars
    }

    let mut matching_anagrams: HashSet<&str> = HashSet::new();

    for anagram in possible_anagrams {
        // convert anagram and word to lower case strings
        let lc_anagram = anagram.to_lowercase();
        let lc_word = word.to_lowercase();

        if lc_word != lc_anagram {
            let sorted_anagram = sort_chars(&lc_anagram);
            let sorted_word = sort_chars(&lc_word);
            if sorted_anagram == sorted_word {
                matching_anagrams.insert(anagram);
            }
        }
    }
    matching_anagrams
}

#[cfg(test)]
mod tests {
    use super::anagrams_for;

    #[test]
    fn should_match_inlets() {
        let anagrams = ["enlists", "google", "inlets", "banana"];
        let word = "listen";
        let af = anagrams_for(&word, &anagrams);
        assert_eq!(af.len(), 1);
        assert!(af.contains("inlets"));
    }

    #[test]
    fn should_match_inlets_but_not_match_listen() {
        let anagrams = ["enlists", "google", "inlets", "banana", "listen"];
        let word = "listen";
        let af = anagrams_for(&word, &anagrams);
        assert_eq!(af.len(), 1);
        assert!(af.contains("inlets"));
    }
}
