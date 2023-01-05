/// # Problem 67 - Add Binary
/// Given two binary strings a and b, return their sum as a binary string.
pub fn add_binary(a: String, b: String) -> String {
    let mut res = String::new();
    let mut carry = 0_u32;

    let bin_chars = if a.len() > b.len() {
        a.chars()
            .rev()
            .zip(b.chars().rev().chain(std::iter::repeat('0')))
    } else {
        b.chars()
            .rev()
            .zip(a.chars().rev().chain(std::iter::repeat('0')))
    };

    for (c1, c2) in bin_chars {
        let mut sum = c1.to_digit(2).unwrap() + c2.to_digit(2).unwrap() + carry;
        if sum >= 2 {
            sum = sum % 2;
            carry = 1;
        } else {
            carry = 0;
        }
        res.push(char::from_digit(sum, 2).unwrap());
    }
    if carry == 1 {
        res.push('1');
    }

    res.chars().rev().collect()
}

fn main() {
    let a = "11".to_string();
    let b = "11".to_string();
    assert_eq!(add_binary(a, b), "110".to_string());

    let a = "11".to_string();
    let b = "1".to_string();
    assert_eq!(add_binary(a, b), "100".to_string());
    //
    // let a = "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string();
    // let b = "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string();
    // let sum = add_binary(a, b);
    // println!("{sum}");
    //assert_eq!(add_binary(a, b), "100".to_string());
}
