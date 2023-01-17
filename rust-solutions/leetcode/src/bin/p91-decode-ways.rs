/// # Problem 91 - decode ways
/// https://leetcode.com/problems/decode-ways/


/// used dynamic programming to keep track of the current number of ways from the previous
/// iteration
pub fn num_decodings(s: String) -> i32 {
    // stores the current number of ways to decode the input
    let mut ways = vec![0_i32; s.len() + 1];
    ways[0] = 1;
    ways[1] = if &s[..1] == "0" {
        0
    } else {
        1
    };
    for i in 2..=s.len() {
        let first = &s[i-1..i];
        let second = &s[i-2..i];
        if first != "0" {
            ways[i] += ways[i-1];
        }
        match &second[..1] {
            "1" => ways[i] += ways[i-2],
            "2" if &second[1..2] <= "6" => ways[i] += ways[i-2],
            _ => ()
        }
    }
    ways[s.len()]
}



fn main() {}

#[cfg(test)]
mod tests {
    use crate::num_decodings;

    #[test]
    fn test1() {
        assert_eq!(num_decodings("27".to_string()), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(num_decodings("226".to_string()), 3);
    }

    #[test]
    fn test9() {
        let s = "111111111111111111111111111111111111111111111".to_string();
        assert_eq!(num_decodings(s), 1836311903);
    }
}

// Recursive Solution that runs in O(2^N) time
// pub fn num_decodings(s: String) -> i32 {
//     fn decode_rec(s: &str, remaining: &str) -> i32 {
//         if s.starts_with("0") {
//             return 0
//         }
//         if s.len() == 2 && s > "26" {
//             return 0
//         }
//         if remaining.is_empty() {
//             return 1
//         }
//         return if remaining.len() > 1 {
//             decode_rec(&remaining[..1], &remaining[1..]) + decode_rec(&remaining[..2], &remaining[2..])
//         } else {
//             decode_rec(&remaining[..1], &remaining[1..])
//         }
//     }
//
//     return if s.len() > 1 {
//         decode_rec(&s[..1], &s[1..]) + decode_rec(&s[..2], &s[2..])
//     } else {
//         decode_rec(&s[..1], &s[1..])
//     }
// }