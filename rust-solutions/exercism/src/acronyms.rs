/// # Exercism - Acronyms
/// given a string of words, convert it to its three letter acronym

pub fn abbreviate(phrase: &str) -> String {
    let mut acro = String::default();
    for word in phrase.replace(&[':', '_', '-'], " ").split(&[' ', '-']) {
        // if word is all uppercase chars, add only the first char
        if !word.is_empty() && word.chars().all(|c| c.is_ascii_uppercase()) {
            acro += &word[0..1];
        } else {
            // add the first alphabetic char and all other UpperCase chars
            for (i, c) in word.chars().enumerate() {
                if c.is_ascii_alphabetic() && (i == 0 || c.is_ascii_uppercase()) {
                    acro += &c.to_uppercase().to_string();
                }
            }
        }
    }

    acro
}

#[cfg(test)]
mod tests {
    use crate::acronyms::abbreviate;

    #[test]
    fn test_split() {
        let s = "Complementary metal-oxide semiconductor";
        let acronym = abbreviate(s);
        assert_eq!(acronym, String::from("CMOS"));
    }

    #[test]
    fn all_caps_word_with_punctuation() {
        assert_eq!(abbreviate("PHP: Hypertext Preprocessor"), "PHP");
    }

    #[test]
    fn consec_delimiters() {
        assert_eq!(abbreviate("Something - I made up from thin air"), "SIMUFTA");
    }
}