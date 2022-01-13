use std::error::Error;
use std::io;
use std::collections::HashMap;
use std::cmp::Reverse;

/// Each program must read from standard input and print the frequencies of unique, space-separated
/// words, in order from most frequent to least frequent. To keep our solutions simple and
/// consistent, here are the (self-imposed) constraints:
/// 1. normalize words to lower case
/// 2. assume words will not have punctuation
/// 3. assume ASCII only characters
/// 4. if frequency of two words is the same, then output in any order
/// 5. single threaded
/// 6. buffer line by line

//type Entry<'a> = (&'a str, &'a usize);

fn main() {

    if let Err(e) = word_count() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// reads a single line from stdin
fn read_from_stdio() -> Result<String, Box<dyn Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

fn word_count() -> Result<(), Box<dyn Error>> {
    let mut hash_map: HashMap<String, usize> = HashMap::new();

    // keep reading lines until empty line entered
    loop {
        let line = read_from_stdio()?;
        if line == "\n" {
            break;
        } else {
            // tokenize words and add them to hash map
            for word in line.split_ascii_whitespace() {
                let norm_word = word.to_ascii_lowercase();
                let word_count = hash_map.entry(norm_word).or_insert(0);
                *word_count += 1;
            }
        }
    }

    // iterate over hashMap K,V pairs and store in vector
    let mut entries: Vec<(String, usize)> = hash_map
        .into_iter()
        .collect();

    // sort the Vec of entries by the entry's second parameter, in reverse order
    entries.sort_by_key(|entry| Reverse(entry.1));

    // print out the entries
    for (word, count) in entries {
        println!("{} {}", word, count);
    }

    Ok(())
}
