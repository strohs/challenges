/// A bounded knapsack problem where we need to select items of weight = 2 or 3 such that
/// their total weight is exactly = to 9

fn main() {
    let max_weight = 9;
    let items = vec![2, 3];
    let max_twos = max_weight / 2;
    let max_threes = max_weight / 3;
    // bound is the maximum number of items we can choose from the items "set"
    let bound = max_twos.max(max_threes); // = 4
    let items = vec![2, 2, 2, 2, 3, 3, 3];

}