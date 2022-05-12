/// # Problem 58 - Length of last word
/// Given a string `s` consisting of some words separated by some number of spaces,
/// return the length of the last word in the string.
///
/// A word is a maximal substring consisting of non-space characters only.
pub fn length_of_last_word(s: String) -> i32 {

    // find the index of the last alphabetic character in the string
    let end_index = *&s[..]
        .bytes()
        .enumerate()
        .rev()
        .find(|(_i, b)| b.is_ascii_alphabetic())
        .map(|(i, _b)| i + 1)
        .unwrap();

    // find the index of the first whitespace character starting from end_index
    let start_index = *&s[..end_index]
        .bytes()
        .enumerate()
        .rev()
        .find(|(_i, b)| b.is_ascii_whitespace())
        .map(|(i, _b)| i + 1)
        .or_else(|| Some((0)))
        .unwrap();

    *&s[start_index..end_index].len() as i32
}

fn main() {
    let len = length_of_last_word(String::from("xyz world   "));
    println!("{len}");

    let len = length_of_last_word(String::from("  xyz guys"));
    println!("{len}");

    let len = length_of_last_word(String::from("guvnor"));
    println!("{len}");
}