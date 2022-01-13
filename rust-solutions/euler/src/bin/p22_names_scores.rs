use std::{io, fs};

/// # Problem 22 - Names Scores
/// Using `names.txt`, a 46K text file containing over five-thousand first names, begin by
/// sorting it into alphabetical order. Then working out the alphabetical value for each name,
/// multiply this value by its alphabetical position in the list to obtain a name score.
///
/// For example, when the list is sorted into alphabetical order, COLIN, which is worth:
/// 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score
/// of 938 × 53 = 49714.
/// What is the total of all the name scores in the file?


const NAME_PATH: &str = "./euler/src/bin/p022_names.txt";

/// read a file into a string
fn file_to_string(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

/// name is a str of upper case ASCII characters. This function converts each character to
/// it's ordinal position in the alphabet, and then returns the sum of all positions.
/// Example: "ABC" = 1 + 2 + 3 = 6
fn name_score(name: &str) -> u32 {
    name.as_bytes()
        .iter()
        .map(|b| (*b - 65 + 1) as u32 )
        .sum::<u32>()
}

fn parse_file() -> Result<Vec<String>, io::Error> {
    let mut name_str = file_to_string(NAME_PATH)?;
    name_str = name_str.replace("\"", "");
    let names = name_str.split(",")
        .map(|s| String::from(s))
        .collect();
    Ok(names)
}

fn main() {
    let mut names = parse_file().unwrap();
    // sort the vec of names
    names.sort();
    let total_score: u64 = names
        .iter()
        .enumerate()
        .map(|(idx, name)| name_score(name) as u64 * (idx as u64 + 1))
        .sum();

    // 871198282
    println!("total name score is {}", total_score);
}
