extern crate rand;
use rand::Rng;

pub fn sort(ls: &mut Vec<i32>) {
    let mut sorted = false;
    while !sorted {
        let mut swap_count = 0;
        for ci in 0..ls.len() {
            let ni = ci + 1;
            if ni < ls.len() && ls[ci] > ls[ni] {
                ls.swap(ci, ni);
                swap_count += 1;
            }
        }
        if swap_count == 0 {
            sorted = true;
        }
    }
}

pub fn gen_rand_vec(size: usize, min: i32, max: i32)  -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut rs = Vec::with_capacity(size);
    for _i in 0..size {
        rs.push( rng.gen_range(min, max));
    }
    rs
}

fn main() {}

#[cfg(test)]
mod tests {

    use super::gen_rand_vec;
    use super::sort;

    #[test]
    fn bubble_sort_ints() {
        let mut ints = gen_rand_vec(100, 0, 100);
        sort(&mut ints);
        assert!( *ints.first().unwrap() <= *ints.last().unwrap())
    }
}