use std::collections::HashMap;

/// find the nth number of a fibonacci sequence
/// our fibonacci sequence starts with: 1   i.e.:  1,1,2,3,5,8,13....
/// if n=6 return 8
/// we'll also try to memoize results in a HashMap

// holds fibonacci numbers in a HashMap
struct Fib {
    //c: u64,
    //n: u64,
    memo: HashMap<u32, u64>,
}

impl Fib {
    fn new() -> Fib {
        let mut memo: HashMap<u32, u64> = HashMap::new();
        memo.insert(1, 1);
        memo.insert(2, 1);
        Fib {
            //c: 1,
            //n: 1,
            memo: HashMap::new(),
        }
    }

    /// returns the nth number of the Fibonacci sequence, 1-based index
    /// index > 93 will panic! with overflow
    fn nth(&mut self, index: u32) -> u64 {
        if index == 0 {
            panic!("index must be > 0")
        }
        if index == 1 || index == 2 {
            return 1;
        }
        // first, check memo for index
        return if let Some(v) = self.memo.get(&index) {
            *v
        } else {
            // check if previous two indices are already in memo
            match (self.memo.get(&(index - 1)), self.memo.get(&(index - 2))) {
                (Some(p1), Some(p2)) => {
                    let next = *p1 + *p2;
                    self.memo.insert(index, next);
                    next
                }
                _ => {
                    // compute fibo
                    let mut prev: u64 = 1;
                    let mut cur: u64 = 1;
                    for _i in 2..index {
                        let next = prev + cur;
                        prev = cur;
                        cur = next;
                    }
                    self.memo.insert(index, cur);
                    cur
                }
            }
        };
    }
}

// impl Iterator for Fib {
//     type Item = u64;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let n = self.c + self.n;
//         self.c = self.n;
//         self.n = n;
//
//         Some(self.n)
//     }
// }

fn main() {
    let mut f = Fib::new();
    dbg!(f.nth(48));
    dbg!(f.nth(49));
    let s: i32 = (0..=100).into_iter().sum();
    dbg!(s);
}
