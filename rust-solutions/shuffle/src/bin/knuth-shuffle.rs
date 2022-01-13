extern crate rand;

use rand::Rng;

// uses knuth shuffle
fn shuffle<T>(ns: &[T]) -> Vec<T>
where T: Copy
{
    let mut v = ns.to_vec();
    let mut rng = rand::thread_rng();
    for i in (0..v.len()).rev() {
        let j: usize = rng.gen_range(0, i + 1);
        v.swap(i, j);
    }
    v
}

fn main() {
    let v1 = vec![1,2,3,4];
    println!("shuf={:?}", shuffle(&v1));
    println!("shuf={:?}", shuffle(&v1));
    println!("shuf={:?}", shuffle(&v1));
    println!("shuf={:?}", shuffle(&v1));
}