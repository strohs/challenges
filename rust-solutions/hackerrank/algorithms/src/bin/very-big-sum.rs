
fn a_very_big_sum(arr: &[i64]) -> i64 {
    arr.iter().sum()
}

fn main() {
    let arr = [1000000001, 1000000002, 1000000003, 1000000004, 1000000005];
    println!("{}", a_very_big_sum(&arr))
}