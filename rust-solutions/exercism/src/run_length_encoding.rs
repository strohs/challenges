/// # Excercism - Run-Length Encoding
/// https://exercism.org/tracks/rust/exercises/run-length-encoding

pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    if source.is_empty() { return encoded }

    let mut cur = source.chars().nth(0).unwrap();
    let mut cur_count = 0;

    for next in source.chars() {
        if cur == next {
            cur_count += 1;
        } else {
            if cur_count > 1 {
                encoded.push_str(&cur_count.to_string());
            }
            encoded.push(cur);
            cur = next;
            cur_count = 1;
        }
    }
    if cur_count > 1 {
        encoded.push_str(&cur_count.to_string());
    }
    encoded.push(cur);

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count_str = String::new();
    for ch in source.chars() {
        if ch.is_ascii_digit() {
            count_str.push(ch);
        } else {
            let count: usize = count_str.parse().unwrap_or(1);
            for _ in 0..count {
                decoded.push(ch);
            }
            count_str = String::new();
        }
    }
    decoded
}

#[cfg(test)]
mod tests {
    use crate::run_length_encoding::{decode, encode};

    #[test]
    fn test_encode() {
        assert_eq!(encode("AABCCCDEEEE"), "2AB3CD4E");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("2AB3CD4E"), "AABCCCDEEEE");
    }

    #[test]
    fn test_decode_one_char() {
        assert_eq!(decode("A"), "A");
    }

}