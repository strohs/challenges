/// # Leetcode 134 - Gas Station
/// https://leetcode.com/problems/gas-station/
pub struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_gas = 0;
        let mut cur_gas = 0;
        let mut cur_idx = 0;
        for i in 0..gas.len() {
            total_gas += gas[i] - cost[i];
            cur_gas += gas[i] - cost[i];
            if cur_gas < 0 {
                cur_gas = 0;
                cur_idx = i + 1;
            }

       }
        return if total_gas < 0 {
            -1
        } else {
            cur_idx as i32
        }
    }
}

fn main() {
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn example1() {
        let gas = vec![1,2,3,4,5];
        let cost = vec![3,4,5,1,2];
        assert_eq!(3, Solution::can_complete_circuit(gas, cost));
    }

    #[test]
    fn example2() {
        let gas = vec![2,3,4];
        let cost = vec![3,4,3];
        assert_eq!(-1, Solution::can_complete_circuit(gas, cost));
    }

    #[test]
    fn example3() {
        let gas = vec![5,1,2,3,4];
        let cost = vec![4,4,1,5,1];
        assert_eq!(4, Solution::can_complete_circuit(gas, cost));
    }
}