/// # [Problem 136 - Single Number](https://leetcode.com/problems/single-number/)
/// - `1 <= nums.length <= 3 * 10^4`
/// - `-3 * 10^4 <= nums[i] <= 3 * 10^4`
///

struct Solution;

impl Solution {

    /// uses bitwise xor based on the fact that a number, if XORed with itself equals zero.
    /// Plus XOR is associative and communative
    pub fn single_number(nums: Vec<i32>) -> i32 {
        return nums.iter().fold(0, |acc, n| acc ^ n);
    }

    // pub fn single_number(nums: Vec<i32>) -> i32 {
    //     let mut counts: Vec<(bool, bool)> = vec![(false, false); 30001];
    //
    //     for i in 0..nums.len() {
    //         let n = nums[i];
    //         let ci = n.abs() as usize;
    //
    //         match counts[ci] {
    //             (cn, cp) if n >= 0 => counts[ci] = (cn, !cp),
    //             (cn, cp) if n < 0 => counts[ci] = (!cn, cp),
    //             _c => panic!("unmatched index from nums {}", ci),
    //         }
    //     }
    //
    //     let mut res = 0_i32;
    //     for (ci, c) in counts.iter().enumerate() {
    //         match *c {
    //             (true, false) => {
    //                 res = ci as i32 * -1;
    //                 break;
    //             },
    //             (false, true) => {
    //                 res = ci as i32;
    //                 break;
    //             },
    //             _ => (),
    //         }
    //     }
    //     res
    // }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn ex1() {
        let nums = vec![2,2,1];
        assert_eq!(Solution::single_number(nums), 1);
    }

    #[test]
    fn ex2() {
        let mut  nums = vec![4,1,2,1,2];
        assert_eq!(Solution::single_number(nums), 4);

        nums = vec![4,1,2,1,2];
        let mut xor = 0;
        for num in nums {
            xor ^= num;
            dbg!(&xor);
        }
    }

    #[test]
    fn ex3() {
        let nums = vec![1,0,1];
        assert_eq!(Solution::single_number(nums), 0);
    }

    #[test]
    fn ex4() {
        let nums = vec![
            -108,977,915,-19,-32,-723,-621,-9,-924,895,-476,-959,828,864,
            -524,-691,-688,747,-8,-471,188,763,326,-1,819,-670,231,-606,-946,
            616,-234,237,-950,-382,-286,-975,853,35,-487,-679,353,-714,495,538,
            647,-473,-713,241,157,-840,-4,904,133,985,-952,-837,-209,935,413,-232,
            -645,-794,-799,584,921,116,-116,321,-208,254,-267,-651,866,-294,713,348,
            276,67,577,-690,490,-997,-156,-344,-967,-414,-164,-108,977,915,-19,-32,
            -723,-621,-9,-924,895,-476,-959,828,864,-524,-691,-688,747,-8,-471,188,
            763,326,-1,819,-670,231,-606,-946,616,-234,237,-950,-382,-286,-975,853,35,
            -487,-679,353,-714,495,538,647,-473,-713,241,157,-840,-4,904,133,985,-952,
            -837,-209,935,413,-232,-645,-794,-799,584,921,116,-116,321,-208,254,-267,-651,
            866,-294,713,348,276,67,577,-690,490,-997,-156,-344,-967,-414,-164,295
        ];
        assert_eq!(Solution::single_number(nums), 295);
    }
}