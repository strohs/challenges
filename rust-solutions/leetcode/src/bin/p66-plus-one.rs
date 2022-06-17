/// # Problem 66 - Plus One
/// You are given a large integer represented as an integer array `digits`, where each `digits[i]`
/// is the ith digit of the integer. The digits are ordered from most significant to least
/// significant in left-to-right order. The large integer does not contain any leading 0's.
///
/// Increment the large integer by one and return the resulting array of digits.
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    let mut digits = digits;

    for i in (0..digits.len()).rev() {
        let plus1 = digits[i] + carry;
        digits[i] = plus1 % 10;
        carry = plus1 / 10;
    }
    if carry == 1 {
        digits.insert(0, 1);
    }
    digits
}


fn main() {

}

#[cfg(test)]
mod tests {
    use crate::plus_one;

    #[test]
    fn example1() {
        let mut digits = vec![1,2,3];
        digits = plus_one(digits);
        assert_eq!(digits, vec![1,2,4]);
    }

    #[test]
    fn example2() {
        let mut digits = vec![9,9,9];
        digits = plus_one(digits);
        assert_eq!(digits, vec![1,0,0,0]);
    }

    #[test]
    fn example3() {
        let mut digits = vec![9];
        digits = plus_one(digits);
        assert_eq!(digits, vec![1,0]);
    }
}