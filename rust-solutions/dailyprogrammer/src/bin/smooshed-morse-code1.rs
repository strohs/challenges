/// in this challenge morse code dots and dashes will be 'smooshed' together. Write a function:
/// `snorse` that takes a string of letters as input, and outputs morse code dots and dashes
/// smooshed together
///
/// # Example
///
/// ```
/// smorse("sos") => "...---..."
/// smorse("daily") => "-...-...-..-.--"
/// smorse("programmer") => ".--..-.-----..-..-----..-."
/// smorse("bits") => "-.....-..."
/// smorse("three") => "-.....-..."
/// ```
///
/// An obvious problem with this system is that decoding is ambiguous. For instance,
/// both `bits` and `three` encode to the same string, so you can't tell which one you would
/// decode to without more information.
///
/// #  Bonus Challenges
/// 1. The sequence `-...-....-.--.` is the code for four different words
///   `(needing, nervate, niding, tiling)`. Find the only sequence that's the code
///   for 13 different words.

use std::collections::HashMap;
use std::{io};
use std::fs::File;
use std::io::BufRead;

// path to the enable1.txt (contains a word list for bonus challenges)
const PATH: &str = "./dailyprogrammer/src/bin/enable1.txt";

// morse code alphabet (space separated)
const LETTERS_A_Z: &str = "abcdefghijklmnopqrstuvwxyz";
const MORSE_A_Z: &str = ".- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -. --- .--. --.- .-. ... - ..- ...- .-- -..- -.-- --..";

// fn read_from_file_to_string(path: &str) -> Result<String, io::Error> {
//     fs::read_to_string(path)
// }

// return a BufReader to lines of a file
fn read_lines(path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

// build a map of letters to morse code
fn build_morse_map() -> HashMap<char, &'static str> {
    LETTERS_A_Z
        .chars()
        .zip(MORSE_A_Z.split(" "))
        .collect::<HashMap<char,&str>>()
}

// map chars of a string into a smooshed morse code string
fn smorse(s: &str, mapping: &HashMap<char, &str>) -> String {
    s.chars()
        .map(|c| mapping.get(&c).unwrap().to_owned())
        .collect::<String>()
}

// bonus challenge: encode all words to "smorse" and store in HashMap<&str, Vec<String>>
fn smorse_word_file(path: &str, mapping: &HashMap<char, &str>) -> HashMap<String, Vec<String>> {
    let mut word_map: HashMap<String,Vec<String>> = HashMap::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(word) = line {
                let smorse = smorse(&word, mapping);
                let words = word_map.entry(smorse).or_insert(vec![]);
                words.push(word);
            }
        }
    }
    word_map
}

fn main() {
    let mmap = build_morse_map();
    //dbg!(mmap);
    // println!("{}", smorse(&"sos", &mmap));
    // println!("{}", smorse(&"daily", &mmap));
    // println!("{}", smorse(&"programmer", &mmap));
    // println!("{}", smorse(&"bits", &mmap));
    // println!("{}", smorse(&"three", &mmap));


    // BONUS 1, find the only sequence that's the code for 13 different words
    let word_map = smorse_word_file(PATH, &mmap);
    if let Some(entry) = word_map.iter().find(|&(_k, words)| words.len() == 13) {
        println!("found sequence {} for words {:?}", entry.0, entry.1);
    } else {
        println!("no code found that encodes 13 different words");
    }

}










#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mmap = build_morse_map();
        assert_eq!(smorse("sos", &mmap), "...---...");
        assert_eq!(smorse("daily", &mmap), "-...-...-..-.--");
        assert_eq!(smorse("programmer", &mmap), ".--..-.-----..-..-----..-.");
        assert_eq!(smorse("bits", &mmap), "-.....-...");
        assert_eq!(smorse("three", &mmap), "-.....-...");
    }
}