use std::ops::RangeInclusive;

/// Solving the 01-Knapsack problem using dynamic programming
/// based on the algorithm described [here](https://youtu.be/cJ21moQpofY)

/// Item is a tuple-struct that holds an items: (index, weight, value)
#[derive(Debug, Clone, Copy)]
pub struct Item(usize, u32, u32);

impl Default for Item {
    fn default() -> Self {
        Self(0, 0, 0)
    }
}


/// returns the items that will have the max value without exceeding `max_weight`
/// `weights` - is the weight of the items
/// `values` - is the items values
/// `max_weight` - the weight that must not be exceeded
pub fn solve(weights: &[u32], values: &[u32], max_weight: u32) -> Vec<Item> {
    let length = values.len();
    // dp is the dynamic programming matrix that holds items on the rows, and 0..=max_weight on the columns
    let mut dp: Vec<Vec<u32>> = vec![vec![0_u32; max_weight as usize + 1]; length + 1];

    // determine the best possible value
    for ii in 1..=(values.len()) {
        for wi in 0..=(max_weight as usize) {
            // if weight of item[ii-1[ will fit in curr weight (wi)
            if weights[ii-1] <= wi as u32 {
                // curr item fits, so best value is curr items value plus the value of the the previous items
                // at index: dp[ii-1][wi - curr_weight]
                let curr_weight = weights[ii - 1] as usize; // the current items weight
                dp[ii][wi] = values[ii-1] + dp[ii-1][wi - curr_weight];
            } else {
                // curr item wont fit in curr weight so best value is the item value in the row above
                dp[ii][wi] = dp[ii-1][wi];
            }
        }
    }
    // best possible value is at the bottom right corner of the dp matrix

    // now find the actual items, by iterating thru the rows in reverse order. Stop iterating when the item index == 0
    // Include an item if the curr value and the value in the row above it differ.
    // If the item is included, subtract it weight from wi to get the next wi index to look at (in the row above)
    // If the curr value and the value in the row above are the same, the curr item is not included and we simply
    // move up one row to examine the previous item
    let mut items: Vec<Item> = Vec::new();
    // wi is the current weight within the dp matrix
    let mut wi = max_weight as usize;
    for ii in (1..=values.len()).rev() {
        let curr_value = dp[ii][wi];
        if curr_value != dp[ii-1][wi] {
            items.push(Item(ii-1, weights[ii-1], values[ii-1]));
            wi -= weights[ii-1] as usize;
        }
    }

    items
}

fn main() {

    let weights = vec![3_u32, 1, 3, 4, 2];
    let values = vec![2_u32, 2, 4, 5 ,3];
    let items = solve(&weights, &values, 7);
    for item in &items {
        println!("{:?}", item);
    }
    let totals = items.iter().fold(Item::default(), |acc, i| Item(0, acc.1 + i.1, acc.2 + i.2));
    println!("totals: weight={} value={}", totals.1, totals.2);


    let weights = [23_u32, 26, 20, 18, 32, 27, 29, 26, 30, 27];
    let values = [505_u32, 352, 458, 220, 354, 414, 498, 545, 473, 543];
    let items = solve(&weights, &values, 67);
    for item in &items {
        println!("{:?}", item);
    }
    let totals = items.iter().fold(Item::default(), |acc, i| Item(0, acc.1 + i.1, acc.2 + i.2));
    println!("totals: weight={} value={}", totals.1, totals.2);

}