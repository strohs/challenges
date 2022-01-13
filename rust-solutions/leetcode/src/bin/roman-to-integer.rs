use std::collections::HashMap;

/// # Problem 13 - Roman Numerals to Integer
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
// Given a Roman Numeral, convert it to an Integer. Input is guaranteed to be within the
// range from 1 to 3999

pub fn roman_to_int(rom: &str) -> i32 {
    let mapper = build_number_map();

    let res = rom
        .chars()
        .scan((i32::MAX, 0), |state, c| {
            // state.0 holds the previous character's integer value
            // state.1 holds the current sum of the roman numerals

            // lookup integer value for current roman numeral character
            let curr = *mapper.get(&c).unwrap();

            // check if previous roman character is less than current character
            if state.0 < curr {
                state.1 -= state.0; // remove prev. character's value from sum
                state.1 += curr - state.0; // add curr - prev to the sum
            } else {
                state.1 += curr;
            }
            state.0 = curr; // set the previous char to the current character

            Some(state.1)
        })
        .last()
        .unwrap();

    res
}

/// build a hashMap of Roman Numeral Characters to their integer value
fn build_number_map() -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::with_capacity(7);
    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);

    map
}

fn main() {
    let result = roman_to_int("IV");
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use crate::roman_to_int;

    #[test]
    fn test3() {
        assert_eq!(roman_to_int("III"), 3);
    }

    #[test]
    fn test4() {
        assert_eq!(roman_to_int("IV"), 4);
    }

    #[test]
    fn test9() {
        assert_eq!(roman_to_int("IX"), 9);
    }

    #[test]
    fn test58() {
        assert_eq!(roman_to_int("LVIII"), 58);
    }

    #[test]
    fn test1994() {
        assert_eq!(roman_to_int("MCMXCIV"), 1994);
    }
}
