use std::collections::HashMap;

fn build_jump_map() -> HashMap<u8, Vec<u8>> {
    let mut map = HashMap::with_capacity(10);
    map.insert(1, vec![6, 8]);
    map.insert(2, vec![7, 9]);
    map.insert(3, vec![4, 8]);
    map.insert(4, vec![3, 9, 0]);
    map.insert(5, vec![]);
    map.insert(6, vec![1, 7, 0]);
    map.insert(7, vec![2, 6]);
    map.insert(8, vec![1, 3]);
    map.insert(9, vec![4, 2]);
    map.insert(0, vec![4, 6]);
    map
}

// h = hops
// m = max number of corresponding digits for a number = 3
// time: O(h^m) = O(h^3)
// mem: O(h*m)
fn knights_dialer(start_number: u8, hops: u32) -> i32 {
    let jumps = build_jump_map();
    let mut h = hops;
    let mut stack = vec![start_number];

    while !stack.is_empty() {
        if h == 0 {
            return stack.len() as i32;
        } else {
            // pop all numbers out of the stack
            // for each popped number, push their jump_map values back into the stack
            let next_nums = stack.drain(..)
                .map(|n| jumps[&n].clone())
                .flatten()
                .collect::<Vec<u8>>();
            for nn in next_nums {
                stack.push(nn);
            }
            h -= 1;
        }
    }
    1
}

fn main() {
    let ans = knights_dialer(6, 2);
    println!("{}", ans); // 6

    println!("{}", knights_dialer(6, 0)); // 1

    println!("{}", knights_dialer(5, 1)); // 1

    println!("{}", knights_dialer(6, 3)); // 16

    println!("{}", knights_dialer(6, 40)); // 18136064
}