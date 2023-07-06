/// # Excercism - Run-Length Encoding
/// https://exercism.org/tracks/rust/exercises/run-length-encoding

pub fn encode(source: &str) -> String {
    let mut encoded = String::new();

    let mut cur = &source[0..1];
    let mut cur_count = 0;

    for i in 0..source.len() {
        match source.get(i..i+1) {
            Some(next) if cur == next  => {
                cur_count += 1;
            },
            Some(next) => {
                if cur_count > 1 {
                    encoded.push_str(&cur_count.to_string());
                }
                encoded.push_str(cur);
                cur = next;
                cur_count = 1;
            },
            None => (),
        }
    }
    if cur_count > 1 {
        encoded.push_str(&cur_count.to_string());
    }
    encoded.push_str(cur);

    encoded
}

pub fn decode(source: &str) -> String {
    unimplemented!("Return the run-length decoding of {source}.");
}

#[cfg(test)]
mod tests {
    use crate::run_length_encoding::encode;

    #[test]
    fn test_encode() {
        assert_eq!(encode(&"AABCCCDEEEE"), "2AB3CD4E");
    }
}