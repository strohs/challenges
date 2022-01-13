
/*
John works at a clothing store. He has a large pile of socks that he must pair by color for sale. Given an array of
integers representing the color of each sock, determine how many pairs of socks with matching colors there are.

For example, there are n = 7 socks with colors ar=[1,2,1,2,1,3,2]. There is one pair of color 1 and one of color 2.
There are three odd socks left, one of each color. The number of pairs is 2.
 */

use std::collections::HashMap;

fn sock_merchant(arr: &[u32]) -> u32 {
    let mut hm: HashMap<u32,u32> = HashMap::new();

    for i in arr {
        let count = hm.entry(*i).or_insert(0);
        *count += 1;
    }
    hm.values()
        .fold(0, |acc,v| acc + (v / 2) )
}

fn main() {
    let arr: [u32;9] = [1,2,1,2,1,2,3,4,1];
    let pc = sock_merchant(&arr);
    println!("number of pairs {}", pc);
}