/// # Problem 122 - Best Time to buy and sell stock 2
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
///

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut total_profit = 0_i32;
    let mut best_profit = 0_i32;
    let mut i = 0_usize;

    while let Some(start_price) = prices.get(i) {
        let mut j = i + 1;
        while let Some(price) = prices.get(j) {
            let profit = *price - start_price;
            if profit >= best_profit {
                best_profit = profit;
            } else {
                break;
            }
            j += 1;
        }
        total_profit += best_profit;
        best_profit = 0;
        i = j;
    }
    total_profit
}

#[cfg(test)]
mod tests {
    use crate::max_profit;

    #[test]
    fn example1() {
        assert_eq!(max_profit(vec![7,1,5,3,6,4]), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(max_profit(vec![1,2,3,4,5]), 4);
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