/// # Problem 43 - Multiply Two Strings
/// Given two non-negative integers `num1` and `num2` represented as strings, return the product of `num1` and `num2`,
/// also represented as a string.
///
/// Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
///
/// ## Example 2
/// Input: num1 = "123", num2 = "456"
/// Output: "56088"

pub fn multiply(num1: String, num2: String) -> String {
    // base condition checks
    match (&num1[..], &num2[..]) {
        (num1, num2) if num1 == "0" || num2 == "0" => return String::from("0"),
        ("1", _) => return num2,
        (_, "1") => return num1,
        _ => (),
    }

    let mut res: Vec<u8> = vec![0; num1.len() + num2.len()];

    // make sure num1 is the smaller of the two Strings
    let (num1, num2) = if num2.len() < num1.len() {
        (num2, num1)
    } else {
        (num1, num2)
    };

    // SAFETY: num1,num2 contain only digit characters, therefore each char will only be one byte
    for (i, c1) in num1.char_indices().rev() {
        for (j, c2) in num2.char_indices().rev() {
            let (d1, d2) = (
                c1.to_digit(10).unwrap() as u8,
                c2.to_digit(10).unwrap() as u8,
            );
            res[i + j] += (d1 * d2) / 10;
            res[i + j + 1] += (d1 * d2) % 10;

            if res[i + j + 1] >= 10 {
                let (quot, rem) = (res[i + j + 1] / 10, res[i + j + 1] % 10);
                res[i + j + 1] = rem;
                res[i + j] += quot;
            }
        }
    }

    // find first non-zero index
    let non_zero_idx = res
        .iter()
        .enumerate()
        .find(|&(_i, n)| *n != 0)
        .map_or(0, |(i, _n)| i);
    res[non_zero_idx..]
        .iter()
        .map(|n| char::from_digit(*n as u32, 10).unwrap())
        .collect::<String>()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::multiply;

    #[test]
    fn test_num1_0() {
        let num1 = String::from("0");
        let num2 = String::from("145");
        assert_eq!(multiply(num1, num2), String::from("0"));
    }

    #[test]
    fn test_num2_0() {
        let num1 = String::from("1230");
        let num2 = String::from("0");
        assert_eq!(multiply(num1, num2), String::from("0"));
    }

    #[test]
    fn test_num1_1() {
        let num1 = String::from("1");
        let num2 = String::from("13");
        assert_eq!(multiply(num1, num2), String::from("13"));
    }

    #[test]
    fn test_num2_1() {
        let num1 = String::from("13421");
        let num2 = String::from("1");
        assert_eq!(multiply(num1, num2), String::from("13421"));
    }

    #[test]
    fn example2() {
        let num1 = String::from("123");
        let num2 = String::from("456");
        assert_eq!(multiply(num1, num2), String::from("56088"));
    }

    #[test]
    fn example3() {
        let num1 = String::from("2");
        let num2 = String::from("6");
        assert_eq!(multiply(num1, num2), String::from("12"));
    }

    #[test]
    fn example4() {
        let num1 = String::from("22");
        let num2 = String::from("66");
        assert_eq!(multiply(num1, num2), String::from("1452"));
    }
}
