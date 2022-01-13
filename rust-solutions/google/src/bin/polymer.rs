use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};

/// taken from advent-of-code 2018, day 5
///  https://adventofcode.com/2018/day/5

/// read the file at `path` into a String
fn read_input(path: &Path) -> std::io::Result<String> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut out_str = String::new();

    reader.read_to_string(&mut out_str)?;
    Ok(out_str)
}


fn should_replace(slice: &str) -> bool {
    if slice[..1].eq_ignore_ascii_case(&slice[1..]) {
        slice[..1] != slice[1..]
    } else {
        false
    }
}

fn main() {
    let path = Path::new("./input.txt");
    let mut s = read_input(path).unwrap();

    let mut i = 0_usize;
    loop {
        if i+1 >= s.len() {
            break;
        }
        if should_replace(&s[i..i+2]) {
            s.replace_range(i..i+2, "");
            if i > 0 {
                i = i - 1;
            }
        } else {
            i = i + 1;
        }

    }

    println!("{}", &s);
    println!("{}", s.len());
}