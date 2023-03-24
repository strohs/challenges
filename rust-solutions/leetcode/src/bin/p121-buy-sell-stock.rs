/// # Problem 121 - Best Time to buy and sell stock
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
///

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut cur_max = prices[0];
    let mut cur_min = prices[0];
    let mut max_profit = 0;

    for i in 0..prices.len() {
        if prices[i] < cur_min {
            cur_min = prices[i];
            cur_max = prices[i];
        }
        if prices[i] > cur_max {
            cur_max = prices[i];
        }
        if cur_max - cur_min > max_profit {
            max_profit = cur_max - cur_min;
        }
    }
    max_profit
}

#[cfg(test)]
mod tests {
    use crate::max_profit;

    #[test]
    fn example1() {
        assert_eq!(max_profit(vec![7,1,5,3,6,4]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(max_profit(vec![7,6,4,3,1]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(max_profit(vec![2, 4, 1]), 2);
    }

    #[test]
    fn one_price_vector() {
        assert_eq!(max_profit(vec![7]), 0);
    }
}

fn main() {}