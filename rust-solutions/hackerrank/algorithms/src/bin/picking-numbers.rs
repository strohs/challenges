/*
Given an array of integers, find and print the maximum number of integers you can select from the
array such that the absolute difference between any two of the chosen integers (in the set) is less
than or equal to 1.
For example, if your array is a = [1,1,2,2,4,4,5,5,5], you can create two sub-arrays meeting the
criterion: [1,1,2,2] and [4,4,5,5,5]. The maximum length subarray has 5 elements.
 */

fn picking_numbers(a: &[i32]) -> usize {
    fn abs_diff(n1: &i32, n2: &i32) -> bool { (n1 - n2).abs() <= 1 }

    let mut sl: Vec<i32> = a.to_vec();
    sl.sort_unstable();
    let mut cur_max: usize = 0;

    for (idx, n) in sl.iter().enumerate() {
        let matching: Vec<&i32> = sl[idx..].iter().take_while( |num| abs_diff(n, num) ).collect();
        if matching.len() > 1 && matching.len() > cur_max {
            cur_max = matching.len();
            println!("matching list {:?}", matching );
        }
    }
    return cur_max
}

fn main() {
    let arr1 = [1,1,2,2,4,4,5,5,5];
    let res = picking_numbers(&arr1);
    println!("res:{}",res);
}