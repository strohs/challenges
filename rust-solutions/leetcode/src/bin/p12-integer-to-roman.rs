use std::collections::HashMap;

/// # Problem 12 - Integer to Roman Numerals
/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M
/// For example, two is written as II in Roman numeral, just two one's added together. Twelve is
/// written as, XII, which is simply X + II. The number twenty seven is written as XXVII,
/// which is XX + V + II.
//
// Roman numerals are usually written largest to smallest from left to right. However, the numeral
// for four is not IIII. Instead, the number four is written as IV. Because the one is before the
// five we subtract it making four. The same principle applies to the number nine, which is written
// as IX. There are six instances where subtraction is used:
//
// * I can be placed before V (5) and X (10) to make 4 and 9.
// * X can be placed before L (50) and C (100) to make 40 and 90.
// * C can be placed before D (500) and M (1000) to make 400 and 900.
//
// Given an integer, convert it to a roman numeral. Input is guaranteed to be within the
// range from 1 to 3999

/// converts `n` to roman numerals by dividing it by the `divisor` and then subtracting
/// `n` from the `divisor`. This repeats until the remainder becomes less than the divisor.
/// Returns a 2-tuple containing the roman numeral digits as a String, and the last remainder
/// obtained from dividing n by the divisor
fn convert(n: i32, divisor: i32, mapper: &HashMap<i32, &str>) -> (String, i32) {
    let mut n = n;
    let mut rom = String::from("");
    while n / divisor > 0 {
        rom += mapper.get(&divisor).unwrap();
        n -= divisor;
    }
    (rom, n)
}

pub fn int_to_roman(num: i32) -> String {
    let mut roman = String::new();
    let mapper = build_int_map();
    let mut rem = num;

    while rem > 0 {
        let (cur_rom, remainder): (String, i32) = {
            if rem >= 1000 {
                convert(rem, 1000, &mapper)
            } else if (500..=999).contains(&rem) {
                if rem / 900 == 1 {
                    convert(rem, 900, &mapper)
                } else {
                    convert(rem, 500, &mapper)
                }
            } else if (100..=499).contains(&rem) {
                if rem / 400 == 1 {
                    convert(rem, 400, &mapper)
                } else {
                    convert(rem, 100, &mapper)
                }
            } else if (50..=99).contains(&rem) {
                if rem / 90 == 1 {
                    convert(rem, 90, &mapper)
                } else {
                    convert(rem, 50, &mapper)
                }
            } else if (10..=49).contains(&rem) {
                if rem / 40 == 1 {
                    convert(rem, 40, &mapper)
                } else {
                    convert(rem, 10, &mapper)
                }
            } else if (5..=9).contains(&rem) {
                if rem / 9 == 1 {
                    convert(rem, 9, &mapper)
                } else {
                    convert(rem, 5, &mapper)
                }
            } else if rem / 4 == 1 {
                (mapper.get(&4).unwrap().to_string(), 0)
            } else {
                (mapper.get(&rem).unwrap().to_string(), 0)
            }
        };
        roman += &cur_rom;
        rem = remainder;
    }

    roman
}

/// build a *static* hashMap of integers to Roman Numerals
fn build_int_map() -> HashMap<i32, &'static str> {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity(19);
    map.insert(0, "");
    map.insert(1, "I");
    map.insert(2, "II");
    map.insert(3, "III");
    map.insert(4, "IV");
    map.insert(5, "V");
    map.insert(6, "VI");
    map.insert(7, "VII");
    map.insert(8, "VIII");
    map.insert(9, "IX");
    map.insert(10, "X");
    map.insert(40, "XL");
    map.insert(50, "L");
    map.insert(90, "XC");
    map.insert(100, "C");
    map.insert(400, "CD");
    map.insert(500, "D");
    map.insert(900, "CM");
    map.insert(1000, "M");

    map
}

fn main() {
    let roms = int_to_roman(1999);
    dbg!(roms);
}

#[cfg(test)]
mod tests {
    use crate::int_to_roman;

    #[test]
    fn test3() {
        assert_eq!(int_to_roman(3), "III");
    }

    #[test]
    fn test4() {
        assert_eq!(int_to_roman(4), "IV");
    }

    #[test]
    fn test9() {
        assert_eq!(int_to_roman(9), "IX");
    }

    #[test]
    fn test58() {
        assert_eq!(int_to_roman(58), "LVIII");
    }

    #[test]
    fn test1994() {
        assert_eq!(int_to_roman(1994), "MCMXCIV");
    }
}
