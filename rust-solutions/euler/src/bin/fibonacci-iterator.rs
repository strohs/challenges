

// holds fibonacci numbers in a HashMap
struct Fib {
    c: i64,
    n: i64,
}

impl Fib {
    fn new() -> Fib {
        Fib {
            c: -1,
            n: 1
        }
    }
}

impl Iterator for Fib {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;

        Some(self.n)
    }
}

fn main() {
    let mut fib = Fib::new();
    let res = fib.nth(0);
    println!("{:?}", res);
    // for n in fib.take(20) {
    //     println!("{}", n);
    // }
}