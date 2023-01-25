
fn main() {
    // let n = 2_u16;
    // let max = 2_u16.pow(n as u32);
    //  for i in (0..=max).rev() {
    //      let xor = i ^ i-1;
    //      let po2 = (xor & (xor - 1)) == 0;
    //      println!("{} ^ {} = {} ({:0b})  power of two? {}", i, i-1, xor, xor, po2);
    //
    //  }
         // power of two: (n & (n - 1)) == 0;
    // last number must be a power of two
    // seauence will have 2^n integers in it: [n, n], [n,n,n,n], [n,n,n,n,n,n,n,n]

    let n = 4;
    let mut v = vec![];
    let mut i = 0;
    while i < (1 << n) {
        let next = i ^ (i >> 1);
        println!("{:4} {:08b}", next, next);
        v.push(next);
        i += 1;
    }

    let s = String::from("abcde");
    let s1 = &s[0..1];
    let s2 = &s[0..2];
    dbg!(s1, s2);
}