// Suppose you dial keys on the keypad using only hops a knight can make. Every time the knight
// lands on a key, we dial that key and make another hop. The starting position counts as
// being dialed.
// How many distinct numbers can you dial in N hops from a particular starting position?
// See https://hackernoon.com/google-interview-questions-deconstructed-the-knights-dialer-f780d516f029
// 6 -> 0,1,7
//   0 -> 4,6
//       4 -> 0,3,9
//       6 -> 0,1,7
//   1 -> 6,8
//       6 -> 0,1,7
//       8 -> 1,3
//   7 -> 2,6
//       2 -> 7,9
//       6 -> 0,1,7
// 6,0,4
// 6,0,6
// 6,1,6
// 6,1,8
// 6,7,2
// 6,7,6

use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEY_MAP: Lazy<HashMap<u8, Vec<u8>>> = Lazy::new(|| {
    let mut hm = HashMap::with_capacity(10);
    hm.insert(1, vec![6, 8]);
    hm.insert(2, vec![7 ,9]);
    hm.insert(3, vec![4 ,8]);
    hm.insert(4, vec![0, 3, 9]);
    hm.insert(5, vec![]);
    hm.insert(6, vec![0, 1, 7]);
    hm.insert(7, vec![2, 6]);
    hm.insert(8, vec![1, 3]);
    hm.insert(9, vec![2 ,4]);
    hm.insert(0, vec![4 ,6]);

    hm
});


fn neighbors(key: u8) -> &'static Vec<u8> {
    KEY_MAP.get(&key).expect("key must be in the range 0 - 9")
}

fn yield_seq_iter(start_pos: u8, n: usize) -> Vec<Vec<u8>> {
    let mut results = vec![vec![start_pos]];
    let mut n = n;

    while n > 0 {
        let mut cur_list: Vec<Vec<u8>> = Vec::new();

        for num_list in &results {
            let nn = neighbors(*num_list.last().unwrap());
            
            for n in nn {
                let mut cur_seq = num_list.clone();
                cur_seq.push(*n);
                cur_list.push(cur_seq);
            }
        }
        results = cur_list;
        n -= 1;
    }

    results
}

fn main() {
    let seqs = yield_seq_iter(6, 2);
    for seq in seqs.iter() {
        println!("{:?}", seq);
    }
}