use std::collections::HashMap;
use std::thread;
//use std::sync::Arc;

/// Count the frequency of **letters** in texts using parallel computation.
/// Parallelism is about doing things in parallel that can also be done sequentially. A
/// common example is counting the frequency of letters. Create a function that returns the
/// total frequency of each letter in a list of texts and that employs parallelism.

/// uses native threads to compute the frequencies of letters within the given `input`.
/// NOTE: since `input` is passed to this function as a `&[&str]`, it will be impossible to share
/// be between threads without using an external crate like crossbeam or rayon.
/// Therefore, the string data will need to be copied before sending it to a thread.
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // create a vec to hold our join handles
    let mut handles = vec![];

    // freqs will hold the final results from all threads
    let mut freqs: HashMap<char, usize> = HashMap::new();

    if input.is_empty() {
        return freqs;
    }

    // determine the size of each "chunk" a thread will receive from the `input` slice
    let chunk_size = (input.len() + (worker_count - 1)) / worker_count;

    for chunk in input.chunks(chunk_size) {
        let chunk_copy: Vec<String> = chunk.iter().map(|s| String::from(*s)).collect();

        // create a worker thread and have each thread compute their own frequency map
        // for their given Vec of input strings. Once the work is done, return a HashMap
        // containing the final results
        let handle = thread::spawn(move || {
            let mut res_map: HashMap<char, usize> = HashMap::new();
            for string in chunk_copy.iter() {
                let fm = letter_freqs(string);
                merge_maps_counts(fm, &mut res_map)
            }
            res_map
        });
        handles.push(handle);
    }

    // now join all the threads and merge their resulting hashmaps into our "final" hashmap
    for handle in handles {
        let res_map = handle.join().expect("a thread may have panicked");
        merge_maps_counts(res_map, &mut freqs);
    }

    freqs
}

/// returns a HashMap that maps each **ALPHABETIC** character in `s` to the number of times it
/// occurs in `s`. if the input string is empty, and empty hashMap is returned
fn letter_freqs(s: &str) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();

    for ch in s.chars() {
        if ch.is_alphabetic() {
            // NOTE: non-ASCII chars will not be converted to lowercase
            let normalized_char = ch.to_ascii_lowercase();
            let ch_count = map.entry(normalized_char).or_insert(0);
            *ch_count += 1;
        }
    }

    map
}

/// merges the `src` HashMap into the `tar` HashMap. Entries with matching keys will have their
/// values summed together. Entries from `src` that don't exist in `tar` will be inserted into
/// `tar`. The `src` HashMap will be consumed
fn merge_maps_counts(src: HashMap<char, usize>, tar: &mut HashMap<char, usize>) {
    for (src_char, src_count) in src {
        let tar_count = tar.entry(src_char).or_insert(0);
        *tar_count += src_count;
    }
}

#[cfg(test)]
mod tests {
    use super::frequency;
    use std::collections::HashMap;

    #[test]
    fn freqs_with_4_workers() {
        let worker_count = 4_usize;
        let inputs = [
            "aaaa,1,2",
            "bbbb,3,4",
            "cccc,5,6",
            "dddd,7,8",
            "Freude schöner Götterfunken",
            "ffff",
            "gggg",
        ];
        let frequencies = frequency(&inputs, worker_count);
        assert_eq!(*frequencies.get(&'c').unwrap(), 5_usize);
    }

    #[test]
    fn test_no_texts() {
        assert_eq!(frequency(&[], 4), HashMap::new());
    }
}
