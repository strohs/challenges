
/// # Leetcode - 345 - Reverse Vowels of a String
/// https://leetcode.com/problems/reverse-vowels-of-a-string/
///
pub fn reverse_vowels(s: String) -> String {
    const VOWELS: [u8; 10] = [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];

    let mut bytes = s.into_bytes();
    let mut left = 0;
    let mut right = bytes.len() - 1;

    while left < right {
        match (bytes[left], bytes[right]) {
            (lc, rc) if VOWELS.contains(&lc) && VOWELS.contains(&rc) => {
                bytes.swap(left, right);
                left += 1;
                right -= 1;
            },
            (lc, rc) if VOWELS.contains(&lc) && !VOWELS.contains(&rc) => {
                right -= 1;
            },
            (lc, rc) if !VOWELS.contains(&lc) && VOWELS.contains(&rc) => {
                left += 1;
            },
            _ => {
                left += 1;
                right -= 1;
            }
        }
    }

    String::from_utf8(bytes).unwrap()
}

fn main() {
    let s = String::from("leetcode");
    println!("{}", reverse_vowels(s));

    let s = String::from("e");
    println!("{}", reverse_vowels(s));
}